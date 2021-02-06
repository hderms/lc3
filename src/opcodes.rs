#[derive(FromPrimitive)]
pub enum OpCodes {
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
