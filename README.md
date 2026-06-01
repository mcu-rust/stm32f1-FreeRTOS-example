# STM32F1-FreeRTOS-Example

This is example code showing how to use [stm32f1-hal](https://crates.io/crates/stm32f1-hal) and [freertos-next](https://crates.io/crates/freertos-next) together.

## Usage
1. Install `gcc-arm-none-eabi` for compiling FreeRtos-Kernel C source code. Use `apt` on Ubuntu or `scoop` on Windows
1. Install Rust toolchain.
2. Run `rustup target add thumbv7m-none-eabi`
3. Run `cargo install flip-link`
2. Clone this repository.
3. Run `cargo check` or use VSCode build task.
4. Run debug in VSCode.
    1. If you are using a different debugger, please modify `.vscode/launch.json` as needed.
