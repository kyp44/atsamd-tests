//! Program to test the RTC-based embassy time driver.
//!
//! Refer to `atsamd-hal` [PR 825](https://github.com/atsamd-rs/atsamd/pull/825).
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use hal::prelude::*;
use shared_metro::prelude::*;
use tasks::test_task;

mod tasks;

const BASE_PERIOD_MS: u32 = 1000;

#[rtic::app(device = pac, dispatchers = [TCC0])]
mod app {
    use super::*;
    use bsp::RedLed;

    #[shared]
    struct Shared {
        display: DisplayDriver,
    }

    #[local]
    struct Local {
        red_led: RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut pkg = SetupPackage::new(cx.device, cx.core);
        let rtc = pkg.setup_rtc_clock().unwrap();

        // Start the monotonic
        Mono::general_start(pkg.delay.free(), rtc);

        // Display selected monotonic and clock
        display_monotonic_info(&mut pkg.display);

        #[cfg(feature = "neopixels")]
        test_neopixels::spawn().ok().unwrap();
        test_1::spawn().ok().unwrap();

        (
            Shared {
                display: pkg.display,
            },
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
