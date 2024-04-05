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
        0b00000000011000010000000110010011,
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

    run_program(&mut state, &mut logic);
}

fn run_program(state: &mut Registers, logic: &mut Logic) {
    //when a program is <eop_buffer> instructions past the last instruction, the program is done executing.
    let eop_buffer = 7;
    let eop_program_count: u32 = ((state.instr_mem.len() as u32) + eop_buffer) * 4;

    let mut end_of_program = false;
    let mut step_count = 0;

    while !(end_of_program) {
        //make backup, if needed.
        /*
        if backups.len() < step_count {            backups.push(Snapshot {
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

        step(state, logic);

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
        print!("r{:?}: {:#34b}\n", i, state.reg_mem[i]);
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

#[cfg(test)]
pub mod instr_tests {
    use crate::*;
    use std::collections::hash_map::*;

    #[test]
    fn smoke_test() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //addi $r1, $r0, 1,
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000001000000000001000010011, //addi $r4, $r0, 2
            0b00000000000100000000001010010011, //addi $r5, $r0, 1
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn lui() {
        // tries a few LUI instructions.
        let instructions = Vec::<u32>::from([
            //    immediate      || rd||opcod|
            0b00000000000000000001000010110111, //lui $r1, 1
            0b00000000000000000010000100110111, //lui $r2, 2
            0b00000000000000001000000110110111, //lui $r3, 8
            0b00000000000000000010001000110111, //lui $r4, 2
            0b11111111111111111111001010110111, //lui $r5, 0b1111111...
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

        run_program(&mut state, &mut logic);

        // rd = imm << 12
        assert_eq!(state.reg_mem[0], (0 as u32) << 12);
        assert_eq!(state.reg_mem[1], (1 as u32) << 12);
        assert_eq!(state.reg_mem[2], (2 as u32) << 12);
        assert_eq!(state.reg_mem[3], (8 as u32) << 12);
        assert_eq!(state.reg_mem[4], (2 as u32) << 12);
        assert_eq!(state.reg_mem[5], (0b11111111111111111111 as u32) << 12);
    }

    #[test]
    fn auipc() {
        // tries a few AUIPC instructions.
        let instructions = Vec::<u32>::from([
            //    immediate      || rd||opcod|
            0b00000000000000001000000010010111, //auipc $r1, 8
            0b00000000000000000100000100010111, //auipc $r2, 4
            0b00000000000000001000000110010111, //auipc $r3, 8
            0b00000000000000000100001000010111, //auipc $r4, 4
            0b00000000000000000000001010010111, //auipc $r5, 0
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

        run_program(&mut state, &mut logic);

        // rd = (imm << 12) + PC
        assert_eq!(state.reg_mem[0], ((0 as u32) << 12));
        assert_eq!(state.reg_mem[1], ((8 as u32) << 12)); //PC = 0
        assert_eq!(state.reg_mem[2], ((4 as u32) << 12) + 4); //PC = 4
        assert_eq!(state.reg_mem[3], ((8 as u32) << 12) + 8); //PC = 8
        assert_eq!(state.reg_mem[4], ((4 as u32) << 12) + 12); //PC = 12
        assert_eq!(state.reg_mem[5], ((0 as u32) << 12) + 16); //PC = 16
    }

    #[test]
    fn smoke_test() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //addi $r1, $r0, 1,
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000001000000000001000010011, //addi $r4, $r0, 2
            0b00000000000100000000001010010011, //addi $r5, $r0, 1
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }
}
