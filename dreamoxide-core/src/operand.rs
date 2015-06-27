use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Error;

/// Represents an operand for instructions
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Operand {
    /// A general register operand, represented by its index.
    /// Banking will be handled by the CPU
    RegisterOperand(u8),
    /// An immediate value/constant
    ImmediateOperand(u8),
    /// A displacement, offset of memory address
    DisplacementOperand(u8)
}

impl Operand {
    /// Checks if the operand is a register
    #[inline(always)]
    pub fn is_register(&self) -> bool {
        match *self {
            Operand::RegisterOperand(_) => true,
            _ => false
        }
    }

    /// Checks if the operand is an immediate value
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        match *self {
            Operand::ImmediateOperand(_) => true,
            _ => false
        }
    }

    /// Checks if the operand is a displacement constant
    #[inline(always)]
    pub fn is_displacement(&self) -> bool {
        match *self {
            Operand::DisplacementOperand(_) => true,
            _ => false
        }
    }

    /// Akin to Option::unwrap. Unwraps the value from
    /// the operand. Possible as all 3 variants are u8
    #[inline(always)]
    pub fn unwrap(&self) -> u8 {
        match *self {
            Operand::RegisterOperand(v) => v,
            Operand::ImmediateOperand(v) => v,
            Operand::DisplacementOperand(v) => v
        }
    }
}

impl Debug for Operand {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Operand::RegisterOperand(reg)      => fmt.write_str((format!("R{}", reg)).as_str()),
            Operand::ImmediateOperand(imm)     => fmt.write_str((format!("#{}", imm)).as_str()),
            Operand::DisplacementOperand(disp) => fmt.write_str((format!("@{}", disp)).as_str())
        }
    }
}
