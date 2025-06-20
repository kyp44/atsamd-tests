#[cfg(any(feature = "rtic"))]
pub mod async_stress {
    use crate::DisplayDriver;
    use embedded_graphics::pixelcolor::BinaryColor;
    use rtic::Mutex;
    use shared::prelude::*;
    use shared::tests::async_stress;

    const COLORS: &[BinaryColor] = &[BinaryColor::On, BinaryColor::Off];

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
