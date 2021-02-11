extern crate num;
#[derive(Clone, Debug)]
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

    pub fn update_by_name(&mut self, register_name: RegisterName, register_value: u16)  {
        let flag = Registers::update_condition_flag(register_value);
        match register_name {
            RegisterName::Pc => self.pc = register_value,
            RegisterName::R0 => self.r0 = register_value,
            RegisterName::R1 => self.r1 = register_value,
            RegisterName::R2 => self.r2 = register_value,
            RegisterName::R3 => self.r3 = register_value,
            RegisterName::R4 => self.r4 = register_value,
            RegisterName::R5 => self.r5 = register_value,
            RegisterName::R6 => self.r6 = register_value,
            RegisterName::R7 => self.r7 = register_value,
            RegisterName::Cond => panic!("Cond register should not be set directly"),
        }
        self.cond = flag;
    }

    pub fn update_by_address(&mut self, register_name: u16, register_value: u16)  {
        let register = num::FromPrimitive::from_u16(register_name);
        match register {
            Some(value) => self.update_by_name(value, register_value),
            None => panic!("unrecognized register"),
        }
    }

    pub fn get_by_address(&self, register_name: u16) -> u16 {
        let register = num::FromPrimitive::from_u16(register_name);
        match register {
            Some(value) => self.get_by_name(value),
            None => panic!("unrecognized register"),
        }
    }
    pub fn get_by_name(&self, register_name: RegisterName) -> u16 {
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

    pub fn increment_pc(&mut self)  {
        self.pc += 1;
    }

    pub fn set_pc(&mut self, pc: u16)  {
        self.pc = pc;
    }

    pub fn get_pc(&self) -> u16  {
        self.pc
    }
}

enum ConditionFlags {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}
