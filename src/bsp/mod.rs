#[cfg(all(feature = "bsp_hifive", not(feature = "bsp_maix_m1w")))]
pub mod hifive;
#[cfg(all(feature = "bsp_hifive", not(feature = "bsp_maix_m1w")))]
pub use hifive::*;
#[cfg(all(feature = "bsp_hifive", not(feature = "bsp_maix_m1w")))]
pub use hifive::Hifive1RevB as target_board;

#[cfg(all(feature = "bsp_maix_m1w", not(feature = "bsp_hifive")))]
pub mod maix_m1w;
#[cfg(all(feature = "bsp_maix_m1w", not(feature = "bsp_hifive")))]
pub use maix_m1w::*;
#[cfg(all(feature = "bsp_maix_m1w", not(feature = "bsp_hifive")))]
pub use maix_m1w::MaixM1W as target_board;