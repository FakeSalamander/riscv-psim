mod decode;
mod execute;
mod fetch;
mod memory;
mod writeback;

pub use crate::isa::isa;
pub use crate::isa::isa::InstrT;
pub use decode::*;
pub use execute::*;
pub use fetch::*;
pub use memory::*;
use std::collections::hash_map::*;
pub use writeback::*;

//this struct holds the Stateful components of the CPU:
//  Each pipeline latch
//  Program Counter
//  Instruction Memory
//  Register Memory
//  Data Memory

#[derive(Clone)]
pub struct Registers {
    pub ifid: IFIDLatch,
    pub idex: IDEXLatch,
    pub exmem: EXMEMLatch,
    pub memwb: MEMWBLatch,

    pub pc: u32,

    pub instr_mem: Vec<u32>,
    pub reg_mem: Vec<u32>,
    pub data_mem: HashMap<u32, u32>,
}

//this structs holds all the wiring of each stage

#[derive(Clone, Default)]
pub struct Logic {
    pub fetch: IFLogic,
    pub decode: IDLogic,
    pub execute: EXLogic,
    pub memory: MEMLogic,
    pub writeback: WBLogic,
}

impl Registers {
    fn update(&mut self, logic: &Logic) {
        let old_rd = self.memwb.rd_index;

        // MEM-WB Latch
        self.memwb.added_pc = self.exmem.added_pc;
        self.memwb.alu_output = self.exmem.alu_output;
        self.memwb.mem_data_out = logic.memory.mem_data_out;
        self.memwb.rd_index = self.exmem.rd_index;

        self.memwb.opcode = self.exmem.opcode;

        self.memwb.instruction = self.exmem.instruction;

        //println!("Old RD:{}", old_rd);

        // Data Memory

        //insert code for Data Mem write here
        //check if instr. is a stor,
        if self.exmem.opcode == 0b0100011 {
            //println!("Storing value to Data Mem!");
            //use these to find the right address to pull from\
            if self.exmem.alu_output == 0 {
                panic!("Instruction tried storing value to data memory address 0.")
            }
            let which_word = (self.exmem.alu_output) / 4;
            let align = self.exmem.alu_output % 4;
            /*println!(
                "whichword: {:?} , Align: {:?}, Funct3: {:?} to-store: {:#034b}",
                which_word, align, self.exmem.funct3, self.exmem.mem_data_in
            );*/
            if self.data_mem.contains_key(&which_word) {
                if self.exmem.funct3 == 0b000 {
                    //Store Byte
                    self.data_mem.insert(
                        which_word,
                        match align {
                            0 => {
                                (self.data_mem.get(&which_word).unwrap()
                                    & 0b11111111111111111111111100000000)
                                    + (self.exmem.mem_data_in & 0b11111111)
                            }
                            1 => {
                                (self.data_mem.get(&which_word).unwrap()
                                    & 0b11111111111111110000000011111111)
                                    + ((self.exmem.mem_data_in & 0b11111111) << 8)
                            }
                            2 => {
                                (self.data_mem.get(&which_word).unwrap()
                                    & 0b11111111000000001111111111111111)
                                    + ((self.exmem.mem_data_in & 0b11111111) << 16)
                            }
                            3 => {
                                (self.data_mem.get(&which_word).unwrap()
                                    & 0b00000000111111111111111111111111)
                                    + ((self.exmem.mem_data_in & 0b11111111) << 24)
                            }
                            _ => panic!("align value is larger than 3!"),
                        },
                    );
                } else if self.exmem.funct3 == 0b001 {
                    //Store Half-Word
                    self.data_mem.insert(
                        which_word,
                        match align {
                            0 => {
                                (self.data_mem.get(&which_word).unwrap()
                                    & 0b11111111111111110000000000000000)
                                    + (self.exmem.mem_data_in & 0b1111111111111111)
                            }
                            1 => panic!("Misaligned store value!"),
                            2 => {
                                (self.data_mem.get(&which_word).unwrap()
                                    & 0b00000000000000001111111111111111)
                                    + ((self.exmem.mem_data_in & 0b1111111111111111) << 16)
                            }
                            3 => panic!("Misaligned store value!"),
                            _ => panic!("Align value is larger thna 3!"),
                        },
                    );
                } else if self.exmem.funct3 == 0b010 {
                    //Store Word
                    if align > 3 {
                        panic!("align value is larger than 3!")
                    }
                    if align > 0 {
                        panic!("Misaligned store value!")
                    }

                    self.data_mem.insert(which_word, self.exmem.mem_data_in);
                } else {
                    panic!("Invalid funct3 on a Store instruction!")
                }
            } else {
                //if there, the value is 0, must be initialized!!
                if self.exmem.funct3 == 0b000 {
                    //Store Byte
                    self.data_mem.insert(
                        which_word,
                        match align {
                            0 => self.exmem.mem_data_in & 0b11111111,
                            1 => (self.exmem.mem_data_in & 0b11111111) << 8,
                            2 => (self.exmem.mem_data_in & 0b11111111) << 16,
                            3 => (self.exmem.mem_data_in & 0b11111111) << 24,
                            _ => panic!("Align value is more than 3!"),
                        },
                    );
                } else if self.exmem.funct3 == 0b001 {
                    //Store Half-Word
                    self.data_mem.insert(
                        which_word,
                        match align {
                            0 => self.exmem.mem_data_in & 0b1111111111111111,
                            1 => panic!("Misaligned store value!"),
                            2 => (self.exmem.mem_data_in & 0b1111111111111111) << 16,
                            3 => panic!("Misaligned store value!"),
                            _ => panic!("Align value is larger thna 3!"),
                        },
                    );
                } else if self.exmem.funct3 == 0b010 {
                    //Store Word
                    if align > 3 {
                        panic!("align value is larger than 3!")
                    }
                    if align > 0 {
                        panic!("Misaligned store value!")
                    }

                    self.data_mem.insert(which_word, self.exmem.mem_data_in);
                } else {
                    panic!("Invalid funct3 on a Store instruction!")
                }
            }
        }

        //EX-MEM Latch

        self.exmem.added_pc = self.idex.added_pc;
        self.exmem.alu_output = logic.execute.alu_output;
        //self.exmem.mem_data_in = logic.execute.formux_r2;
        self.exmem.mem_data_in = self.idex.r2_data;
        self.exmem.rd_index = self.idex.rd_index;

        self.exmem.opcode = self.idex.opcode;
        self.exmem.funct3 = self.idex.funct3;

        self.exmem.instruction = self.idex.instruction;

        // ID-EX Latch
        self.idex.base_pc = self.ifid.base_pc;
        self.idex.added_pc = self.ifid.added_pc;

        self.idex.r1_data = logic.decode.regmem_r1;
        self.idex.r2_data = logic.decode.regmem_r2;
        self.idex.immediates = logic.decode.immediates;
        self.idex.rd_index = logic.decode.decode_rd;

        self.idex.opcode = logic.decode.decode_opcode;
        self.idex.funct3 = logic.decode.decode_funct3;
        self.idex.funct7 = logic.decode.decode_funct7;
        self.idex.r2_index = logic.decode.decode_r2;
        self.idex.r1_index = logic.decode.decode_r1;

        self.idex.instruction = self.ifid.instruction;

        // Register Memory. Write to it.
        assert!(old_rd < 0b100000); //Register indices are always 5 bits or less.
        if (old_rd != 0) {
            self.reg_mem[old_rd as usize] = logic.writeback.wb_data;
        }

        // IF-ID latch. Transfer
        self.ifid.base_pc = self.pc;
        self.ifid.added_pc = logic.fetch.pcadder_out;
        self.ifid.instruction = logic.fetch.instruction_out;

        // Program Counter.
        self.pc = logic.fetch.pcmux_out;
    }
}

