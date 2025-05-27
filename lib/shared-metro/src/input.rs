use crate::hal::{ehal::digital::InputPin, gpio};
use derive_more::From;
use shared::prelude::*;

type ButtonRaw<P> = gpio::Pin<P, gpio::Input<gpio::PullUp>>;

#[derive(From)]
pub struct Button<P: gpio::PinId>(ButtonRaw<P>);
impl<P: gpio::PinId> Input for Button<P> {
    fn wait_for_button(&mut self) {
        // Wait for button to be unpressed
        while self.0.is_low().unwrap() {}

        // Now wait for it to be pressed
        while self.0.is_high().unwrap() {}
    }
}

pub struct Buttons {
    pub button_a: Button<gpio::PA07>,
    pub button_b: Button<gpio::PA20>,
    pub button_c: Button<gpio::PA15>,
}

impl Input for Buttons {
    fn wait_for_button(&mut self) {
        // Wait for all buttons to be unpressed
        while self.button_a.0.is_low().unwrap()
            || self.button_b.0.is_low().unwrap()
            || self.button_c.0.is_low().unwrap()
        {}

        // Now wait for any to be pressed
        while self.button_a.0.is_high().unwrap()
            && self.button_b.0.is_high().unwrap()
            && self.button_c.0.is_high().unwrap()
        {}
    }
}
