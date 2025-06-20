use embedded_graphics::prelude::*;
use embedded_graphics::{mono_font, pixelcolor::BinaryColor, primitives, text};
use hal::prelude::*;
use rtic::Mutex;
use shared_metro::prelude::*;

const COLORS: &[BinaryColor] = &[BinaryColor::On, BinaryColor::Off];

pub async fn test_task<D: Mutex<T = DisplayDriver>>(
    mut display: D,
    position: u32,
    delay_ms: u32,
) -> ! {
    // TODO: This calculate can be abstracted out for re-use
    const PATCH_SIZE: u32 = 10;
    const ROW_LEN: u32 = 128 / PATCH_SIZE;

    let rectangle = primitives::Rectangle::new(
        Point::new(
            ((position % ROW_LEN) * PATCH_SIZE) as i32,
            ((position / ROW_LEN) * PATCH_SIZE) as i32,
        ),
        Size::new(PATCH_SIZE, PATCH_SIZE),
    );

    for color in COLORS.iter().cycle() {
        display.lock(|disp| {
            rectangle
                .into_styled(primitives::PrimitiveStyle::with_fill(*color))
                .draw(disp)
                .unwrap();
            disp.flush();
        });

        Mono::delay_ms(delay_ms).await;
    }

    unreachable!();
}

pub async fn clock_task<D: Mutex<T = DisplayDriver>>(mut display: D) -> ! {
    loop {
        let style = DisplayTextStyle::new(
            Point::new(0, 64),
            None,
            mono_font::MonoTextStyleBuilder::new()
                .font(&DisplayDriver::FONT)
                .text_color(BinaryColor::On)
                .background_color(BinaryColor::Off)
                .build(),
            text::TextStyleBuilder::new()
                .baseline(text::Baseline::Bottom)
                .build(),
        );

        display.lock(|d| {
            write!(DisplayWriter::new(d, style), "0x{:X}", Mono::now().ticks()).unwrap();
        });

        Mono::delay_ms(500).await;
    }
}
