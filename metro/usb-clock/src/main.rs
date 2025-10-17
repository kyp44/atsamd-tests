//! Just a basic program to test the display and its buttons.
#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use core::str;

use atsamd_usb_serial::heapless::String;
use atsamd_usb_serial::prelude::*;

use hal::prelude::*;
use hal::rtc;
use shared_metro::prelude::*;

#[entry]
fn main() -> ! {
    // Setup stuff
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    let mut rtc = rtc::Rtc::clock_mode(pkg.setup_rtc_clock().unwrap().0, 1024.Hz(), &mut pkg.pm);
    let mut usb_serial: UsbSerial = UsbSerial::new(
        &mut pkg.nvic,
        bsp::usb_allocator(
            pkg.usb,
            &mut pkg.clocks,
            &mut pkg.pm,
            pkg.usb_dm,
            pkg.usb_dp,
        ),
        StringDescriptors::new(LangID::EN)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST"),
        UsbVidPid(0x16c0, 0x27dd),
        true,
    )
    .unwrap();

    let mut screens = Screens::new(pkg.display, pkg.buttons);
    let mut writer = screens.new_screen();
    writeln!(writer, "Connect to the USB device and send something...").unwrap();
    writer.flush();

    // Wait until something is received
    loop {
        if usb_serial.read(&mut [0u8; 1]) > 0 {
            break;
        }
        pkg.delay.delay_ms(250_u32);
        pkg.red_led.toggle().unwrap();
    }

    pkg.red_led.set_high().unwrap();

    let mut writer = screens.new_screen();
    writeln!(writer, "Time can be set by sending:").unwrap();
    writeln!(writer, "time=HH:MM:SS").unwrap();
    writer.flush();

    loop {
        // Write the current time
        let time = rtc.current_time();
        writeln!(
            usb_serial,
            "{:02}:{:02}:{:02}\r",
            time.hours, time.minutes, time.seconds
        )
        .ok();
        usb_serial.flush().ok();

        pkg.delay.delay_ms(1000u32);

        // Look for setting of time
        let mut buffer: String<64> = String::new();
        usb_serial.read_string(&mut buffer).unwrap();
        let mut read = buffer.as_str();

        while read.len() > 5 {
            match Time::parse(read) {
                Ok((remaining, time)) => {
                    read = remaining;

                    rtc.set_time(time.into());
                }
                _ => break,
            };
        }
    }
}

#[derive(Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}
impl From<Time> for rtc::Datetime {
    fn from(value: Time) -> Self {
        Self {
            seconds: value.second,
            minutes: value.minute,
            hours: value.hour,
            day: 0,
            month: 0,
            year: 0,
        }
    }
}

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, u8 as u8p},
    combinator::{map, opt},
};

impl Time {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            (
                opt(char('\r')),
                tag("time="),
                u8p,
                char(':'),
                u8p,
                char(':'),
                u8p,
                char('\r'),
            ),
            |(_, _, hour, _, minute, _, second, _)| Self {
                hour,
                minute,
                second,
            },
        )
        .parse(input)
    }
}
