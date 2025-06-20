use crate::{
    display::{Display, DisplayTextStyle, DisplayWriter},
    monotonic::Mono,
};
use atsamd_hal::prelude::*;
use core::fmt::Write;
use embedded_graphics::prelude::*;
use embedded_graphics::{primitives, text};
use rtic::Mutex;

struct Patch {
    rectangle: primitives::Rectangle,
}
impl Patch {
    pub fn new<const PATCH_SIZE: u32>(screen_width: u32, position: u32) -> Self {
        let row_len: u32 = screen_width / PATCH_SIZE;

        Self {
            rectangle: primitives::Rectangle::new(
                Point::new(
                    ((position % row_len) * PATCH_SIZE) as i32,
                    ((position / row_len) * PATCH_SIZE) as i32,
                ),
                Size::new(PATCH_SIZE, PATCH_SIZE),
            ),
        }
    }

    pub fn draw<D: Display>(&self, display: &mut D, color: D::Color)
    where
        D::Error: core::fmt::Debug,
    {
        self.rectangle
            .into_styled(primitives::PrimitiveStyle::with_fill(color))
            .draw(display)
            .unwrap();
        display.flush();
    }
}

pub async fn test_task<const PATCH_SIZE: u32, D: Display, M: Mutex<T = D>>(
    colors: &[D::Color],
    mut display: M,
    position: u32,
    delay_ms: u32,
) -> !
where
    D::Error: core::fmt::Debug,
{
    let patch = display.lock(|disp| Patch::new::<PATCH_SIZE>(disp.size().width, position));

    for color in colors.iter().cycle() {
        display.lock(|disp| {
            patch.draw(disp, *color);
        });

        Mono::delay_ms(delay_ms).await;
    }

    unreachable!();
}

pub async fn clock_task<D: Display, M: Mutex<T = D>>(mut display: M) -> ! {
    loop {
        display.lock(|d| {
            let style = DisplayTextStyle::new(
                Point::new(0, 64),
                None,
                D::character_style(),
                text::TextStyleBuilder::new()
                    .baseline(text::Baseline::Bottom)
                    .build(),
            );

            write!(DisplayWriter::new(d, style), "0x{:X}", Mono::now().ticks()).unwrap();
        });

        Mono::delay_ms(500).await;
    }
}
