mod decode;
mod execute;
mod fetch;
mod memory;
mod writeback;

pub use decode::*;
pub use execute::*;
pub use fetch::*;
pub use memory::*;
pub use writeback::*;

//this struct holds the Stateful components of the CPU:
//  Each pipeline latch
//  Program Counter
//  Instruction Memory
//  Register Memory
//  Data Memory
struct Registers {
    ifid: IFIDLatch,
    idex: IDEXLatch,
    exmem: EXMEMLatch,
    memwb: MEMWBLatch,

    pc: u32,

    instr_mem: Vec<u32>,
    reg_mem: Vec<u32>,
    data_mem: Vec<u32>,
}

//this structs holds all the wiring of each stage
struct Logic {
    fetch: IFLogic,
    decode: IDLogic,
    execute: EXLogic,
    memory: MEMLogic,
    writeback: WBLogic,
}

impl Registers {
    fn update(&mut self, logic: &Logic) {
        // MEM-WB Latch
        self.memwb.added_pc = self.exmem.added_pc;
        self.memwb.alu_output = self.exmem.alu_output;
        self.memwb.mem_data_out = logic.memory.mem_data_out;
        self.memwb.rd_index = self.exmem.rd_index;

        // Data Memory

        //insert code for Data Mem read/write here

        //EX-MEM Latch

        self.exmem.added_pc = self.idex.added_pc;
        self.exmem.alu_output = logic.execute.alu_output;
        self.exmem.mem_data_in = logic.execute.formux_r2;
        self.exmem.rd_index = self.idex.rd_index;

        // ID-EX Latch
        self.idex.base_pc = self.ifid.base_pc;
        self.idex.added_pc = self.ifid.added_pc;

        self.idex.r1_data = logic.decode.regmem_r1;
        self.idex.r2_data = logic.decode.regmem_r2;
        self.idex.immediates = logic.decode.immediates;
        self.idex.rd_index = logic.decode.decode_rd;

        self.idex.opcode = logic.decode.decode_opcode;

        // Register Memory. Write to it.
        assert!(self.memwb.rd_index < 0b100000); //Register indices are always 5 bits or less.
        if (self.memwb.rd_index != 0) {
            self.reg_mem[self.memwb.rd_index as usize] = logic.writeback.wb_data;
        }

        // IF-ID latch. Transfer
        self.ifid.base_pc = self.pc;
        self.ifid.added_pc = logic.fetch.pcadder_out;
        self.ifid.instruction = logic.fetch.instruction_out;
    }
}

impl Logic {
    fn update(&mut self, state: &Registers) {
        // IF Stage

        //PCMux: First, check if opcode FROM EX STAGE is Jump, Branching, or neither
        if (state.idex.opcode == 0b1101111 || state.idex.opcode == 0b1100111) {
            self.fetch.pcmux_out = self.execute.alu_output; //initiates jump by setting PC to result of address adition
        } else if (state.idex.opcode == 0b1100011 && self.execute.branch_taken) {
            //if branch taken!
            self.fetch.pcmux_out = self.execute.alu_output; //initiates jump
        } else {
            //if not branch, or branch not taken
            self.fetch.pcmux_out = self.fetch.pcadder_out;
        }

        self.fetch.instruction_out = state.instr_mem[(state.pc / 4) as usize];

        self.fetch.pcadder_out = state.pc + 4;
    }
}
