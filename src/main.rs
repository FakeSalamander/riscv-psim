use std::time::Duration;
use std::{hash::Hash, io::stdin, thread::sleep};

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
        //sleep(Duration::new(1, 0));

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
    fn jalr() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000100000000000000110010011, //8: addi $r3, $r0, 8
            //____imm____||_r1|000|_rd||_op__|
            0b00000001100000011000001101100111, //12: jalr $r6, $r3, 24      //jump to instr $r3 + 24 (32)
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            0b00000000000000000000000000000000, //24: nop
            0b00000000001000000000001000010011, //28: addi $r4, $r0, 2
            0b00000000000100000000001010010011, //32: addi $r5, $r0, 1
            0b00000000000000000000000000000000, //36: nop
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 16);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn jal() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            //_|________|
            0b00000001010000000000001101101111, //12: jal $r6, 20      //jump 5instr forward to 32
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            0b00000000000000000000000000000000, //24: nop
            0b00000000001000000000001000010011, //28: addi $r4, $r0, 2
            0b00000000000100000000001010010011, //32: addi $r5, $r0, 1
            0b00000000000000000000000000000000, //36: nop
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 16);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn beq() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            0b00000000000000000000000000000000, //12: nop
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            //||____||_r2||_r1|000|__|||__op_|
            0b00000010001000100000000001100011, //24: beq $r2, $r4, 32  //if $r2 == $r4, jump to 56.
            0b00000000000000000000000000000000, //28: nop
            0b00000000000000000000000000000000, //32: nop
            0b00000000000000000000000000000000, //36: nop
            0b00000000001000000000001000010011, //40: addi $r4, $r0, 2
            0b00000000000100000000001010010011, //44: addi $r5, $r0, 1
            0b00000000000000000000000000000000, //48: nop
            0b00000001010000000000001101101111, //52: jal $r6, -40      //jump 10instr back to PC 12.
            0b00000000000000000000000000000000, //56: nop
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 56);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn bne() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            0b00000000000000000000000000000000, //12: nop
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            //||____||_r2||_r1|001|__|||__op_|
            0b00000010100000100001000001100011, //24: bne $r4, $r8, 32  //if $r4 != $r8, jump to 56.
            0b00000000000000000000000000000000, //28: nop
            0b00000000000000000000000000000000, //32: nop
            0b00000000000000000000000000000000, //36: nop
            0b00000000001000000000001000010011, //40: addi $r4, $r0, 2
            0b00000000000100000000001010010011, //44: addi $r5, $r0, 1
            0b00000000000000000000000000000000, //48: nop
            0b00000001010000000000001101101111, //52: jal $r6, -40      //jump 10instr back to PC 12.
            0b00000000000000000000000000000000, //56: nop
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 56);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn blt() {
        // A single huge program with many varied instructions.
        // Serves as a simple "are there bugs?" test, like a cheksum.
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            0b00000000000000000000000000000000, //12: nop
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            //||____||_r2||_r1|100|__|||__op_|
            0b00000010010000001100000001100011, //24: blt $r1, $r4, 32  //if $r1 < $r4, jump to 56.
            0b00000000000000000000000000000000, //28: nop
            0b00000000000000000000000000000000, //32: nop
            0b00000000000000000000000000000000, //36: nop
            0b00000000001000000000001000010011, //40: addi $r4, $r0, 2
            0b00000000000100000000001010010011, //44: addi $r5, $r0, 1
            0b00000000000000000000000000000000, //48: nop
            0b00000001010000000000001101101111, //52: jal $r6, -40      //jump 10instr back to PC 12.
            0b00000000000000000000000000000000, //56: nop
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

        run_program(&mut state, &mut logic);

        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 56);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn addi() {
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
