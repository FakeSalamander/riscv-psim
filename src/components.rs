mod fetch;
mod execute;
mod decode;
mod memory;
mod writeback;

pub use fetch::*;
pub use execute::*;
pub use decode::*;
pub use memory::*;
pub use writeback::*;

pub enum LatchMode {
    Transfer,
    Stall,
    Bubble,
}

//this struct holds the Stateful components of the CPU:
//  Each latch's mode
//  Program Counter
//  Instruction Memory
//  Register Memory
//  Data Memory
struct State {
    ifid_mode : LatchMode,
    idex_mode : LatchMode,
    exmem_mode : LatchMode,
    memwb_mode : LatchMode,

    pc_stall : bool,
    pc : u32,

    instr_mem : Vec<u32>,
    reg_mem : Vec<u32>,
    data_mem : Vec<u32>,
}

//this structs holds all the wiring of each stage
struct Logic {
    fetch : IFLogic,
    decode : IDLogic,
    execute : EXLogic,
    memory : MEMLogic,
    writeback : WBLogic,
}

impl Logic {
    fn update(&mut self,)
}


