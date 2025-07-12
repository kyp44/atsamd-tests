//! Tests the CRC32 function of the DSU peripheral.
#![no_std]
#![no_main]

use heapless::String;
use shared_pygamer::prelude::*;

// Some data in flash
static DATA: &str = "This is a test string";

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let (mut _buses, clocks, _tokens) = hal::clock::v2::clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    // Some data in RAM
    let mut data: String<30> = String::new();
    data.push_str(DATA).unwrap();

    // TODO: Temporarily disable the bus clocks to see whether it works
    _buses.ahb.disable(clocks.ahbs.dsu);
    _buses.apb.disable(clocks.apbs.dsu);

    // Create the DSU
    let mut dsu = hal::dsu::Dsu::new(peripherals.dsu, &peripherals.pac).unwrap();

    // Calculate CRC32 in both flash and RAM
    panic!(
        "TODO FLASH: {}",
        dsu.crc32(DATA.as_ptr() as u32, DATA.len() as u32).unwrap()
    );
    panic!(
        "TODO RAM: {}",
        dsu.crc32(data.as_str().as_ptr() as u32, data.len() as u32)
            .unwrap()
    );

    // Just show that the test has completed
    let pkg = SetupPackage::new(
        unsafe { Peripherals::steal() },
        CorePeripherals::take().unwrap(),
    );

    Screens::new(pkg.display, pkg.buttons).test_complete();
}
