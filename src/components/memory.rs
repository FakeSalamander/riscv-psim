use crate::components::*;

//the EX-MEM Latch
#[derive(Clone, Default)]
pub struct EXMEMLatch {
    pub added_pc: u32,
    pub alu_output: u32,
    pub mem_data_in: u32,
    pub rd_index: u8,

    pub opcode: u8,
    pub funct3: u8,

    pub instruction: u32,
}

//wires for the MEM stage
#[derive(Clone, Default)]
pub struct MEMLogic {
    pub mem_data_out: u32,
}
