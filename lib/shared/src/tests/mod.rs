use crate::{Input, display::Display, screens::ScreensGen};
use core::fmt::Write;

#[cfg(any(feature = "rtic-metro", feature = "rtic-pygamer"))]
pub mod async_stress;
mod delay_ns;
mod rtc;

impl<D: Display, I: Input> ScreensGen<D, I>
where
    D::Error: core::fmt::Debug,
{
    pub fn test_complete(mut self) -> ! {
        let mut writer = self.new_screen();

        writeln!(writer, "The test is complete.\n\nReset to run again.").unwrap();
        writer.flush();
        loop {
            cortex_m::asm::wfi();
        }
    }
}