impl Logic {
    fn update(&mut self, state: &Registers) {
        // =========================
        // WB Stage
        // =========================

        // Just need to handle WB Multiplexor.
        if state.memwb.opcode == 0b1101111 || state.memwb.opcode == 0b1100111 {
            //JAL, JALR, store (pc+4) into RD
            self.writeback.wb_used = 2;
            self.writeback.wb_data = state.memwb.added_pc;
        } else if state.memwb.opcode == 0b0000011 {
            // LB, LH, LW, LBU, LHU. The load instructions all load the memory read data into RD
            self.writeback.wb_used = 1;
            self.writeback.wb_data = state.memwb.mem_data_out;
        } else if state.memwb.opcode == 0b1100011 || state.memwb.opcode == 0b0100011 {
            // Branches & Stores. These write nothing to RD at all!
            self.writeback.wb_used = 3;
            self.writeback.wb_data = 0xdeadbeef; //special value representing null
        } else {
            //all other opcodes return the ALU's result to the RD
            self.writeback.wb_data = state.memwb.alu_output;
        }

        //======================
        // EX Stage
        //======================

        //figures out Instruction Type of the instruction from the ID-EX Latch.
        // used by many EX-stage components
        let instr_type: InstrT = isa::get_instruction_type(state.idex.opcode);

        // Forwarding Multiplexors. Decides if it must perform EX-EX or MEM-EX forwarding.
        // R1 first.

        if state.idex.r1_index == 0 {
            // if R1 is $r0, do NOT forward. it is a constant register.
            self.execute.r1_forwarded = 0;
            self.execute.formux_r1 = state.idex.r1_data;
        } else if matches!(instr_type, InstrT::Utype) || matches!(instr_type, InstrT::Jtype) {
            // U-type & J-type have no R1. do nothing.
            self.execute.r1_forwarded = 0;
            self.execute.formux_r1 = state.idex.r1_data;
        } else {
            // Forwarding might be needed. Check RD index of instructions further along.
            if state.idex.r1_index == state.exmem.rd_index {
                //need to EX-EX forward!
                //println!("EX-EX forward!");
                self.execute.r1_forwarded = 1;
                self.execute.formux_r1 = state.exmem.alu_output;
            } else if state.idex.r1_index == state.memwb.rd_index {
                //need to MEM-EX forward!
                //println!("EX-MEM Forward!");
                self.execute.r1_forwarded = 2;
                self.execute.formux_r1 = self.writeback.wb_data;
            } else {
                // no forwarding needed!
                self.execute.r1_forwarded = 0;
                self.execute.formux_r1 = state.idex.r1_data;
            }
        }

        if state.idex.r2_index == 0 {
            // if R2 is $r0, do NOT forward. it is a constant register.
            self.execute.r2_forwarded = 0;
            self.execute.formux_r2 = state.idex.r2_data;
        } else if matches!(instr_type, InstrT::Utype)
            || matches!(instr_type, InstrT::Jtype)
            || matches!(instr_type, InstrT::Itype)
        {
            // U-type, J-type, & I-type have no R2. do nothing.
            self.execute.r2_forwarded = 0;
            self.execute.formux_r2 = state.idex.r2_data;
        } else {
            // Forwarding might be needed. Check RD index of instructions further along.
            if state.idex.r2_index == state.exmem.rd_index {
                //need to EX-EX forward!
                self.execute.r2_forwarded = 1;
                self.execute.formux_r2 = state.exmem.alu_output;
            } else if state.idex.r2_index == state.memwb.rd_index {
                //need to MEM-EX forward!
                self.execute.r2_forwarded = 2;
                self.execute.formux_r2 = state.memwb.alu_output;
            } else {
                // no forwarding needed!
                self.execute.r2_forwarded = 0;
                self.execute.formux_r2 = state.idex.r2_data;
            }
        }

        // R1-PC Multiplexor.
        // Decides if Operand 1 is the R1 value or the Program Count.
        // Uses Instruction Type to decide.

        if (matches!(instr_type, InstrT::Utype)
            || matches!(instr_type, InstrT::Jtype)
            || matches!(instr_type, InstrT::Btype))
        {
            self.execute.op1 = state.idex.base_pc;
            self.execute.pc_used = true;
        } else {
            self.execute.op1 = self.execute.formux_r1;
            self.execute.pc_used = false;
        }

        // R2-Immediates Multiplexor.
        // Same thing for Op2, but between R2 value and Immediates value.
        if matches!(instr_type, InstrT::Rtype) {
            self.execute.op2 = self.execute.formux_r2;
            self.execute.imm_used = false;
        } else {
            self.execute.op2 = state.idex.immediates;
            self.execute.imm_used = true;
        }

        // Branch Comparator.
        // If instruction is a branch, compares R1 and R2 to see if branch is taken,
        // while ALU calculates jump address.

        // Checks based off the 3bit funct3-code and R1 & R2, if the instruction is a Branch, whether a branch happens or not.
        self.execute.branch_taken = match state.idex.funct3 {
            0b000 => state.idex.r1_data == state.idex.r2_data, //BEQ
            0b001 => state.idex.r1_data != state.idex.r2_data, //BNE
            0b100 => (state.idex.r1_data as i32) < (state.idex.r2_data as i32), //BLT
            0b101 => (state.idex.r1_data as i32) > (state.idex.r2_data as i32), //BGE
            0b110 => state.idex.r1_data < state.idex.r2_data,  //BLTU
            0b111 => state.idex.r1_data > state.idex.r2_data,  //BGEU
            _ => false,                                        //not a branching instruction.
        };

        if state.idex.opcode != 0b1100011 {
            self.execute.branch_taken = false;
        } //if it's not a branch, should be false regardless.

        //ALU
        //actually computes the instruction!  for signed operations, convert Ops to signed then convert result to unsigned.
        //println!("Op1: {}", self.execute.op1 as i32);
        //println!("Op2: {}", self.execute.op2 as i32);
        /*println!(
            "Out: {}",
            ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32
        ); */
        self.execute.alu_output = match state.idex.opcode {
            0b0110111 => self.execute.op2, //LUI, just put in immediate as is
            0b0010111 => self.execute.op1 + self.execute.op2, //AUIPC, add PC and  shifted Imm, store in RD
            0b1101111 => ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32, //JAL, add PC and Imm, store in RD, jump there
            0b1100111 => {
                (((self.execute.op2 as i32) + (self.execute.op1 as i32)) as u32)
                    & 0b11111111111111111111111111111110
            } //JALR, add the R1 and imm together then set lowest bit to 0.
            0b1100011 => ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32, //Branches. Perform signed addition between PC and Imm to figure out new PC.
            0b0000011 => ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32, // LB/LH/LW, add R1 and Imm offset to get source memory address.
            0b0100011 => ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32, // SB/SH/SW, add R1 and Imm offset to get destination memory address. R2 goes straight to Mem (thru EX-MEM latch).
            0b0010011 => match state.idex.funct3 {
                //Register-Immediate instructions
                0b000 => ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32, //ADDI, siggned add R1 and Imm
                0b010 => {
                    if (self.execute.op1 as i32) < (self.execute.op2 as i32) {
                        1
                    } else {
                        0
                    }
                } //STLI, check if R1 < Imm
                0b011 => {
                    if self.execute.op1 < self.execute.op2 {
                        1
                    } else {
                        0
                    }
                } //STLIU, STLI but unsigned.
                0b100 => self.execute.op1 ^ self.execute.op2, //XORI, bitwise exclusive-or on R1 and Imm.
                0b110 => self.execute.op1 | self.execute.op2, //ORI, bitwise or on R1 and Imm.
                0b111 => self.execute.op1 & self.execute.op2, //ANDI, bitwise and on R1 and Imm.
                0b001 => {
                    if self.execute.op2 > 31 {
                        panic!()
                    } else {
                        self.execute.op1 << self.execute.op2
                    }
                } // SLLI, shift R1 left by  shamt ([4-0] of Imm) bits.
                0b101 => match self.execute.op2 >> 5 {
                    0b0000000 => self.execute.op1 >> self.execute.op2, //SRLI, shift R1 right logically by shamt bits
                    0b0100000 => {
                        ((self.execute.op1 as i32) >> ((self.execute.op2 - 0b010000000000) as i32))
                            as u32
                    } //SRAI, shift R1 right arithmetically by shamt bits
                    _ => panic!("Invalid upper Imm. bits for Right Shift Instruction!"),
                },
                _ => panic!("funct3-code is bigger than 3 bits! this shouldnt happen!!!"),
            },
            0b0110011 => match state.idex.funct3 {
                //Register-Register instructions
                0b000 => match state.idex.funct7 {
                    0b0000000 => ((self.execute.op1 as i32) + (self.execute.op2 as i32)) as u32, //ADD
                    0b0100000 => ((self.execute.op1 as i32) - (self.execute.op2 as i32)) as u32, //SUB
                    _ => panic!("Invalid funct7 for ADD/SUB instruction"),
                },
                0b001 => self.execute.op1 << (self.execute.op2 & 0b11111), //SLL, shift left logical. Shift R1 left by the lowest 5 bits of R2
                0b010 => {
                    if (self.execute.op1 as i32) < (self.execute.op2 as i32) {
                        1
                    } else {
                        0
                    }
                } //SLT,  signed less than
                0b011 => {
                    if self.execute.op1 < self.execute.op2 {
                        1
                    } else {
                        0
                    }
                } //SLTU, unsigned less than
                0b100 => self.execute.op1 ^ self.execute.op2, //XOR, bitwise exclusive or
                0b101 => match state.idex.funct7 {
                    0b0000000 => self.execute.op1 >> (self.execute.op2 & 0b11111), //SRL, shift right logical. Shift R1 logically right by the lowest 5 bits or R2
                    0b0100000 => ((self.execute.op1 as i32) >> (self.execute.op2 & 0b11111)) as u32, //SRA, shift right arithmetic.
                    _ => panic!("Invalid upper Imm. bits for Right Shift Instruction!"),
                },
                0b110 => self.execute.op1 | self.execute.op2, //OR, bitwise or
                0b111 => self.execute.op1 & self.execute.op2, //AND, bitwise and
                _ => panic!("funct3-code is bigger than 3 bits! this shouldnt happen!!!"),
            },
            0 => 0, //NOP Instruction. Does nothing.
            _ => panic!("Invalid or Unimplemented Instruction!"),
        };

        //=================================
        // IF Stage
        //=================================

        //must update PCAdder first.
        self.fetch.pcadder_out = state.pc + 4;

        //PCMux: First, check if opcode FROM EX STAGE is Jump, Branching, or neither
        if (state.idex.opcode == 0b1101111 || state.idex.opcode == 0b1100111) {
            self.fetch.jumped = true;
            self.fetch.pcmux_out = self.execute.alu_output; //initiates jump by setting PC to result of address adition
        } else if (state.idex.opcode == 0b1100011 && self.execute.branch_taken) {
            //if branch taken!
            self.fetch.jumped = true;
            self.fetch.pcmux_out = self.execute.alu_output; //initiates jump
        } else {
            //if not branch, or branch not taken
            self.fetch.jumped = false;
            self.fetch.pcmux_out = self.fetch.pcadder_out;
        }

        if ((state.pc / 4) as usize) >= state.instr_mem.len() {
            //if reached end of program... put in NOPs to let the previous instructions finish.
            self.fetch.instruction_out = 0;
        } else {
            self.fetch.instruction_out = state.instr_mem[(state.pc / 4) as usize];
        }

        //==============================
        // ID Stage
        //==============================

        // Decoder
        //gets the opcode, r1 index, r2 index, and destination register index out of the instruction, even if they end up being unused.
        //uses bit-wise AND operation on a mask in order to get the desired bits, dividing to rem

        // need to get lowest 7 bits out, just use a mask to get (6-0)
        self.decode.decode_opcode = (state.ifid.instruction & 0b1111111) as u8;
        // need to get bits (19-15) out. use mask, then shift right all the zero'd bits
        self.decode.decode_r1 = ((state.ifid.instruction & 0b11111000000000000000) >> 15) as u8;
        // need to get bits (24-20) out.
        self.decode.decode_r2 =
            ((state.ifid.instruction & 0b1111100000000000000000000) >> 20) as u8;
        // need to get bits (11-7) out... unless B or S, those have no rd
        let instr_type: InstrT = isa::get_instruction_type(self.decode.decode_opcode);
        if matches!(instr_type, InstrT::Stype) || matches!(instr_type, InstrT::Stype) {
            //This operation has no register output. discard write to $r0
            self.decode.decode_rd = 0;
        } else {
            //isn't S or B type, needs an rd.
            self.decode.decode_rd = ((state.ifid.instruction & 0b111110000000) >> 7) as u8;
        }

        // need to get the bits (14-12) out.
        self.decode.decode_funct3 = ((state.ifid.instruction & 0b111000000000000) >> 12) as u8;
        //need to get the bits (31-25) out.
        self.decode.decode_funct7 =
            ((state.ifid.instruction & 0b11111110000000000000000000000000) >> 25) as u8;

        // Register Memory: Read.
        assert!(self.decode.decode_r1 < 32 && self.decode.decode_r2 < 32);
        self.decode.regmem_r1 = state.reg_mem[self.decode.decode_r1 as usize];
        self.decode.regmem_r2 = state.reg_mem[self.decode.decode_r2 as usize];

        //secret forwarding! takes care of small data hazard that wouldn't happen in-model.
        if self.decode.decode_r1 == state.memwb.rd_index {
            self.decode.regmem_r1 = self.writeback.wb_data;
        } else if self.decode.decode_r2 == state.memwb.rd_index {
            self.decode.regmem_r2 = self.writeback.wb_data;
        }

        // Immediates Decoder
        //rearranges the immediates of an instruction by type, so they're where the ALU expects them.
        if matches!(instr_type, InstrT::Rtype) {
            self.decode.immediates = 0; //Outputs a useless value. R-Type has no immediates.
        } else if matches!(instr_type, InstrT::Itype) {
            //in this one, simply take the 31st thru 12th bits! they're already where they want to be.
            self.decode.immediates = ((state.ifid.instruction as i32) >> 20) as u32;
        } else if matches!(instr_type, InstrT::Stype) {
            //(31-25) goes to [11-5],  (11-7) goes to [4-0]. do each separately, then bitwise OR

            //                       the (31-25) is converted to signed so that it does an arithmetic right shift
            self.decode.immediates =
                ((((state.ifid.instruction & 0b11111110000000000000000000000000) as i32) >> 20)
                    as u32)
                    | ((state.ifid.instruction & 0b111110000000) >> 7);
        } else if matches!(instr_type, InstrT::Btype) {
            //A (31) to [12] ,B (30-25) to [10-5], C (11-8) to [4-1], D (7) to [11]
            let imm_a: u32 = (((state.ifid.instruction & 0b10000000000000000000000000000000)
                as i32)
                >> 19) as u32;
            let imm_b: u32 = (state.ifid.instruction & 0b01111110000000000000000000000000) >> 20;
            let imm_c: u32 = (state.ifid.instruction & 0b00000000000000000000111100000000) >> 7;
            let imm_d: u32 = (state.ifid.instruction & 0b00000000000000000000000010000000) << 4;
            //println!("{:#b}",imm_a);
            //println!("{:#b}",imm_b);
            //println!("{:#b}",imm_c);
            //println!("{:#b}",imm_d);

            self.decode.immediates =
                ((((imm_a | imm_b | imm_c | imm_d) << 19) as i32) >> 19) as u32;
        //the wonky shifting just sign-extends the 12-bit Imm preemptively
        } else if matches!(instr_type, InstrT::Utype) {
            //(31-12) goes to [31-12]... so just mask the rest!
            self.decode.immediates = state.ifid.instruction & 0b11111111111111111111000000000000;
        } else {
            //only J-type left!  E  (31) to [20], F  (30-21) to [10-1], G  (20) to [11],  H  (19-12) to [19-12]
            let imm_e: u32 = (state.ifid.instruction & 0b10000000000000000000000000000000) >> 11;
            let imm_f: u32 = (state.ifid.instruction & 0b01111111111000000000000000000000) >> 20;
            let imm_g: u32 = (state.ifid.instruction & 0b00000000000100000000000000000000) >> 9;
            let imm_h: u32 = state.ifid.instruction & 0b00000000000011111111000000000000;
            //println!("{:#b}", imm_e);
            //println!("{:#b}", imm_f);
            //println!("{:#b}", imm_g);
            //println!("{:#b}", imm_h);

            self.decode.immediates =
                ((((imm_e | imm_f | imm_g | imm_h) << 11) as i32) >> 11) as u32;
            //same here, sign-shifts the 20-bit Imm
        }

        // =========================
        // MEM Stage
        // =========================

        //Reading Data Memory is the only thing that happens in this stage.
        //check if instruction is a load.
        if state.exmem.opcode == 0b0000011 {
            let which_word = (state.exmem.alu_output) / 4;
            let align = state.exmem.alu_output % 4;
            if state.data_mem.contains_key(&which_word) {
                if state.exmem.funct3 == 0b000 {
                    //Load Byte, need to sign extend.
                    self.memory.mem_data_out = match align {
                        0 => {
                            ((((state.data_mem.get(&which_word).unwrap()
                                & 0b00000000000000000000000011111111)
                                as i32)
                                << 24)
                                >> 24) as u32
                        }
                        1 => {
                            ((((state.data_mem.get(&which_word).unwrap()
                                & 0b00000000000000001111111100000000)
                                as i32)
                                << 16)
                                >> 24) as u32
                        }
                        2 => {
                            ((((state.data_mem.get(&which_word).unwrap()
                                & 0b00000000111111110000000000000000)
                                as i32)
                                << 8)
                                >> 24) as u32
                        }
                        3 => {
                            (((state.data_mem.get(&which_word).unwrap()
                                & 0b11111111000000000000000000000000)
                                as i32)
                                >> 24) as u32
                        }
                        _ => panic!("Align is greater than 3!"),
                    }
                } else if state.exmem.funct3 == 0b001 {
                    //Load Half-Word, need to sign extend.
                    self.memory.mem_data_out = match align {
                        0 => {
                            ((((state.data_mem.get(&which_word).unwrap()
                                & 0b00000000000000001111111111111111)
                                as i32)
                                << 16)
                                >> 16) as u32
                        }
                        1 => panic!("Misaligned Load!"),
                        2 => {
                            (((state.data_mem.get(&which_word).unwrap()
                                & 0b11111111111111110000000000000000)
                                as i32)
                                >> 16) as u32
                        }
                        3 => panic!("Misaligned Load!"),
                        _ => panic!("Align is greater than 3!"),
                    }
                } else if state.exmem.funct3 == 0b010 {
                    //Load Word.
                    if align > 3 {
                        panic!("Align is greater than 3!")
                    }
                    if align < 0 {
                        panic!("Misaligned Load!")
                    }
                    self.memory.mem_data_out = *state.data_mem.get(&which_word).unwrap();
                } else if state.exmem.funct3 == 0b100 {
                    //Load Byte Unsigned. No sign extend
                    self.memory.mem_data_out = match align {
                        0 => {
                            (state.data_mem.get(&which_word).unwrap()
                                & 0b00000000000000000000000011111111)
                        }
                        1 => {
                            (state.data_mem.get(&which_word).unwrap()
                                & 0b00000000000000001111111100000000)
                                >> 8
                        }
                        2 => {
                            (state.data_mem.get(&which_word).unwrap()
                                & 0b00000000111111110000000000000000)
                                >> 16
                        }
                        3 => {
                            (state.data_mem.get(&which_word).unwrap()
                                & 0b11111111000000000000000000000000)
                                >> 24
                        }
                        _ => panic!("Align is greater than 3!"),
                    }
                } else if state.exmem.funct3 == 0b101 {
                    //Load Half Word Unsigned. No sign extend.
                    self.memory.mem_data_out = match align {
                        0 => {
                            (state.data_mem.get(&which_word).unwrap()
                                & 0b00000000000000001111111111111111)
                        }
                        1 => panic!("Misaligned Load!"),
                        2 => {
                            (state.data_mem.get(&which_word).unwrap()
                                & 0b11111111111111110000000000000000)
                                >> 16
                        }
                        3 => panic!("Misaligned Load!"),
                        _ => panic!("Align is greater than 3!"),
                    }
                } else {
                    panic!("Invalid funct3 for a Load Instruction!")
                }
            } else {
                //if this value has never been accessed before, it is trivially zero!
                self.memory.mem_data_out = 0;
            }
        } else {
            //if no value is read... output useless value
            self.memory.mem_data_out = 0xdeadbeef;
        }
    }
}

pub fn step(state: &mut Registers, logic: &mut Logic) {
    state.update(logic);
    logic.update(state);
}
