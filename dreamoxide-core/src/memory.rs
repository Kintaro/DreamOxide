use MemoryField;

use std::fs::File;
use std::io::Read;
use std::usize;
use std::cmp;
use std::iter;
use latest::value::{Sender, Receiver};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryRange(pub usize, pub usize);

impl MemoryRange {
    /// Checks if the given address falls within the memory range
    #[inline]
    pub fn is_within(&self, address: usize) -> bool {
        let &MemoryRange(l, h) = self;

        address >= l && address <= h
    }
}

pub struct MappedIO {
    /// The memory address range for the mapped io region
    pub range: MemoryRange,
    /// The sending channel for issuing read/write commands
    /// to the mapped region
    pub sender: Sender<(usize, Option<u32>)>,
    /// THe receiving channel for reading from the mapped region
    pub receiver: Receiver<u32>
}

pub struct Memory {
    pub data : Box<[MemoryField]>,
    pub mapped: Vec<MappedIO>,
    pub min_mapped: usize,
    pub max_mapped: usize
}

impl Memory {
    /// Creates a new memory unit. Although the addresses span the whole range
    /// of 32 bit, only 26MB are necessary.
    pub fn new() -> Memory {
        Memory {
            //data: (0..0x20000000).map(|_| MemoryField::MemoryCell(0)).collect(),
            data: iter::repeat(MemoryField::MemoryCell(0)).take(0x20000000).collect::<Vec<MemoryField>>().into_boxed_slice(),
            mapped: Vec::new(),
            min_mapped: usize::MAX,
            max_mapped: 0
        }
    }

    /// Maps the given address to the correct address,
    /// taking care of mirrored areas and removing MMU
    /// flags from the actual pointer.
    //#[inline]
    pub fn map(pointer: usize) -> usize {
        // First remove the 3 most significant bits
        let mapped = pointer & 0x1FFFFFFF;
        // Then map to the designated region
        match mapped {
            // Mirror of VRAM
            0xa5000000 ... 0xa57fffff => mapped - 0x95000000,
            // Mirror of memory mapped registers
            0xff000000 ... 0xffffffff => mapped - 0xe0000000,
            _                         => mapped
        }
    }

    /// A first quick check if the pointer is within a guaranteed
    /// safe region or not.
    //#[inline]
    pub fn is_io_register(&self, pointer: usize) -> bool {
        pointer > self.min_mapped && pointer < self.max_mapped
    }

    pub fn register_mapped_io(&mut self, mapped: MappedIO) {
        let MemoryRange(mi, ma) = mapped.range;
        self.min_mapped = cmp::min(self.min_mapped, mi);
        self.max_mapped = cmp::max(self.max_mapped, ma);
        self.mapped.push(mapped);
        self.mapped.sort_by(|a, b| a.range.cmp(&b.range));
    }

    //#[inline(always)]
    pub fn try_mapped_write(&self, address: usize, value: u32) -> bool {
        let addr = Memory::map(address);
        if !self.is_io_register(addr) { return false; }
        match self.mapped.iter().find(|ref mapped| mapped.range.is_within(addr)) {
            Some(ref mapped_io) => {
                mapped_io.sender.send((addr, Some(value)));
                true
            },
            _ => false
        }
    }

    #[inline(always)]
    pub fn try_mapped_read(&self, address: usize) -> Option<u32> {
        let addr = Memory::map(address);
        if !self.is_io_register(addr) { return None; }

        match self.mapped.iter().find(|ref mapped| mapped.range.is_within(addr)) {
            Some(ref mapped_io) => {
                mapped_io.sender.send((addr, None));
                Some(mapped_io.receiver.recv().unwrap_or(0))
            },
            _ => None
        }
    }

    /// Sign-extends an unsigned byte to a signed integer
    #[inline(always)]
    pub fn sign_extend_u8(val: u8) -> i32 {
        val as i8 as i32
    }

    /// Sign-extends an unsigned word to a signed integer
    #[inline(always)]
    pub fn sign_extend_u16(val: u16) -> i32 {
        val as i16 as i32
    }

    /// Reads an unsigned byte from memory
    #[inline(always)]
    pub fn read_u8(&self, address: usize) -> u8 {
        let offset = address % 2;
        let val = self.read_u16(address);

        ((val & (0xFF << (8 * offset))) >> (8 * offset)) as u8
    }

    #[inline(always)]
    pub fn read_i8(&self, address: usize) -> i8 {
        self.read_u8(address) as i8
    }

    #[inline(always)]
    pub fn read_u16(&self, address: usize) -> u16 {
        if let Some(v) = self.try_mapped_read(address) {
            return v as u16;
        }
        match self.access(address) {
            &MemoryField::MemoryCell(v) => v,
            _ => panic!("Can only read from memory cell!")
        }
    }

    #[inline(always)]
    pub fn read_u16_raw(&self, address: usize) -> u16 {
        match self.access(address) {
            &MemoryField::MemoryCell(v) => v,
            _ => panic!("Can only read from memory cell!")
        }
    }

    #[inline(always)]
    pub fn read_u32(&self, address: usize) -> u32 {
        if let Some(v) = self.try_mapped_read(address) {
            return v;
        }
        let v1 = self.read_u16_raw(address) as u32;
        let v2 = self.read_u16_raw(address + 2) as u32;

        (v2 << 16) | v1
    }

    //#[inline(always)]
    pub fn write_u8(&mut self, address: usize, value: u8) {
        if self.try_mapped_write(address, value as u32) {
            return;
        }

        let offset = 1 - address % 2;
        let v = self.read_u8(address) as u16;
        let mask = 0xFF << (offset * 8);
        let w = (value as u16) << ((1 - offset) * 8);

        *self.access_mut(address) = MemoryField::MemoryCell((v & mask) | w);
    }

    //#[inline(always)]
    pub fn write_u16(&mut self, address: usize, value: u16) {
        if self.try_mapped_write(address, value as u32) {
            return;
        }
        *self.access_mut(address) = MemoryField::MemoryCell(value);
    }

    //#[inline(always)]
    pub fn write_u32(&mut self, address: usize, value: u32) {
        if self.try_mapped_write(address, value) {
            return;
        }

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
