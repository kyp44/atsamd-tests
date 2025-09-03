//! Tests robust USB as serial output.
#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use core::cell::OnceCell;
use core::sync::atomic;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use hal::prelude::*;
use hal::usb::UsbBus;
use pac::interrupt;
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );

    let bus_allocator = unsafe {
        let _ = USB_ALLOCATOR.set(pkg.usb_pins.init(pkg.usb, &mut pkg.clocks, &mut pkg.mclk));
        USB_ALLOCATOR.get().unwrap()
    };

    unsafe {
        // Setup the USB device
        let _ = USB_SERIAL.set(SerialPort::new(bus_allocator));
        let _ = USB_BUS.set(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")])
                .expect("Failed to set strings")
                .device_class(USB_CLASS_CDC)
                .build(),
        );

        // Enable the USB interrupts
        pkg.nvic.set_priority(interrupt::USB_OTHER, 1);
        pkg.nvic.set_priority(interrupt::USB_TRCPT0, 1);
        pkg.nvic.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    let mut screens = Screens::new(pkg.display, pkg.buttons);
    let mut writer = screens.new_screen();
    core::write!(
        writer,
        "Connect to the USB serial device and send something..."
    )
    .unwrap();

    // Wait for USB user
    while !USER_PRESENT.load(atomic::Ordering::Acquire) {
        pkg.delay.delay_ms(250u32);
        pkg.red_led.toggle().unwrap();
    }
    pkg.red_led.set_high().unwrap();

    serial_writeln!("Starting the loop...");

    let mut iterations = 0u32;
    loop {
        pkg.delay.delay_ms(1000u32);
        serial_writeln!("Loop iteration number {iterations}");
        iterations += 1;
    }
}

static mut USB_ALLOCATOR: OnceCell<UsbBusAllocator<UsbBus>> = OnceCell::new();
static mut USB_BUS: OnceCell<UsbDevice<UsbBus>> = OnceCell::new();
static mut USB_SERIAL: OnceCell<SerialPort<UsbBus>> = OnceCell::new();
static USER_PRESENT: atomic::AtomicBool = atomic::AtomicBool::new(false);

/// Borrows the global singleton `UsbSerial` for a brief period with interrupts
/// disabled
///
/// # Arguments
/// `borrower`: The closure that gets run borrowing the global `UsbSerial`
///
/// # Safety
/// the global singleton `UsbSerial` can be safely borrowed because we disable
/// interrupts while it is being borrowed, guaranteeing that interrupt handlers
/// like `USB` cannot mutate `UsbSerial` while we are as well.
///
/// # Panic
/// If `init` has not been called and we haven't initialized our global
/// singleton `UsbSerial`, we will panic.
fn usbserial_get<T, R>(borrower: T) -> R
where
    T: Fn(&mut SerialPort<UsbBus>) -> R,
{
    usb_free(|_| unsafe {
        let usb_serial = USB_SERIAL.get_mut().expect("UsbSerial not initialized");
        borrower(usb_serial)
    })
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
fn usb_free<F, R>(f: F) -> R
where
    F: FnOnce(&cortex_m::interrupt::CriticalSection) -> R,
{
    NVIC::mask(interrupt::USB_OTHER);
    NVIC::mask(interrupt::USB_TRCPT0);
    NVIC::mask(interrupt::USB_TRCPT1);

    let r = f(&unsafe { cortex_m::interrupt::CriticalSection::new() });

    unsafe {
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    };

    r
}

/// Writes the given message out over USB serial.
///
/// # Arguments
/// * println args: variable arguments passed along to `core::write!`
///
/// # Warning
/// as this function deals with a static mut, and it is also accessed in the
/// USB interrupt handler, we both have unsafe code for unwrapping a static mut
/// as well as disabling of interrupts while we do so.
///
/// # Safety
/// the only time the static mut is used, we have interrupts disabled so we know
/// we have sole access
#[macro_export]
macro_rules! serial_writeln {
    ($($tt:tt)+) => {{
        use core::fmt::Write;

        let mut s: heapless::String<256> = heapless::String::new();
        core::write!(&mut s, $($tt)*).unwrap();
        usbserial_get(|usbserial| {
            usbserial.write(s.as_bytes()).ok();
            usbserial.write("\r\n".as_bytes()).ok();
        });
    }};
}

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.get_mut()
            && let Some(serial) = USB_SERIAL.get_mut()
        {
            usb_dev.poll(&mut [serial]);
            let mut buf = [0u8; 64];

            if let Ok(count) = serial.read(&mut buf)
                && count > 0
            {
                USER_PRESENT.store(true, atomic::Ordering::Release);

                // We don't want to echo back
                //serial.write(&buf[..count]).unwrap();
            }
        }
    }
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}
