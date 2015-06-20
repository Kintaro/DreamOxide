use MemoryField;

use std::fs::File;
use std::io::Read;

pub struct Memory {
    pub data : Vec<MemoryField>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: (0..0x1800000).map(|_| MemoryField::MemoryCell(0)).collect() }
    }

    #[inline(always)]
    pub fn map(pointer: usize) -> usize {
        pointer & 0x1FFFFFFF
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        let offset = 1 - address % 2;

        if let &MemoryField::MemoryCell(val) = self.access(address) {
            ((val & (0xF << (8 * offset))) >> (8 * offset)) as u8
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn read_u16<'a>(&'a self, address: usize) -> &'a MemoryField {
        &self.data[address]
    }

    #[inline(always)]
    pub fn read_u16_mut<'a>(&'a mut self, address: usize) -> &'a mut MemoryField {
        &mut self.data[Memory::map(address) / 2]
    }

    #[inline(always)]
    pub fn read_u32(&self, address: usize) -> u32 {
        if let &MemoryField::MemoryCell(v1) = self.access(address) {
            if let &MemoryField::MemoryCell(v2) = self.access(address + 2) {
                ((v1 as u32) << 16) | (v2 as u32)
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn access<'a>(&'a self, address: usize) -> &'a MemoryField {
        &self.data[Memory::map(address) / 2]
    }

    pub fn access_mut<'a>(&'a mut self, address: usize) -> &'a mut MemoryField {
        &mut self.data[Memory::map(address) / 2]
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
            *self.access_mut(Memory::map(start + i)) = MemoryField::MemoryCell(v);
        }

        size
    }
}
