use crate::{display::Display, screens::ScreensGen, Input};
use atsamd_hal::{
    delay::Delay,
    ehal::delay::DelayNs,
    rtc::{Count32Mode, Rtc},
    timer::{Count16, TimerCounter},
};
use core::fmt::Write;

const DELAY_SECS: u32 = 3;
const DELAY_MILLIS: u32 = DELAY_SECS * 1000;

impl<D: Display, I: Input> ScreensGen<D, I>
where
    D::Error: core::fmt::Debug,
{
    fn test_delay<E: DelayNs>(&mut self, name: &str, delay: &mut E) {
        let mut writer = self.new_screen();

        writeln!(writer, "Delaying {DELAY_SECS} for `{name}`...").unwrap();
        writer.flush();
        delay.delay_ms(DELAY_MILLIS);
        writeln!(writer, "Waiting another {DELAY_SECS} seconds...").unwrap();
        writer.flush();
        delay.delay_ms(DELAY_MILLIS);
        writeln!(writer, "One more delay of {DELAY_SECS} seconds...").unwrap();
        writer.flush();
        delay.delay_ms(DELAY_MILLIS);
        writeln!(writer, "\n`DelayNs` test for `{name}` complete!").unwrap();
        self.wait_for_button();
    }

    pub fn hal_delay_ns_test<TC: Count16>(
        mut self,
        mut delay: Delay,
        mut timer: TimerCounter<TC>,
        mut _rtc: Rtc<Count32Mode>,
    ) -> !
    where
        D::Error: core::fmt::Debug,
    {
        let mut writer = self.new_screen();
        writeln!(writer, "Press button to start the tests").unwrap();
        self.wait_for_button();

        loop {
            self.test_delay("Delay", &mut delay);
            self.test_delay("TimerCounter", &mut timer);
            // TODO: This depends on the RTC rework PR being merged: https://github.com/atsamd-rs/atsamd/pull/845
            //test_delay(&mut screens, "Rtc", &mut rtc);
        }
    }
}
