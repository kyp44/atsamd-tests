#![no_std]
#![no_main]

use hal::rtc::Rtc;
use shared_metro::prelude::*;

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    // Setup stuff
    let mut pkg = setup(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    )
    .await;

    tests::hal_rtc(
        Screens::new(pkg.display, pkg.buttons),
        Rtc::count32_mode(pkg.rtc, RTC_CLOCK_RATE, &mut pkg.pm),
    ).await;
}
