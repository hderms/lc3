const MEMORY_SIZE: usize = u16::MAX as usize;
use crate::opcodes::Opcodes;
use crate::registers::{RegisterName, Registers};
use crate::util;
pub struct Machine {
    memory: [u16; MEMORY_SIZE],
    registers: Registers,
    running: bool,
}
impl Machine {
    pub fn empty() -> Machine {
        Machine {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
            running: false,
        }
    }

    pub fn start(mut self) -> Machine {
        self.registers.set_pc(0x300);
        self.running = true;
        self
    }
    pub fn step(&mut self) {
        let next_instruction = self.memory[self.registers.get_by_name(RegisterName::Pc) as usize];
        self.registers.increment_pc();
        let opcode: u16 = next_instruction >> 12;
        let maybe_opcode: Option<Opcodes> = num::FromPrimitive::from_u16(opcode);
        match maybe_opcode {
            // Some(Opcodes::branch) => branch(next_instruction),
            Some(Opcodes::Add) => self.add(next_instruction),
            Some(Opcodes::Load )=> self.load(next_instruction),
            Some(Opcodes::Store )=> self.store(next_instruction),
            Some(Opcodes::JumpRegister )=> self.jump_register(next_instruction),
            Some(Opcodes::And )=> self.and(next_instruction),
            // Some(Opcodes::loadregister )=> instruction::load_register(),
            // Some(Opcodes::storeregister )=> instruction::store_register(),
            // Some(Opcodes::rti )=> instruction::noop(), //unused
            // Some(Opcodes::not )=> instruction::not(),
            Some(Opcodes::LoadIndirect )=> self.load_indirect(next_instruction),
            // Some(Opcodes::storeindirect )=> instruction::store_indirect(),
            // Some(Opcodes::jump )=> instruction::jump(),
            // Some(Opcodes::reserved )=> instruction::noop(), //unused
            // Some(Opcodes::loadeffectiveaddress )=> instruction::load_effective_address(),
            // Some(Opcodes::executetrap )=> instruction::execute_trap(),
            Some(_) => println!("not implemented yet"),
            None => panic!("unrecognized opcode"),
        }
    }
    fn immediate_or_source(registers: Registers, instruction: u16) -> (u16, u16) {
        let sr = (instruction >> 6) & 0b111;
        let mode = (instruction >> 5) & 0b1;
        let op1 = registers.get_by_address(sr);

        let op2 = if mode == 1 {
            //immediate mode
            let immediate = util::sign_extend(instruction & 0b11111, 5);
            immediate as u16
        } else {
            let sr2 = instruction & 0b111;
            registers.get_by_address(sr2)
        };
        (op1, op2)
    }

    fn add(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let (op1, op2) = Machine::immediate_or_source(self.registers, instruction);
        let result = (op1.wrapping_add(op2)) as u16;
        self.registers = self.registers.update_by_address(dr, result);
    }

    fn and(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let (op1, op2) = Machine::immediate_or_source(self.registers, instruction);
        let result = (op1 & op2) as u16;
        self.registers = self.registers.update_by_address(dr, result);
    }


    fn load_indirect(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let pc_offset_9 = instruction & 0x1FF;
        let sign_extended_pc_offset = util::sign_extend(pc_offset_9, 9);
        let pointer = self.registers.get_pc() + sign_extended_pc_offset;
        let address = self.memory[pointer as usize]; 
        let final_address = self.memory[address as usize];
        self.registers = self.registers.update_by_address(dr, final_address);
        
    }

    fn load(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let pc_offset_9 = instruction & 0x1FF;
        let sign_extended_pc_offset = util::sign_extend(pc_offset_9, 9);
        let pointer = self.registers.get_pc() + sign_extended_pc_offset;
        let address = self.memory[pointer as usize]; 
        self.registers = self.registers.update_by_address(dr, address);
        
    }

    fn store(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0b111;
        let pc_offset_9 = instruction & 0x1FF;
        let sign_extended_pc_offset = util::sign_extend(pc_offset_9, 9);
        let pointer = self.registers.get_by_name(RegisterName::Pc) + sign_extended_pc_offset;
        let register_value = self.registers.get_by_address(sr);
        self.memory[pointer as usize] = register_value; 
        
    }

