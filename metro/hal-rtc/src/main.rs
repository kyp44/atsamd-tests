#![no_std]
#![no_main]

use hal::rtc::Rtc;
use shared_metro::prelude::*;

#[entry]
fn main() -> ! {
    // Setup stuff
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );
    pkg.setup_rtc_clock();

    let screens = Screens::new(pkg.display, pkg.buttons);
    let rtc = Rtc::count32_mode(pkg.rtc, RTC_CLOCK_RATE, &mut pkg.pm);

    tests::hal_rtc(screens, rtc);

    /* tests::hal_rtc(
        Screens::new(pkg.display, pkg.buttons),
        Rtc::count32_mode(pkg.rtc, RTC_CLOCK_RATE, &mut pkg.pm),
    ); */
}
