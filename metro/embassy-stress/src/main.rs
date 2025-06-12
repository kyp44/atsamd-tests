//! Program to test the RTC-based embassy time driver.
//!
//! Refer to `atsamd-hal` [PR 825](https://github.com/atsamd-rs/atsamd/pull/825).
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use hal::prelude::*;
use shared_metro::prelude::*;
use static_cell::StaticCell;
use tasks::test_task;

mod tasks;

const BASE_PERIOD_MS: u64 = 1000;

hal::embassy_time!(Mono);

type SharedDisplay = Mutex<NoopRawMutex, DisplayDriver>;
static DISPLAY: StaticCell<SharedDisplay> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Setup stuff
    let mut pkg = SetupPackage::new(
        Peripherals::take().unwrap(),
        CorePeripherals::take().unwrap(),
    );
    let (_, rtc_clock) = pkg.setup_rtc_clock().unwrap();

    let display = DISPLAY.init(Mutex::new(pkg.display));

    // Intialize the time driver
    unsafe { Mono::init(rtc_clock) };

    pkg.red_led.set_high().unwrap();

    spawn_tasks(spawner, display);

    // TOOD: Enhacements could include showing counter and selected clock
    // like the pygamer RTIC version, which should of course be abstracted out is used.
    // Counter can be obtained from `embassy_time::Instant::now()::as_ticks()`
}

#[inline]
fn spawn_tasks(spawner: Spawner, display: &'static SharedDisplay) {
    spawner.must_spawn(test_task_1(display));
    spawner.must_spawn(test_task_2(display));
    spawner.must_spawn(test_task_3(display));
    spawner.must_spawn(test_task_4(display));
    spawner.must_spawn(test_task_5(display));
    spawner.must_spawn(test_task_6(display));
    spawner.must_spawn(test_task_7(display));
    spawner.must_spawn(test_task_8(display));
    spawner.must_spawn(test_task_9(display));
    spawner.must_spawn(test_task_10(display));
    spawner.must_spawn(test_task_11(display));
    spawner.must_spawn(test_task_12(display));
    spawner.must_spawn(test_task_13(display));
    spawner.must_spawn(test_task_14(display));
    spawner.must_spawn(test_task_15(display));
    spawner.must_spawn(test_task_16(display));
    spawner.must_spawn(test_task_17(display));
    spawner.must_spawn(test_task_18(display));
    spawner.must_spawn(test_task_19(display));
    spawner.must_spawn(test_task_20(display));
    spawner.must_spawn(test_task_21(display));
    spawner.must_spawn(test_task_22(display));
    spawner.must_spawn(test_task_23(display));
    spawner.must_spawn(test_task_24(display));
    spawner.must_spawn(test_task_25(display));
    spawner.must_spawn(test_task_26(display));
    spawner.must_spawn(test_task_27(display));
    spawner.must_spawn(test_task_28(display));
    spawner.must_spawn(test_task_29(display));
    spawner.must_spawn(test_task_30(display));
    spawner.must_spawn(test_task_31(display));
    spawner.must_spawn(test_task_32(display));
    spawner.must_spawn(test_task_33(display));
    spawner.must_spawn(test_task_34(display));
    spawner.must_spawn(test_task_35(display));
    spawner.must_spawn(test_task_36(display));
    spawner.must_spawn(test_task_37(display));
    spawner.must_spawn(test_task_38(display));
    spawner.must_spawn(test_task_39(display));
    spawner.must_spawn(test_task_40(display));
    spawner.must_spawn(test_task_41(display));
    spawner.must_spawn(test_task_42(display));
    spawner.must_spawn(test_task_43(display));
    spawner.must_spawn(test_task_44(display));
    spawner.must_spawn(test_task_45(display));
    spawner.must_spawn(test_task_46(display));
    spawner.must_spawn(test_task_47(display));
    spawner.must_spawn(test_task_48(display));
    spawner.must_spawn(test_task_49(display));
    spawner.must_spawn(test_task_50(display));
    spawner.must_spawn(test_task_51(display));
    spawner.must_spawn(test_task_52(display));
    spawner.must_spawn(test_task_53(display));
    spawner.must_spawn(test_task_54(display));
    spawner.must_spawn(test_task_55(display));
    spawner.must_spawn(test_task_56(display));
    spawner.must_spawn(test_task_57(display));
    spawner.must_spawn(test_task_58(display));
    spawner.must_spawn(test_task_59(display));
    spawner.must_spawn(test_task_60(display));
}

