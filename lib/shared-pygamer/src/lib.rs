#![no_std]

use bsp::{Pins, RedLed};
use display::DisplayDriver;
use embedded_graphics::prelude::*;
use hal::{clock::GenericClockController, delay::Delay};
use input::Buttons;
use shared::prelude::*;

mod display;
mod input;

pub mod prelude {
    #[cfg(feature = "neopixels")]
    pub use super::NeoPixelsDriver;
    pub use super::{display::DisplayDriver, input::Buttons, Screens, SetupPackage};
    pub use bsp::entry;
    pub use shared::prelude::*;
    #[cfg(feature = "neopixels")]
    pub use smart_leds::{SmartLedsWrite, RGB8};
}

#[cfg(feature = "neopixels")]
pub type NeoPixelsDriver = ws2812_spi::Ws2812<bsp::pins::NeopixelSpi>;

pub type Screens = ScreensGen<DisplayDriver, Buttons>;

pub struct SetupPackage {
    pub delay: Delay,
    pub display: DisplayDriver,
    pub buttons: Buttons,
    #[cfg(feature = "neopixels")]
    pub neopixels: NeoPixelsDriver,
    pub red_led: RedLed,
    pub tc4: pac::Tc4,
    rtc: Option<pac::Rtc>,
    pub clocks: GenericClockController,
    pub mclk: pac::Mclk,
    pub osc32kctrl: pac::Osc32kctrl,
}
impl SetupPackage {
    pub fn new(mut peripherals: pac::Peripherals, core: pac::CorePeripherals) -> Self {
        // NOTE: We would like to use the v2 of the clock module, but this is not yet integrated
        // into the rest of the HAL or the `pygamer` BSP. For example, the display `init` method
        // below requires clock v1 parameters.
        /* let (mut buses, clocks, tokens) = clock_system_at_reset(
            peripherals.oscctrl,
            peripherals.osc32kctrl,
            peripherals.gclk,
            peripherals.mclk,
            &mut peripherals.nvmctrl,
        ); */

        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.gclk,
            &mut peripherals.mclk,
            &mut peripherals.osc32kctrl,
            &mut peripherals.oscctrl,
            &mut peripherals.nvmctrl,
        );

        let pins = Pins::new(peripherals.port).split();
        let mut delay = Delay::new(core.SYST, &mut clocks);
        // Here is how this can be initialized using the clock v2 API instead
        //let mut delay = Delay::new_with_source(core.SYST, clocks.gclk0);

        // Initialize the display
        let (mut display, _backlight) = pins
            .display
            .init(
                &mut clocks,
                peripherals.sercom4,
                &mut peripherals.mclk,
                peripherals.tc2,
                &mut delay,
            )
            .unwrap();
        display.clear(DisplayDriver::BACKGROUND_COLOR).unwrap();

        #[cfg(feature = "neopixels")]
        let neopixels = pins.neopixel.init_spi(
            &mut clocks,
            // Unfortunately, the SPI driver requires a clock pin, even though it's not used by the
            // neopixels.
            pins.i2c.scl,
            peripherals.sercom2,
            &mut peripherals.mclk,
        );

        Self {
            delay,
            display: display.into(),
            buttons: pins.buttons.init().into(),
            #[cfg(feature = "neopixels")]
            neopixels,
            red_led: pins.led_pin.into(),
            tc4: peripherals.tc4,
            rtc: Some(peripherals.rtc),
            clocks,
            mclk: peripherals.mclk,
            osc32kctrl: peripherals.osc32kctrl,
        }
    }

    pub fn setup_rtc_clock(&mut self) -> Option<pac::Rtc> {
        // NOTE: Selecting the RTC clock requires the clocks v2 API on SAMx5x chips!
        #[cfg(feature = "clock1k")]
        self.osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp1k());
        #[cfg(feature = "clock32k")]
        self.osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp32k());

        self.rtc.take()
    }
}
