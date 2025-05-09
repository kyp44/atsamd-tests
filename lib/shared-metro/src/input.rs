use crate::hal::{bind_interrupts, ehal_async::digital::Wait, eic, gpio};
use embassy_futures::select::select3;
use shared::prelude::*;

bind_interrupts!(pub struct ButtonIrq {
    EIC => eic::InterruptHandler;
});

type Button<P, C> = eic::ExtInt<gpio::Pin<P, gpio::Interrupt<gpio::PullUp>>, C, eic::EicFuture>;

pub struct Buttons {
    pub button_a: Button<gpio::PA07, eic::Ch7>,
    pub button_b: Button<gpio::PA20, eic::Ch4>,
    pub button_c: Button<gpio::PA15, eic::Ch15>,
}

impl Input for Buttons {
    async fn wait_for_button(&mut self) {
        let _ = select3(
            self.button_a.wait_for_falling_edge(),
            self.button_b.wait_for_falling_edge(),
            self.button_c.wait_for_falling_edge(),
        )
        .await;

        // The implementation of the ExtInt does not have a drop to cancel, but we can do this
        self.button_a.sense(eic::Sense::None);
        self.button_b.sense(eic::Sense::None);
        self.button_c.sense(eic::Sense::None);
    }
}
