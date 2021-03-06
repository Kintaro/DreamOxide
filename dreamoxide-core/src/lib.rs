#![feature(step_by)]
#![feature(convert)]
#![feature(slice_patterns)]
#![feature(asm)]
pub use operand::Operand;
pub use instruction::Instruction;
pub use instruction::InstructionGroup;
pub use register::StatusRegister;
pub use register::GeneralRegister;
pub use register::FloatingPointRegister;
pub use memory_field::MemoryField;
pub use memory::Memory;
pub use memory::MemoryRange;
pub use memory::MappedIO;
pub use bsc::Bsc;
pub use dsp::Dsp;
pub use instruction_executer::InstructionExecuter;
pub use cpu::Cpu;
pub use cpu::FPSCR_MASK;
pub use instruction_decoder::InstructionDecoder;

extern crate latest;

pub mod operand;
pub mod bsc;
pub mod dsp;
pub mod instruction;
pub mod instruction_executer;
pub mod instruction_decoder;
pub mod memory_field;
pub mod register;
pub mod cpu;
pub mod memory;
