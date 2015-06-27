pub use MappedIO;
pub use MemoryRange;
pub use Memory;

pub use std::sync::mpsc::{ Sender, Receiver, channel };

pub struct Bsc {
    pub receiver: Receiver<(usize, Option<u32>)>,
    pub sender: Sender<u32>,
    pub pctra: u32,
    pub pdtra: u16,
    pub pctrb: u32,
    pub pdtrb: u16,
}

impl Bsc {
    /// Creates a new bus controller and registers
    /// its mapped region with the memory controller
    pub fn new(mem: &mut Memory) -> Bsc {
        // Create the channels
        let (stx, srx): (Sender<(usize, Option<u32>)>, Receiver<(usize, Option<u32>)>) = channel();
        let (rtx, rrx): (Sender<u32>, Receiver<u32>) = channel();

        // Define the mapped region
        let mapped = MappedIO {
            range: MemoryRange(0x1f80002c, 0x1f800048),
            sender: stx,
            receiver: rrx
        };

        // Register the mapped region
        mem.register_mapped_io(mapped);

        // Create the controller
        Bsc {
            receiver: srx,
            sender: rtx,
            pctra: 0,
            pdtra: 0,
            pctrb: 0,
            pdtrb: 0
        }
    }

    #[inline(always)]
    pub fn run(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                (address, Some(value)) => {
                    match address {
                        0x1f80002c => self.write_pctra(value),
                        0x1f800030 => self.write_pdtra(value as u16),
                        0x1f800040 => self.write_pctrb(value),
                        0x1f800044 => self.write_pdtrb(value as u16),
                        _          => ()
                    }
                },
                (address, None) => {
                    let answer = match address {
                        0x1f80002c => self.pctra,
                        0x1f800030 => self.read_pdtra() as u32,
                        0x1f800040 => self.pctrb,
                        0x1f800044 => self.read_pdtrb() as u32,
                        _          => 0
                    };

                    self.sender.send(answer).unwrap();
                }
            }
        }
    }

    /// Read from port data register A
    #[inline(always)]
    fn read_pdtra(&mut self) -> u16 {
        let mut input_mask = 0;
        let mut output_mask = 0;

        for i in (0..16) {
            let bits = (self.pctra >> (i << 1)) & 0x03;
            if bits == 2 { input_mask |= (1 << i); }
            else if bits != 0 { output_mask |= (1 << i); }
        }

        if (self.pdtra | !output_mask) & 0x3 == 0x3 {
            self.pdtra |= 0x3;
        } else {
            self.pdtra &= !0x3;
        }

        (0x300 & input_mask) | self.pdtra
    }

    /// Read from port data register B
    #[inline(always)]
    fn read_pdtrb(&mut self) -> u16 {
        let mut input_mask = 0;
        let mut output_mask = 0;

        for i in (0..16) {
            let bits = (self.pctrb >> (i << 1)) & 0x03;
            if bits == 2 { input_mask |= (1 << i); }
            else if bits != 0 { output_mask |= (1 << i); }
        }

        if (self.pdtrb | !output_mask) & 0x3 == 0x3 {
            self.pdtrb |= 0x3;
        } else {
            self.pdtrb &= !0x3;
        }

        self.pdtrb
    }

    #[inline(always)]
    fn write_pctra(&mut self, value: u32) {
        self.pctra = value;
    }

    #[inline(always)]
    fn write_pctrb(&mut self, value: u32) {
        self.pctrb = value;
    }

    #[inline(always)]
    fn write_pdtra(&mut self, value: u16) {
        self.pdtra = value;
    }

    #[inline(always)]
    fn write_pdtrb(&mut self, value: u16) {
        self.pdtrb = value;
    }
}
