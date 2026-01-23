use freertos_build::prelude::*;

fn main() {
    let mut b = freertos_build::Builder::new();
    b.cpu_clock(72.MHz());
    b.heap_size(16 * 1024);
    b.use_preemption(true);
    b.max_task_priorities(5);
    b.interrupt_priority_bits(4, 5, 15);
    b.minimal_stack_size(40);
    b.max_task_name_len(8);
    b.compile().unwrap();
}
