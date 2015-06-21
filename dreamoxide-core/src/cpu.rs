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
    pub registers: [GeneralRegister; 16],
    pub fpu_registers: [FloatingPointRegister; 16],
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
            registers: [GeneralRegister { value: 0 }; 16],
            fpu_registers: [FloatingPointRegister { value: 0.0 }; 16],
            macl: GeneralRegister { value: 0 },
            mach: GeneralRegister { value: 0 },
            dbr: GeneralRegister { value: 0 },
            gbr: GeneralRegister { value: 0 },
            vbr: GeneralRegister { value: 0 },
            ssr: GeneralRegister { value: 0 },
            spc: GeneralRegister { value: 0 },
            fpscr: GeneralRegister { value: 0 },
            fpul: GeneralRegister { value: 0 }
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
                InstructionExecuter::execute(self, mem, inst);
                if inst == Instruction::Nop {
                    println!("Could not decode {:04x}", val);
                }
                self.pc += 2;
            }
        }
    }

    pub fn fpu<'a>(&'a self, reg: Operand) -> &'a FloatingPointRegister {
        &self.fpu_registers[reg.unwrap() as usize]
    }

    pub fn fpu_mut<'a>(&'a mut self, reg: Operand) -> &'a mut FloatingPointRegister {
        &mut self.fpu_registers[reg.unwrap() as usize]
    }
}

impl Index<Operand> for Cpu {
    type Output = GeneralRegister;

    fn index<'a>(&'a self, _index: Operand) -> &'a GeneralRegister {
        &self.registers[_index.unwrap() as usize]
    }
}

impl IndexMut<Operand> for Cpu {
    fn index_mut<'a>(&'a mut self, _index: Operand) -> &'a mut GeneralRegister {
        &mut self.registers[_index.unwrap() as usize]
    }
}
