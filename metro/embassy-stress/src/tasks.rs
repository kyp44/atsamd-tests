use core::ops::DerefMut;
use embassy_time::Timer;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives};
use shared_metro::prelude::*;

use super::SharedDisplay;

const COLORS: &[BinaryColor] = &[BinaryColor::On, BinaryColor::Off];

pub async fn test_task(display: &'static SharedDisplay, position: u32, delay_ms: u64) -> ! {
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
        let mut guard = display.lock().await;
        let disp = guard.deref_mut();

        rectangle
            .into_styled(primitives::PrimitiveStyle::with_fill(*color))
            .draw(disp)
            .unwrap();
        disp.flush();
        drop(guard);

        Timer::after_millis(delay_ms).await;
    }

    unreachable!();
}
