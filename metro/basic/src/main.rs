#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use bsp::RedLed;
use hal::{ehal_async::digital::Wait, prelude::*, time::Hertz};
use shared_metro::prelude::{embedded_graphics::prelude::Point, *};

#[embassy_executor::task]
async fn blinky(mut red_led: RedLed) {
    loop {
        Mono::delay(1u32.secs()).await;
        red_led.toggle().unwrap();
    }
}

#[embassy_executor::main]
async fn main(s: embassy_executor::Spawner) {
    // Setup stuff
    let mut pkg = setup(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    )
    .await;

    // Start monotonic
    // TODO: Maybe change this to a SysTick or RTC embassy driver.
    let gclk0 = pkg.clocks.gclk0();
    let freq: Hertz = gclk0.into();
    Mono::start(pkg.syst, freq.to_Hz());

    // Start blinking the red LED to make sure there is no panic
    s.must_spawn(blinky(pkg.red_led));

    // Display style
    let mut buttons = pkg.buttons;

    loop {
        let style = pkg.display.display_text_style(Point::zero());
        let mut writer = DisplayWriter::new(&mut pkg.display, style);
        writeln!(writer, "Hey there! Press the A button.").unwrap();
        writer.flush().await;
        buttons.button_a.wait_for_falling_edge().await.unwrap();

        writeln!(writer, "Now press the B button.").unwrap();
        writer.flush().await;
        buttons.button_b.wait_for_falling_edge().await.unwrap();

        writeln!(writer, "Lastly, press the C button.").unwrap();
        writer.flush().await;
        buttons.button_c.wait_for_falling_edge().await.unwrap();

        writeln!(writer, "Now press any button to panic.").unwrap();
        writer.flush().await;
        buttons.wait_for_button().await;

        // Note that this fails if we try to flush.
        pkg.display.clear();
    }
}
