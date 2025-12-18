# STM32F1-FreeRTOS-Example

This is example code showing how to use [stm32f1-hal](https://crates.io/crates/stm32f1-hal) and [freertos-next](https://crates.io/crates/freertos-next) together.

It also uses FreeRTOS-Kernel at V11.2.0 as submodule.

## Usage
1. Install Rust toolchain.
2. Run `rustup target add thumbv7m-none-eabi`
    1. You may need to install `gcc-arm-none-eabi`, use `apt` on Ubuntu or `scoop` on Windows.
2. Clone this repository.
3. Run `cargo check` or use VSCode build task.
4. Run debug in VSCode.
    1. If you are using a different debugger, please modify `.vscode/launch.json` as needed.
