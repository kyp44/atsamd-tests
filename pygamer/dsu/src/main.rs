//! Tests the CRC32 function of the DSU peripheral.
#![no_std]
#![no_main]

use aligned::{A4, Aligned};
use shared_pygamer::prelude::*;

type Data = [u8; 24];

// Some data in flash, the length is divisible by 4 so should be word aligned
//#[repr(align(4))]
//static DATA: [u8; 24] = *b"This is a test string!!!";
static DATA: Aligned<A4, Data> = Aligned(*b"This is a test string!!!");

// The CRC32 for the above data string
const CRC32: u32 = 0xE8C27689;

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
    let data = DATA;

    // Create the DSU
    let mut dsu = hal::dsu::Dsu::new(
        peripherals.dsu,
        clocks.ahbs.dsu,
        clocks.apbs.dsu,
        &mut peripherals.pac,
    )
    .unwrap();

    // Calculate and verify CRC32 in both flash and RAM
    let flash_addr = DATA.as_ptr() as u32;
    let flash_crc = dsu.crc32(flash_addr, DATA.len() as u32).unwrap();
    let ram_addr = data.as_ptr() as u32;
    let ram_crc = dsu.crc32(ram_addr, data.len() as u32).unwrap();
    assert_eq!(flash_crc, CRC32);
    assert_eq!(ram_crc, CRC32);

    // Just show that the test has completed
    let pkg = SetupPackage::new(
        unsafe { Peripherals::steal() },
        CorePeripherals::take().unwrap(),
    );

    // Show stuff
    let mut screens = Screens::new(pkg.display, pkg.buttons);
    let mut writer = screens.new_screen();
    writeln!(writer, "Flash address: {flash_addr:08X}").unwrap();
    writeln!(writer, "Flash CRC32: {flash_crc:08X}").unwrap();
    writeln!(writer, "RAM address: {ram_addr:08X}").unwrap();
    writeln!(writer, "RAM CRC32: {ram_crc:08X}").unwrap();
    screens.wait_for_button();
    screens.test_complete();
}
