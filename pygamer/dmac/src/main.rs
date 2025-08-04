//! Tests `dmac` module documentation examples.
#![no_std]
#![no_main]

use hal::dmac::{
    DmaController, PriorityLevel, Transfer,
    dma_controller::{TriggerAction, TriggerSource},
};
use shared_pygamer::prelude::*;

#[entry]
fn main() -> ! {
    // Basic example
    let (dmac, ahb_clk) = {
        let mut peripherals = Peripherals::take().unwrap();
        let (mut _buses, clocks, _tokens) = hal::clock::v2::clock_system_at_reset(
            peripherals.oscctrl,
            peripherals.osc32kctrl,
            peripherals.gclk,
            peripherals.mclk,
            &mut peripherals.nvmctrl,
        );

        let mut dmac = DmaController::new(peripherals.dmac, clocks.ahbs.dmac);
        // Get individual handles to DMA channels
        let mut channels = dmac.split();

        // Initialize DMA Channel 0
        let chan0 = channels.0.init(PriorityLevel::Lvl0);

        // Initialize buffers
        const LENGTH: usize = 50;
        let buf_src: &'static mut [u8; LENGTH] =
            cortex_m::singleton!(: [u8; LENGTH] = [0xff; LENGTH]).unwrap();
        let buf_dest: &'static mut [u8; LENGTH] =
            cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

        // Setup a DMA transfer (memory-to-memory -> incrementing source, incrementing destination)
        // NOTE: buf_src and buf_dest should be either:
        // &'static mut T, &'static mut [T], or &'static mut [T; N] where T: BeatSize
        let xfer = Transfer::new(chan0, buf_src, buf_dest, false)
            .unwrap()
            .begin(TriggerSource::Disable, TriggerAction::Block);

        // Wait for transfer to complete and grab resulting buffers
        let (chan0, _buf_src, _buf_dest) = xfer.wait();

        // (Optional) free the [`DmaController`] struct and return the underlying resources
        channels.0 = chan0.into();
        let (dmac, ahb_clk) = dmac.free(channels);

        // EXAMPLE ENDS HERE
        (dmac, ahb_clk)
    };

    // Initialize DMA Channel 0
    let chan0 = DmaController::new(dmac, ahb_clk)
        .split()
        .0
        .init(PriorityLevel::Lvl0);

    // Initialize buffers
    const LENGTH: usize = 50;
    let buf_src: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0xff; LENGTH]).unwrap();
    let buf_dest: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

    let mut xfer = Transfer::new(chan0, buf_src, buf_dest, false)
        .unwrap()
        .begin(TriggerSource::Disable, TriggerAction::Block);

    // Transfer cycling example
    {
        const LENGTH: usize = 50;
        let new_source: &'static mut [u8; LENGTH] =
            cortex_m::singleton!(: [u8; LENGTH] = [0xff; LENGTH]).unwrap();
        let new_destination: &'static mut [u8; LENGTH] =
            cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

        // Assume xfer is a `Busy` `Transfer`
        let (_old_source, _old_dest) = xfer.recycle(new_source, new_destination).unwrap();
    }

    // Just show that the test has completed
    let pkg = SetupPackage::new(
        unsafe { Peripherals::steal() },
        CorePeripherals::take().unwrap(),
    );

    Screens::new(pkg.display, pkg.buttons).test_complete();
}
