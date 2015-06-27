use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct GeneralRegister {
    pub value: u32
}

#[derive(Copy, Clone)]
pub struct FloatingPointRegister {
    pub value: f32
}

#[derive(Copy, Clone)]
pub struct StatusRegister {
    pub value: u32
}

impl StatusRegister {
    pub fn is_user_mode(&self) -> bool {
        self.value & (1 << 30) == 0
    }

    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        self.value & (1 << 30) > 0
    }

    #[inline(always)]
    pub fn is_banked(&self) -> bool {
        self.value & (1 << 29) > 0
    }

    pub fn is_interrupt(&self) -> bool {
        self.value & (1 << 28) > 0
    }

    pub fn is_fpu_disabled(&self) -> bool {
        self.value & (1 << 15) > 0
    }

    pub fn is_m(&self) -> bool {
        self.value & (1 << 9) > 0
    }

    pub fn is_q(&self) -> bool {
        self.value & (1 << 8) > 0
    }

    pub fn imask(&self) -> u8 {
        ((self.value & (3 << 5)) >> 5) as u8
    }

    pub fn is_saturated(&self) -> bool {
        self.value & 0x2 > 0
    }

    pub fn set_saturated_cond(&mut self, val: bool) {
        if val {
            self.value |= 2;
        } else {
            self.value &= 0xFFFFFFFD;
        }
    }

    #[inline(always)]
    pub fn is_carry(&self) -> bool {
        self.value & 0x1 > 0
    }

    #[inline(always)]
    pub fn set_carry_cond(&mut self, val: bool) {
        if val {
            self.value |= 1;
        } else {
            self.value &= 0xFFFFFFFE;
        }
    }
}

impl Debug for StatusRegister {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str((format!("{}{}{}{}{}{}{}{}{}",
                               if self.is_user_mode() { "U" } else { "P" },
                               if self.is_banked() { "B" } else { "-" },
                               if self.is_interrupt() { "I" } else { "-" },
                               if self.is_fpu_disabled() { "F" } else { "-" },
                               if self.is_m() { "M" } else { "-" },
                               if self.is_q() { "Q" } else { "-" },
                               self.imask(),
                               if self.is_saturated() { "S" } else { "-" },
                               if self.is_carry() { "T" } else { "-" }
                               )).as_str())
    }
}
