#[derive(FromPrimitive)]
pub enum Opcodes {
    Branch = 0,                //BR
    Add = 1,                   //ADD
    Load = 2,                  //LD
    Store = 3,                 //ST
    JumpRegister = 4,          //JSR
    And = 5,                   //AND
    LoadRegister = 6,          //LDR
    StoreRegister = 7,         //STR
    Rti = 8,                   //RTI (Unused)
    Not = 9,                   //NOT
    LoadIndirect = 10,         //LDI
    StoreIndirect = 11,        //STI
    Jump = 12,                 //JMP
    Reserved = 13,             //RES (Unused)
    LoadEffectiveAddress = 14, //LEA
    ExecuteTrap = 15,          //TRAP
}
