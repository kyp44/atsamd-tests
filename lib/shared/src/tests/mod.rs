// TODO: Once the PR gets merged, this will not longer need to be feature-gated
#[cfg(feature = "hal-rtc-test")]
pub mod hal_rtc;
#[cfg(feature = "hal-rtc-test")]
pub use hal_rtc::hal_rtc;
