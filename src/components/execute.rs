use crate::components::*;

//the ID-EX Latch
#[derive(Clone, Copy, Default)]
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

    pub instruction: u32,
    pub ex_stall: u8, //Stall or Bubble?
}

impl IDEXLatch {
    pub fn bubble(&mut self) {
        self.base_pc = 0;
        self.added_pc = 0;

        self.r1_data = 0;
        self.r2_data = 0;
        self.immediates = 0;
        self.rd_index = 0;
        self.opcode = 0;
        self.funct3 = 0;
        self.r1_index = 0;
        self.r2_index = 0;
        self.instruction = 0;
    }
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
    //these are just used to visually display the multiplexor.
    pub r1_forwarded: u8, //0 - no forwarding. 1 - EX-EX, 2 - MEM-EX
    pub r2_forwarded: u8,
    pub pc_used: bool,
    pub imm_used: bool,
}
