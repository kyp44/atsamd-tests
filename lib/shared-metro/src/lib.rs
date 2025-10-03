#![no_std]

use bsp::{Pins, RedLed};
pub use display::DisplayDriver;
use hal::{
    clock::{ClockGenId, ClockSource, GenericClockController, RtcClock},
    delay::Delay,
    gpio::{PA24, PA25, Pin, Reset},
};
pub use input::{Button, Buttons};
use shared::prelude::*;

mod display;
mod input;
pub mod tests;

pub mod prelude {
    pub use super::{Buttons, DisplayDriver, Screens, SetupPackage};
    pub use bsp::entry;
    pub use shared::prelude::*;
}

pub type Screens = ScreensGen<DisplayDriver, Buttons>;

pub struct SetupPackage {
    pub delay: Delay,
    pub display: DisplayDriver,
    pub buttons: Buttons,
    pub red_led: RedLed,
    rtc: Option<pac::Rtc>,
    pub tc4: pac::Tc4,
    pub clocks: GenericClockController,
    pub pm: pac::Pm,
    pub usb: pac::Usb,
    pub usb_dm: Pin<PA24, Reset>,
    pub usb_dp: Pin<PA25, Reset>,
    pub nvic: pac::NVIC,
}
impl SetupPackage {
    pub fn new(mut peripherals: pac::Peripherals, core: pac::CorePeripherals) -> Self {
        use hal::fugit::RateExtU32;
        use sh1107::prelude::*;

        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );
        let pins = Pins::new(peripherals.port);

        // Setup the delay
        let delay = Delay::new(core.SYST, &mut clocks);

        // Setup the display
        let i2c = bsp::i2c_master(
            &mut clocks,
            400.kHz(),
            peripherals.sercom3,
            &mut peripherals.pm,
            pins.sda,
            pins.scl,
        );
        let mut display: GraphicsMode<_> = sh1107::Builder::new()
            .with_size(DisplaySize::Display64x128)
            .with_rotation(DisplayRotation::Rotate90)
            .connect_i2c(i2c)
            .into();

        display.init().unwrap();
        display.clear();
        display.flush().unwrap();

        Self {
            delay,
            display: display.into(),
            buttons: Buttons {
                button_a: pins.d9.into_pull_up_input().into(),
                button_b: pins.d6.into_pull_up_input().into(),
                button_c: pins.d5.into_pull_up_input().into(),
            },
            red_led: pins.d13.into(),
            rtc: Some(peripherals.rtc),
            tc4: peripherals.tc4,
            clocks,
            pm: peripherals.pm,
            usb: peripherals.usb,
            usb_dm: pins.usb_dm,
            usb_dp: pins.usb_dp,
            nvic: core.NVIC,
        }
    }

    pub fn setup_rtc_clock(&mut self) -> Option<(pac::Rtc, RtcClock)> {
        self.rtc.take().map(|rtc| {
            #[cfg(not(feature = "clock32k"))]
            let divider = 32;
            #[cfg(feature = "clock32k")]
            let divider = 1;

            let rtc_clock_src = self
                .clocks
                .configure_gclk_divider_and_source(
                    ClockGenId::Gclk3,
                    divider,
                    ClockSource::Xosc32k,
                    false,
                )
                .unwrap();

            self.clocks.configure_standby(ClockGenId::Gclk3, true);
            (rtc, self.clocks.rtc(&rtc_clock_src).unwrap())
        })
    }
}
