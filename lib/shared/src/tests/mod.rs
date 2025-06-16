use crate::{display::Display, screens::ScreensGen, Input};
use core::fmt::Write;

mod delay_ns;
// TODO: Once the PR gets merged, this will not longer need to be feature-gated
#[cfg(feature = "rtc-test")]
mod rtc;

impl<D: Display, I: Input> ScreensGen<D, I>
where
    D::Error: core::fmt::Debug,
{
    fn test_complete(mut self) -> ! {
        let mut writer = self.new_screen();

        writeln!(writer, "The test is complete.\n\nReset to run again.").unwrap();
        writer.flush();
        loop {
            cortex_m::asm::wfi();
        }
    }
}
