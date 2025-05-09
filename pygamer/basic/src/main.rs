#![no_std]
#![no_main]

use hal::prelude::*;
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    let mut pkg = setup(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    let mut screens = Screens::new(pkg.display, pkg.buttons);

    write!(screens.new_screen(), "Hello world!").unwrap();
    pkg.delay.delay_ms(2000u16);

    panic!("Panic demonstration");
}
