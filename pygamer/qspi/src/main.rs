//! Tests the `qspi` example from the `pygamer` BSP for Ashcon Mohseninia.
//! See here: https://github.com/atsamd-rs/atsamd/pull/926
#![no_std]
#![no_main]

use bsp::{Pins, RedLed};
use hal::{
    delay::Delay,
    prelude::*,
    qspi::{self, Command},
};
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    let (mut delay, mut red_led) = example();

    loop {
        red_led.toggle();
        delay.delay_ms(1000u16);
    }
}

#[cfg(feature = "clock-v1")]
fn example() -> (Delay, RedLed) {
    use hal::clock::GenericClockController;

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let sets = Pins::new(peripherals.port).split();

    let mut flash = sets.flash.init(&mut peripherals.mclk, peripherals.qspi);

    // Startup delay. Can't find documented but Adafruit use 5ms
    delay.delay_ms(5u8);
    // Reset. It is recommended to check the BUSY(WIP?) bit and the SUS before reset
    wait_ready(&mut flash);
    flash.run_command(Command::EnableReset).unwrap();
    flash.run_command(Command::Reset).unwrap();
    // tRST(30μs) to reset. During this period, no command will be accepted
    delay.delay_ms(1u8);

    // Check for GD25Q64C JEDEC ID
    let mut read_buf = [0u8; 3];
    flash.read_command(Command::ReadId, &mut read_buf).unwrap();
    assert_eq!(read_buf, [0x17, 0x40, 0xc8]);

    // 120MHz / 2 = 60mhz
    // faster than 104mhz at 3.3v would require High Performance Mode
    flash.set_clk_divider(2);

    // Enable Quad SPI mode. Requires write enable. Check WIP.
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_command(Command::WriteStatus2, &[0x02]).unwrap();
    wait_ready(&mut flash);

    // Chip Erase. Requires write enable. Check WIP.
    flash.run_command(Command::WriteEnable).unwrap();
    flash.erase_command(Command::EraseChip, 0x0).unwrap();
    // Worst case up to 140 seconds!
    wait_ready(&mut flash);

    // Page Program. Requires write enable. Check WIP.
    // If more than 256 bytes are sent to the device, previously latched data
    // are discarded and the last 256 data bytes are guaranteed to be
    // programmed correctly within the same page. If less than 256 data
    // bytes are sent to device, they are correctly programmed at the
    // requested addresses without having any effects on the other bytes of
    // the same page

    let write_buf = [0x0d, 0xd0, 0x01, 0xc0];
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_memory(0, &write_buf);
    wait_ready(&mut flash);

    // Read back data
    // datasheet claims 6BH needs a single dummy byte, but doesnt work then
    // adafruit uses 8, and the underlying implementation uses 8 atm as well
    let mut read_buf = [0u8; 4];
    flash.read_memory(0, &mut read_buf);
    assert_eq!(read_buf, write_buf);

    // END OF ACTUAL EXAMPLE
    (delay, sets.led_pin.into())
}

#[cfg(not(feature = "clock-v1"))]
fn example() {
    use hal::{clock::v2::clock_system_at_reset, qspi::QspiBuilder};

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let (_buses, clocks, _tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    let (mut delay, gclk0) = Delay::new_with_source(core.SYST, clocks.gclk0);

    let sets = Pins::new(peripherals.port).split();

    let apb_qspi = clocks.apbs.qspi;
    let ahb_qspi = clocks.ahbs.qspi;

    let (mut flash, _gclk0) = QspiBuilder::new(
        sets.flash.sclk,
        sets.flash.cs,
        sets.flash.data0,
        sets.flash.data1,
        sets.flash.data2,
        sets.flash.data3,
    )
    // QSPI freq can never be more than 1/2 of the CPU freq.
    // CPU is running at 48Mhz by default, so max QSPI speed
    // like this is 24Mhz
    .with_freq(24_000_000)
    .with_mode(qspi::QspiMode::_0)
    .build(peripherals.qspi, ahb_qspi, apb_qspi, gclk0)
    .unwrap();

    // Startup delay. Can't find documented but Adafruit use 5ms
    delay.delay_ms(5u8);
    // Reset. It is recommended to check the BUSY(WIP?) bit and the SUS before reset
    wait_ready(&mut flash);
    flash.run_command(Command::EnableReset).unwrap();
    flash.run_command(Command::Reset).unwrap();
    // tRST(30μs) to reset. During this period, no command will be accepted
    delay.delay_ms(1u8);

    // Check for GD25Q64C JEDEC ID
    let mut read_buf = [0u8; 3];
    flash.read_command(Command::ReadId, &mut read_buf).unwrap();
    assert_eq!(read_buf, [0x17, 0x40, 0xc8]);

    // Enable Quad SPI mode. Requires write enable. Check WIP.
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_command(Command::WriteStatus2, &[0x02]).unwrap();
    wait_ready(&mut flash);

    // Chip Erase. Requires write enable. Check WIP.
    flash.run_command(Command::WriteEnable).unwrap();
    flash.erase_command(Command::EraseChip, 0x0).unwrap();
    // Worst case up to 140 seconds!
    wait_ready(&mut flash);

    // Page Program. Requires write enable. Check WIP.
    // If more than 256 bytes are sent to the device, previously latched data
    // are discarded and the last 256 data bytes are guaranteed to be
    // programmed correctly within the same page. If less than 256 data
    // bytes are sent to device, they are correctly programmed at the
    // requested addresses without having any effects on the other bytes of
    // the same page

    let write_buf = [0x0d, 0xd0, 0x01, 0xc0];
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_memory(0, &write_buf);
    wait_ready(&mut flash);

    // Read back data
    // datasheet claims 6BH needs a single dummy byte, but doesnt work then
    // adafruit uses 8, and the underlying implementation uses 8 atm as well
    let mut read_buf = [0u8; 4];
    flash.read_memory(0, &mut read_buf);
    assert_eq!(read_buf, write_buf);

    /// END OF ACTUAL EXAMPLE
    sets.led_pin.into()
}

/// Wait for the write-in-progress and suspended write/erase.
fn wait_ready(flash: &mut qspi::Qspi<qspi::OneShot>) {
    while flash_status(flash, Command::ReadStatus) & 0x01 != 0 {}
    while flash_status(flash, Command::ReadStatus2) & 0x80 != 0 {}
}

/// Returns the contents of the status register indicated by cmd.
fn flash_status(flash: &mut qspi::Qspi<qspi::OneShot>, cmd: Command) -> u8 {
    let mut out = [0u8; 1];
    flash.read_command(cmd, &mut out).ok().unwrap();
    out[0]
}
