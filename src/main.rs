#![no_std]
#![no_main]
#![allow(dead_code)]

mod i2c_task;
mod led_task;
mod os;
mod uart_task;

use i2c_task::I2cTask;
use led_task::LedTask;
use os::*;
use uart_task::UartTask;

use core::panic::PanicInfo;
use freertos_next::*;
use hal::{
    Mcu,
    cortex_m::asm,
    cortex_m_rt::entry,
    dma::DmaPriority,
    gpio::PinState,
    i2c,
    pac::{self, Interrupt},
    rcc, uart,
};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    Task::new()
        .name("default")
        .stack_size_bytes(2000)
        .start(move |_| {
            app_main();
        })
        .unwrap();
    FreeRtosUtils::start_scheduler();
}

fn app_main() -> ! {
    // clock --------------------------------------------------------

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.init();
    let sysclk = 72.MHz();
    let mut rcc = dp
        .RCC
        .init()
        .freeze(rcc::Config::hse(8.MHz()).sysclk(sysclk), &mut flash.acr);
    assert_eq!(rcc.clocks().sysclk(), sysclk);
    cp.SYST.init_sys_tick_instant();

    // Prepare ------------------------------------------------------

    let afio = dp.AFIO.init(&mut rcc);
    let mut mcu = Mcu::new(rcc, afio, cp.SCB.init(), cp.NVIC.init(), dp.EXTI);

    // The number of `priority` must be >= `configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY`,
    // which means it's lower than the limit, unless you are 100% sure
    // that the interrupt function doesn't call any OS APIs.
    mcu.nvic.set_priority(Interrupt::I2C1_EV, 5, true);
    mcu.nvic.set_priority(Interrupt::I2C1_ER, 5, true);
    mcu.nvic.set_priority(Interrupt::DMA1_CHANNEL4, 6, true);
    mcu.nvic.set_priority(Interrupt::DMA1_CHANNEL5, 6, true);

    // Peripherals --------------------------------------------------

    let mut gpioa = dp.GPIOA.split(&mut mcu.rcc);
    let mut gpiob = dp.GPIOB.split(&mut mcu.rcc);
    let dma1 = dp.DMA1.split(&mut mcu.rcc);

    // Blink --------------------------------------------------------

    let led = gpiob
        .pb0
        .into_open_drain_output_with_state(&mut gpiob.crl, PinState::High);
    let mut led = LedTask::new(led);

    Task::new()
        .name("LEd")
        .stack_size_bytes(500)
        .start(move |_| led.run())
        .unwrap();

    // UART ---------------------------------------------------------

    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);
    let mut dma_tx = dma1.4;
    let mut dma_rx = dma1.5;
    dma_tx.set_priority(DmaPriority::Medium);
    dma_rx.set_priority(DmaPriority::Medium);

    let config = uart::Config::default();
    let (Some(uart_tx), Some(uart_rx)) =
        dp.USART1
            .init::<OS>(&mut mcu)
            .into_tx_rx((pin_tx, pin_rx), config, &mut mcu)
    else {
        panic!()
    };
    let (uart_rx, mut rx_it) = uart_rx.into_dma_circle(dma_rx, 64, 10.millis());
    let (uart_tx, mut tx_it) = uart_tx.into_dma_ringbuf(dma_tx, 32, 10.millis());
    all_it::DMA1_CH4_CB.set(&mut mcu, move || {
        tx_it.interrupt_reload();
    });
    all_it::DMA1_CH5_CB.set(&mut mcu, move || {
        rx_it.interrupt_notify();
    });

    let (mut tx, mut rx) = UartTask::new(uart_tx, uart_rx, 32);

    Task::new()
        .name("TX")
        .stack_size_bytes(1000)
        .start(move |_| tx.run())
        .unwrap();

    // I2C ----------------------------------------------------------

    #[cfg(feature = "i2c")]
    {
        let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
        let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);
        let (bus, mut it, mut it_err) =
            dp.I2C1
                .init::<OS>(&mut mcu)
                .into_interrupt_bus((scl, sda), 4, &mut mcu);
        all_it::I2C1_EVENT_CB.set(&mut mcu, move || it.handler());
        all_it::I2C1_ERR_CB.set(&mut mcu, move || it_err.handler());
        let dev = bus.new_device(i2c::Address::Seven(0b1101000), 200.kHz());

        let mut i2c = I2cTask::new(dev);
        Task::new()
            .name("I2C")
            .stack_size_bytes(1000)
            .start(move |_| i2c.run())
            .unwrap();
    }

    rx.run()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    asm::bkpt();
    loop {}
}

mod all_it {
    use super::hal::interrupt_handler;
    interrupt_handler!(
        (DMA1_CHANNEL4, DMA1_CH4_CB),
        (DMA1_CHANNEL5, DMA1_CH5_CB),
        (I2C1_EV, I2C1_EVENT_CB),
        (I2C1_ER, I2C1_ERR_CB),
    );
}
