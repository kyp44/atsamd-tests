use derive_more::From;
use embedded_graphics::{mono_font, pixelcolor::Rgb565, prelude::*};
use shared::prelude::*;

#[derive(From)]
pub struct DisplayDriver(bsp::DisplayDriver);
impl OriginDimensions for DisplayDriver {
    fn size(&self) -> Size {
        self.0.size()
    }
}
impl DrawTarget for DisplayDriver {
    type Color = <bsp::DisplayDriver as DrawTarget>::Color;
    type Error = <bsp::DisplayDriver as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels)
    }
}

impl Display for DisplayDriver {
    const FONT: mono_font::MonoFont<'static> = mono_font::ascii::FONT_5X8;
    const BACKGROUND_COLOR: Self::Color = Rgb565::WHITE;
    const TEXT_COLOR: Self::Color = Rgb565::BLACK;
    const PANIC_BACKGROUND_COLOR: Self::Color = Rgb565::RED;
    const PANIC_TEXT_COLOR: Self::Color = Rgb565::BLACK;

    fn flush(&mut self) {
        // This display has no need to flush
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let pkg = unsafe {
        crate::SetupPackage::new(pac::Peripherals::steal(), pac::CorePeripherals::steal())
    };

    panic_display(pkg.display, info);
}
