#![no_std]
#![no_main]

use bsp::{Pins, RedLed};
use hal::{
    clock::v2::{
        clock_system_at_reset,
        dpll::Dpll,
        gclk::{Gclk, GclkDiv16},
        pclk::Pclk,
    },
    pac::Peripherals,
    prelude::*,
};
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    // Tests the example clock tree setup from the v2 documentation
    let mut pac = Peripherals::take().unwrap();
    let (_buses, clocks, tokens) = clock_system_at_reset(
        pac.oscctrl,
        pac.osc32kctrl,
        pac.gclk,
        pac.mclk,
        &mut pac.nvmctrl,
    );
    let pins = Pins::new(pac.port).split();

    let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
    let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
    let (pclk_dpll0, _gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
    let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
        .loop_div(100, 0)
        .enable();

    let mut red_led: RedLed = pins.led_pin.into();
    red_led.set_high().unwrap();

    // This freezes, which is what I was confirming for Justin.
    while !dpll0.is_ready() {}
    let (_gclk0, _dfll, _dpll0) = clocks.gclk0.swap_sources(dfll, dpll0);

    red_led.set_low().unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}
