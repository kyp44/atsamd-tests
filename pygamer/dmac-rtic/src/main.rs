//! A basic test of the RTC-based RTIC monotonic.
#![no_std]
#![no_main]

use bsp::{Pins, RedLed};
use hal::clock::v2::{
    Enabled, Tokens,
    osculp32k::{OscUlp32k, OscUlp32kBase},
};
use hal::prelude::*;
use shared_pygamer::prelude::*;

struct OtherStuff {
    red_led: RedLed,
}
impl OtherStuff {
    fn new(
        rtc: pac::Rtc,
        port: pac::Port,
        osculp32k_base: Enabled<OscUlp32kBase>,
        tokens: Tokens,
    ) -> Self {
        // Setup the RTC clock at 32 kHz
        let (osculp32k, _) = OscUlp32k::enable(tokens.osculp32k.osculp32k, osculp32k_base);
        let (_, _) = hal::clock::v2::rtcosc::RtcOsc::enable(tokens.rtcosc, osculp32k);

        // Start the monotonic
        Mono::start(rtc);

        let pins = Pins::new(port).split();
        Self {
            red_led: pins.led_pin.into(),
        }
    }

    async fn do_other_stuff(&mut self) {
        Mono::delay_ms(500).await;
        self.red_led.toggle().unwrap();
    }
}

// ACTUAL EXAMPLE STARTS HERE
use atsamd_hal::dmac::*;

atsamd_hal::bind_multiple_interrupts!(struct Irqs {
    DMAC: [DMAC_0] => InterruptHandler;
});

#[rtic::app(device = pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        other_stuff: OtherStuff,
        channel: Channel<Ch0, ReadyFuture>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals = cx.device;
        let (_buses, clocks, tokens) = atsamd_hal::clock::v2::clock_system_at_reset(
            peripherals.oscctrl,
            peripherals.osc32kctrl,
            peripherals.gclk,
            peripherals.mclk,
            &mut peripherals.nvmctrl,
        );

        // Setup the DMA controller
        let mut dmac = DmaController::new(peripherals.dmac, clocks.ahbs.dmac).into_future(Irqs);

        // Get individual handles to DMA channels
        let channels = dmac.split();

        // Initialize DMA Channel 0
        let channel = channels.0.init(PriorityLevel::Lvl0);

        task::spawn().ok().unwrap();

        (
            Shared {},
            Local {
                other_stuff: OtherStuff::new(
                    peripherals.rtc,
                    peripherals.port,
                    clocks.osculp32k_base,
                    tokens,
                ),
                channel,
            },
        )
    }

    #[task(priority = 1, local=[other_stuff, channel])]
    async fn task(cx: task::Context) {
        const LENGTH: usize = 50;
        let mut source = [0xff; LENGTH];
        let mut dest = [0x00; LENGTH];

        // Just do a transfer repeatedly and then other stuff
        loop {
            cx.local
                .channel
                .transfer_future(
                    &mut source,
                    &mut dest,
                    TriggerSource::Disable,
                    TriggerAction::Block,
                )
                .await
                .unwrap();

            cx.local.other_stuff.do_other_stuff().await;
        }
    }
}
