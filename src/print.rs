use crate::bsp::critical_section;
use crate::bsp::CONSOLE;
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::console::interface::Write;
    critical_section(|_| {
        CONSOLE
            .lock()
            .borrow_mut()
            .as_mut()
            .unwrap()
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
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}
