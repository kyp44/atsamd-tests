//! Tests HAL items for which `embedded_hal::delay::DelayNs` is implemented.
#![no_std]
#![no_main]

use core::fmt::Write;
use hal::ehal::delay::DelayNs;
use hal::{rtc::Rtc, timer::TimerCounter};
use shared::prelude::*;

const DELAY_SECS: u32 = 3;
const DELAY_MILLIS: u32 = DELAY_SECS * 1000;

fn test_delay<D: DelayNs>(screens: &mut Screens, name: &str, delay: &mut D) {
    let mut writer = screens.new_screen();

    writeln!(writer, "Delaying {DELAY_SECS} for `{name}`...").unwrap();
    delay.delay_ms(DELAY_MILLIS);
    writeln!(writer, "Waiting another {DELAY_SECS} seconds...").unwrap();
    delay.delay_ms(DELAY_MILLIS);
    writeln!(writer, "One more delay of {DELAY_SECS} seconds...").unwrap();
    delay.delay_ms(DELAY_MILLIS);
    writeln!(writer, "\n`DelayNs` test for `{name}` complete!").unwrap();
    screens.wait_for_button();
}

#[entry]
fn main() -> ! {
    let mut pkg = setup(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    // Setup up the execution package
    let mut screens = Screens::new(
        pkg.display,
        pkg.buttons,
        BACKGROUND_COLOR,
        display_text_style(),
    );

    // Setup the timer
    let gclk = pkg.clocks.gclk0();
    let tc4_tc5_clock = pkg.clocks.tc4_tc5(&gclk).unwrap();
    let mut timer = TimerCounter::tc4_(&tc4_tc5_clock, pkg.tc4, &mut pkg.mclk);

    // Setup the RTC
    let mut rtc = Rtc::count32_mode(pkg.rtc, RTC_CLOCK_RATE, &mut pkg.mclk);

    let mut writer = screens.new_screen();
    writeln!(writer, "Press A to start the tests").unwrap();
    screens.wait_for_button();

    loop {
        test_delay(&mut screens, "Delay", &mut pkg.delay);
        test_delay(&mut screens, "TimerCounter", &mut timer);
        //exec.test_delay("Rtc", &mut rtc);
    }
}
