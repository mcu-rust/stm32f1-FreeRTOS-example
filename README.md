# STM32F1-FreeRTOS-Example

This is example code showing how to use [stm32f1-hal](https://crates.io/crates/stm32f1-hal) and [freertos-next](https://crates.io/crates/freertos-next) together.

It also uses FreeRTOS-Kernel at V11.2.0 as submodule.

## Usage
1. Install Rust toolchain.
2. Run `rustup target add thumbv7m-none-eabi`
    1. You may need to install `gcc-arm-none-eabi`, use `apt` on Ubuntu or `scoop` on Windows.
2. Clone the code.
2. Run `git submodule update --init`
3. Run `cargo check` or use VSCode build task.
