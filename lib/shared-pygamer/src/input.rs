use crate::bsp::{ButtonReader, Keys};
use core::future::poll_fn;
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
    async fn wait_for_button(&mut self) {
        poll_fn::<(), _>(|_| {
            'main: loop {
                for event in self.0.events() {
                    match event {
                        Keys::StartDown | Keys::ADown => break 'main,
                        _ => {}
                    }
                }
            }
            core::task::Poll::Ready(())
        })
        .await;
    }
}
