extern crate num;
#[derive(Copy, Clone, Debug)]
pub struct Registers {
    pc: u16,
    cond: u16,
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
}
#[derive(Clone, Copy, FromPrimitive)]
pub enum RegisterName {
    Pc = 0,
    Cond = 1,
    R0 = 2,
    R1 = 3,
    R2 = 4,
    R3 = 5,
    R4 = 6,
    R5 = 7,
    R6 = 8,
    R7 = 9,
}
impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            cond: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
        }
    }

    pub fn update_by_name(self, register_name: RegisterName, register_value: u16) -> Registers {
        let flag = Registers::update_condition_flag(register_value);
        let mut copied = self;
        match register_name {
            RegisterName::Pc => copied.pc = register_value,
            RegisterName::R0 => copied.r0 = register_value,
            RegisterName::R1 => copied.r1 = register_value,
            RegisterName::R2 => copied.r2 = register_value,
            RegisterName::R3 => copied.r3 = register_value,
            RegisterName::R4 => copied.r4 = register_value,
            RegisterName::R5 => copied.r5 = register_value,
            RegisterName::R6 => copied.r6 = register_value,
            RegisterName::R7 => copied.r7 = register_value,
            RegisterName::Cond => panic!("Cond register should not be set directly"),
        }
        copied.cond = flag;
        copied
    }

    pub fn update_by_address(self, register_name: u16, register_value: u16) -> Registers {
        let register = num::FromPrimitive::from_u16(register_name);
        match register {
            Some(value) => self.update_by_name(value, register_value),
            None => panic!("unrecognized register"),
        }
    }

    pub fn get_by_address(self, register_name: u16) -> u16 {
        let register = num::FromPrimitive::from_u16(register_name);
        match register {
            Some(value) => self.get_by_name(value),
            None => panic!("unrecognized register"),
        }
    }
    pub fn get_by_name(self, register_name: RegisterName) -> u16 {
        match register_name {
            RegisterName::Pc => self.pc,
            RegisterName::Cond => self.cond,
            RegisterName::R0 => self.r0,
            RegisterName::R1 => self.r1,
            RegisterName::R2 => self.r2,
            RegisterName::R3 => self.r3,
            RegisterName::R4 => self.r4,
            RegisterName::R5 => self.r5,
            RegisterName::R6 => self.r6,
            RegisterName::R7 => self.r7,
        }
    }
    fn update_condition_flag(register_value: u16) -> u16 {
        let flag = if register_value >> 15 == 1 {
            ConditionFlags::Negative
        } else if register_value == 0 {
            ConditionFlags::Zero
        } else {
            ConditionFlags::Positive
        };
        flag as u16
    }
}

enum ConditionFlags {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}
