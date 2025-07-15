//! Tests module documentation example for the ICM.
#![no_std]
#![no_main]

use hal::icm::{HashArea, Icm, RegionDesc, Regions, icm_algorithm};
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    // SHA Test data
    static MESSAGE_REF0: [u32; 16] = [
        0x11111111, 0x22222222, 0x33333333, 0x44444444, 0x55555555, 0x66666666, 0x77777777,
        0x88888888, 0x99999999, 0xaaaaaaaa, 0xbbbbbbbb, 0xcccccccc, 0xdddddddd, 0xeeeeeeee,
        0xffffffff, 0x00000000,
    ];

    static MESSAGE_REF1: [u32; 16] = [
        0x80636261, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x18000000,
    ];

    // Expected SHA1 sum result
    static _MESSAGE_SHA1_RES: [u32; 8] = [
        0x363e99a9, 0x6a810647, 0x71253eba, 0x6cc25078, 0x9dd8d09c, 0x00000000, 0x00000000,
        0x00000000,
    ];

    // Expected SHA224 sum result
    static _MESSAGE_SHA224_RES: [u32; 8] = [
        0x227d0923, 0x22d80534, 0x77a44286, 0xb355a2bd, 0xe4bcad2a, 0xf7b3a0bd, 0xa79d6ce3,
        0x00000000,
    ];

    // Expected SHA256 sum result
    static _MESSAGE_SHA256_RES: [u32; 8] = [
        0xbf1678ba, 0xeacf018f, 0xde404141, 0x2322ae5d, 0xa36103b0, 0x9c7a1796, 0x61ff10b4,
        0xad1500f2,
    ];
    static mut HASH: HashArea = HashArea::default();
    static mut ICM_REGION_DESC: Regions = Regions::default();

    // Alternatively
    //use cortex_m::singleton;
    //let hasharea: &'static mut HashArea = singleton!(: HashArea = HashArea::default()).unwrap();

    // Setup the clocks
    let mut peripherals = Peripherals::take().unwrap();
    let (mut buses, clocks, tokens) = hal::clock::v2::clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    // Enable the APB clock
    let apb_clk = buses.apb.enable(tokens.apbs.icm);

    // Create new ICM
    let mut icm = Icm::new(peripherals.icm, clocks.ahbs.icm, apb_clk);

    // Reset the ICM, clearing past error states
    icm.swrst();

    // End of Monitoring is permitted
    icm.set_eomdis(false);
    // Write Back is permitted
    icm.set_wbdis(false);
    // Secondary List branching is forbidden
    icm.set_slbdis(false);
    // Automatic Switch to Compare is disabled
    icm.set_ascd(false);

    // Region Descriptor
    let mut icm_region_desc = Regions::default();

    // Get the interface for Region0 and enable monitoring
    let icm_region0 = icm.enable_region0();
    icm_region0.enable_monitoring();

    // Setup desired interrupts
    //
    // Region Hash Completed
    icm_region0.set_rhc_int();

    // Region0 raddr
    icm_region_desc
        .region0
        .set_region_address(MESSAGE_REF0.as_ptr());

    // Configure the RCFG

    // Some are default values, just as an example

    // Activate Write back (should be true when comparing memory)
    icm_region_desc.region0.rcfg.set_cdwbn(false);
    // Should the ICM controller loop back to DSCR after this region?
    icm_region_desc.region0.rcfg.set_wrap(false);
    // Set this as the end of descriptor linked list
    icm_region_desc.region0.rcfg.set_eom(false);
    // The RHC flag is set when the field NEXT = 0
    // in a descriptor of the main or second list
    icm_region_desc.region0.rcfg.set_rhien(false);
    // Set Algorithm to SHA1
    icm_region_desc.region0.rcfg.set_algo(icm_algorithm::Sha1);

    // Get the interface for region1
    let icm_region1 = icm.enable_region1();

    // Enable region monitoring
    icm_region1.enable_monitoring();

    // Setup desired interrupts
    //
    // Region Hash Completed
    icm_region1.set_rhc_int();

    // Region1 raddr
    icm_region_desc
        .region1
        .set_region_address(MESSAGE_REF1.as_ptr());

    // Configure the RCFG
    // The RHC flag is set when the field NEXT = 0
    // in a descriptor of the main or second list
    icm_region_desc.region1.rcfg.set_rhien(false);
    // Set Algorithm to SHA1
    icm_region_desc.region1.rcfg.set_algo(icm_algorithm::Sha1);

    // Get the interface for region2
    let icm_region2 = icm.enable_region2();

    // Enable region monitoring
    icm_region2.enable_monitoring();

    // Setup desired interrupts
    //
    // Region Hash Completed
    icm_region2.set_rhc_int();

    // Region2 raddr
    icm_region_desc
        .region2
        .set_region_address(MESSAGE_REF1.as_ptr());

    // Configure the RCFG
    // The RHC flag is set when the field NEXT = 0
    // in a descriptor of the main or second list
    icm_region_desc.region2.rcfg.set_rhien(false);
    // Set Algorithm to SHA224
    icm_region_desc.region2.rcfg.set_algo(icm_algorithm::Sha224);

    // Get the interface for region3
    let icm_region3 = icm.enable_region3();

    // Enable region monitoring
    icm_region3.enable_monitoring();

    // Setup desired interrupts
    //
    // Region Hash Completed
    icm_region3.set_rhc_int();

    // Region3 raddr
    icm_region_desc
        .region3
        .set_region_address(MESSAGE_REF1.as_ptr());

    // Configure the RCFG
    //
    // Set this as the end of descriptor linked list
    icm_region_desc.region3.rcfg.set_eom(true);
    // The RHC flag is set when the field NEXT = 0
    // in a descriptor of the main or second list
    icm_region_desc.region3.rcfg.set_rhien(false);
    // Set Algorithm to SHA256
    icm_region_desc.region3.rcfg.set_algo(icm_algorithm::Sha256);

    // Hash Area
    // Set HASH addr to the beginning of the Hash area
    icm.set_hash_addr(HASH);

    // Move the icm_region_desc into static
    *ICM_REGION_DESC = icm_region_desc;
    // Set DSCR to the beginning of the region descriptor
    icm.set_dscr_addr(&ICM_REGION_DESC.region0);
    // the same but via helper function
    //ICM_REGION_DESC.region0.set_dscr_addr(&icm);

    // Start the ICM calculation
    icm.enable();

    // Setup memory region monitoring
    // Monitor all 4 memory regions

    // Setup the compare regions
    let mut message_region0_sha1 = MESSAGE_REF0;
    let mut message_region1_sha1 = MESSAGE_REF1;
    let mut message_region2_sha224 = MESSAGE_REF1;
    let mut message_region3_sha256 = MESSAGE_REF1;

    // Reset the ICM, clearing past error states
    icm.swrst();

    // End of Monitoring is permitted
    icm.set_eomdis(false);
    // Write Back is permitted
    icm.set_wbdis(false);
    // Secondary List branching is forbidden
    icm.set_slbdis(false);
    // Automatic Switch to Compare is disabled
    icm.set_ascd(false);

    // Also possible to directly edit `ICM_REGION_DESC`
    // in an unsafe block
    let mut icm_region_desc = Regions::default();

    // Setup region 0 to monitor memory
    icm_region_desc
        .region0
        .set_region_address(&message_region0_sha1);
    icm_region_desc
        .region0
        .rcfg
        .reset_region_configuration_to_default();
    icm_region_desc.region0.rcfg.set_algo(icm_algorithm::Sha1);
    // Activate Compare Digest (should be true when comparing memory)
    icm_region_desc.region0.rcfg.set_cdwbn(true);
    // Digest Mismatch Interrupt Disable (enabled)
    icm_region_desc.region0.rcfg.set_dmien(false);

    // Set Region Mismatch Interrupt
    icm_region0.set_rdm_int();

    // Setup region 1 to monitor memory
    icm_region_desc
        .region1
        .set_region_address(&message_region1_sha1);
    icm_region_desc
        .region1
        .rcfg
        .reset_region_configuration_to_default();
    icm_region_desc.region1.rcfg.set_algo(icm_algorithm::Sha1);
    // Activate Compare Digest (should be true when comparing memory)
    icm_region_desc.region1.rcfg.set_cdwbn(true);
    // Digest Mismatch Interrupt Disable (enabled)
    icm_region_desc.region1.rcfg.set_dmien(false);

    // Set Region Mismatch Interrupt
    icm_region1.set_rdm_int();

    // Setup region 2 to monitor memory
    icm_region_desc
        .region2
        .set_region_address(&message_region2_sha224);
    icm_region_desc
        .region2
        .rcfg
        .reset_region_configuration_to_default();
    icm_region_desc.region2.rcfg.set_algo(icm_algorithm::Sha224);
    // Activate Compare Digest (should be true when comparing memory)
    icm_region_desc.region2.rcfg.set_cdwbn(true);
    // Digest Mismatch Interrupt Disable (enabled)
    icm_region_desc.region2.rcfg.set_dmien(false);

    // Set Region Mismatch Interrupt
    icm_region2.set_rdm_int();

    // Setup region 3 to monitor memory
    icm_region_desc
        .region3
        .set_region_address(&message_region3_sha256);
    icm_region_desc
        .region3
        .rcfg
        .reset_region_configuration_to_default();
    icm_region_desc.region3.rcfg.set_algo(icm_algorithm::Sha256);
    // Activate Compare Digest (should be true when comparing memory)
    icm_region_desc.region3.rcfg.set_cdwbn(true);
    // Digest Mismatch Interrupt Disable (enabled)
    icm_region_desc.region3.rcfg.set_dmien(false);
    // Wrap
    icm_region_desc.region3.rcfg.set_wrap(true);

    // Set Region Mismatch Interrupt
    icm_region3.set_rdm_int();

    // Modify regions to trigger interrupts
    message_region0_sha1[3] = 0xDEAD_BEEF;
    message_region1_sha1[4] = 0xDEAD_BEEF;
    message_region2_sha224[5] = 0xDEAD_BEEF;
    message_region3_sha256[6] = 0xDEAD_BEEF;

    icm.enable();

    // Just show that the test has completed
    let pkg = SetupPackage::new(
        unsafe { Peripherals::steal() },
        CorePeripherals::take().unwrap(),
    );

    // Show stuff
    Screens::new(pkg.display, pkg.buttons).test_complete();
}
