use crate::components::*;


//The IF-ID Latch
#[derive(Clone, Default)]
pub struct IFIDLatch {
    pub base_pc: u32,
    pub added_pc: u32,
    pub instruction: u32,
}

//holds all of the wiring of the ID stage
#[derive(Clone, Default)]
pub struct IDLogic {
    pub decode_r1: u8,
    pub decode_r2: u8,
    pub decode_opcode: u8,
    pub decode_rd: u8,
    pub decode_funct3: u8,
    pub decode_funct7: u8,

    pub regmem_r1: u32,
    pub regmem_r2: u32,

    pub immediates: u32,
}
