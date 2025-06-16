//! Program to test HAL items for which `embedded_hal::delay::DelayNs` is implemented.
//!
//! Refer to `atsamd-hal` [PR 880](https://github.com/atsamd-rs/atsamd/pull/880).
#![no_std]
#![no_main]

use hal::{rtc::Rtc, timer::TimerCounter};
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );
    let rtc = pkg.setup_rtc_clock().unwrap();

    // Setup the timer
    let gclk = pkg.clocks.gclk0();
    let tc4_tc5_clock = pkg.clocks.tc4_tc5(&gclk).unwrap();
    let timer = TimerCounter::tc4_(&tc4_tc5_clock, pkg.tc4, &mut pkg.mclk);

    // Setup the RTC
    let rtc = Rtc::count32_mode(rtc, RTC_CLOCK_RATE, &mut pkg.mclk);

    // Run the test
    Screens::new(pkg.display, pkg.buttons).delay_ns_test(pkg.delay, timer, rtc);
}
