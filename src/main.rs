use machine::Machine;

extern crate num;
#[macro_use]
extern crate num_derive;
mod instruction_builder;
mod machine;
mod opcodes;
mod registers;
mod util;

fn main() {
    let mut machine = Machine::empty().start();
    machine.step();
}
