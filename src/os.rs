pub use freertos_next as freertos;
pub use stm32f1_hal::{
    self as hal, embedded_hal::digital::StatefulOutputPin, embedded_io, i2c::BusDevice, prelude::*,
    ringbuf,
};

use freertos::{FreeRTOS, TaskNotifier, os_type_alias};
use hal::{os_trait, timer::SysTickInstant};

pub type OS = FreeRTOS<TaskNotifier>;
os_type_alias!(OS);
