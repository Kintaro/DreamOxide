pub use Instruction;

#[derive(Copy, Clone)]
pub enum MemoryField {
    InstructionCell(Instruction),
    MemoryCell(u16),
}

impl MemoryField {
    pub fn is_instruction(&self) -> bool {
        match *self {
            MemoryField::InstructionCell(_) => true,
            _                               => false
        }
    }

    pub fn is_memory(&self) -> bool {
        match *self {
            MemoryField::MemoryCell(_) => true,
            _                          => false
        }
    }

    pub fn get_instruction(&self) -> Option<Instruction> {
        match *self {
            MemoryField::InstructionCell(inst) => Some(inst),
            _                                  => None
        }
    }

    pub fn set_cell_value(&mut self, value: u16) {
        match *self {
            MemoryField::MemoryCell(ref mut x) => *x = value,
            _                                  => ()
        }
    }

    pub fn get_memory(&self) -> u16 {
        match *self {
            MemoryField::MemoryCell(x) => x,
            _                          => 0
        }
    }
}
