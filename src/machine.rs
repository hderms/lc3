const MEMORY_SIZE: usize = u16::MAX as usize;
use crate::registers::{RegisterName, Registers};
use crate::opcodes::OpCodes;
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
        self.registers.update_by_name(RegisterName::Pc, 0x300);
        self.running = true;
        self
    }
    fn step(&mut self) {
        let next_instruction = self.memory[self.registers.get_by_name(RegisterName::Pc) as usize];
        let opcode: u16 = next_instruction >> 12;
        let maybe_opcode: Option<OpCodes> = num::FromPrimitive::from_u16(opcode);
        match maybe_opcode {
            // Some(OpCodes::Branch) => branch(next_instruction),
            Some(OpCodes::Add) => self.add(next_instruction),
            // Some(OpCodes::Load )=> Instruction::load(),
            // Some(OpCodes::Store )=> Instruction::store(),
            // Some(OpCodes::JumpRegister )=> Instruction::jump(),
            // Some(OpCodes::And )=> Instruction::and(),
            // Some(OpCodes::LoadRegister )=> Instruction::load_register(),
            // Some(OpCodes::StoreRegister )=> Instruction::store_register(),
            // Some(OpCodes::Rti )=> Instruction::noop(), //Unused
            // Some(OpCodes::Not )=> Instruction::not(),
            // Some(OpCodes::LoadIndirect )=> Instruction::load_indirect(),
            // Some(OpCodes::StoreIndirect )=> Instruction::store_indirect(),
            // Some(OpCodes::Jump )=> Instruction::jump(),
            // Some(OpCodes::Reserved )=> Instruction::noop(), //Unused
            // Some(OpCodes::LoadEffectiveAddress )=> Instruction::load_effective_address(),
            // Some(OpCodes::ExecuteTrap )=> Instruction::execute_trap(),
            Some(_) => println!("not implemented yet"),
            None => panic!("Unrecognized opcode"),
        }
    }

    fn add(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0b111;
        let sr = (instruction >> 6) & 0b111;
        let mode = (instruction >> 5) & 0b1;
        let op1 = self.registers.get_by_address(sr);

        let op2 = if mode == 1 {
            //immediate mode
            let immediate = instruction & 0b1111;
            immediate as u16
        } else {
            let sr2 = instruction & 0b111;
            self.registers.get_by_address(sr2)
        };
        let result = (op1 + op2) as u16;
        self.registers = self.registers.update_by_address(dr, result);
    }
}
#[cfg(test)]
mod tests {
    use crate::registers::RegisterName;
    use crate::instruction_builder::{add, increment};
    use super::*;
    #[test]
    fn it_can_add() {
        let mut machine = Machine::empty().start();
        let increment_r1 = increment(RegisterName::R1);
        let increment_r2 = increment(RegisterName::R2);
        machine.add(increment_r1);
        machine.add(increment_r1);
        machine.add(increment_r1);
        println!("{:?}", machine.registers);
        assert_eq!(machine.registers.get_by_name(RegisterName::R1), 3);

        let add_r1_r2_place_in_r3 =
            add(RegisterName::R1, RegisterName::R2, RegisterName::R3);
        machine.add(increment_r2);
        machine.add(add_r1_r2_place_in_r3);
        println!("{:?}", machine.registers);
        assert_eq!(machine.registers.get_by_name(RegisterName::R3), 4);

    }

}