extern crate num;
#[macro_use]
extern crate num_derive;
const MEMORY_SIZE: usize = u16::MAX as usize;
#[derive(FromPrimitive)]
enum OpCodes {
    Branch = 0,
    Add = 1,
    Load = 2,
    Store = 3,
    JumpRegister = 4,
    And = 5,
    LoadRegister = 6,
    StoreRegister = 7,
    Rti = 8, //Unused
    Not = 9,
    LoadIndirect = 10,
    StoreIndirect = 11,
    Jump = 12,
    Reserved = 13, //Unused
    LoadEffectiveAddress = 14,
    ExecuteTrap = 15,
}
struct Machine {
    memory: [u16; MEMORY_SIZE],
    registers: Registers,
    running: bool,
}
impl Machine {
    fn empty() -> Machine {
        Machine {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
            running: false,
        }
    }

    fn start(mut self) -> Machine {
        self.registers.pc = 0x300;
        self.running = true;
        self
    }
    fn step(&mut self) {
        let next_instruction = self.memory[self.registers.pc as usize];
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
        self.registers = self.registers.update_register_by_address(dr, result);
    }
}
#[derive(Copy, Clone, Debug)]
struct Registers {
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
enum RegisterName {
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
    fn new() -> Registers {
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

    fn update_register(self, register_name: RegisterName, register_value: u16) -> Registers {
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

    fn update_register_by_address(self, register_name: u16, register_value: u16) -> Registers {
        let register = num::FromPrimitive::from_u16(register_name);
        match register {
            Some(value) => self.update_register(value, register_value),
            None => panic!("unrecognized register"),
        }
    }

    fn get_by_address(self, register_name: u16) -> u16 {
        let register = num::FromPrimitive::from_u16(register_name);
        match register {
            Some(value) => self.get_by_name(value),
            None => panic!("unrecognized register"),
        }
    }
    fn get_by_name(self, register_name: RegisterName) -> u16 {
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
struct InstructionBuilder {}
impl InstructionBuilder {
    fn increment(register: RegisterName) -> u16 {
        let r_value = register as u16;
        (1 << 12) | ((r_value) << 9) | ((r_value) << 6) | 1 << 5 | 1
    }

    fn add(source_1: RegisterName, source_2: RegisterName, dest: RegisterName) -> u16 {
        (1 << 12) | ((dest as u16) << 9) | ((source_1 as u16) << 6) | 0 << 5 | source_2 as u16
    }
}

fn main() {
    let mut machine = Machine::empty().start();
    let increment_r1 = InstructionBuilder::increment(RegisterName::R1);
    let increment_r2 = InstructionBuilder::increment(RegisterName::R2);
    machine.add(increment_r1);
    machine.add(increment_r1);
    machine.add(increment_r1);
    println!("{:?}", machine.registers);
    assert_eq!(machine.registers.get_by_name(RegisterName::R1), 3);

    let add_r1_r2_place_in_r3 =
        InstructionBuilder::add(RegisterName::R1, RegisterName::R2, RegisterName::R3);
    machine.add(increment_r2);
    machine.add(add_r1_r2_place_in_r3);
    println!("{:?}", machine.registers);
    assert_eq!(machine.registers.get_by_name(RegisterName::R3), 4);
}
