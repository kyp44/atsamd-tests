use crate::hal::{
    gpio,
    sercom::{self, i2c},
};
use crate::SetupPackage;
use derive_more::From;
use embedded_graphics::{mono_font, pixelcolor::BinaryColor, prelude::*};
use sh1107::prelude::*;
use shared::prelude::*;

type RawDisplayDriver = GraphicsMode<
    I2cInterface<
        i2c::I2c<
            i2c::Config<
                i2c::Pads<
                    sercom::Sercom3,
                    gpio::Pin<gpio::PA22, gpio::Alternate<gpio::pin::C>>,
                    gpio::Pin<gpio::PA23, gpio::Alternate<gpio::pin::C>>,
                >,
            >,
        >,
    >,
>;

#[derive(From)]
pub struct DisplayDriver(RawDisplayDriver);
impl DisplayDriver {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn free(self) -> RawDisplayDriver {
        self.0
    }
}
impl OriginDimensions for DisplayDriver {
    fn size(&self) -> Size {
        self.0.size()
    }
}
impl DrawTarget for DisplayDriver {
    type Color = <RawDisplayDriver as DrawTarget>::Color;
    type Error = <RawDisplayDriver as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels)
    }
}
impl Display for DisplayDriver {
    const FONT: mono_font::MonoFont<'static> = mono_font::ascii::FONT_4X6;
    const BACKGROUND_COLOR: Self::Color = BinaryColor::Off;
    const TEXT_COLOR: Self::Color = BinaryColor::On;
    const PANIC_BACKGROUND_COLOR: Self::Color = BinaryColor::On;
    const PANIC_TEXT_COLOR: Self::Color = BinaryColor::Off;

    fn flush(&mut self) {
        self.0.flush().unwrap();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let pkg =
        unsafe { SetupPackage::new(pac::Peripherals::steal(), pac::CorePeripherals::steal()) };
    panic_display(pkg.display, info);
}
