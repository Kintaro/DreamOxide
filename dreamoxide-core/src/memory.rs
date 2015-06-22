use MemoryField;

use std::fs::File;
use std::io::Read;

pub struct Memory {
    pub data : Vec<MemoryField>
}

impl Memory {
    /// Creates a new memory unit. Although the addresses span the whole range
    /// of 32 bit, only 26MB are necessary.
    pub fn new() -> Memory {
        Memory {
            data: (0..0x20000000).map(|_| MemoryField::MemoryCell(0)).collect() }
    }

    /// Maps the given address to the correct address,
    /// taking care of mirrored areas and removing MMU
    /// flags from the actual pointer.
    #[inline(always)]
    pub fn map(pointer: usize) -> usize {
        let mapped = pointer & 0x1FFFFFFF;

        match mapped {
            // Boot and flash rom
            0x00000000 ... 0x03ffffff => mapped,
            // Video RAM
            0x04000000 ... 0x07ffffff => mapped,
            // Undefined
            0x08000000 ... 0x0bffffff => mapped,
            // System RAM
            0x10000000 ... 0x13ffffff => mapped,
            // Modem
            0x14000000 ... 0x17ffffff => mapped,
            // Internal I/O registers
            0x1c000000 ... 0x1fffffff => mapped,
            // PVR registers
            0xa05f8000 ... 0xa06fffff => mapped,
            // SPU registers
            0xa0700000 ... 0xa07fffff => mapped,
            // Sound RAM
            0xa0800000 ... 0xa09fffff => mapped,
            // Parallel Port
            0xa1000000 ... 0xa1ffffff => mapped,
            // GD Rom
            0xa2000000 ... 0xa4ffffff => mapped,
            // Mirror of VRAM
            0xa5000000 ... 0xa57fffff => mapped - 0xa1000000,
            // Mirror of memory mapped registers
            0xff000000 ... 0xffff0fff => mapped - 0xe0000000,
            _                         => mapped
        }
    }

    /// Sign-extends an unsigned byte to a signed integer
    pub fn sign_extend_u8(val: u8) -> i32 {
        val as i8 as i32
    }

    /// Sign-extends an unsigned word to a signed integer
    pub fn sign_extend_u16(val: u16) -> i32 {
        val as i16 as i32
    }

    /// Reads an unsigned byte from memory
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
    pub fn read_i8(&self, address: usize) -> i8 {
        self.read_u8(address) as i8
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
