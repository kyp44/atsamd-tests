use crate::hal::{
    self, gpio,
    sercom::{self, i2c},
};
use derive_more::From;
use embedded_graphics::{mono_font, pixelcolor::BinaryColor, prelude::*};
use sh1107::prelude::*;
use shared::prelude::*;

hal::bind_interrupts!(pub struct DisplayIrq {
    SERCOM3 => i2c::InterruptHandler<pac::Sercom3>;
});

type RawDisplayDriver = GraphicsMode<
    I2cInterface<
        i2c::I2cFuture<
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

    async fn flush(&mut self) {
        self.0.flush().await.unwrap();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use bsp::Pins;
    use hal::prelude::*;

    /* let mut pkg = unsafe {
        block_on(setup(
            pac::Peripherals::steal(),
            pac::CorePeripherals::steal(),
        ))
    };
    cortex_m::interrupt::disable();
    let mut delay = hal::delay::Delay::new(pkg.syst, &mut pkg.clocks);
    loop {
        let _ = pkg.red_led.toggle();
        DelayNs::delay_ms(&mut delay, 200);
    }*/
    //panic_display(pkg.display, info);

    let mut peripherals = unsafe { pac::Peripherals::steal() };
    let _core = unsafe { pac::CorePeripherals::steal() };
    let mut clocks = hal::clock::GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port);

    // Setup the display
    let i2c = bsp::i2c_master(
        &mut clocks,
        400.kHz(),
        peripherals.sercom3,
        &mut peripherals.pm,
        pins.sda,
        pins.scl,
    )
    .into_future(DisplayIrq);

    let mut disp: GraphicsMode<_> = sh1107::Builder::new()
        .with_size(DisplaySize::Display64x128)
        .with_rotation(DisplayRotation::Rotate90)
        .connect_i2c(i2c)
        .into();

    let _ = block_on(disp.init());
    let _ = block_on(disp.flush());
    panic_display(DisplayDriver(disp), info);
}
