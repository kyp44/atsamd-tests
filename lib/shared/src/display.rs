use core::fmt::Write;
use embedded_graphics::{
    mono_font::{self, MonoTextStyle},
    prelude::*,
    text,
};

// TODO: No crate that does this exists, should we generalize and publish it?
pub struct DisplayTextStyle<C> {
    /// Location of the text on the screen.
    pub position: Point,
    /// Optional box size for character wrapping.
    pub char_box_size: Option<Size>,
    pub character_style: mono_font::MonoTextStyle<'static, C>,
    pub text_style: text::TextStyle,
}
impl<C> DisplayTextStyle<C> {
    pub fn new(
        position: Point,
        box_size: Option<Size>,
        character_style: mono_font::MonoTextStyle<'static, C>,
        text_style: text::TextStyle,
    ) -> Self {
        Self {
            position,
            char_box_size: box_size.map(|s| {
                Size::new(
                    s.width / character_style.font.character_size.width,
                    s.height / character_style.font.character_size.height,
                )
            }),

            character_style,
            text_style,
        }
    }
}

pub trait Display: DrawTarget + OriginDimensions {
    const FONT: mono_font::MonoFont<'static>;
    const BACKGROUND_COLOR: Self::Color;
    const TEXT_COLOR: Self::Color;
    const PANIC_BACKGROUND_COLOR: Self::Color;
    const PANIC_TEXT_COLOR: Self::Color;

    fn flush(&mut self);

    #[inline]
    fn character_style() -> MonoTextStyle<'static, Self::Color> {
        mono_font::MonoTextStyleBuilder::new()
            .font(&Self::FONT)
            .text_color(Self::TEXT_COLOR)
            .background_color(Self::BACKGROUND_COLOR)
            .build()
    }

    #[inline]
    fn display_text_style(&self, position: Point) -> DisplayTextStyle<Self::Color> {
        DisplayTextStyle::new(
            position,
            Some(self.size()),
            Self::character_style(),
            text::TextStyleBuilder::new()
                .baseline(text::Baseline::Top)
                .build(),
        )
    }
}

pub struct DisplayWriter<'a, D: Display> {
    /// The display to which to write
    display: &'a mut D,
    /// The text style
    display_text_style: DisplayTextStyle<<D as DrawTarget>::Color>,
    /// Current cursor location in character space.
    char_cursor: Point,
}
impl<'a, D: Display> DisplayWriter<'a, D> {
    pub fn new(display: &'a mut D, text_style: DisplayTextStyle<<D as DrawTarget>::Color>) -> Self {
        Self {
            display,
            display_text_style: text_style,
            char_cursor: Point::zero(),
        }
    }

    #[inline]
    pub fn flush(&mut self) {
        self.display.flush()
    }

    fn newline(&mut self) {
        self.char_cursor.x = 0;
        self.char_cursor.y += 1;
    }
}
impl<D: Display> core::fmt::Write for DisplayWriter<'_, D> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Iterate over lines
        'lines: for (line_num, mut line) in s.lines().enumerate() {
            // Advance to the next line if applicable
            if line_num > 0 {
                self.newline();
            }

            // Display the line, adding a line break if we run out of space
            loop {
                let style = &self.display_text_style;

                if line.is_empty() {
                    break;
                }
                // If we are outside our box then we do not want to display anything else
                if let Some(cs) = style.char_box_size
                    && self.char_cursor.y >= cs.height as i32
                {
                    break 'lines;
                }

                // First break the string at any newline
                let (line_s, rem_s) = match style.char_box_size {
                    Some(cs) => {
                        // Iterator of character indices in the current string
                        let mut char_idxs = line.char_indices();

                        // Advance by the number of characters left on the current line
                        let _ =
                            char_idxs.advance_by(cs.width as usize - self.char_cursor.x as usize);

                        let idx = char_idxs.next().map(|t| t.0).unwrap_or(line.len());
                        line.split_at_checked(idx).ok_or(core::fmt::Error)?
                    }
                    None => (line, ""),
                };

                text::Text::with_text_style(
                    line_s,
                    style.position
                        + Point::new(
                            self.char_cursor.x
                                * style.character_style.font.character_size.width as i32,
                            self.char_cursor.y
                                * style.character_style.font.character_size.height as i32,
                        ),
                    style.character_style,
                    style.text_style,
                )
                .draw(self.display)
                .map_err(|_| core::fmt::Error)?;

                // Update cursor
                self.char_cursor.x += line_s.len() as i32;

                // Advance to the next line if applicable
                if let Some(cs) = style.char_box_size
                    && self.char_cursor.x >= cs.width as i32
                {
                    self.newline();
                }

                line = rem_s;
            }
        }

        // If the last character is a newline, this would not be accounted for above
        if s.ends_with("\n") {
            self.newline();
        }

        Ok(())
    }
}

#[inline]
pub fn panic_display<D: Display>(mut display: D, info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    let style = DisplayTextStyle::new(
        Point::zero(),
        Some(display.size()),
        mono_font::MonoTextStyleBuilder::new()
            .font(&D::FONT)
            .text_color(D::PANIC_TEXT_COLOR)
            .background_color(D::PANIC_BACKGROUND_COLOR)
            .build(),
        text::TextStyleBuilder::new()
            .alignment(text::Alignment::Left)
            .baseline(text::Baseline::Top)
            .build(),
    );

    let _ = write!(DisplayWriter::new(&mut display, style), "{info}");
    display.flush();

    // Just go to sleep forever
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