#[embassy_executor::task]
pub async fn test_task_1(display: &'static SharedDisplay) -> ! {
    test_task(display, 0, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_2(display: &'static SharedDisplay) -> ! {
    test_task(display, 1, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_3(display: &'static SharedDisplay) -> ! {
    test_task(display, 2, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_4(display: &'static SharedDisplay) -> ! {
    test_task(display, 3, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_5(display: &'static SharedDisplay) -> ! {
    test_task(display, 4, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_6(display: &'static SharedDisplay) -> ! {
    test_task(display, 5, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_7(display: &'static SharedDisplay) -> ! {
    test_task(display, 6, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_8(display: &'static SharedDisplay) -> ! {
    test_task(display, 7, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_9(display: &'static SharedDisplay) -> ! {
    test_task(display, 8, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_10(display: &'static SharedDisplay) -> ! {
    test_task(display, 9, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_11(display: &'static SharedDisplay) -> ! {
    test_task(display, 10, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_12(display: &'static SharedDisplay) -> ! {
    test_task(display, 11, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_13(display: &'static SharedDisplay) -> ! {
    test_task(display, 12, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_14(display: &'static SharedDisplay) -> ! {
    test_task(display, 13, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_15(display: &'static SharedDisplay) -> ! {
    test_task(display, 14, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_16(display: &'static SharedDisplay) -> ! {
    test_task(display, 15, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_17(display: &'static SharedDisplay) -> ! {
    test_task(display, 16, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_18(display: &'static SharedDisplay) -> ! {
    test_task(display, 17, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_19(display: &'static SharedDisplay) -> ! {
    test_task(display, 18, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_20(display: &'static SharedDisplay) -> ! {
    test_task(display, 19, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_21(display: &'static SharedDisplay) -> ! {
    test_task(display, 20, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_22(display: &'static SharedDisplay) -> ! {
    test_task(display, 21, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_23(display: &'static SharedDisplay) -> ! {
    test_task(display, 22, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_24(display: &'static SharedDisplay) -> ! {
    test_task(display, 23, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_25(display: &'static SharedDisplay) -> ! {
    test_task(display, 24, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_26(display: &'static SharedDisplay) -> ! {
    test_task(display, 25, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_27(display: &'static SharedDisplay) -> ! {
    test_task(display, 26, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_28(display: &'static SharedDisplay) -> ! {
    test_task(display, 27, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_29(display: &'static SharedDisplay) -> ! {
    test_task(display, 28, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_30(display: &'static SharedDisplay) -> ! {
    test_task(display, 29, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_31(display: &'static SharedDisplay) -> ! {
    test_task(display, 30, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_32(display: &'static SharedDisplay) -> ! {
    test_task(display, 31, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_33(display: &'static SharedDisplay) -> ! {
    test_task(display, 32, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_34(display: &'static SharedDisplay) -> ! {
    test_task(display, 33, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_35(display: &'static SharedDisplay) -> ! {
    test_task(display, 34, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_36(display: &'static SharedDisplay) -> ! {
    test_task(display, 35, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_37(display: &'static SharedDisplay) -> ! {
    test_task(display, 36, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_38(display: &'static SharedDisplay) -> ! {
    test_task(display, 37, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_39(display: &'static SharedDisplay) -> ! {
    test_task(display, 38, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_40(display: &'static SharedDisplay) -> ! {
    test_task(display, 39, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_41(display: &'static SharedDisplay) -> ! {
    test_task(display, 40, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_42(display: &'static SharedDisplay) -> ! {
    test_task(display, 41, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_43(display: &'static SharedDisplay) -> ! {
    test_task(display, 42, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_44(display: &'static SharedDisplay) -> ! {
    test_task(display, 43, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_45(display: &'static SharedDisplay) -> ! {
    test_task(display, 44, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_46(display: &'static SharedDisplay) -> ! {
    test_task(display, 45, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_47(display: &'static SharedDisplay) -> ! {
    test_task(display, 46, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_48(display: &'static SharedDisplay) -> ! {
    test_task(display, 47, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_49(display: &'static SharedDisplay) -> ! {
    test_task(display, 48, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_50(display: &'static SharedDisplay) -> ! {
    test_task(display, 49, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_51(display: &'static SharedDisplay) -> ! {
    test_task(display, 50, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_52(display: &'static SharedDisplay) -> ! {
    test_task(display, 51, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_53(display: &'static SharedDisplay) -> ! {
    test_task(display, 52, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_54(display: &'static SharedDisplay) -> ! {
    test_task(display, 53, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_55(display: &'static SharedDisplay) -> ! {
    test_task(display, 54, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_56(display: &'static SharedDisplay) -> ! {
    test_task(display, 55, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_57(display: &'static SharedDisplay) -> ! {
    test_task(display, 56, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_58(display: &'static SharedDisplay) -> ! {
    test_task(display, 57, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_59(display: &'static SharedDisplay) -> ! {
    test_task(display, 58, BASE_PERIOD_MS).await;
}

#[embassy_executor::task]
pub async fn test_task_60(display: &'static SharedDisplay) -> ! {
    test_task(display, 59, BASE_PERIOD_MS).await;
}
