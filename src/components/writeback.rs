use crate::components::*;

pub struct MEMWBLatch {
    pub added_pc: u32,
    pub alu_output: u32,
    pub mem_data_out: u32,
    pub rd_index: u8,
}

pub struct WBLogic {
    pub wb_data: u32,
}
