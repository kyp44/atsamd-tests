use crate::bsp::{ButtonReader, Keys};
use derive_more::From;
use shared::prelude::*;

#[derive(From)]
pub struct Buttons(ButtonReader);
impl Buttons {
    pub fn free(self) -> ButtonReader {
        self.0
    }
}

impl Input for Buttons {
    // Note that this will block the thread until a button is pressed!
    fn wait_for_button(&mut self) {
        'main: loop {
            for event in self.0.events() {
                match event {
                    Keys::StartDown | Keys::ADown => break 'main,
                    _ => {}
                }
            }
        }
    }
}
