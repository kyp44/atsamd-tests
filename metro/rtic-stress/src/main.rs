//! Program to test the RTC-based embassy time driver.
//!
//! Refer to `atsamd-hal` [PR 825](https://github.com/atsamd-rs/atsamd/pull/825).
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use hal::prelude::*;
use shared_metro::prelude::*;
use tasks::test_task;

mod tasks;

const BASE_PERIOD_MS: u32 = 1000;

#[rtic::app(device = pac, dispatchers = [TCC0])]
mod app {
    use super::*;
    use bsp::RedLed;

    #[shared]
    struct Shared {
        display: DisplayDriver,
    }

    #[local]
    struct Local {
        red_led: RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut pkg = SetupPackage::new(cx.device, cx.core);
        let (rtc, _) = pkg.setup_rtc_clock().unwrap();

        // Start the monotonic
        Mono::general_start(pkg.delay.free(), rtc);

        // Display selected monotonic and clock
        display_monotonic_info(&mut pkg.display);

        // Spawn tasks
        // NOTE: This does not work with more than 7 tasks, likely for memory reasons.
        clock_task::spawn().unwrap();
        spawn_tasks();

        (
            Shared {
                display: pkg.display,
            },
            Local {
                red_led: pkg.red_led,
            },
        )
    }

    #[idle(local = [red_led])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
            cx.local.red_led.toggle().unwrap();
        }
    }

    #[task(priority = 1, shared=[display])]
    async fn clock_task(cx: clock_task::Context) {
        tasks::clock_task(cx.shared.display).await
    }

    #[inline]
    fn spawn_tasks() {
        task_1::spawn().ok().unwrap();
        task_2::spawn().ok().unwrap();
        task_3::spawn().ok().unwrap();
        task_4::spawn().ok().unwrap();
        task_5::spawn().ok().unwrap();
        task_6::spawn().ok().unwrap();
        task_7::spawn().ok().unwrap();
    }

    #[task(priority = 1, shared=[display])]
    async fn task_1(cx: task_1::Context) {
        test_task(cx.shared.display, 0, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn task_2(cx: task_2::Context) {
        test_task(cx.shared.display, 1, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn task_3(cx: task_3::Context) {
        test_task(cx.shared.display, 2, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn task_4(cx: task_4::Context) {
        test_task(cx.shared.display, 3, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn task_5(cx: task_5::Context) {
        test_task(cx.shared.display, 4, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn task_6(cx: task_6::Context) {
        test_task(cx.shared.display, 5, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn task_7(cx: task_7::Context) {
        test_task(cx.shared.display, 6, BASE_PERIOD_MS).await
    }
}
