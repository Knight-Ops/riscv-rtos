# RISCV RTOS
Super basic boot loader for Hifive1 Rev B/FE310 and Maix M1W/K210

## Current issues
[] I do not like the `.cargo/config` setup. The linker scripts shouldn't be here because we can target other boards, but this seems like the only way to pass in -C link-args into `cargo objcopy` maybe these should be in a `build.rs`? Something to look into