//! Program to test the RTC-based embassy time driver.
//!
//! Refer to `atsamd-hal` [PR 825](https://github.com/atsamd-rs/atsamd/pull/825).
//!
//! TODO: Until the clock v2 is better supported, this has to be just a simple test that blinks the LED.
//! In particular, we cannot use the display because it requires a [`DelayNs`](hal::ehal::delay::DelayNs) source and neither [`delay::Delay`](hal::delay::Delay)
//! nor a [`TimerCounter`] can be used for that using the clock v2 API alone, even with the v2 compatibility
//! proof conversions. Even more specifically, `Delay` requires a v1 `GenericClockController` reference
//! while [`TimerCounter`] requires a [`pac::Mclk`] reference
#![no_std]
#![no_main]

use bsp::{Pins, RedLed};
use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, rtcosc::RtcOsc};
use hal::prelude::*;
use shared_pygamer::prelude::*;

// TODO: Add more tasks later, see above
//mod tasks;
//use tasks::{clock_task, test_task};

const BASE_PERIOD_MS: u64 = 500;

hal::embassy_time!(Mono);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut peripherals = Peripherals::take().unwrap();

    // Uses the clock v2 API to get the default clock tree with the main clock at 48 MHz.
    let (_buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port).split();

    // Setup the TC4/TC5 clock as a timer is needed for a delay source for the display driver
    //let (pclk_tc4tc5, gclk0) = Pclk::enable(tokens.pclks.tc4_tc5, clocks.gclk0);
    //let timer = TimerCounter::tc4_(&pclk_tc4tc5.into(), peripherals.tc4, &mut peripherals.mclk);

    // Enable the internal 32 kHz oscilllator and use it for the RTC clock
    let (osculp32k, _) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
    let (rtc_osc, _) = RtcOsc::enable(tokens.rtcosc, osculp32k);

    // Intialize the time driver
    unsafe { Mono::init(rtc_osc) };

    let mut red_led: RedLed = pins.led_pin.into();
    loop {
        Timer::after_millis(BASE_PERIOD_MS).await;
        red_led.toggle().unwrap();
    }
}
