#![no_std]
#![feature(let_chains)]
#![feature(iter_advance_by)]
#![feature(generic_const_exprs)]

use core::future::Future;

#[cfg(all(
    feature = "rtic",
    not(any(feature = "clock1k", feature = "clock32k", feature = "systick"))
))]
compile_error!("When using RTIC a clock rate or systick must be specified");

#[cfg(all(feature = "clock1k", feature = "clock32k"))]
compile_error!("Cannot select both clocks.");

#[cfg(any(feature = "clock1k", feature = "clock32k"))]
use atsamd_hal::time::Hertz;

mod display;
#[cfg(any(feature = "clock1k", feature = "clock32k", feature = "systick"))]
mod monotonic;
mod screens;
#[cfg(any(feature = "metro", feature = "pygamer"))]
pub mod tests;

#[cfg(feature = "clock1k")]
pub const RTC_CLOCK_RATE: Hertz = Hertz::from_raw(1024);
#[cfg(feature = "clock32k")]
pub const RTC_CLOCK_RATE: Hertz = Hertz::from_raw(32768);

pub mod prelude {
    pub use super::display::*;
    #[cfg(any(feature = "clock1k", feature = "clock32k", feature = "systick"))]
    pub use super::monotonic::{display_monotonic_info, Mono};
    pub use super::Input;
    #[cfg(any(feature = "clock1k", feature = "clock32k"))]
    pub use super::RTC_CLOCK_RATE;
    pub use super::{block_on, screens::ScreensGen};
    #[cfg(feature = "metro")]
    pub use metro_m0::{self as bsp, hal, pac};
    #[cfg(feature = "pygamer")]
    pub use pygamer::{self as bsp, hal, pac};
    #[cfg(feature = "systick")]
    pub use rtic_monotonics::Monotonic;

    // Re-exports
    pub use core::fmt::Write;
    pub use cortex_m;
    pub use embedded_graphics;
    #[cfg(any(feature = "metro", feature = "pygamer"))]
    pub use pac::{CorePeripherals, Peripherals};
    #[cfg(feature = "rtic")]
    pub use rtic;
}
pub trait Input {
    fn wait_for_button(&mut self);
}

pub fn block_on<F: Future>(mut future: F) -> F::Output {
    use core::pin::Pin;
    use core::task::{Context, Poll};
    use futures_task::noop_waker;

    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut pinned = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // Wait for an interrupt
                //cortex_m::asm::wfi();
            }
        }
    }
}
