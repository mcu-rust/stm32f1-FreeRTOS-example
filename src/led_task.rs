use crate::os::*;

pub struct LedTask<P> {
    led: P,
    interval: Timeout,
}

impl<P> LedTask<P>
where
    P: StatefulOutputPin,
{
    pub fn new(led: P) -> Self {
        Self {
            led,
            interval: Timeout::millis(500),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            // test long interval timeout
            if self.interval.timeout() {
                self.led.toggle().ok();
            }

            OS::delay().delay_ms(100);
        }
    }
}
