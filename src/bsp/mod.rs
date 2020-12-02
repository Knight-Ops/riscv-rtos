#[cfg(feature = "bsp_hifive")]
pub mod hifive;
#[cfg(feature = "bsp_hifive")]
pub use hifive::*;

#[cfg(feature = "bsp_maix_m1w")]
pub mod maix_m1w;
#[cfg(feature = "bsp_maix_m1w")]
pub use maix_m1w::*;
