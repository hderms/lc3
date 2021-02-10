
#[cfg(test)]
pub mod instructions {

    use crate::registers::RegisterName;
    pub fn increment(register: RegisterName) -> u16 {
        let r_value = register as u16;
        (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | 1
    }

    pub fn decrement(register: RegisterName) -> u16 {
        let r_value = register as u16;
        (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | 0b11111
    }

    pub fn add_immediate(register: RegisterName, value: u16) -> u16 {
        //will only take 4 bytes of u16 to add
        let r_value = register as u16;
        (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | (value & 0b1111)
    }

    pub fn store(register: RegisterName, pc_offset: u16) -> u16 {
        //will only take 4 bytes of u16 to add
        let source = register as u16;
        (0b11 << 12) | ((source) << 9)  |  (pc_offset & 0b111111111)
    }

    pub fn load(register: RegisterName, pc_offset: u16) -> u16 {
        //will only take 4 bytes of u16 to add
        let destination = register as u16;
        (0b10 << 12) | ((destination) << 9)  |  (pc_offset & 0b111111111)
    }

    pub fn load_indirect(register: RegisterName, pc_offset: u16) -> u16 {
        //will only take 4 bytes of u16 to add
        let destination = register as u16;
        (0b1010 << 12) | ((destination) << 9)  |  (pc_offset & 0b111111111)
    }

    pub fn add(source_1: RegisterName, source_2: RegisterName, dest: RegisterName) -> u16 {
        (1 << 12) | ((dest as u16) << 9) | ((source_1 as u16) << 6) | 0 << 5 | source_2 as u16
    }

}