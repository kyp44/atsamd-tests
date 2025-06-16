use crate::hal::prelude::*;
use atsamd_hal::{fugit::ExtU64, rtc::rtic::rtc_clock, rtc_monotonic};
use core::fmt::Write;
use core::future::Future;
use embedded_graphics::mono_font;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text;

use crate::display::{DisplayTextStyle, DisplayWriter};
use crate::BACKGROUND_COLOR;
use crate::FONT;

#[cfg(not(any(feature = "clock1k", feature = "clock32k")))]
compile_error!("A clock feature must be specified");

#[cfg(feature = "clock1k")]
type _ClockRate = rtc_clock::Clock1k;
#[cfg(feature = "clock32k")]
type _ClockRate = rtc_clock::Clock32k;

#[cfg(feature = "systick")]
rtic_monotonics::systick_monotonic!(Mono, 200);
#[cfg(not(feature = "systick"))]
rtc_monotonic!(Mono, _ClockRate);

#[cfg(any(feature = "systick"))]
impl Mono {
    #[inline]
    pub fn delay_ms(delay: u32) -> impl Future<Output = ()> {
        Self::delay(delay.millis())
    }
}

#[cfg(not(any(feature = "systick")))]
impl Mono {
    #[inline]
    pub fn delay_ms(delay: u32) -> impl Future<Output = ()> {
        Self::delay(u64::from(delay).millis())
    }
}

impl Mono {
    pub fn general_start(_syst: pac::SYST, _rtc: pac::Rtc) {
        #[cfg(feature = "systick")]
        Mono::start(_syst, 120_000_000);
        #[cfg(not(feature = "systick"))]
        Mono::start(_rtc);
    }
}

pub fn display_monotonic_info(display: &mut DisplayDriver) {
    let style = DisplayTextStyle::new(
        Point::new(160 - FONT.character_size.width as i32 * 9, 128),
        None,
        // Use inverted colors here
        mono_font::MonoTextStyleBuilder::new()
            .font(&crate::FONT)
            .text_color(BACKGROUND_COLOR)
            .background_color(Rgb565::BLACK)
            .build(),
        text::TextStyleBuilder::new()
            .baseline(text::Baseline::Bottom)
            .build(),
    );

    let mut writer = DisplayWriter::new(display, &style);

    #[cfg(feature = "systick")]
    write!(writer, "systick  ").unwrap();
    #[cfg(not(feature = "systick"))]
    write!(writer, "mode0 ").unwrap();

    #[cfg(feature = "clock1k")]
    write!(writer, " 1k").unwrap();
    #[cfg(feature = "clock32k")]
    write!(writer, "32k").unwrap();
}
