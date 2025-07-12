//! Tests encryption and decryption using the AES peripheral.
#![no_std]
#![no_main]

use hal as atsamd_hal;
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    use atsamd_hal::aes::*;

    // AES RustCrypto Example
    let mut peripherals = atsamd_hal::pac::Peripherals::take().unwrap();
    let (mut buses, _clocks, tokens) = atsamd_hal::clock::v2::clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    // Enable the APB clock
    let apb_clk = buses.apb.enable(tokens.apbs.aes);

    // Setup the AES peripheral
    let aes = atsamd_hal::aes::Aes::new(peripherals.aes, apb_clk);

    // Activate the RustCrypto backend
    let crypto = aes.activate_rustcrypto_backend();

    // Set up key and data block
    let key = GenericArray::from_slice(&[0u8; 16]);
    let mut block = aes::Block::default();

    // Initialize cipher
    let cipher = crypto.into_128bit(key);

    // This copies the entire block
    let block_copy = block;

    // Encrypt block in-place and verify that it is different
    cipher.encrypt_block(&mut block);
    assert_ne!(block, block_copy);

    // Decrypt it back and verify that is the same as it was
    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);

    // Just show that the test has completed
    let pkg = SetupPackage::new(
        unsafe { Peripherals::steal() },
        CorePeripherals::take().unwrap(),
    );

    Screens::new(pkg.display, pkg.buttons).test_complete();
}
