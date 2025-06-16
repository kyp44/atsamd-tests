use crate::display::{Display, DisplayTextStyle, DisplayWriter};
use crate::prelude::pac;
use atsamd_hal::prelude::*;
#[cfg(any(feature = "clock1k", feature = "clock32k"))]
use atsamd_hal::{fugit::ExtU64, rtc::rtic::rtc_clock, rtc_monotonic};
use core::fmt::Write;
use core::future::Future;
use embedded_graphics::{mono_font, prelude::*, text};
#[cfg(feature = "systick")]
use rtic_monotonics::Monotonic;

#[cfg(not(any(feature = "clock1k", feature = "clock32k", feature = "systick")))]
compile_error!("A clock rate feature or systick must be specified");

#[cfg(not(any(feature = "metro", feature = "pygamer")))]
compile_error!("A platform feature must be specified");

#[cfg(feature = "clock1k")]
type _ClockRate = rtc_clock::Clock1k;
#[cfg(feature = "clock32k")]
type _ClockRate = rtc_clock::Clock32k;

#[cfg(feature = "systick")]
rtic_monotonics::systick_monotonic!(Mono, 200);
#[cfg(not(feature = "systick"))]
rtc_monotonic!(Mono, _ClockRate);

#[cfg(feature = "systick")]
impl Mono {
    #[inline]
    pub fn delay_ms(delay: u32) -> impl Future<Output = ()> {
        Self::delay(delay.millis())
    }
}

#[cfg(not(feature = "systick"))]
impl Mono {
    #[inline]
    pub fn delay_ms(delay: u32) -> impl Future<Output = ()> {
        Self::delay(u64::from(delay).millis())
    }
}

impl Mono {
    pub fn general_start(_syst: crate::prelude::pac::SYST, _rtc: pac::Rtc) {
        #[cfg(feature = "systick")]
        Mono::start(_syst, 120_000_000);
        #[cfg(not(feature = "systick"))]
        Mono::start(_rtc);
    }
}

pub fn display_monotonic_info<D: Display>(display: &mut D) {
    let style = DisplayTextStyle::new(
        Point::new(
            (display.size().width - D::FONT.character_size.width * 9) as i32,
            display.size().height as i32,
        ),
        None,
        // Use inverted colors here
        mono_font::MonoTextStyleBuilder::new()
            .font(&D::FONT)
            .text_color(D::BACKGROUND_COLOR)
            .background_color(D::TEXT_COLOR)
            .build(),
        text::TextStyleBuilder::new()
            .baseline(text::Baseline::Bottom)
            .build(),
    );

    let mut writer = DisplayWriter::new(display, style);

    #[cfg(feature = "systick")]
    write!(writer, "systick  ").unwrap();
    #[cfg(not(feature = "systick"))]
    write!(writer, "mode0 ").unwrap();

    #[cfg(feature = "clock1k")]
    write!(writer, " 1k").unwrap();
    #[cfg(feature = "clock32k")]
    write!(writer, "32k").unwrap();
}
