use MemoryField;

use std::fs::File;
use std::io::Read;

pub struct Memory {
    pub data : Vec<MemoryField>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: (0..0x1FFFFF).map(|_| MemoryField::MemoryCell(0)).collect() }
    }

    #[inline(always)]
    pub fn read_u16<'a>(&'a self, address: usize) -> &'a MemoryField {
        &self.data[address]
    }

    #[inline(always)]
    pub fn read_u16_mut<'a>(&'a mut self, address: usize) -> &'a mut MemoryField {
        &mut self.data[address]
    }

    pub fn read_from_file(&mut self, name: &str, start: usize) -> usize {
        let mut f = File::open(name).unwrap();
        let mut fdata : Vec<u8> = Vec::new();
        let mut size = f.read_to_end(&mut fdata).unwrap();

        println!("Loading {} into memory ({})", name, size);

        for i in (0..size).step_by(2) {
            let h : u8 = fdata[i];
            let l : u8 = fdata[i + 1];
            let v = ((l as u16) << 8) + (h as u16);
            self.data[start + i / 2] = MemoryField::MemoryCell(v);
        }

        size
    }
}
