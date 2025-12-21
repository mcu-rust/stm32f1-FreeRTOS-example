pub use freertos_next as freertos;
pub use stm32f1_hal::{
    self as hal, embedded_hal::digital::StatefulOutputPin, embedded_io, prelude::*, ringbuf,
};

pub type OS = FreeRTOS<TaskNotifier>;
pub type Mutex<T> = os_trait::Mutex<OS, T>;
pub type OsTimeoutState = <OS as OsInterface>::TimeoutState;

use freertos::{FreeRTOS, TaskNotifier};
use hal::{os_trait, timer::SysTickInstant};
