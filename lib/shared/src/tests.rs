use crate::{
    display::{Display, DisplayWriter},
    screens::ScreensGen,
    Input,
};
use atsamd_hal::{
    ehal::delay::DelayNs,
    prelude::_embedded_hal_timer_CountDown as Countdown,
    prelude::*,
    rtc::{ClockMode, Count32Mode, Datetime, Rtc},
};
use core::fmt::Write;
use nb::block;

const DELAY_SECS: u32 = 3;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct FormattedDatetime(pub Datetime);
impl core::fmt::Debug for FormattedDatetime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "20{:02}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.0.year, self.0.month, self.0.day, self.0.hours, self.0.minutes, self.0.seconds
        )
    }
}

trait RtcExt: Sized {
    type Count: Copy + Eq + Default + core::fmt::Debug;
    const NUM_SAMPLES: usize;

    fn count(&self) -> Self::Count;

    fn wait_for_count_change(&self) -> Self::Count {
        let mut last_count = self.count();

        loop {
            let count = self.count();

            if count != last_count {
                break count;
            }

            last_count = count;
        }
    }

    fn get_counts(&self) -> [Self::Count; Self::NUM_SAMPLES] {
        let mut counts = [Default::default(); Self::NUM_SAMPLES];
        counts[0] = self.count();
        for i in 1..Self::NUM_SAMPLES {
            counts[i] = self.wait_for_count_change();
        }

        counts
    }
}

impl RtcExt for Rtc<Count32Mode> {
    type Count = u32;
    const NUM_SAMPLES: usize = 8;

    fn count(&self) -> Self::Count {
        self.count32()
    }
}

impl RtcExt for Rtc<ClockMode> {
    type Count = FormattedDatetime;
    const NUM_SAMPLES: usize = 5;

    fn count(&self) -> Self::Count {
        FormattedDatetime(self.current_time())
    }
}

impl<D: Display, I: Input> ScreensGen<D, I>
where
    D::Error: core::fmt::Debug,
{
    fn show_counts<R: RtcExt>(&mut self, rtc: &mut R, msg: &str) -> DisplayWriter<D>
    where
        [(); R::NUM_SAMPLES]:,
    {
        let mut writer = self.new_screen();
        write!(writer, "Gathering counter samples...").unwrap();
        writer.flush();
        let counts = rtc.get_counts();
        let mut writer = self.new_screen();
        writeln!(writer, "{msg}:").unwrap();
        for count in counts {
            writeln!(writer, "{:?}", count).unwrap();
        }

        self.wait_for_button()
    }

    fn date_rollover(
        &mut self,
        rtc: &mut Rtc<ClockMode>,
        year: u8,
        month: u8,
        day: u8,
        msg: &str,
    ) -> DisplayWriter<D> {
        rtc.set_time(Datetime {
            seconds: 57,
            minutes: 59,
            hours: 23,
            day,
            month,
            year,
        });
        self.show_counts(rtc, msg)
    }
}

pub fn hal_rtc<D: Display, I: Input>(mut screens: ScreensGen<D, I>, rtc: Rtc<Count32Mode>) -> !
where
    D::Error: core::fmt::Debug,
{
    let mut rtc_count = Some(rtc);

    loop {
        let mut rtc = rtc_count.take().unwrap();

        // Count 32 tests
        let _ = screens.show_counts(&mut rtc, "Basic counter test");
        rtc.set_count32(1_000);
        let _ = screens.show_counts(&mut rtc, "Set counter test");
        // This should set set the prescalar to 1024
        rtc.reset_and_compute_prescaler(1u32.hours());
        let mut writer = screens.show_counts(&mut rtc, "Reset with prescalar");

        // Periodic countdown timer test with the ehal 0.2 `Countdown` trait
        writeln!(
            writer,
            "Starting periodic RTC timer for {DELAY_SECS} seconds..."
        )
        .unwrap();
        writer.flush();
        Countdown::start(&mut rtc, DELAY_SECS.secs());
        block!(Countdown::wait(&mut rtc)).unwrap();
        writeln!(writer, "Waiting another {DELAY_SECS} seconds...").unwrap();
        writer.flush();
        block!(Countdown::wait(&mut rtc)).unwrap();
        writeln!(writer, "One more delay of {DELAY_SECS} seconds...").unwrap();
        writer.flush();
        block!(Countdown::wait(&mut rtc)).unwrap();
        writeln!(writer, "Periodic `Countdown` test complete!").unwrap();

        // Delay with the ehal `DelayNs` trait
        let mut writer = screens.wait_for_button();
        writeln!(writer, "Delaying for {DELAY_SECS} seconds...").unwrap();
        writer.flush();
        rtc.delay_ms(DELAY_SECS as u32 * 1000);
        writeln!(writer, "Delaying another {DELAY_SECS} seconds...").unwrap();
        writer.flush();
        rtc.delay_ms(DELAY_SECS as u32 * 1000);
        writeln!(writer, "`DelayNs` test complete!").unwrap();
        let _ = screens.wait_for_button();

        // Now test clock mode
        let mut rtc = rtc.into_clock_mode();
        screens.show_counts(&mut rtc, "Basic clock mode test");
        screens.date_rollover(&mut rtc, 24, 2, 28, "Happy leap day");
        screens.date_rollover(&mut rtc, 25, 2, 28, "No leap day here");
        screens.date_rollover(&mut rtc, 29, 12, 31, "Happy new year");

        rtc_count = Some(rtc.into_count32_mode());
    }
}
