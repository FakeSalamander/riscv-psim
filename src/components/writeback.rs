use crate::components::*;

#[derive(Clone, Default)]
pub struct MEMWBLatch {
    pub added_pc: u32,
    pub alu_output: u32,
    pub mem_data_out: u32,
    pub rd_index: u8,

    pub opcode: u8,

    pub instruction: u32,
}

#[derive(Clone, Default)]
pub struct WBLogic {
    pub wb_data: u32,

    //just used to visually display the multiplexor
    pub wb_used: u8, //0 if ALU, 2 if Mem, 3 if PC
}
