use crate::os::*;

pub struct LedTask<P> {
    led: P,
}

impl<P> LedTask<P>
where
    P: StatefulOutputPin,
{
    pub fn new(led: P) -> Self {
        Self { led }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.led.toggle().ok();
            OS::delay().delay_ms(200);
        }
    }
}
