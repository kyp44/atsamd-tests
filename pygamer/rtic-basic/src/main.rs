//! A basic test of the RTC-based RTIC monotonic.
#![no_std]
#![no_main]

use bsp::RedLed;
use embedded_graphics::prelude::*;
use hal::prelude::*;
use rtic::Mutex;
use shared_pygamer::prelude::*;

const BASE_PERIOD_MS: u32 = 250;

async fn test_task<D: Mutex<T = DisplayDriver>>(
    mut display: D,
    task_num: u8,
    cycle_time_ms: u32,
    task_id: Option<u32>,
) -> ! {
    let mut cycles: usize = 0;

    loop {
        display.lock(|d| {
            let style = d.display_text_style(Point::new(10, 20 * (task_num + 1) as i32));

            write!(
                DisplayWriter::new(d, style),
                "Task number {task_num} cycles: {cycles}",
            )
            .unwrap();
        });

        match task_id {
            Some(_id) => {
                Mono::delay_ms(cycle_time_ms).await;
            }
            None => Mono::delay_ms(cycle_time_ms).await,
        }

        cycles = cycles.wrapping_add(1);
    }
}

#[rtic::app(device = pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;

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
        let rtc = pkg.setup_rtc_clock().unwrap();

        // Start the monotonic
        Mono::general_start(pkg.delay.free(), rtc);

        // Display selected monotonic and clock
        display_monotonic_info(&mut pkg.display);

        test_1::spawn().ok().unwrap();
        test_2::spawn().ok().unwrap();
        test_3::spawn().ok().unwrap();
        test_4::spawn().ok().unwrap();

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
    async fn test_1(cx: test_1::Context) {
        test_task(cx.shared.display, 1, BASE_PERIOD_MS, Some(1)).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_2(cx: test_2::Context) {
        test_task(cx.shared.display, 2, BASE_PERIOD_MS * 2, Some(2)).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_3(cx: test_3::Context) {
        test_task(cx.shared.display, 3, BASE_PERIOD_MS * 3, Some(3)).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_4(cx: test_4::Context) {
        test_task(cx.shared.display, 4, BASE_PERIOD_MS * 4, Some(4)).await;
    }
}
