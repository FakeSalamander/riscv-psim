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
        print!("r{:?}: {:#034b}\n", i, state.reg_mem[i]);
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

        //Checks for output correctness.
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
        // Load Upper Immediate:
        // rd = imm << 12
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

        //Checks for output correctness.
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
        // Add Upper Immediate (to) Program Count:
        // rd = PC + (imm << 12)
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

        //Checks for output correctness.
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
        //Jump And Link Register:
        // rd = PC + 4;
        // PC = r1 + imm
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

        //Checks for output correctness.
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
        //Jump And Link:
        // rd = PC+4 ; PC += imm
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

        //Checks for output correctness.
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
        // Branch (if) EQual:
        // if (rs1 == rs2): PC += imm
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

        //Checks for output correctness.
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
        // Branch (if) Not Equal:
        // if (rs1 != rs2): PC += imm
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

        //Checks for output correctness.
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
        // Branch (if) Less Than
        // if (rs1 < rs2): PC += imm
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

        //Checks for output correctness.
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
    fn bge() {
        // Branch (if) Greater (or) Equal
        // if (rs1 > rs2): PC += imm
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            0b00000000000000000000000000000000, //12: nop
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            //||____||_r2||_r1|101|__|||__op_|
            0b00000010000100100101000001100011, //24: bge $r4, $r1, 32  //if $r4 >= $r1, jump to 56.
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

        //Checks for output correctness.
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
    fn bltu() {
        // Branch (if) Less Than Unsigned:
        //  if (rs1 <{unsigned} rs2): PC += imm
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            0b00000000000000000000000000000000, //12: nop
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            //||____||_r2||_r1|110|__|||__op_|
            0b00000010010000001110000001100011, //24: bltu $r1, $r4, 32  //if $r1 < $r4 (unsigned), jump to 56.
            0b00000000000000000000000000000000, //28: nop
            0b00000000000000000000000000000000, //32: nop
            0b00000000000000000000000000000000, //36: nop
            0b11111111111111111111001000110111, //40: lui $r4, 0b1111111...
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], (0b11111111111111111111 as u32) << 12);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 56);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn bgeu() {
        // Branch (if) Greater (or) Equal Unsigned:
        //  if (rs1 >={unsigned} rs2): PC += imm
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //0: addi $r1, $r0, 1
            0b00000000001000000000000100010011, //4: addi $r2, $r0, 2
            0b00000000001100000000000110010011, //8: addi $r3, $r0, 3
            0b00000000000000000000000000000000, //12: nop
            0b00000000000000000000000000000000, //16: nop
            0b00000000000000000000000000000000, //20: nop
            //||____||_r2||_r1|111|__|||__op_|
            0b00000010000100100111000001100011, //24: bgeu $r4, $r1, 32  //if $r4 >= $r1 (unsigned), jump to 56.
            0b00000000000000000000000000000000, //28: nop
            0b00000000000000000000000000000000, //32: nop
            0b00000000000000000000000000000000, //36: nop
            0b11111111111111111111001000110111, //40: lui $r4, 0b1111111...
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], (0b11111111111111111111 as u32) << 12);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 56);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sw() {
        // Store Word:
        // Mem[r1+imm][0:31] = rs2
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|010|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
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

        //Checks for output correctness.
        assert_eq!(state.data_mem[&2], 0b11111111111111111111000000000000);
        for i in 2..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sh() {
        // Store Half-word:
        // Mem[r1+imm][0:15] = rs2[0:15]
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|001|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
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

        //Checks for output correctness.
        assert_eq!(state.data_mem[&2], 0b11111111111111110101000000000000);
        for i in 3..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sb() {
        // Store Byte:
        // Mem[r1+imm][0:7] = rs2[0:7]
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|000|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
            0b00000000001100000000010000100011, //sb $r3, 8($r0)
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

        //Checks for output correctness.
        assert_eq!(state.data_mem[&2], 0b11111111111111110101000000001000);
        for i in 4..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn lb() {
        // Load Byte:
        // rd = Mem[r1+imm][0:7]
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|000|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
            0b00000000001100000000010000100011, //sb $r3, 8($r0)
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000100000000000001010000011, //lb 8($r0), $r5
            0b00000000100100000000001100000011, //lb 9($r0), $r6
            0b00000000101000000000001110000011, //lb 10($r0), $r7
            0b00000000101100000000010000000011, //lb 11($r0), $r8
            0b00000001100000000000000010000011, //lb 24($r0), $r1
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 8);
        assert_eq!(state.reg_mem[6], 0b01010000);
        //not unsigned, so the following 2 need to sign-extend
        assert_eq!(state.reg_mem[7], (((0b11111111 << 24) as i32) >> 24) as u32);
        assert_eq!(state.reg_mem[8], (((0b11111111 << 24) as i32) >> 24) as u32);
        for i in 9..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn lh() {
        // Load Half-word:
        // rd = Mem[r1+imm][0:15]
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|000|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
            0b00000000001100000000010000100011, //sb $r3, 8($r0)
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:
            //|__imm_____||_r1|001|_rd||__op_|
            0b00000000100000000001001010000011, //lh 8($r0), $r5
            0b00000000101000000001001110000011, //lh 10($r0), $r7
            0b00000001100000000001000010000011, //lh 24($r0), $r1
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 0b0101000000001000);
        //not unsigned, so the following needs to sign-extend
        assert_eq!(
            state.reg_mem[7],
            (((0b1111111111111111 << 16) as i32) >> 16) as u32
        );
        for i in 9..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn lw() {
        // Load Word:
        // rd = Mem[r1+imm][0:31]
        let instructions = Vec::<u32>::from([
            //U-type:
            //|_____imm__________||_rd||__op_|
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|000|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
            0b00000000001100000000010000100011, //sb $r3, 8($r0)
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:
            //|__imm_____||_r1|010|_rd||__op_|
            0b00000000100000000010001010000011, //lw 8($r0), $r5
            0b00000001100000000010000010000011, //lw 24($r0), $r1
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 0b11111111111111110101000000001000);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn lbu() {
        // Load Byte Unsigned:
        // rd = Mem[r1+imm][0:7] {unsigned}
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|000|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
            0b00000000001100000000010000100011, //sb $r3, 8($r0)
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:
            //|__imm_____||_r1|100|_rd||__op_|
            0b00000000100000000100001010000011, //lbu 8($r0), $r5
            0b00000000100100000100001100000011, //lbu 9($r0), $r6
            0b00000000101000000100001110000011, //lbu 10($r0), $r7
            0b00000000101100000100010000000011, //lbu 11($r0), $r8
            0b00000001100000000100000010000011, //lbu 24($r0), $r1
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 8);
        assert_eq!(state.reg_mem[6], 0b01010000);
        //unsigned, so the following should NOT be sign-extended
        assert_eq!(state.reg_mem[7], 0b11111111);
        assert_eq!(state.reg_mem[8], 0b11111111);
        for i in 9..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn lhu() {
        // Load Half-word Unsigned:
        // rd = Mem[r1+imm][0:15] {unsigned}
        let instructions = Vec::<u32>::from([
            0b11111111111111111111000010110111, //lui $r1, 0b1111111...
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //S-Type:
            //|_____||_r2||_r1|000|___||__op_|
            0b00000000000100000010010000100011, //sw $r1, 8($r0)
            0b00000000001000000001010000100011, //sh $r2, 8($r0)
            0b00000000001100000000010000100011, //sb $r3, 8($r0)
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:
            //|__imm_____||_r1|101|_rd||__op_|
            0b00000000100000000101001010000011, //lh 8($r0), $r5
            0b00000000101000000101001110000011, //lh 10($r0), $r7
            0b00000001100000000101000010000011, //lh 24($r0), $r1
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 0b0101000000001000);
        //unsigned, so the following needs to NOT sign-extend
        assert_eq!(state.reg_mem[7], 0b1111111111111111);
        for i in 8..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn addi() {
        // ADD Immediate:
        // rd = r1 + imm
        let instructions = Vec::<u32>::from([
            //I-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000000100000000000010010011, //addi $r1, $r0, 1
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

        //Checks for output correctness.
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
    fn slti() {
        // Set Less Than Immediate:
        // rd = if (r1 < imm) then 1, else 0
        let instructions = Vec::<u32>::from([
            //R-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000000100000000000010010011, //addi $r1, $r0, 1,
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000001000000000001000010011, //addi $r4, $r0, 2
            0b00000000000100000000001010010011, //addi $r5, $r0, 1
            //U-type:
            //|_____imm__________||_rd||__op_|
            0b11111111111111111111001100110111, //lui $r6, 0b1111111...
            //I-Type:           f3
            //|__imm_____||_r1|010|_rd||__op_|
            0b00000000010000001010001110010011, //slti $r7, $r1, 4
            0b00000000010000010010010000010011, //slti $r8, $r2, 4
            0b00000000010000011010010010010011, //slti $r9, $r3, 4
            0b00000000010000100010010100010011, //slti $r10, $r4, 4
            0b00000000010000101010010110010011, //slti $r11, $r5, 4
            0b00000000010000110010011000010011, //slti $r12, $r6, 4
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 0b11111111111111111111 << 12);
        assert_eq!(state.reg_mem[7], 1);
        assert_eq!(state.reg_mem[8], 1);
        assert_eq!(state.reg_mem[9], 0);
        assert_eq!(state.reg_mem[10], 1);
        assert_eq!(state.reg_mem[11], 1);
        //This is signed,  so $r6 should be read as a negative number. This IS less than!
        assert_eq!(state.reg_mem[12], 1);
        for i in 13..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sltiu() {
        // Set Less Than Immediate Unsigned:
        // rd = if (r1 < imm {unsigned}) then 1, else 0
        let instructions = Vec::<u32>::from([
            //I-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000000100000000000010010011, //addi $r1, $r0, 1,
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000001000000000001000010011, //addi $r4, $r0, 2
            0b00000000000100000000001010010011, //addi $r5, $r0, 1
            //U-type:
            //|_____imm__________||_rd||__op_|
            0b11111111111111111111001100110111, //lui $r6, 0b1111111...
            //I-Type:           f3
            //|__imm_____||_r1|011|_rd||__op_|
            0b00000000010000001011001110010011, //sltiu $r7, $r1, 4
            0b00000000010000010011010000010011, //sltiu $r8, $r2, 4
            0b00000000010000011011010010010011, //sltiu $r9, $r3, 4
            0b00000000010000100011010100010011, //sltiu $r10, $r4, 4
            0b00000000010000101011010110010011, //sltiu $r11, $r5, 4
            0b00000000010000110011011000010011, //sltiu $r12, $r6, 4
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 2);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 0b11111111111111111111 << 12);
        assert_eq!(state.reg_mem[7], 1);
        assert_eq!(state.reg_mem[8], 1);
        assert_eq!(state.reg_mem[9], 0);
        assert_eq!(state.reg_mem[10], 1);
        assert_eq!(state.reg_mem[11], 1);
        //This is unsigned,  so $r6 should be read as a huge positive number. This IS NOT less than!
        assert_eq!(state.reg_mem[12], 0);
        for i in 13..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn xori() {
        // eXclusive OR Immediate:
        // rd = r1 xor imm
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //addi $r1, $r0, 1
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            //I-Type:           f3
            //|__imm_____||_r1|100|_rd||__op_|
            0b01111111111100001100001000010011, //xori $r4, $r1, 0b011111111111
            0b01111111111100010100001010010011, //xori $r5, $r2, 0b011111111111
            0b01111111111100011100001100010011, //xori $r6, $r3, 0b011111111111
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0b011111111110);
        assert_eq!(state.reg_mem[5], 0b011111111101);
        assert_eq!(state.reg_mem[6], 0b011111110111);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn ori() {
        // OR Immediate:
        // rd = r1 | imm
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //addi $r1, $r0, 1
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:           f3
            //|__imm_____||_r1|110|_rd||__op_|
            0b01111111000100001110001000010011, //xori $r4, $r1, 0b011111110001
            0b01111111000100010110001010010011, //xori $r5, $r2, 0b011111110001
            0b01111111000100011110001100010011, //xori $r6, $r3, 0b011111110001
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0b011111110001);
        assert_eq!(state.reg_mem[5], 0b011111110011);
        assert_eq!(state.reg_mem[6], 0b011111111001);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn andi() {
        // AND Immediate:
        // rd = r1 & imm
        let instructions = Vec::<u32>::from([
            0b00000001000100000000000010010011, //addi $r1, $r0, 17
            0b00000001001000000000000100010011, //addi $r2, $r0, 18
            0b00000001100000000000000110010011, //addi $r3, $r0, 24
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            //I-Type:           f3
            //|__imm_____||_r1|111|_rd||__op_|
            0b01111110111100001111001000010011, //andi $r4, $r1, 0b011111111111
            0b01111110111100010111001010010011, //andi $r5, $r2, 0b011111111111
            0b01111110111100011111001100010011, //andi $r6, $r3, 0b011111111111
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 17);
        assert_eq!(state.reg_mem[2], 18);
        assert_eq!(state.reg_mem[3], 24);
        assert_eq!(state.reg_mem[4], 1);
        assert_eq!(state.reg_mem[5], 2);
        assert_eq!(state.reg_mem[6], 8);
        for i in 7..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn slli() {
        // Shift Left Logical Immediate
        // rd = r1 << shf
        let instructions = Vec::<u32>::from([
            0b01010101010100000000000010010011, //addi $r1, $r0, 0b010101010101
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            // I-type: Shift variation
            //0000000|shf||_r1|001|_rd||__op_|
            0b00000000000100001001000100010011, //slli $r2, $r1, 1
            0b00000000001000001001000110010011, //slli $r3, $r1, 2
            0b00000000001100001001001000010011, //slli $r4, $r1, 3
            0b00000001111100001001001010010011, //slli $r5, $r1, 31
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 0b010101010101);
        assert_eq!(state.reg_mem[2], 0b0101010101010);
        assert_eq!(state.reg_mem[3], 0b01010101010100);
        assert_eq!(state.reg_mem[4], 0b010101010101000);
        assert_eq!(state.reg_mem[5], 0b10000000000000000000000000000000);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn srli() {
        // Shift Right Logical Immediate
        // rd = r1 << shf  {shift 0's into new bits}
        let instructions = Vec::<u32>::from([
            0b10010010010010010011000010110111, //lui $r1, 0b10010010010010010011
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            // I-type: Shift variation
            //0000000|shf||_r1|101|_rd||__op_|
            0b00000000000100001101000100010011, //srli $r2, $r1, 1
            0b00000000001000001101000110010011, //srli $r3, $r1, 2
            0b00000000001100001101001000010011, //srli $r4, $r1, 3
            0b00000001111100001101001010010011, //srli $r5, $r1, 31
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 0b10010010010010010011 << 12);
        assert_eq!(state.reg_mem[2], 0b10010010010010010011 << 11);
        assert_eq!(state.reg_mem[3], 0b10010010010010010011 << 10);
        assert_eq!(state.reg_mem[4], 0b10010010010010010011 << 9);
        assert_eq!(state.reg_mem[5], 1);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn srai() {
        // Shift Right Arithmetic Immediate
        // rd = r1 << shf {shift sign bit into new bits}
        let instructions = Vec::<u32>::from([
            0b10010010010010010011000010110111, //lui $r1, 0b10010010010010010011
            0b01010010010010010011000100110111, //lui $r2, 0b01010010010010010011
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            // I-type: Shift variation
            //0100000|shf||_r1|101|_rd||__op_|
            0b01000000000100001101000110010011, //srai $r3, $r1, 1
            0b01000000001000001101001000010011, //srai $r4, $r1, 2
            0b01000000001100001101001010010011, //srai $r5, $r1, 3
            0b01000001111100001101001100010011, //srai $r6, $r1, 31
            0b01000000000100010101001110010011, //srai $r7, $r2, 1
            0b01000000001000010101010000010011, //srai $r8, $r2, 2
            0b01000000001100010101010010010011, //srai $r9, $r2, 3
            0b01000001111100010101010100010011, //srai $r10, $r2, 31
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 0b10010010010010010011 << 12);
        assert_eq!(state.reg_mem[2], 0b01010010010010010011 << 12);
        assert_eq!(state.reg_mem[3], 0b110010010010010010011 << 11);
        assert_eq!(state.reg_mem[4], 0b1110010010010010010011 << 10);
        assert_eq!(state.reg_mem[5], 0b11110010010010010010011 << 9);
        assert_eq!(state.reg_mem[6], 0b11111111111111111111111111111111);
        assert_eq!(state.reg_mem[7], 0b01010010010010010011 << 11);
        assert_eq!(state.reg_mem[8], 0b01010010010010010011 << 10);
        assert_eq!(state.reg_mem[9], 0b01010010010010010011 << 9);
        assert_eq!(state.reg_mem[10], 0);

        for i in 11..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn add() {
        // ADD:
        // rd = r1 + r2
        let instructions = Vec::<u32>::from([
            //I-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000001100000000000010010011, //addi $r1, $r0, 3
            0b00000000011000000000000100010011, //addi $r2, $r0, 6
            //R-Type:           f3
            //0000000|_r2||_r1|000|_rd||__op_|
            0b00000000001000001000000110110011, //add $r3, $r1, $r2
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

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 3);
        assert_eq!(state.reg_mem[2], 6);
        assert_eq!(state.reg_mem[3], 9);
        for i in 4..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }
}
