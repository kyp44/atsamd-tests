use crate::{
    Input,
    display::{Display, DisplayWriter},
};
use derive_new::new;
use embedded_graphics::{prelude::*, text};

#[derive(new)]
pub struct ScreensGen<D: Display + 'static, I> {
    display: D,
    input: I,
}
impl<D: Display, I: Input> ScreensGen<D, I>
where
    D::Error: core::fmt::Debug,
{
    pub fn new_screen(&mut self) -> DisplayWriter<'_, D> {
        self.display.clear(D::BACKGROUND_COLOR).unwrap();

        let style = self.display.display_text_style(Point::zero());
        DisplayWriter::new(&mut self.display, style)
    }

    fn button_message(&mut self) {
        text::Text::with_text_style(
            "Press a button to continue...",
            Point::new(0, (self.display.size().height - 1) as i32),
            D::character_style(),
            text::TextStyleBuilder::new()
                .baseline(text::Baseline::Bottom)
                .build(),
        )
        .draw(&mut self.display)
        .unwrap();
    }

    pub fn wait_for_button(&mut self) -> DisplayWriter<'_, D> {
        self.button_message();
        self.display.flush();

        self.input.wait_for_button();
        self.new_screen()
    }
}
