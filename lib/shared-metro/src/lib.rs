#![no_std]

use bsp::{Pins, RedLed};
pub use display::{DisplayDriver, DisplayIrq};
use hal::clock::GenericClockController;
pub use input::{ButtonIrq, Buttons};
use shared::prelude::*;

mod display;
mod input;

pub mod prelude {
    pub use super::{setup, ButtonIrq, Buttons, DisplayDriver, DisplayIrq, Screens};
    pub use bsp::entry;
    pub use shared::prelude::*;
}

pub type Screens = ScreensGen<DisplayDriver, Buttons>;

pub struct SetupPackage {
    pub display: DisplayDriver,
    pub buttons: Buttons,
    pub red_led: RedLed,
    pub rtc: pac::Rtc,
    pub clocks: GenericClockController,
    pub pm: pac::Pm,
    pub syst: pac::SYST,
}

pub async fn setup(mut peripherals: pac::Peripherals, core: pac::CorePeripherals) -> SetupPackage {
    use hal::fugit::RateExtU32;
    use sh1107::prelude::*;

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port);

    // Setup the external interrupt controller and async buttons
    let gclk0 = clocks.gclk0();
    let eic_channels = hal::eic::Eic::new(
        &mut peripherals.pm,
        clocks.eic(&gclk0).unwrap(),
        peripherals.eic,
    )
    .into_future(ButtonIrq)
    .split();
    let mut button_a = eic_channels.7.with_pin(pins.d9.into_pull_up_interrupt());
    let mut button_b = eic_channels.4.with_pin(pins.d6.into_pull_up_interrupt());
    let mut button_c = eic_channels.15.with_pin(pins.d5.into_pull_up_interrupt());
    button_a.filter(true);
    button_b.filter(true);
    button_c.filter(true);

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

    let mut display: GraphicsMode<_> = sh1107::Builder::new()
        .with_size(DisplaySize::Display64x128)
        .with_rotation(DisplayRotation::Rotate90)
        .connect_i2c(i2c)
        .into();

    //display.init().await.unwrap();
    display.init().await.unwrap();
    display.clear();
    display.flush().await.unwrap();

    SetupPackage {
        display: display.into(),
        buttons: Buttons {
            button_a,
            button_b,
            button_c,
        },
        red_led: pins.d13.into(),
        rtc: peripherals.rtc,
        clocks,
        pm: peripherals.pm,
        syst: core.SYST,
    }
}
