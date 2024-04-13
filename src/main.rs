use std::env;
use std::fs::read_to_string;
use std::io;
use std::time::Duration;
use std::{hash::Hash, io::stdin, thread::sleep};

use components::*;
use isa::isa::get_instruction_type;

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

    //get commandline arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("The program needs 1 filename, and just 1, as its argument.");
    }
    //now args[1] is the filename...
    println!("{}", args[1]);

    // code for obtaining instructions here. Not sure how to do it... from a file, maybe?

    let mut instructions = Vec::new();

    for line in read_to_string(&args[1]).unwrap().lines() {
        instructions.push(u32::from_str_radix(line, 2).unwrap());
    }

    /*let instructions = Vec::<u32>::from([
        0b00000000000100000000000010010011,
        0b00000000001000000000000100010011,
        0b00000000011000010000000110010011,
        0b00000000001000000000001000010011,
        0b00000000000100000000001010010011,
    ]);*/

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

    run_program(&mut state, &mut logic, true);
}

//Actually runs the program in the simulated CPU
fn run_program(state: &mut Registers, logic: &mut Logic, interactive: bool) {
    //ADDITIONAL SETUP:

    //a vector of snapshots to make rewinding possible.
    let mut backups: Vec<Snapshot> = Vec::new();

    //when a program is <eop_buffer> instructions past the last instruction, the program is done executing.
    let eop_buffer = 5;
    let eop_program_count: u32 = ((state.instr_mem.len() as u32) + eop_buffer) * 4;

    let mut end_of_program = false;
    let mut step_count = 0;

    //used for getting user input
    let user_input = &mut String::new();
    let stdin = stdin();

    while !(end_of_program) {
        //make backup, if needed.

        if backups.len() < step_count {
            backups.push(Snapshot {
                state: state.clone(),
                logic: logic.clone(),
            });
        }

        display_cpu(&state, &logic);

        //get user input for next step.
        user_input.clear();
        if interactive {
            println!("What now? [n - next cycle, b - prev. cycle]:");
            stdin.read_line(user_input);
        } else {
            //if this is a test, dont get user input, just go to next step every time.
            user_input.push_str("n\n");
        }
        println!("{}", user_input);

        if user_input == "n\n" {
            //if n, proceed to next step.
            step(state, logic);

            step_count += 1;
            //check if program is over
            //println!("PC: {}", state.pc);
            if state.pc >= eop_program_count {
                end_of_program = true;
            }
        } else if user_input == "b\n" {
            //load backup, go one step back!!
            if step_count == 0 {
                println!("Can't go back any further!");
            } else {
                step_count -= 1;
                *state = backups[step_count].state.clone();
                *logic = backups[step_count].logic.clone();
            }
        }
    }
}

