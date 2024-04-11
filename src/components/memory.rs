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
    pub mem_stall: u8, //stall or bubble?
}

impl EXMEMLatch {
    pub fn bubble(&mut self) {
        self.added_pc = 0;
        self.alu_output = 0xdeadbeef; //cannot write to Data Mem address 0, so just in case.
        self.mem_data_in = 0;
        self.rd_index = 0;
        self.opcode = 0;
        self.funct3 = 0;

        self.instruction = 0;
    }
}

//wires for the MEM stage
#[derive(Clone, Default)]
pub struct MEMLogic {
    pub mem_data_out: u32,
}
