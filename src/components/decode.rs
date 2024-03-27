use crate::components::*;
use crate::isa::isa;
use crate::isa::isa::InstrT;

//The IF-ID Latch
pub struct IFIDLatch {
    pub base_pc: u32,
    pub added_pc: u32,
    pub instruction: u32,
}

//holds all of the wiring of the ID stage
pub struct IDLogic {
    pub decode_r1: u8,
    pub decode_r2: u8,
    pub decode_opcode: u8,
    pub decode_rd: u8,

    pub regmem_r1: u32,
    pub regmem_r2: u32,

    pub immediates: u32,
}