//Displays the current state of the CPU in an ASCII-based UI
fn display_cpu(state: &Registers, logic: &Logic) {
    /* OLD diplay function
    print!("STEP {:?}:\n", step);
    for i in 0..32 {
        print!("r{:?}: {:#034b}\n", i, state.reg_mem[i]);
    }

    print!("RDIndex: {}\n", state.memwb.rd_index);
    print!("ALU_out: {}\n", state.memwb.alu_output);
    print!("WB_mux:  {}\n", logic.writeback.wb_data);
    */

    println!("*********************************************************************************************************************************************************************************************************");

    println!("                                                                   ┌────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────{:#07b}────┐", state.memwb.rd_index);
    let if_instr = display_instruction(&logic.fetch.instruction_out);
    let id_instr = display_instruction(&state.ifid.instruction);
    let ex_instr = display_instruction(&state.idex.instruction);
    let mem_instr = display_instruction(&state.exmem.instruction);
    let wb_instr = display_instruction(&state.memwb.instruction);
    println!(
        "-IF:{}---------------ID:{}-----------------------EX:{}-------MEM:{}-------WB:{}-------",
        if_instr, id_instr, ex_instr, mem_instr, wb_instr
    );
    println!("                                                                   │ ┌───────────────────────────────────┬───────────────────────────────────────────────────────────────────────────────{:#010x}──┐ │", logic.writeback.wb_data);
    println!("                                                                   │ │                                   │                                                                                           │ │");
    println!("┌──────{:#010x}──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐                                                 │ │", logic.execute.alu_output);
    println!("│                                                                  │ │                                   │                                         │                                                 │ │");
    println!("│ ┌─────────────────────┐          ┌─────┐                         │ │                           ┌─────┐ │                                         │  ┌─────┐                      ┌─────┐           │ │" );
    println!("│ │                     │          │IF/ID│                         │ │                           │ID/EX│ │                                         │  │ EX/ │                      │ MEM │           │ │");
    println!("│ │         ┌─┐    ┌─\\  │{:#010x}│     │                         │ │                           │     │ │                                         │  │ MEM │                      │ /WB │           │ │", logic.fetch.pcadder_out);
    println!("│ │         │4├───►│  │ │    │     │     │                         │ │                           │     │ │                                         │  │     │                      │     │           │ │");
    println!("│ │         └─┘    │+ ├─┴────┴────►│added├─{:#010x}───────────────────────────────────────────►│added├───{:#010x}────────────────────────────────►│added├───────{:#010x}────►│added├───┐       │ │",state.ifid.added_pc, state.idex.added_pc, state.exmem.added_pc);
    println!("│ │              ┌►│  │            │  pc │                         │ │                           │  pc │ │                                         │  │  pc │                      │  pc │   │       │ │");
    println!("│ │              │ └─/             │     │                         │ │                           │     │ │                                         │  │     │                      │     │{:#010x} │ │", state.memwb.added_pc);
    println!("│ │              │                 │     │                         │ │                           │     │ │ ┌─{:#010x}───────────┐                │  │     │                      │     │   │       │ │", state.idex.base_pc);
    println!("│ │              ├────────────────►│base ├─{:#010x}───────────────────────────────────────────►│base ├───┘                      │ ┌─\\            │  │     │                      │     │   │       │ │", state.ifid.base_pc);
    println!("│ │              │                 │  pc │                         │ │                           │  pc │ │             ┌──\\       └►│{} │           │  │     │   ┌───{:#010x}────►│ALU  ├─┐ │ WB    │ │", if logic.execute.pc_used {"►"} else {" "} ,state.exmem.alu_output);
    println!("│ │ PC           │  ┌────────┐     │     │                         │ │  ┌─────────────────┐      │     │ ├────MEM-EX──►│{}  │        │  ├─┐         │  │     │   │                  │  Out│ │ │ Mux.  │ │", if logic.execute.r1_forwarded == 2 {"►"} else {" "});
    println!("│ │ Mux.         │  │ Instr. │     │     │   ┌───────┐             │ │  │  Register Mem.  │      │     │ │             │ R1├─────┬─►│{} │ │         │  │     │   │                  │     │ │ │ ┌─\\   │ │", if !logic.execute.pc_used {"►"} else {" "});
    println!("│ │ ┌─\\     ┌──┐ │  │  Mem.  │     │     │ ┌►│Decoder├─{:#07b}───┐ │ └─►│Wb data      Reg1├─┬───►│R1   ├───{:#010x}─►│{}  │     │  └─/  ▼         │  │     │   │                  │     │ │ └►│{} │  │ │", logic.decode.decode_r1, state.idex.r1_data, if logic.execute.r1_forwarded == 0 {"►"} else {" "}, if logic.writeback.wb_used == 2 {"►"} else {" "});
    println!("│ └►│{} │    │PC│ │  │        │     │     │ │ │       │           │ │    │                 │ │    │ Data│ │             │   │     │     ┌─────────┐ │  │     │   │                  │     │ │   │  │  │ │", if !logic.fetch.jumped {"►"} else {" "});
    println!("│   │  ├─┬─►│  ├─┼─►│addr    │     │     │ │ │       ├─{:#07b}─┐ │ └───►│Wb idx           │ │    │     │ │ ┌───EX-EX──►└{}─/      │     │Op1      │ │  │     │   │  ┌──────────┐    │     │ └─┬►│{} ├──┘ │", logic.decode.decode_r2, if logic.execute.r1_forwarded == 1 {"►"} else {"─"}, if logic.writeback.wb_used == 0 {"►"} else {" "});
    println!("└──►│{} │ │  │  │ │  │     ins├─┬──►│instr├─┤ │       │         │ │      │                 │ │    │     │ │ │                     │     │         │ │  │     │   │  │ DATA MEM.│    │     │   │ │  │    │", if logic.fetch.jumped {"►"} else {" "});
    println!("    └─/  │  └──┘ │  └────────┘ │   │     │ │ │ opcode├───┐     │ └─────►│R1 idx           │ │    │     │ └──────MEM-EX──►┌{}─\\    │     │   ALU   │ │  │     │   │  │      Read├─┬─►│Mem  ├─┬──►│{} │    │", if logic.execute.r1_forwarded == 2 {"►"} else {"─"}, if logic.writeback.wb_used == 1 {"►"} else {" "});
    println!("         │ {} │             │   │     │ │ │       │   │     │        │                 │ │    │     │   │             │   │   │     │      Out├─┼─►│ALU  ├───┼──┤Addr   Out│ │  │  Out│ │ │ └─/     │", if logic.pc_stall { "STALL" } else {"PASS "});
    println!("         │       │             │   │     │ │ │ rd idx├─┐ │     └───────►│R2 idx       Reg2├───┬─►│R2   ├─────{:#010x}─►│{}  │   │     │Op2      │ │  │  Out│   │  │          │ │  │     │ │ │         │", state.idex.r2_data, if logic.execute.r1_forwarded == 0 {"►"} else {" "});
    println!(" {:#010x}   {:#010x}       │   │     │ │ └───────┘ │ │              └─────────────────┘ │ │  │ Data│   │             │ R2├─┐ │     └─────────┘ │  │     │   │  │          │ │  │     │ │{:#010x} │", logic.fetch.pcmux_out, state.pc, state.memwb.alu_output);
    println!("                               │   │     │ │           │ │ ┌──────┐                {:#010x} │  │     │   ├─────EX-EX──►│{}  │ │ │  ┌─\\  ▲         │  │     │ ┌────┤DataIn    │ │  │     │ │           │", logic.decode.regmem_r1, if logic.execute.r1_forwarded == 1 {"►"} else {" "});
    println!("                      {:#010x}   │     │ │           │ └►│ Imm. │                           │  │     │   │             └──/  ├───►│{} │ │ {:#010x} │     │ │ │  └──────────┘ │  │     │{:#010x}   │", logic.fetch.instruction_out,if !logic.execute.imm_used {"►"} else {" "} ,logic.execute.alu_output, state.memwb.mem_data_out);
    println!("                                   │     │ │           │   │Decode│                  {:#010x}  │     │   │                   │ │  │  ├─┘            │     │ │ │  {}     │  │     │             │", logic.decode.regmem_r2, if logic.memory.memmem_fwd {"MEM-MEM!"} else {"        "});
    println!("                                   │     │ ├──────────────►│      ├─{:#010x}─┐                 │     │   │ ┌─{:#010x}──────────►│{} │              │     │ │ │      {:#010x}  │     │             │", logic.decode.immediates, state.idex.immediates, if logic.execute.imm_used {"►"} else {" "}, logic.memory.mem_data_out );
    println!("                                   │     │ │           │   └──────┘            └────────────────►│Imms.├─────┘                 │ │  └─/               │     │ │ │                  │     │             │");
    println!("                                   │     │ {:#010x}  │                                         │     │   │                   │ │                    │     │ │ │                  │     │             │", state.ifid.instruction);
    println!("                                   │     │             └────────────{:#07b}─────────────────────►│Rd Id├───────┐               ├────────{:#010x}───►│Mem  ├─┴────{:#010x}      │     │             │", logic.decode.decode_rd, logic.execute.formux_r2, state.exmem.mem_data_in);
    println!("                                   │     │                                                       │     │   │   │               │ │                    │  In │   │                  │     │             │");
    println!("                                   │     │                                                       │     │   │   │               │ │  ┌─────────┐       │     │   │                  │     │             │");
    println!("                                   │     │                                                       │     │   │   │               │ └─►│ Branch? │       │     │   │                  │     │             │");
    println!("                                   │     │                                                       │     │   │   │               │    │         │       │     │   │                  │     │             │");
    println!("                                   │     │                                                       │     │   │   │               └───►│    {}    │       │     │   │                  │     │             │", if logic.execute.branch_taken {"Y"} else {"N"});
    println!("                                   │     │                                                       │     │   │   │                    └─────────┘       │     │   │                  │     │             │");
    println!("                                   │     │                                                       │     │   │   │                                      │     │   │                  │     │             │");
    println!("                                   │     │                                                       │     │   │   └────────────{:#07b}──────────────────►│Rd Id├────────{:#07b}──────►│Rd In├─────────────┘", state.idex.rd_index, state.exmem.rd_index);
    println!("                                   └─────┘                                                       └─────┘   │                                          └─────┘   │                  └─────┘              ");
    println!("                                    {}                                                         {}    │                                           {}    │                   {}               ", match state.ifid.id_stall {0 => "PASS ", 1 => "STALL", _ => "BUBBL"}, match state.idex.ex_stall {0 => "PASS ", 1 => "STALL", _ => "BUBBL"}, match state.exmem.mem_stall {0 => "PASS ", 1 => "STALL", _ => "BUBBL"}, match state.memwb.wb_stall {0 => "PASS ", 1 => "STALL", _ => "BUBBL"});
    println!("                                                                                                           └─{:#010x}─────────────────────────────────────────┘                                      ", state.exmem.alu_output);

    println!("");
    println!("*********************************************REGISTER MEMORY*********************************************************************************************************************************************");
    for r in 0..32 {
        print!("$r{:#02}: {:#010x}   ", r, state.reg_mem[r]);
        if (r + 1) % 8 == 0 {
            print!("\n");
        }
    }
}

