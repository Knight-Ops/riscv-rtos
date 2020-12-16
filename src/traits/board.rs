use core::cell::RefCell;

use crate::{sync::Spinlock};
/// This trait represents everything we need from a BSP to create a bootloader
/// The bootloader code should be generic and shouldn't depend on anything specific to
/// a board
pub trait BoardSupportPackage {
    /// This is expected to set up everything required for println! to work
    fn early_init();

    /// Define the Wrapper type that implements crate::console::interface::All
    type ConsoleType: crate::traits::console::interface::All;
    /// Retrieve a reference (likely to a global) of the Console 
    fn get_console() -> &'static Spinlock<RefCell<Option<Self::ConsoleType>>>;

    type CriticalSectionType;
    fn critical_section<F, R>(f: F) -> R where F: FnOnce(&Self::CriticalSectionType) -> R;

}