#![no_std]
#![no_main]

use embedded_graphics::prelude::*;
use hal::prelude::*;
use shared_pygamer::prelude::*;

#[rtic::app(device = pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;
    use bsp::RedLed;
    use hal::delay::Delay;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        red_led: RedLed,
        display: DisplayDriver,
        delay: Delay,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut pkg = setup(cx.device, cx.core);

        // Start the monotonic
        Mono::start(pkg.rtc);

        // Display selected monotonic and clock
        display_monotonic_info(&mut pkg.display);

        test_1::spawn().ok().unwrap();

        (
            Shared {},
            Local {
                red_led: pkg.red_led,
                display: pkg.display,
                delay: pkg.delay,
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

    #[task(priority = 1, local=[display, delay])]
    async fn test_1(cx: test_1::Context) {
        const DELAY_MS: u32 = 1000;

        let mut cycles: usize = 0;

        loop {
            let style = cx.local.display.display_text_style(Point::new(10, 20 * 2));

            write!(
                DisplayWriter::new(cx.local.display, style),
                "Task cycles: {cycles}",
            )
            .unwrap();

            // Async delay, to add to timer queue and pend
            #[cfg(feature = "debug")]
            Mono::delay_ms_debug(DELAY_MS, 1).await;
            #[cfg(not(feature = "debug"))]
            Mono::delay_ms(DELAY_MS).await;

            // Blocking delay, during which the RTC should be disabled
            cx.local.delay.delay_ms(DELAY_MS);

            cycles = cycles.wrapping_add(1);
        }
    }
}
