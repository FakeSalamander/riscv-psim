use crate::components::*;

//the EX-MEM Latch
pub struct EXMEMLatch {
    pub added_pc: u32,
    pub alu_output: u32,
    pub mem_data_in: u32,
    pub rd_index: u8,
}

//wires for the MEM stage
pub struct MEMLogic {
    pub mem_data_out: u32,
}
