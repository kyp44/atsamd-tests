//! Just a basic program to test the display and its buttons.
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use shared_metro::prelude::{embedded_graphics::prelude::*, *};

#[entry]
fn main() -> ! {
    // Setup stuff
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    let mut buttons = pkg.buttons;

    let style = pkg.display.display_text_style(Point::zero());
    let mut writer = DisplayWriter::new(&mut pkg.display, style);
    writeln!(writer, "Hey there! Press the A button.").unwrap();
    writer.flush();
    buttons.button_a.wait_for_button();

    writeln!(writer, "Now press the B button.").unwrap();
    writer.flush();
    buttons.button_b.wait_for_button();

    writeln!(writer, "Lastly, press the C button.").unwrap();
    writer.flush();
    buttons.button_c.wait_for_button();

    writeln!(writer, "Now press any button to panic.").unwrap();
    writer.flush();
    buttons.wait_for_button();

    panic!("Test panic!");
}