    fn jump_register(&mut self, instruction: u16) {
        let mode = instruction >> 11;
        self.registers.update_by_name(RegisterName::R7, self.registers.get_pc());
        let address = if mode == 1 {
             util::sign_extend(instruction & 0x7FF, 10)
        } else {
             let register = (instruction >> 6)  & 0b111;
             self.registers.get_by_address(register)
        };
        self.registers.set_pc(address);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction_builder::instructions::{add, add_immediate, and_register, and_immediate, decrement, increment, store, load, load_indirect};
    use crate::registers::RegisterName;
    #[test]
    fn it_can_add_in_immediate_mode() {
        let mut machine = Machine::empty().start();
        let increment_r1 = increment(RegisterName::R1);
        let decrement_r1 = decrement(RegisterName::R1);
        machine.add(increment_r1);
        machine.add(increment_r1);
        machine.add(increment_r1);
        machine.add(decrement_r1);
        assert_eq!(machine.registers.get_by_name(RegisterName::R1), 2);
    }


    #[test]
    fn it_can_represent_adding_twos_complement_resulting_in_overflow() {
        let mut machine = Machine::empty().start();
        let decrement_r1 = decrement(RegisterName::R1);
        machine.add(decrement_r1);
        machine.add(decrement_r1);
        assert_eq!(machine.registers.get_by_name(RegisterName::R1) as i16, -2);
    }

    #[test]
    fn it_can_add_in_register_mode() {
        let mut machine = Machine::empty().start();
        let increment_r1 = increment(RegisterName::R1);
        let increment_r2 = increment(RegisterName::R2);
        machine.add(increment_r1);
        machine.add(increment_r1);
        machine.add(increment_r1);
        machine.add(increment_r2);
        machine.add(increment_r2);

        let add_r1_r2_place_in_r3 = add(RegisterName::R1, RegisterName::R2, RegisterName::R3);
        machine.add(add_r1_r2_place_in_r3);
        assert_eq!(machine.registers.get_by_name(RegisterName::R3), 5);
    }

    #[test]
    fn it_can_and_in_immediate_mode() {
        let mut machine = Machine::empty().start();
        machine.registers = machine.registers.update_by_name(RegisterName::R1, 0b11);
        let and_r1 = and_immediate(RegisterName::R1, 0b1);
        machine.and(and_r1);
        assert_eq!(machine.registers.get_by_name(RegisterName::R1), 1);
    }

    #[test]
    fn it_can_and_in_register_mode() {
        let mut machine = Machine::empty().start();
        machine.registers = machine.registers.update_by_name(RegisterName::R1, 0b11);

        machine.registers = machine.registers.update_by_name(RegisterName::R2, 0b11);
        let and_r1 = and_register(RegisterName::R1, RegisterName::R2, RegisterName::R3);
        machine.and(and_r1);
        assert_eq!(machine.registers.get_by_name(RegisterName::R3), 0b11);
    }

    #[test]
    fn it_can_store() {
        let mut machine = Machine::empty().start();
        let increment_r1 = increment(RegisterName::R1);
        let store_r1_in_mem100 = store(RegisterName::R1, 100);
        machine.add(increment_r1);
        machine.add(increment_r1);
        machine.store(store_r1_in_mem100);

        let final_address = machine.registers.get_by_name(RegisterName::Pc) + 100;
        assert_eq!(machine.memory[final_address as usize], 2);
    }

    #[test]
    fn it_can_load() {
        let mut machine = Machine::empty().start();
        let load_memory_address = load(RegisterName::R1, 200);
        let pc = machine.registers.get_by_name(RegisterName::Pc);
        machine.memory[(pc + 200) as usize] = 42;
        machine.load(load_memory_address);

        assert_eq!(machine.registers.get_by_name(RegisterName::R1), 42);
    }

    #[test]
    fn it_can_load_indirect() {
        let mut machine = Machine::empty().start();
        let load_memory_address = load_indirect(RegisterName::R1, 200);
        let pc = machine.registers.get_by_name(RegisterName::Pc);
        machine.memory[(pc + 200) as usize] = 42;
        machine.memory[42] = 99;
        machine.load_indirect(load_memory_address);

        assert_eq!(machine.registers.get_by_name(RegisterName::R1), 99);
    }
}
