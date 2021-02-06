use crate::registers::RegisterName;
#[cfg(test)]
pub fn increment(register: RegisterName) -> u16 {
    let r_value = register as u16;
    (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | 1
}

pub fn decrement(register: RegisterName) -> u16 {
    let r_value = register as u16;
    (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | 0b11111
}

#[cfg(test)]
pub fn add_immediate(register: RegisterName, value: u16) -> u16 {
    //will only take 4 bytes of u16 to add
    let r_value = register as u16;
    (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | (value & 0b1111)
}

#[cfg(test)]
pub fn add(source_1: RegisterName, source_2: RegisterName, dest: RegisterName) -> u16 {
    (1 << 12) | ((dest as u16) << 9) | ((source_1 as u16) << 6) | 0 << 5 | source_2 as u16
}
