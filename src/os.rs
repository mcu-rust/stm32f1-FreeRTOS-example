pub use freertos_next;
pub use stm32f1_hal::{
    self as hal, embedded_hal::digital::StatefulOutputPin, embedded_io, prelude::*, ringbuf,
};

pub type OS = FreeRTOS<SysTickInstant, TaskNotifier>;
pub type Mutex<T> = os_trait::Mutex<OS, T>;

use freertos_next::{FreeRTOS, TaskNotifier};
use hal::{os_trait, timer::SysTickInstant};
