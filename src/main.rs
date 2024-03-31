use std::{hash::Hash, io::stdin};

use components::*;

pub mod components;
pub mod isa;

// a "snapshot", or backup, of the state of a CPU in a given cycle.
struct Snapshot {
    state: Registers,
    logic: Logic,
}

fn main() {
    use crate::components::*;
    use std::collections::hash_map::*;

    println!("Hello, world!");

    // code for obtaining instructions here. Not sure how to do it... from a file, maybe?

    let instructions = Vec::<u32>::from([
        0b00000000000100000000000010010011,
        0b00000000001000000000000100010011,
        0b00000000100000000000000110010011,
        0b00000000001000000000001000010011,
        0b00000000000100000000001010010011,
    ]);

    //CPU SETUP: Initializes the state and logic structs.
    let mut state = Registers {
        ifid: IFIDLatch::default(),
        idex: IDEXLatch::default(),
        exmem: EXMEMLatch::default(),
        memwb: MEMWBLatch::default(),

        pc: 0,

        instr_mem: instructions,
        reg_mem: vec![0; 32], //makes a vector of 32 zeroes.
        data_mem: HashMap::new(),
    };

    let mut logic = Logic::default();

    //ADDITIONAL SETUP:

    //a vector of snapshots to make rewinding possible.
    let mut backups: Vec<Snapshot> = Vec::new();

    //when a program is <eop_buffer> instructions past the last instruction, the program is done executing.
    let eop_buffer = 7;
    let eop_program_count: u32 = ((state.instr_mem.len() as u32) + eop_buffer) * 4;

    let mut end_of_program = false;
    let mut step_count = 0;

    while !(end_of_program) {
        //make backup, if needed.
        /*
        if backups.len() < step_count {
            backups.push(Snapshot {
                state: state.clone(),
                logic: logic.clone(),
            });
        }
        */

        display_cpu(&state, &logic, &step_count);

        /*
        print!("What now? [n - next cycle, b - prev. cycle]");
        let user_input = stdin();
        */

        step(&mut state, &mut logic);

        step_count += 1;
        //check if program is over
        println!("PC: {}", state.pc);
        if state.pc >= eop_program_count {
            end_of_program = true;
        }
    }
}

fn display_cpu(state: &Registers, logic: &Logic, step: &i32) {
    //for now, I'll just make it display the registers.
    print!("STEP {:?}:\n", step);
    for i in 0..32 {
        print!("r{:?}: {:?}\n", i, state.reg_mem[i]);
    }

    print!("RDIndex: {}\n", state.memwb.rd_index);
    print!("ALU_out: {}\n", state.memwb.alu_output);
    print!("WB_mux:  {}\n", logic.writeback.wb_data);
}

/*

fn make_backup() {

}

fn load_backup() {

}

fn step_forward() {

}

*/

// also need to do...
// code assembler
