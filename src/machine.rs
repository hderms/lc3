const MEMORY_SIZE: usize = u16::MAX as usize;
use util::sign_extend;

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
    pub fn write_memory(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    pub fn get_memory(&mut self, address: u16) -> u16 {
        self.memory[address as usize]
    }
    pub fn step(&mut self) {
        let pc = self.registers.get_pc();
        let next_instruction = self.get_memory(pc);
        self.registers.increment_pc();
        let opcode: u16 = next_instruction >> 12;
        let maybe_opcode: Option<Opcodes> = num::FromPrimitive::from_u16(opcode);
        match maybe_opcode {
            Some(Opcodes::Branch) => self.branch(next_instruction),
            Some(Opcodes::Add) => self.add(next_instruction),
            Some(Opcodes::Load) => self.load(next_instruction),
            Some(Opcodes::Store) => self.store(next_instruction),
            Some(Opcodes::JumpRegister) => self.jump_register(next_instruction),
            Some(Opcodes::And) => self.and(next_instruction),
            Some(Opcodes::LoadRegister) => self.load_register(next_instruction),
            Some(Opcodes::StoreRegister) => self.store_register(next_instruction),
            Some(Opcodes::Rti) => (), //unused
            Some(Opcodes::Not) => self.not(next_instruction),
            Some(Opcodes::LoadIndirect) => self.load_indirect(next_instruction),
            // Some(Opcodes::storeindirect )=> instruction::store_indirect(),
            // Some(Opcodes::jump )=> instruction::jump(),
            Some(Opcodes::Reserved) => (), //unused
            // Some(Opcodes::loadeffectiveaddress )=> instruction::load_effective_address(),
            // Some(Opcodes::executetrap )=> instruction::execute_trap(),
            Some(_) => println!("not implemented yet"),
            None => panic!("unrecognized opcode"),
        }
    }
    fn immediate_or_source(registers: &Registers, instruction: u16) -> (u16, u16) {
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
        let (op1, op2) = Machine::immediate_or_source(&self.registers, instruction);
        let result = (op1.wrapping_add(op2)) as u16;
        self.registers
            .update_by_address_set_condition_flag(dr, result);
    }

    fn and(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let (op1, op2) = Machine::immediate_or_source(&self.registers, instruction);
        let result = (op1 & op2) as u16;
        self.registers
            .update_by_address_set_condition_flag(dr, result);
    }

    fn branch(&mut self, instruction: u16) {
        let nzp = (instruction >> 9) & 0b111;
        let pc_offset = instruction & 0x1FF;
        let sign_extended_pc_offset = sign_extend(pc_offset, 9);
        let cond_flag = self.registers.get_cond_flag();
        let pc = self.registers.get_pc();
        if cond_flag & nzp > 0 {
            self.registers.set_pc(pc + sign_extended_pc_offset)
        }
    }

    fn load_indirect(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let pc_offset_9 = instruction & 0x1FF;
        let sign_extended_pc_offset = util::sign_extend(pc_offset_9, 9);
        let pointer = self.registers.get_pc() + sign_extended_pc_offset;
        let address = self.get_memory(pointer);
        let final_address = self.get_memory(address);
        self.registers
            .update_by_address_set_condition_flag(dr, final_address);
    }

    fn load(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let pc_offset_9 = instruction & 0x1FF;
        let sign_extended_pc_offset = util::sign_extend(pc_offset_9, 9);
        let pointer = self.registers.get_pc() + sign_extended_pc_offset;
        let address = self.get_memory(pointer);
        self.registers
            .update_by_address_set_condition_flag(dr, address);
    }

    fn store(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0b111;
        let pc_offset_9 = instruction & 0x1FF;
        let sign_extended_pc_offset = util::sign_extend(pc_offset_9, 9);
        let pointer = self.registers.get_by_name(RegisterName::Pc) + sign_extended_pc_offset;
        let register_value = self.registers.get_by_address(sr);
        self.memory[pointer as usize] = register_value;
        self.write_memory(pointer, register_value);
    }

    fn jump_register(&mut self, instruction: u16) {
        let mode = (instruction >> 11) & 1;
        let pc = self.registers.get_pc();
        self.registers.update_by_name(RegisterName::R7, pc);
        let address = if mode == 1 {
            let immediate = instruction & 0x7FF;
            let sign_extended_offset = util::sign_extend(immediate, 11);
            pc.wrapping_add(sign_extended_offset)
        } else {
            let register = (instruction >> 6) & 0b111;
            self.registers.get_by_address(register)
        };
        self.registers.set_pc(address);
    }

    fn not(&mut self, instruction: u16) {
        let source = (instruction >> 6) & 0b111;
        let destination = (instruction >> 9) & 0b111;
        let register_value = self.registers.get_by_address(source);
        self.registers
            .update_by_address_set_condition_flag(destination, !register_value);
    }

    fn load_register(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let base_r = (instruction >> 6) & 0b111;
        let offset_6 = sign_extend(instruction & 0x3F, 6);
        let base_address = self.registers.get_by_address(base_r);
        let address = offset_6.wrapping_add(base_address);
        let cell = self.get_memory(address);
        self.registers
            .update_by_address_set_condition_flag(dr, cell);
    }

    fn store_register(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0b111;
        let base_r = (instruction >> 6) & 0b111;
        let offset_6 = sign_extend(instruction & 0x3F, 6);
        let base_address = self.registers.get_by_address(base_r);
        let address = base_address.wrapping_add(offset_6);
        let source_value = self.registers.get_by_address(sr);
        self.write_memory(address, source_value);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction_builder::instructions::{
        add, add_immediate, and_immediate, and_register, branch, decrement, increment, jump_offset,
        jump_register, load, load_indirect, load_register, not, store, store_register,
    };
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
        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R1, 0b11);
        let and_r1 = and_immediate(RegisterName::R1, 0b1);
        machine.and(and_r1);
        assert_eq!(machine.registers.get_by_name(RegisterName::R1), 1);
    }

    #[test]
    fn it_can_and_in_register_mode() {
        let mut machine = Machine::empty().start();
        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R1, 0b11);

        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R2, 0b11);
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

    #[test]
    fn it_can_jump_register_immediate() {
        let mut machine = Machine::empty().start();
        machine.registers.set_pc(0x42);
        let pc_before = machine.registers.get_pc();
        assert_eq!(pc_before, 0x42);
        let jump_immediate_instruction = jump_offset(200);
        machine.jump_register(jump_immediate_instruction);
        let pc_after = machine.registers.get_pc();

        assert_eq!(machine.registers.get_by_name(RegisterName::R7), pc_before);
        assert_eq!(pc_after, 0x42 + 200);
    }

    #[test]
    fn it_can_jump_register_offset() {
        let mut machine = Machine::empty().start();
        machine.registers.set_pc(0x42);
        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R3, 42);
        let pc_before = machine.registers.get_pc();
        let jump_register_instruction = jump_register(RegisterName::R3);
        machine.jump_register(jump_register_instruction);
        let pc_after = machine.registers.get_pc();

        assert_eq!(machine.registers.get_by_name(RegisterName::R7), pc_before);
        assert_eq!(pc_after, 42);
    }

    #[test]
    fn it_can_not() {
        let mut machine = Machine::empty().start();
        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R3, 0xFF);
        let not_instruction = not(RegisterName::R3, RegisterName::R4);
        machine.not(not_instruction);
        let result = machine.registers.get_by_name(RegisterName::R4);

        assert_eq!(result, 0xFF00);
    }

    #[test]
    fn it_can_branch() {
        let mut machine = Machine::empty().start();
        let negative_two: i16 = -2;
        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R3, negative_two as u16);
        let pc_before = machine.registers.get_pc();
        let jump_neg = branch(crate::registers::ConditionFlag::Negative, 3);
        let jump_zero = branch(crate::registers::ConditionFlag::Zero, 5);
        let jump_pos = branch(crate::registers::ConditionFlag::Positive, 7);
        machine.branch(jump_neg);
        let pc_after = machine.registers.get_pc();

        assert_eq!(pc_before + 3, pc_after);
        let pc_before = machine.registers.get_pc();
        machine.branch(jump_zero);
        let pc_after = machine.registers.get_pc();
        assert_eq!(pc_before, pc_after);

        machine.branch(jump_pos);
        let pc_after = machine.registers.get_pc();
        assert_eq!(pc_before, pc_after);
    }

    #[test]
    fn it_can_load_register() {
        let mut machine = Machine::empty().start();
        machine
            .registers
            .update_by_name_set_condition_flag(RegisterName::R1, 10); //base register

        machine.write_memory(9, 99);
        let load_register_instruction = load_register(RegisterName::R1, 0b111111, RegisterName::R2);
        machine.load_register(load_register_instruction);
        let destination = machine.registers.get_by_name(RegisterName::R2);
        assert_eq!(destination, 99);
    }

    #[test]
    fn it_can_store_register() {
        let mut machine = Machine::empty().start();
        machine.registers.update_by_name(RegisterName::R1, 10); //base register

        machine.registers.update_by_name(RegisterName::R2, 42);
        let store_register_instruction = store_register(RegisterName::R1, 1, RegisterName::R2);
        machine.store_register(store_register_instruction);
        let cell = machine.get_memory(11);
        assert_eq!(cell, 42);
    }
}
