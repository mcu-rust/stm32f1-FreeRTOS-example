use crate::os::*;
use embedded_hal::spi::{Operation, SpiDevice};

// W25Q
const REG_READ_ID: u8 = 0x9F;

pub struct SpiTask<D> {
    dev: D,
    interval: Timeout,
    buf: [u8; 4],
}

impl<D: SpiDevice> SpiTask<D> {
    pub fn new(dev: D) -> Self {
        Self {
            dev,
            interval: Timeout::millis(100),
            buf: [0; 4],
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            if self.interval.timeout() {
                self.dev
                    .transaction(&mut [
                        Operation::Write(&[REG_READ_ID]),
                        Operation::Read(&mut self.buf[..3]),
                    ])
                    .unwrap();
            }
        }
    }
}
