use MemoryField;

use std::fs::File;
use std::io::Read;

pub struct Memory {
    pub data : Vec<MemoryField>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { 
            data: (0..0x20000000).map(|_| MemoryField::MemoryCell(0)).collect() }
    }

    #[inline(always)]
    pub fn map(pointer: usize) -> usize {
        pointer & 0x1FFFFFFF
    }

    #[inline(always)]
    pub fn read_u8(&self, address: usize) -> u8 {
        let offset = 1 - address % 2;

        if let &MemoryField::MemoryCell(val) = self.access(address) {
            ((val & (0xFF << (8 * offset))) >> (8 * offset)) as u8
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn read_u16(&self, address: usize) -> u16 {
        match self.access(address) {
            &MemoryField::MemoryCell(v) => v,
            _ => panic!("Can only read from memory cell!")
        }
    }

    #[inline(always)]
    pub fn read_u32(&self, address: usize) -> u32 {
        let v1 = self.read_u16(address) as u32;
        let v2 = self.read_u16(address + 2) as u32;

        (v2 << 16) | v1
    }

    #[inline(always)]
    pub fn write_u8(&mut self, address: usize, value: u8) {
        let offset = 1 - address % 2;
        let v = self.read_u8(address) as u16;
        let mask = 0xFF << (offset * 8);
        let w = (value as u16) << ((1 - offset) * 8);

        *self.access_mut(address) = MemoryField::MemoryCell((v & mask) | w);
    }

    #[inline(always)]
    pub fn write_u16(&mut self, address: usize, value: u16) {
        if address == 0x8c000122 {
            println!("Writing {:4x}", value);
        }
        *self.access_mut(address) = MemoryField::MemoryCell(value);
    }

    #[inline(always)]
    pub fn write_u32(&mut self, address: usize, value: u32) {
        let v1 = MemoryField::MemoryCell((value >> 16) as u16);
        let v2 = MemoryField::MemoryCell((value & 0x0000FFFF) as u16);

        *self.access_mut(address) = v2;
        *self.access_mut(address + 2) = v1;
    }

    #[inline(always)]
    pub fn access<'a>(&'a self, address: usize) -> &'a MemoryField {
        &self.data[Memory::map(address) / 2]
    }

    #[inline(always)]
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
