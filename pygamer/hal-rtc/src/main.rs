#![no_std]
#![no_main]

use hal::rtc::Rtc;
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );
    let rtc = pkg.setup_rtc_clock().unwrap();

    tests::hal_rtc(
        Screens::new(pkg.display, pkg.buttons),
        Rtc::count32_mode(rtc, RTC_CLOCK_RATE, &mut pkg.mclk),
    );
}
