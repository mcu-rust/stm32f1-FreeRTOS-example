#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;
use freertos_rust::*;
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    Task::new()
        .name("default")
        .stack_size(4000 / 4)
        .start(move |_| {
            app_main();
        })
        .unwrap();
    FreeRtosUtils::start_scheduler();
}

fn app_main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    // let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let _clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);

    let mut gpio = dp.GPIOB.split();

    let mut led = gpio
        .pb0
        .into_open_drain_output_with_state(&mut gpio.crl, PinState::High);

    let mut _cnt: u32 = 0;

    loop {
        led.set_high();
        CurrentTask::delay(Duration::ms(500));
        led.set_low();
        CurrentTask::delay(Duration::ms(500));
        _cnt += 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    asm::bkpt();
    loop {}
}
