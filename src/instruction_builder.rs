use crate::registers::RegisterName;
    pub fn increment(register: RegisterName) -> u16 {
        let r_value = register as u16;
        (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | 1
    }

    pub fn add(source_1: RegisterName, source_2: RegisterName, dest: RegisterName) -> u16 {
        (1 << 12) | ((dest as u16) << 9) | ((source_1 as u16) << 6) | 0 << 5 | source_2 as u16
    }