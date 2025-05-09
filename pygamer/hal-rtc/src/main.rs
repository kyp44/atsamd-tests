#![no_std]
#![no_main]

use hal::rtc::Rtc;
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    let mut pkg = setup(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    block_on(tests::hal_rtc(
        Screens::new(pkg.display, pkg.buttons),
        Rtc::count32_mode(pkg.rtc, RTC_CLOCK_RATE, &mut pkg.mclk),
    ));
}
