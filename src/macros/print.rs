use crate::BoardSupportPackage;
use crate::target_board;
use core::fmt;

use crate::mut_spinlock;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::traits::console::interface::Write;
    target_board::critical_section(|_| {
        mut_spinlock!(target_board::get_console())
            .write_fmt(args)
    });
}

/// Prints without a newline.
///
/// Carbon copy from https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::macros::print::_print(format_args_nl!($($arg)*));
    })
}
