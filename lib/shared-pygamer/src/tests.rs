#[cfg(any(feature = "rtic"))]
pub mod async_stress {
    use crate::DisplayDriver;
    use embedded_graphics::pixelcolor::Rgb565;
    use embedded_graphics::prelude::*;
    use rtic::Mutex;
    use shared::prelude::*;
    use shared::tests::async_stress;

    const COLORS: &[Rgb565] = &[
        Rgb565::RED,
        Rgb565::GREEN,
        Rgb565::BLUE,
        Rgb565::CYAN,
        Rgb565::MAGENTA,
        Rgb565::YELLOW,
        Rgb565::BLACK,
        Rgb565::CSS_PINK,
        Rgb565::CSS_ORANGE,
        Rgb565::CSS_PURPLE,
        Rgb565::WHITE,
        Rgb565::CSS_GRAY,
    ];

    #[inline]
    pub async fn test_task<M: Mutex<T = DisplayDriver>>(
        display: M,
        position: u32,
        delay_ms: u32,
    ) -> ! {
        async_stress::test_task::<10, _, _>(COLORS, display, position, delay_ms).await;
    }

    pub use async_stress::clock_task;
}
