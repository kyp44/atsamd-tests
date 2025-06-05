//! Program to test the RTC-based embassy time driver.
//!
//! Refer to `atsamd-hal` [PR TODO](https://github.com/atsamd-rs/atsamd/pull/TODO).
#![no_std]
#![no_main]

use bsp::{Pins, RedLed};
use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, rtcosc::RtcOsc};
use hal::prelude::*;
use shared_pygamer::prelude::*;

// TODO: Add more tasks later, see below
//mod tasks;
//use tasks::{clock_task, test_task};

const BASE_PERIOD_MS: u64 = 500;

hal::embassy_time!(Mono);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Setup stuff
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );
    let (rtc, rtc_clock) = pgk.setup_rtc_clock().unwrap();

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
