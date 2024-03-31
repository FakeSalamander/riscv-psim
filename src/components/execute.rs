use crate::components::*;

//the ID-EX Latch
#[derive(Clone, Default)]
pub struct IDEXLatch {
    pub base_pc: u32,
    pub added_pc: u32,

    pub r1_data: u32,
    pub r2_data: u32,
    pub immediates: u32,
    pub rd_index: u8,

    pub opcode: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub r2_index: u8,
    pub r1_index: u8,
}

//holds the wires for the EX Stage
#[derive(Clone, Default)]
pub struct EXLogic {
    pub formux_r1: u32,
    pub formux_r2: u32,
    pub op1: u32,
    pub op2: u32,
    pub alu_output: u32,

    pub branch_taken: bool,
}
