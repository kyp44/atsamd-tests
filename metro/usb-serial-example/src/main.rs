//! Just a basic program to test the display and its buttons.
#![no_std]
#![no_main]

use atsamd_usb_serial::{UsbSerial, usb_device::bus::UsbBusAllocator};
use hal::prelude::*;
use hal::usb::UsbBus;
use pac::NVIC;
use shared_metro::prelude::*;

#[entry]
fn main() -> ! {
    // Setup stuff
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    let usb_allocator = bsp::usb_allocator(
        pkg.usb,
        &mut pkg.clocks,
        &mut pkg.pm,
        pkg.usb_dm,
        pkg.usb_dp,
    );

    let _ = usb_serial_example(&mut pkg.nvic, usb_allocator);

    loop {}
}

#[inline]
fn usb_serial_example(nvic: &mut NVIC, usb_allocator: UsbBusAllocator<UsbBus>) -> UsbSerial<64> {
    use atsamd_usb_serial::prelude::*;

    // Uncomment this for the example!
    // let usb_allocator = ...;

    let mut usb_serial: UsbSerial<64> = UsbSerial::new(
        nvic,
        usb_allocator,
        StringDescriptors::new(LangID::EN)
            .manufacturer("Your company")
            .product("Serial port")
            .serial_number("TEST"),
        UsbVidPid(0x16c0, 0x27dd),
        true,
    )
    .unwrap();

    // Wait until something is received
    while usb_serial.read(&mut [0u8; 1]) < 0 {
        // Maybe just delay here
    }

    // Write some raw bytes
    usb_serial
        .write(&[0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x21])
        .unwrap();

    // End of the actual example
    usb_serial
}
