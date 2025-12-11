use crate::os::{
    embedded_io::{Read, Write},
    ringbuf::{Consumer, Producer, RingBuffer},
    *,
};

pub struct UartTask;
impl UartTask {
    pub fn new<R, W>(tx: W, rx: R, size: usize) -> (UartTxTask<W>, UartRxTask<R>)
    where
        W: Write,
        R: Read,
    {
        let (w, r) = RingBuffer::new(size);
        (UartTxTask { tx, r }, UartRxTask { rx, w })
    }
}

pub struct UartTxTask<W: Write> {
    tx: W,
    r: Consumer<u8>,
}

impl<W: Write> UartTxTask<W> {
    pub fn run(&mut self) -> ! {
        loop {
            if let Some(chunk) = self.r.get_read_chunk() {
                if let Ok(size) = self.tx.write(chunk.get_slice()) {
                    chunk.commit(size);
                }
            }
            OS::delay().delay_ms(1);
        }
    }
}

pub struct UartRxTask<R: Read> {
    rx: R,
    w: Producer<u8>,
}

impl<R: Read> UartRxTask<R> {
    pub fn run(&mut self) -> ! {
        loop {
            if let Some(mut chunk) = self.w.get_write_chunk_uninit() {
                if let Ok(size) = self.rx.read(chunk.get_mut_slice()) {
                    unsafe { chunk.commit(size) }
                }
            }
            OS::delay().delay_ms(1);
        }
    }
}
