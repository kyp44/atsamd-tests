//! A test of the RTC-based RTIC monotonic that just prints out subsequent RTC count values.
#![no_std]
#![no_main]

use embedded_graphics::prelude::*;
use hal::prelude::*;
use hal::rtic_time::Monotonic;
use shared_pygamer::prelude::*;

const NUM_SAMPLES: usize = 100;

fn wait_for_count_change() -> <Mono as Monotonic>::Instant {
    let mut last_count = Mono::now();

    loop {
        let count = Mono::now();

        if count != last_count {
            break count;
        }

        last_count = count;
    }
}

#[rtic::app(device = pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;
    use bsp::RedLed;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        red_led: RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut pkg = SetupPackage::new(cx.device, cx.core);
        let rtc = pkg.setup_rtc_clock().unwrap();

        // Display selected monotonic and clock
        display_monotonic_info(&mut pkg.display);

        // Start the monotonic
        Mono::general_start(pkg.delay.free(), rtc);

        // Show the count sequence
        let mut counts = [0; NUM_SAMPLES];

        // The the array with monotonic counts
        counts[0] = Mono::now().ticks();
        for i in 1..NUM_SAMPLES {
            counts[i] = wait_for_count_change().ticks();
        }

        let style = pkg.display.display_text_style(Point::zero());
        write!(DisplayWriter::new(&mut pkg.display, style), "{counts:?}").unwrap();

        (
            Shared {},
            Local {
                red_led: pkg.red_led,
            },
        )
    }

    #[idle(local = [red_led])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
            cx.local.red_led.toggle().unwrap();
        }
    }
}
