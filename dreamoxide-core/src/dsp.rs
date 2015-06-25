pub use MappedIO;
pub use MemoryRange;
pub use Memory;

pub use std::sync::mpsc::{ Sender, Receiver, channel };

pub struct Dsp {
    pub receiver: Receiver<(usize, Option<u32>)>,
    pub sender: Sender<u32>,
    pub av_ctrl: u32,
}

impl Dsp {
    /// Creates a new bus controller and registers
    /// its mapped region with the memory controller
    pub fn new(mem: &mut Memory) -> Dsp {
        // Create the channels
        let (stx, srx): (Sender<(usize, Option<u32>)>, Receiver<(usize, Option<u32>)>) = channel();
        let (rtx, rrx): (Sender<u32>, Receiver<u32>) = channel();

        // Define the mapped region
        let mapped = MappedIO {
            range: MemoryRange(0x00702c00, 0x00702c00),
            sender: stx,
            receiver: rrx
        };

        // Register the mapped region
        mem.register_mapped_io(mapped);

        // Create the controller
        Dsp {
            receiver: srx,
            sender: rtx,
            av_ctrl: 1,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                (address, Some(value)) => {
                    match address {
                        0x00702C00 => self.write_av_ctrl(value),
                        _          => ()
                    }
                },
                (address, None) => {
                    let answer = match address {
                        0x00702C00 => self.read_av_ctrl(),
                        _          => 0
                    };

                    self.sender.send(answer).unwrap();
                }
            }
        }
    }

    /// Read from port data register A
    fn read_av_ctrl(&mut self) -> u32 {
        self.av_ctrl
    }

    fn write_av_ctrl(&mut self, value: u32) {
        self.av_ctrl = value;
    }
}
