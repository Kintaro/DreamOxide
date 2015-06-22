use StatusRegister;
use GeneralRegister;
use FloatingPointRegister;
use Memory;
use MemoryField;
use InstructionExecuter;
use InstructionDecoder;
use Instruction;
use Operand;

use std::ops::Index;
use std::ops::IndexMut;

pub const FPSCR_MASK : u32 = 0x003FFFFF;

pub struct Cpu {
    pub pc: u32,
    pub pr: u32,
    pub status: StatusRegister,
    pub registers: [GeneralRegister; 24],
    pub fpu_registers: [FloatingPointRegister; 32],
    pub macl: GeneralRegister,
    pub mach: GeneralRegister,
    pub dbr: GeneralRegister,
    pub gbr: GeneralRegister,
    pub vbr: GeneralRegister,
    pub ssr: GeneralRegister,
    pub spc: GeneralRegister,
    pub fpscr: GeneralRegister,
    pub fpul: GeneralRegister,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0xA0000000,
            pr: 0,
            status: StatusRegister { value: 0 },
            registers: [GeneralRegister { value: 0 }; 24],
            fpu_registers: [FloatingPointRegister { value: 0.0 }; 32],
            macl: GeneralRegister { value: 0 },
            mach: GeneralRegister { value: 0 },
            dbr: GeneralRegister { value: 0 },
            gbr: GeneralRegister { value: 0 },
            vbr: GeneralRegister { value: 0 },
            ssr: GeneralRegister { value: 0 },
            spc: GeneralRegister { value: 0 },
            fpscr: GeneralRegister { value: 0 },
            fpul: GeneralRegister { value: 0 },
        }
    }

    pub fn step(&mut self, mem: &mut Memory) {
        match mem.access(self.pc as usize) {
            &MemoryField::InstructionCell(inst) => {
                InstructionExecuter::execute(self, mem, inst);
                self.pc += 2;
            },
            &MemoryField::MemoryCell(val) => {
                if val == 0xFFFF {
                    return;
                }
                let inst = InstructionDecoder::decode(val);
                *mem.access_mut(self.pc as usize) = MemoryField::InstructionCell(inst);

                if inst == Instruction::Unknown {
                    println!("[0x{:08x}] Could not decode {:04x}", self.pc, val);
                }

                InstructionExecuter::execute(self, mem, inst);
                                self.pc += 2;
            }
        }
    }

    pub fn fpu<'a>(&'a self, reg: Operand) -> &'a FloatingPointRegister {
        let bank = if self.fpscr.value & 0x200000 != 0 { 16 } else { 0 };
        &self.fpu_registers[bank + reg.unwrap() as usize]
    }

    pub fn fpu_mut<'a>(&'a mut self, reg: Operand) -> &'a mut FloatingPointRegister {
        let bank = if self.fpscr.value & 0x200000 != 0 { 16 } else { 0 };
        &mut self.fpu_registers[bank + reg.unwrap() as usize]
    }
}

impl Index<Operand> for Cpu {
    type Output = GeneralRegister;

    fn index<'a>(&'a self, _index: Operand) -> &'a GeneralRegister {
        let bank = if self.status.is_banked() && self.status.is_privileged() && _index.unwrap() < 8 { 16 } else { 0 };
        &self.registers[bank + _index.unwrap() as usize]
    }
}

impl IndexMut<Operand> for Cpu {
    fn index_mut<'a>(&'a mut self, _index: Operand) -> &'a mut GeneralRegister {
        let bank = if self.status.is_banked() && self.status.is_privileged() && _index.unwrap() < 8 { 16 } else { 0 };
        &mut self.registers[bank + _index.unwrap() as usize]
    }
}
