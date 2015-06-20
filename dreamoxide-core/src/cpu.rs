use StatusRegister;
use GeneralRegister;
use Memory;
use MemoryField;
use InstructionExecuter;
use InstructionDecoder;
use Operand;

use std::ops::Index;
use std::ops::IndexMut;

pub struct Cpu {
    pub pc: u32,
    pub status: StatusRegister,
    pub registers: [GeneralRegister; 16],
    pub macl: GeneralRegister,
    pub mach: GeneralRegister
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0xA0000000,
            status: StatusRegister { value: 0 },
            registers: [GeneralRegister { value: 0 }; 16],
            macl: GeneralRegister { value: 0 },
            mach: GeneralRegister { value: 0 }
        }
    }

    pub fn step(&mut self, mem: &mut Memory) {
        match mem.access(self.pc as usize) {
            &MemoryField::InstructionCell(inst) => {
                InstructionExecuter::execute(self, mem, inst);

                if !InstructionDecoder::alters_pc(inst) {
                    self.pc += 2;
                }
            },
            &MemoryField::MemoryCell(val) => {
                let inst = InstructionDecoder::decode(val);
                *mem.access_mut(self.pc as usize) = MemoryField::InstructionCell(inst);
                self.step(mem);
            }
        }
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
