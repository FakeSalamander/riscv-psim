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
}