fn display_instruction(instr: &u32) -> String {
    //converts a 32-bit instruction into a string of human-readable assembly
    let opcode = instr & 0b1111111;
    let rd = ((instr & 0b111110000000) >> 7);
    let funct3 = ((instr & 0b111000000000000) >> 12);
    let funct7 = ((instr & 0b11111110000000000000000000000000) >> 25);
    let r1 = ((instr & 0b11111000000000000000) >> 15);
    let r2 = ((instr & 0b1111100000000000000000000) >> 20);

    let typ = get_instruction_type(opcode as u8);

    let mut assembly: String = "".to_string();

    //Note: the immediates of LUI and AUIPC are the only ones to be unsigned, since they get pushed all the way to the top of their output.
    if opcode == 0b0110111 {
        //LUI
        let imm = (instr & 0b11111111111111111111) >> 12;
        assembly = "lui $r".to_owned() + &rd.to_string() + ", " + &format!("{:#x}", imm);
        //         6                     2               2       7
        // 17 chars at most
    } else if opcode == 0b0010111 {
        //AUIPC
        let imm = (instr & 0b11111111111111111111) >> 12;
        assembly = "auipc $r".to_owned() + &rd.to_string() + ", " + &format!("{:#x}", imm);
        // 19 at most
    } else if opcode == 0b1101111 {
        //JAL
        let imm_e: u32 = (instr & 0b10000000000000000000000000000000) >> 11;
        let imm_f: u32 = (instr & 0b01111111111000000000000000000000) >> 20;
        let imm_g: u32 = (instr & 0b00000000000100000000000000000000) >> 9;
        let imm_h: u32 = instr & 0b00000000000011111111000000000000;

        let imm = (((imm_e | imm_f | imm_g | imm_h) << 11) as i32) >> 11;
        assembly = "jal $r".to_owned() + &rd.to_string() + ", " + &format!("{:#x}", imm);
        // 17 at most
    } else if opcode == 0b1100111 {
        let imm = ((*instr as i32) >> 20);

        assembly = "jalr $r".to_owned()
            + &rd.to_string()
            + ", $r"
            + &r1.to_string()
            + ", "
            + &format!("{:#x}", imm);
        // 7 + 2 + 5 + 2 + 2 + 7
        // 25 at most
    } else if opcode == 0b1100011 {
        //Branches! need to look at funct3 to decide which.

        //A (31) to [12] ,B (30-25) to [10-5], C (11-8) to [4-1], D (7) to [11]
        let imm_a: u32 = (((instr & 0b10000000000000000000000000000000) as i32) >> 19) as u32;
        let imm_b: u32 = (instr & 0b01111110000000000000000000000000) >> 20;
        let imm_c: u32 = (instr & 0b00000000000000000000111100000000) >> 7;
        let imm_d: u32 = (instr & 0b00000000000000000000000010000000) << 4;
        //println!("{:#b}",imm_a);
        //println!("{:#b}",imm_b);
        //println!("{:#b}",imm_c);
        //println!("{:#b}",imm_d);

        let imm = ((((imm_a | imm_b | imm_c | imm_d) << 19) as i32) >> 19);
        let instr_name = match funct3 {
            0b000 => "beq",
            0b001 => "bne",
            0b100 => "blt",
            0b101 => "bge",
            0b110 => "bltu",
            0b111 => "bgeu",
            _ => panic!("Invalid funct3 for a branch! {:b}", instr),
        };
        assembly = instr_name.to_owned()
            + " $r"
            + &r1.to_string()
            + ", $r"
            + &r2.to_string()
            + ", "
            + &format!("{:#x}", imm);
    } else if opcode == 0b0000011 {
        //Load! Look at funct3 to decide which.
        let imm = ((*instr as i32) >> 20);

        let instr_name = match funct3 {
            0b000 => "lb",
            0b001 => "lh",
            0b010 => "lw",
            0b100 => "lbu",
            0b101 => "lhu",
            _ => panic!("Invalid funct3 for a load! {:b}", instr),
        };
        assembly = instr_name.to_owned()
            + " $r"
            + &rd.to_string()
            + ", "
            + &format!("{:#x}", imm)
            + "($r"
            + &r1.to_string()
            + ")";
    } else if opcode == 0b0100011 {
        // Store! Look at funct3 to decide which.

        let imm = (((instr & 0b11111110000000000000000000000000) as i32) >> 20)
            | (((instr & 0b111110000000) >> 7) as i32);

        let instr_name = match funct3 {
            0b000 => "sb",
            0b001 => "sh",
            0b010 => "sw",
            _ => panic!("Invalid funct3 for a store! {:b}", instr),
        };

        assembly = instr_name.to_owned()
            + " $r"
            + &r2.to_string()
            + ", "
            + &format!("{:#x}", imm)
            + "($r"
            + &r1.to_string()
            + ")";
    } else if opcode == 0b0010011 {
        // Register-Immediate Operation! Look at funct3 to figure which.
        let imm = ((*instr as i32) >> 20);
        //use r2 as shamt for the shifts, they take up same space in instruction.

        let instr_name = match funct3 {
            0b000 => "addi",
            0b010 => "slti",
            0b011 => "sltiu",
            0b100 => "xori",
            0b110 => "ori",
            0b111 => "andi",

            0b001 => "slli",
            0b101 => match funct7 {
                0b0000000 => "srli",
                0b0100000 => "srai",
                _ => "wrsh!!",
            },
            _ => panic!("Invalid funct3 for an R-I op! {:b}", instr),
        };

        if funct3 == 0b001 || funct3 == 0b101 {
            //shifts use the shamt instead of the full immediate field.
            assembly = instr_name.to_owned()
                + " $r"
                + &rd.to_string()
                + ", $r"
                + &r1.to_string()
                + ", "
                + &r2.to_string();
        } else {
            //for the rest..
            assembly = instr_name.to_owned()
                + " $r"
                + &rd.to_string()
                + ", $r"
                + &r1.to_string()
                + ", "
                + &format!("{:#x}", imm);
        }
    } else if opcode == 0b0110011 {
        //Register-Register Instructions! Look at funct3/7 to see which.
        // No immediates.
        let instr_name = match funct3 {
            0b000 => match funct7 {
                0b0000000 => "add",
                0b0100000 => "sub",
                _ => panic!("invalid funct7 for add/sub! {:b}", instr),
            },
            0b001 => "sll",
            0b010 => "slt",
            0b011 => "sltu",
            0b100 => "xor",
            0b101 => match funct7 {
                0b0000000 => "srl",
                0b0100000 => "sra",
                _ => panic!("invalid funct7 for srl/a! {:b}", instr),
            },
            0b110 => "or",
            0b111 => "and",
            _ => panic!("Invalid funct3 for an R-R op! {:b}", instr),
        };

        assembly = instr_name.to_owned()
            + " $r"
            + &rd.to_string()
            + ", $r"
            + &r1.to_string()
            + ", $r"
            + &r2.to_string();
    } else if opcode == 0 {
        assembly = "nop".to_owned();
    } else {
        assembly = "".to_owned();
        panic!("Invalid instruction opcode! {:b}", instr);
    }
    while assembly.len() < 25 {
        assembly += "-";
    }
    return assembly;
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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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
            0b00000000100000011000000110010011, //16: addi $r3, $r3, 8
            0b00000000100000011000000110010011, //20: addi $r3, $r3, 8
            0b00000000000000000000000000000000, //24: nop
            0b00000000001000000000001000010011, //28: addi $r4, $r0, 2
            0b00000000000100000000001010010011, //32: addi $r5, $r0, 1
            0b00000000000000000000000000000000, //36: nop
        ]);
        //instructions 16-28 should get skipped

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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
            0b01111111000100001110001000010011, //ori $r4, $r1, 0b011111110001
            0b01111111000100010110001010010011, //ori $r5, $r2, 0b011111110001
            0b01111111000100011110001100010011, //ori $r6, $r3, 0b011111110001
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

        run_program(&mut state, &mut logic, false);

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
            0b01111110111100001111001000010011, //andi $r4, $r1, 0b011111101111
            0b01111110111100010111001010010011, //andi $r5, $r2, 0b011111101111
            0b01111110111100011111001100010011, //andi $r6, $r3, 0b011111101111
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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 3);
        assert_eq!(state.reg_mem[2], 6);
        assert_eq!(state.reg_mem[3], 9);
        for i in 4..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sub() {
        // SUB:
        // rd = r1 = r2
        let instructions = Vec::<u32>::from([
            //I-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000001100000000000010010011, //addi $r1, $r0, 3
            0b00000000011000000000000100010011, //addi $r2, $r0, 6
            //R-Type:           f3
            //0100000|_r2||_r1|000|_rd||__op_|
            0b01000000000100010000000110110011, //sub $r3, $r2, $r1
            0b01000000001000001000001000110011, //sub $r4, $r1, $r2
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 3);
        assert_eq!(state.reg_mem[2], 6);
        assert_eq!(state.reg_mem[3], 3);
        assert_eq!(state.reg_mem[4], (i32::from(-3)) as u32);
        for i in 5..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sll() {
        // Shift Left Logical
        // rd = r1 << r2[0:4]
        let instructions = Vec::<u32>::from([
            0b01010101010100000000000010010011, //addi $r1, $r0, 0b010101010101
            0b00000000000100000000001100010011, //addi $r6, $r0, 1
            0b00000000001000000000001110010011, //addi $r7, $r0, 2
            0b00000000001100000000010000010011, //addi $r8, $r0, 3
            0b00000001111100000000010010010011, //addi $r9, $r0, 31
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            // R-type:
            //0000000|_r2||_r1|001|_rd||__op_|
            0b00000000011000001001000100110011, //sll $r2, $r1, $r6
            0b00000000011100001001000110110011, //sll $r3, $r1, $r7
            0b00000000100000001001001000110011, //sll $r4, $r1, $r8
            0b00000000100100001001001010110011, //sll $r5, $r1, $r9
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 0b010101010101);
        assert_eq!(state.reg_mem[2], 0b0101010101010);
        assert_eq!(state.reg_mem[3], 0b01010101010100);
        assert_eq!(state.reg_mem[4], 0b010101010101000);
        assert_eq!(state.reg_mem[5], 0b10000000000000000000000000000000);
        assert_eq!(state.reg_mem[6], 1);
        assert_eq!(state.reg_mem[7], 2);
        assert_eq!(state.reg_mem[8], 3);
        assert_eq!(state.reg_mem[9], 31);
        for i in 10..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn slt() {
        // Set Less Than:
        // rd = if (r1 < r2) then 1, else 0
        let instructions = Vec::<u32>::from([
            //I-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000000100000000000010010011, //addi $r1, $r0, 1,
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000001000000000001000010011, //addi $r4, $r0, 2
            0b00000000000100000000001010010011, //addi $r5, $r0, 1
            0b00000000010000000000011010010011, //addi $r13, $r0, 4
            //U-type:
            //|_____imm__________||_rd||__op_|
            0b11111111111111111111001100110111, //lui $r6, 0b1111111...
            //R-Type:           f3
            //0000000|_r2||_r1|010|_rd||__op_|
            0b00000000110100001010001110110011, //slt $r7, $r1, $r13
            0b00000000110100010010010000110011, //slt $r8, $r2, $r13
            0b00000000110100011010010010110011, //slt $r9, $r3, $r13
            0b00000000110100100010010100110011, //slt $r10, $r4, $r13
            0b00000000110100101010010110110011, //slt $r11, $r5, $r13
            0b00000000110100110010011000110011, //slt $r12, $r6, $r13
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

        run_program(&mut state, &mut logic, false);

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
        assert_eq!(state.reg_mem[13], 4);
        for i in 14..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sltu() {
        // Set Less Than Unsigned:
        // rd = if (r1 <{unsigned} r2) then 1, else 0
        let instructions = Vec::<u32>::from([
            //I-Type:           f3
            //|__imm_____||_r1|000|_rd||__op_|
            0b00000000000100000000000010010011, //addi $r1, $r0, 1,
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b00000000001000000000001000010011, //addi $r4, $r0, 2
            0b00000000000100000000001010010011, //addi $r5, $r0, 1
            0b00000000010000000000011010010011, //addi $r13, $r0, 4
            //U-type:
            //|_____imm__________||_rd||__op_|
            0b11111111111111111111001100110111, //lui $r6, 0b1111111...
            //R-Type:           f3
            //0000000|_r2||_r1|011|_rd||__op_|
            0b00000000110100001011001110110011, //sltu $r7, $r1, $r13
            0b00000000110100010011010000110011, //sltu $r8, $r2, $r13
            0b00000000110100011011010010110011, //sltu $r9, $r3, $r13
            0b00000000110100100011010100110011, //sltu $r10, $r4, $r13
            0b00000000110100101011010110110011, //sltu $r11, $r5, $r13
            0b00000000110100110011011000110011, //sltu $r12, $r6, $r13
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

        run_program(&mut state, &mut logic, false);

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
        //This is unsigned,  so $r6 should be read as a big positive number. This IS NOT less than!
        assert_eq!(state.reg_mem[12], 0);
        assert_eq!(state.reg_mem[13], 4);
        for i in 14..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn xor() {
        // eXclusive OR:
        // rd = r1 xor r2
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //addi $r1, $r0, 1
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b01111111111100000000001110010011, //addi $r7, $r0, 0b011111111111
            //R-Type:           f3
            //0000000|_r2||_r1|100|_rd||__op_|
            0b00000000011100001100001000110011, //xor $r4, $r1, 0b011111111111
            0b00000000011100010100001010110011, //xor $r5, $r2, 0b011111111111
            0b00000000011100011100001100110011, //xor $r6, $r3, 0b011111111111
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0b011111111110);
        assert_eq!(state.reg_mem[5], 0b011111111101);
        assert_eq!(state.reg_mem[6], 0b011111110111);
        assert_eq!(state.reg_mem[7], 0b011111111111);
        for i in 8..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn srl() {
        // Shift Right Logical
        // rd = r1 << shf  {shift 0's into new bits}
        let instructions = Vec::<u32>::from([
            0b10010010010010010011000010110111, //lui $r1, 0b10010010010010010011
            0b00000000000100000000001100010011, //addi $r6, $r0, 1
            0b00000000001000000000001110010011, //addi $r7, $r0, 2
            0b00000000001100000000010000010011, //addi $r8, $r0, 3
            0b00000001111100000000010010010011, //addi $r9, $r0, 31
            // R-type: Shift variation
            //0000000|_r2||_r1|101|_rd||__op_|
            0b00000000011000001101000100110011, //srl $r2, $r1, $r6
            0b00000000011100001101000110110011, //srl $r3, $r1, $r7
            0b00000000100000001101001000110011, //srl $r4, $r1, $r8
            0b00000000100100001101001010110011, //srl $r5, $r1, $r9
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 0b10010010010010010011 << 12);
        assert_eq!(state.reg_mem[2], 0b10010010010010010011 << 11);
        assert_eq!(state.reg_mem[3], 0b10010010010010010011 << 10);
        assert_eq!(state.reg_mem[4], 0b10010010010010010011 << 9);
        assert_eq!(state.reg_mem[5], 1);
        assert_eq!(state.reg_mem[6], 1);
        assert_eq!(state.reg_mem[7], 2);
        assert_eq!(state.reg_mem[8], 3);
        assert_eq!(state.reg_mem[9], 31);
        for i in 10..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn sra() {
        // Shift Right Arithmetic
        // rd = r1 << r2 {shift sign bit into new bits}
        let instructions = Vec::<u32>::from([
            0b10010010010010010011000010110111, //lui $r1, 0b10010010010010010011
            0b01010010010010010011000100110111, //lui $r2, 0b01010010010010010011
            0b00000000000100000000010110010011, //addi $r11, $r0, 1
            0b00000000001000000000011000010011, //addi $r12, $r0, 2
            0b00000000001100000000011010010011, //addi $r13, $r0, 3
            0b00000001111100000000011100010011, //addi $r14, $r0, 31
            // R-type: Shift variation
            //0100000|_r2||_r1|101|_rd||__op_|
            0b01000000101100001101000110110011, //srai $r3, $r1, $r11
            0b01000000110000001101001000110011, //srai $r4, $r1, $r12
            0b01000000110100001101001010110011, //srai $r5, $r1, $r13
            0b01000000111000001101001100110011, //srai $r6, $r1, $r14
            0b01000000101100010101001110110011, //srai $r7, $r2, $r11
            0b01000000110000010101010000110011, //srai $r8, $r2, $r12
            0b01000000110100010101010010110011, //srai $r9, $r2, $r13
            0b01000000111000010101010100110011, //srai $r10, $r2, $r14
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

        run_program(&mut state, &mut logic, false);

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
        assert_eq!(state.reg_mem[11], 1);
        assert_eq!(state.reg_mem[12], 2);
        assert_eq!(state.reg_mem[13], 3);
        assert_eq!(state.reg_mem[14], 31);

        for i in 15..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn or() {
        // OR:
        // rd = r1 | r2
        let instructions = Vec::<u32>::from([
            0b00000000000100000000000010010011, //addi $r1, $r0, 1
            0b00000000001000000000000100010011, //addi $r2, $r0, 2
            0b00000000011000010000000110010011, //addi $r3, $r2, 6
            0b01111111000100000000001110010011, //addi $r7, $r0, 0b011111110001
            //R-Type:           f3
            //0000000|_r2||_r1|110|_rd||__op_|
            0b00000000011100001110001000110011, //or $r4, $r1, $r7
            0b00000000011100010110001010110011, //or $r5, $r2, $r7
            0b00000000011100011110001100110011, //or $r6, $r3, $r7
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 1);
        assert_eq!(state.reg_mem[2], 2);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0b011111110001);
        assert_eq!(state.reg_mem[5], 0b011111110011);
        assert_eq!(state.reg_mem[6], 0b011111111001);
        assert_eq!(state.reg_mem[7], 0b011111110001);
        for i in 8..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn and() {
        // AND:
        // rd = r1 & r2
        let instructions = Vec::<u32>::from([
            0b00000001000100000000000010010011, //addi $r1, $r0, 17
            0b00000001001000000000000100010011, //addi $r2, $r0, 18
            0b00000001100000000000000110010011, //addi $r3, $r0, 24
            0b01111110111100000000001110010011, //addi $r7, $r0, 0b011111101111
            //R-Type:           f3
            //0000000|_r2||_r1|111|_rd||__op_|
            0b00000000011100001111001000110011, //and $r4, $r1, $r7
            0b00000000011100010111001010110011, //and $r5, $r2, $r7
            0b00000000011100011111001100110011, //and $r6, $r3, $r7
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.reg_mem[0], 0);
        assert_eq!(state.reg_mem[1], 17);
        assert_eq!(state.reg_mem[2], 18);
        assert_eq!(state.reg_mem[3], 24);
        assert_eq!(state.reg_mem[4], 1);
        assert_eq!(state.reg_mem[5], 2);
        assert_eq!(state.reg_mem[6], 8);
        assert_eq!(state.reg_mem[7], 0b011111101111);
        for i in 8..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    #[test]
    fn memmem_forwarding() {
        //Tests for LOAD-STORE hazards.
        // The last two instruction, a LW followed by a SW targeting the same register, should result in MEM-MEM forwarding triggering.
        let instructions = Vec::<u32>::from([
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            //S-Type:
            //|_____||_r2||_r1|010|___||__op_|
            0b00000000001000000010010000100011, //sw $r2, 8($r0)
            //I-Type:
            //|__imm_____||_r1|010|_rd||__op_|
            0b00000000100000000010001010000011, //lw 8($r0), $r5
            //S-Type:
            //|_____||_r2||_r1|010|___||__op_|
            0b00000000010100000010011000100011, //sw $r1, 12($r0)
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.data_mem[&2], 0b01010101010101010101000000000000);
        assert_eq!(state.data_mem[&3], 0b01010101010101010101000000000000);

        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], (0b01010101010101010101 as u32) << 12);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }

    /*#[test]
    fn load_alu_hazard() {
        //Tests for LOAD-ALU hazards.
        // The last two instruction, a LW followed by a SW targeting the same register, should result in a stall followed by a MEM-EX Fwd.
        let instructions = Vec::<u32>::from([
            0b01010101010101010101000100110111, //lui $r2, 0b0101010...
            0b00000000100000000000000110010011, //addi $r3, $r0, 8
            //S-Type:
            //|_____||_r2||_r1|010|___||__op_|
            0b00000000001000000010010000100011, //sw $r2, 8($r0)
            //I-Type:
            //|__imm_____||_r1|010|_rd||__op_|
            0b00000000100000000010001010000011, //lw 8($r0), $r5
            0b00000000100000101000001010010011, //addi $r5, $r5, 8
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

        run_program(&mut state, &mut logic, false);

        //Checks for output correctness.
        assert_eq!(state.data_mem[&2], 0b01010101010101010101000000000000);

        assert_eq!(state.reg_mem[1], 0);
        assert_eq!(state.reg_mem[2], (0b01010101010101010101 as u32) << 12);
        assert_eq!(state.reg_mem[3], 8);
        assert_eq!(state.reg_mem[4], 0);
        assert_eq!(state.reg_mem[5], 0b01010101010101010101000000001000);
        for i in 6..32 {
            assert_eq!(state.reg_mem[i], 0);
        }
    }*/
}
