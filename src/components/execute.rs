use crate::components::*;
use crate::isa::isa;
use crate::isa::isa::InstrT;
pub struct IDEXLatch<'a> {
    pub base_pc_in: u32,
    pub base_pc_out: u32,
    pub base_pc_out_chk: bool,

    pub added_pc_in: u32,
    pub added_pc_out: u32,
    pub added_pc_out_chk: bool,

    pub r1_data_in: u32,
    pub r1_data_out: u32,
    pub r1_data_out_chk: bool,

    pub r2_data_in: u32,
    pub r2_data_out: u32,
    pub r2_data_out_chk: bool,

    pub immediates_in: u32,
    pub immediates_out: u32,
    pub immediates_out_chk: bool,

    pub rd_index_in: u8,
    pub rd_index_out: u8,
    pub rd_index_out_chk: bool,

    pub ifid_latch_ptr: Option<&'a IFIDLatch<'a>>,
    pub reg_mem_ptr: Option<&'a RegMem<'a>>,
    pub imm_dec_ptr: Option<&'a ImmDecoder<'a>>,
    pub instr_dec_ptr: Option<&'a InstrDecoder<'a>>,

    //these won't be displayed on interface!
    pub opcode_in: u8,
    pub opcode_out: u8,
    pub opcode_out_chk: bool,

    pub funct3_in: u8,
    pub funct3_out: u8,
    pub funct3_out_chk: bool,

    pub funct7_in: u8,
    pub funct7_out: u8,
    pub funct7_out_chk: bool,

    pub r1_index_in: u8,
    pub r1_index_out: u8,
    pub r1_index_out_chk: bool,

    pub r2_index_in: u8,
    pub r2_index_out: u8,
    pub r2_index_out_chk: bool,
}

impl IDEXLatch<'_> {
    pub fn grab_input(&mut self) {
        if !(self.ifid_latch_ptr.unwrap().base_pc_out_chk
            && self.ifid_latch_ptr.unwrap().added_pc_out_chk)
        {
            panic!("ID-EX Latch tried to grab the program counts from IF-ID Latch before it was ready!");
        } else if !(self.reg_mem_ptr.unwrap().r1_data_out_chk
            && self.reg_mem_ptr.unwrap().r2_data_out_chk)
        {
            panic!(
                "ID-EX Latch tried to grab the R1 and R2 data from the RegMem before it was ready!"
            );
        } else if !(self.imm_dec_ptr.unwrap().immediates_out_chk) {
            panic!("ID-EX Latch tried to grab the immediates from ImmDecoder before it was ready!");
        } else if !(self.instr_dec_ptr.unwrap().rd_index_out_chk
            && self.instr_dec_ptr.unwrap().r1_index_out_chk
            && self.instr_dec_ptr.unwrap().r2_index_out_chk)
        {
            panic!("ID-EX Latch tried to grab the R1, R2, and RD index from InstrDecoder before it was ready!");
        } else if !(self.instr_dec_ptr.unwrap().opcode_out_chk
            && self.instr_dec_ptr.unwrap().funct3_out_chk
            && self.instr_dec_ptr.unwrap().funct7_out_chk)
        {
            panic!("ID-EX latch tried to grab the opcode and funct-codes from InstrDecoder before it was ready!")
        } else {
            self.base_pc_in = self.ifid_latch_ptr.unwrap().base_pc_out;
            self.added_pc_in = self.ifid_latch_ptr.unwrap().added_pc_out;
            self.r1_data_in = self.reg_mem_ptr.unwrap().r1_data_out;
            self.r2_data_in = self.reg_mem_ptr.unwrap().r2_data_out;
            self.immediates_in = self.imm_dec_ptr.unwrap().immediates_out;
            self.rd_index_in = self.instr_dec_ptr.unwrap().rd_index_out;

            self.opcode_in = self.instr_dec_ptr.unwrap().opcode_out;
            self.funct3_in = self.instr_dec_ptr.unwrap().funct3_out;
            self.funct7_in = self.instr_dec_ptr.unwrap().funct7_out;
            self.r1_index_in = self.instr_dec_ptr.unwrap().r1_index_out;
            self.r2_index_in = self.instr_dec_ptr.unwrap().r2_index_out;
        }
    }

    pub fn open_latch(&mut self) {
        self.base_pc_out = self.base_pc_in;
        self.added_pc_out = self.added_pc_in;
        self.r1_data_out = self.r1_data_in;
        self.r2_data_out = self.r2_data_in;
        self.immediates_out = self.immediates_in;
        self.rd_index_out = self.rd_index_in;

        self.base_pc_out_chk = true;
        self.added_pc_out_chk = true;
        self.r1_data_out_chk = true;
        self.r2_data_out_chk = true;
        self.immediates_out_chk = true;
        self.rd_index_out_chk = true;

        self.opcode_out = self.opcode_in;
        self.opcode_out_chk = true;

        self.funct3_out = self.funct3_in;
        self.funct3_out_chk = true;

        self.funct7_out = self.funct7_in;
        self.funct7_out_chk = true;

        self.r1_index_out = self.r1_index_in;
        self.r1_index_out_chk = true;

        self.r2_index_out = self.r2_index_in;
        self.r2_index_out_chk = true;
    }
}

//I dont understand how these work, so I'll leave them be for now.
pub struct R1ForMux<'a> {
    pub normal_r1_in: u32, //from IDEX latch
    pub exex_r1_in: u32,   //from EX-MEM latch
    pub memex_r1_in: u32,  //from WB Mux

    pub r1_out: u32,
    pub r1_out_chk: bool,

    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
    pub exmem_latch_ptr: Option<&'a EXMEMLatch<'a>>,
    pub memwb_latch_ptr: Option<&'a MEMWBLatch<'a>>,
    pub wb_mux_ptr: Option<&'a WBMux<'a>>,

    //not shown on GUI
    pub exex_rd_in: u8,  //from EX-MEM latch
    pub memex_rd_in: u8, //from MEM-WB latch
    pub r1_index_in: u8, //from ID-EX latch
}

impl R1ForMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().r1_data_out_chk
            && self.idex_latch_ptr.unwrap().r1_index_out_chk)
        {
            panic!("R1-Forwarding Mux tried grabbing normal R1 data, R1 index before ID-EX Latch was ready!");
        } else if !(self.exmem_latch_ptr.unwrap().rd_index_out_chk
            && self.exmem_latch_ptr.unwrap().result_out_chk)
        {
            panic!("R1-Forwarding Mux tried grabbing  ex-ex forwarding RD index & ex-ex R1 data before EX-MEM Latch was ready!");
        } else if !(self.memwb_latch_ptr.unwrap().rd_index_out_chk) {
            panic!("R1-Forwarding Mux tried grabbing mem-ex forwarding RD index before MEM-WB Latch was ready!");
        } else if !(self.wb_mux_ptr.unwrap().wb_data_out_chk) {
            panic!("R1-Forwarding Mux tried grabbing mem-ex R1 data before WB Mux was ready!");
        } else {
            self.normal_r1_in = self.idex_latch_ptr.unwrap().r1_data_out;
            self.exex_r1_in = self.exmem_latch_ptr.unwrap().result_out;
            self.memex_r1_in = self.wb_mux_ptr.unwrap().wb_data_out;

            self.exex_rd_in = self.exmem_latch_ptr.unwrap().rd_index_out;
            self.memex_rd_in = self.memwb_latch_ptr.unwrap().rd_index_out;
            self.r1_index_in = self.idex_latch_ptr.unwrap().r1_index_out;
        }
    }

    pub fn decide(&mut self) {
        if self.r1_index_in == self.exex_rd_in {
            //if the output of the previous instruction is the input of this one...
            self.r1_out = self.exex_r1_in;
        } else if self.r1_index_in == self.memex_rd_in {
            //if the output of the 2nd-previous instruction is the input of this one...
            self.r1_out = self.memex_r1_in;
        } else {
            // otherwise, proceed as normal.
            self.r1_out = self.normal_r1_in;
        }
        self.r1_out_chk = true;
    }
}

pub struct R2ForMux<'a> {
    pub normal_r2_in: u32, //from IDEX latch
    pub exex_r2_in: u32,   //from EX-MEM latch
    pub memex_r2_in: u32,  //from WB Mux

    pub r2_out: u32,
    pub r2_out_chk: bool,

    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
    pub exmem_latch_ptr: Option<&'a EXMEMLatch<'a>>,
    pub memwb_latch_ptr: Option<&'a MEMWBLatch<'a>>,
    pub wb_mux_ptr: Option<&'a WBMux<'a>>,

    //not shown on GUI
    pub exex_rd_in: u8,  //from EX-MEM latch
    pub memex_rd_in: u8, //from MEM-WB latch
    pub r2_index_in: u8, //from ID-EX latch
}

impl R2ForMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().r2_data_out_chk
            && self.idex_latch_ptr.unwrap().r2_index_out_chk)
        {
            panic!("R2-Forwarding Mux tried grabbing normal R2 data, R2 index before ID-EX Latch was ready!");
        } else if !(self.exmem_latch_ptr.unwrap().rd_index_out_chk
            && self.exmem_latch_ptr.unwrap().result_out_chk)
        {
            panic!("R2-Forwarding Mux tried grabbing  ex-ex forwarding RD index & ex-ex R2 data before EX-MEM Latch was ready!");
        } else if !(self.memwb_latch_ptr.unwrap().rd_index_out_chk) {
            panic!("R2-Forwarding Mux tried grabbing mem-ex forwarding RD index before MEM-WB Latch was ready!");
        } else if !(self.wb_mux_ptr.unwrap().wb_data_out_chk) {
            panic!("R2-Forwarding Mux tried grabbing mem-ex R2 data before WB Mux was ready!");
        } else {
            self.normal_r2_in = self.idex_latch_ptr.unwrap().r2_data_out;
            self.exex_r2_in = self.exmem_latch_ptr.unwrap().result_out;
            self.memex_r2_in = self.wb_mux_ptr.unwrap().wb_data_out;

            self.exex_rd_in = self.exmem_latch_ptr.unwrap().rd_index_out;
            self.memex_rd_in = self.memwb_latch_ptr.unwrap().rd_index_out;
            self.r2_index_in = self.idex_latch_ptr.unwrap().r2_index_out;
        }
    }

    pub fn decide(&mut self) {
        if self.r2_index_in == self.exex_rd_in {
            //if the output of the previous instruction is the input of this one...
            self.r2_out = self.exex_r2_in;
        } else if self.r2_index_in == self.memex_rd_in {
            //if the output of the 2nd-previous instruction is the input of this one...
            self.r2_out = self.memex_r2_in;
        } else {
            // otherwise, proceed as normal.
            self.r2_out = self.normal_r2_in;
        }
        self.r2_out_chk = true;
    }
}
//////
///
///
///
pub struct R1PCMux<'a> {
    pub r1_in: u32,
    pub pc_in: u32,
    pub opcode_in: u8,

    pub op1_out: u32,
    pub op1_out_chk: bool,

    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
    pub r1for_mux_ptr: Option<&'a R1ForMux<'a>>,
}

impl R1PCMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().opcode_out_chk
            && self.idex_latch_ptr.unwrap().base_pc_out_chk)
        {
            panic!("R1-PC Mux tried to get opcode and PC from ID-EX Latch before it was ready!");
        } else if !(self.r1for_mux_ptr.unwrap().r1_out_chk) {
            panic!("R1-PC Mux tried to get R1 data from R1-Forward Mux before it was read!");
        } else {
            self.r1_in = self.r1for_mux_ptr.unwrap().r1_out;
            self.pc_in = self.idex_latch_ptr.unwrap().base_pc_out;
            self.opcode_in = self.idex_latch_ptr.unwrap().opcode_out;
        }
    }

    pub fn decide(&mut self) {
        let instr_type: InstrT = isa::get_instruction_type(self.opcode_in);

        if (matches!(instr_type, InstrT::Utype)
            || matches!(instr_type, InstrT::Jtype)
            || matches!(instr_type, InstrT::Btype))
        {
            self.op1_out = self.pc_in;
        } else {
            self.op1_out = self.r1_in;
        }
        self.op1_out_chk = true;
    }
}

pub struct R2ImmMux<'a> {
    pub r2_in: u32,
    pub immediates_in: u32,
    pub opcode_in: u8,

    pub op2_out: u32,
    pub op2_out_chk: bool,

    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
    pub r2for_mux_ptr: Option<&'a R2ForMux<'a>>,
}

impl R2ImmMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().opcode_out_chk
            && self.idex_latch_ptr.unwrap().immediates_out_chk)
        {
            panic!("R2-PC Mux tried to get opcode and Immediates from ID-EX Latch before it was ready!");
        } else if !(self.r2for_mux_ptr.unwrap().r2_out_chk) {
            panic!("R2-PC Mux tried to get R2 data from R2-Forward Mux before it was read!");
        } else {
            self.r2_in = self.r2for_mux_ptr.unwrap().r2_out;
            self.immediates_in = self.idex_latch_ptr.unwrap().immediates_out;
            self.opcode_in = self.idex_latch_ptr.unwrap().opcode_out;
        }
    }

    pub fn decide(&mut self) {
        let instr_type: InstrT = isa::get_instruction_type(self.opcode_in);

        if matches!(instr_type, InstrT::Rtype) {
            self.op2_out = self.r2_in;
        } else {
            self.op2_out = self.immediates_in;
        }
        self.op2_out_chk = true;
    }
}

pub struct BranchComparator<'a> {
    pub r1_in: u32,
    pub r2_in: u32,

    pub branches_out: bool,
    pub branches_out_chk: bool,

    pub r1for_mux_ptr: Option<&'a R1ForMux<'a>>,
    pub r2for_mux_ptr: Option<&'a R2ForMux<'a>>,

    //wont be displayed
    pub funct3_in: u8,
    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
}

impl BranchComparator<'_> {
    pub fn grab_input(&mut self) {
        if !(self.r1for_mux_ptr.unwrap().r1_out_chk) {
            panic!("BranchComparator tried getting R1 value before R1-Forward Mux was ready!");
        } else if !(self.r2for_mux_ptr.unwrap().r2_out_chk) {
            panic!("BranchComparator tried getting R2 value before R2-Forward Mux was ready!");
        } else if !(self.idex_latch_ptr.unwrap().funct3_out_chk) {
            panic!("BranchComparator tried getting funct3-code before ID-EX Latch was ready!");
        } else {
            self.r1_in = self.r1for_mux_ptr.unwrap().r1_out;
            self.r2_in = self.r2for_mux_ptr.unwrap().r2_out;

            self.funct3_in = self.idex_latch_ptr.unwrap().funct3_out;

            self.branches_out_chk = true;
        }
    }

    pub fn compare(&mut self) {
        // Checks based off the 3bit funct3-code and R1 & R2, if the instruction is a Branch, whether a branch happens or not.
        self.branches_out = match self.funct3_in {
            0b000 => self.r1_in == self.r2_in,                  //BEQ
            0b001 => self.r1_in != self.r2_in,                  //BNE
            0b100 => (self.r1_in as i32) < (self.r2_in as i32), //BLT
            0b101 => (self.r1_in as i32) > (self.r2_in as i32), //BGE
            0b110 => self.r1_in < self.r2_in,                   //BLTU
            0b111 => self.r1_in > self.r2_in,                   //BGEU
            _ => false,                                         //not a branching instruction.
        };
    }
}

pub struct ALUnit<'a> {
    pub op1_in: u32,
    pub op2_in: u32,

    pub result_out: u32,
    pub result_out_chk: bool,

    pub r1pc_mux_ptr: Option<&'a R1PCMux<'a>>,
    pub r2imm_mux_ptr: Option<&'a R2ImmMux<'a>>,

    //not listed on GUI!
    pub opcode_in: u8,
    pub funct3_in: u8,
    pub funct7_in: u8,
    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
}

impl ALUnit<'_> {
    pub fn grab_input(&mut self) {
        if !(self.r1pc_mux_ptr.unwrap().op1_out_chk) {
            panic!("ALU tried grabbing Op1 before R1-PC Mux was ready!");
        } else if !(self.r2imm_mux_ptr.unwrap().op2_out_chk) {
            panic!("ALU tried grabbing Op2 before R2-Imm Mux was ready!");
        } else if !(self.idex_latch_ptr.unwrap().opcode_out_chk
            && self.idex_latch_ptr.unwrap().funct3_out_chk
            && self.idex_latch_ptr.unwrap().funct7_out_chk)
        {
            panic!("ALU tried grabbing opcode & funct-codes before IDEX Latch was ready!");
        } else {
            self.op1_in = self.r1pc_mux_ptr.unwrap().op1_out;
            self.op2_in = self.r2imm_mux_ptr.unwrap().op2_out;

            self.opcode_in = self.idex_latch_ptr.unwrap().opcode_out;
            self.funct3_in = self.idex_latch_ptr.unwrap().funct3_out;
            self.funct7_in = self.idex_latch_ptr.unwrap().funct7_out;
        }
    }

    pub fn compute(&mut self) {
        //actually computes the instruction!  for signed operations, convert Ops to signed then convert result to unsigned.
        println!("{}", self.op1_in as i32);
        println!("{}", self.op2_in as i32);
        println!("{}", ((self.op1_in as i32) + (self.op2_in as i32)) as u32);
        self.result_out = match self.opcode_in {
            0b0110111 => self.op2_in,               //LUI, just put in immediate as is
            0b0010111 => self.op1_in + self.op2_in, //AUIPC, add PC and  shifted Imm, store in RD
            0b1101111 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, //JAL, add PC and Imm, store in RD, jump there
            0b1100111 => {
                (((self.op2_in as i32) + (self.op1_in as i32)) as u32)
                    & 0b11111111111111111111111111111110
            } //JALR, add the R1 and imm together then set lowest bit to 0.
            0b1100011 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, //Branches. Perform signed addition between PC and Imm to figure out new PC.
            0b0000011 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, // LB/LH/LW, add R1 and Imm offset to get source memory address.
            0b0100011 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, // SB/SH/SW, add R1 and Imm offset to get destination memory address. R2 goes straight to Mem (thru EX-MEM latch).
            0b0010011 => match self.funct3_in {
                //Register-Immediate instructions
                0b000 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, //ADDI, siggned add R1 and Imm
                0b010 => {
                    if (self.op1_in as i32) < (self.op2_in as i32) {
                        1
                    } else {
                        0
                    }
                } //STLI, check if R1 < Imm
                0b011 => {
                    if self.op1_in < self.op2_in {
                        1
                    } else {
                        0
                    }
                } //STLIU, STLI but unsigned.
                0b100 => self.op1_in ^ self.op2_in, //XORI, bitwise exclusive-or on R1 and Imm.
                0b110 => self.op1_in | self.op2_in, //ORI, bitwise or on R1 and Imm.
                0b111 => self.op1_in & self.op2_in, //ANDI, bitwise and on R1 and Imm.
                0b001 => {
                    if self.op2_in > 31 {
                        panic!()
                    } else {
                        self.op1_in << self.op2_in
                    }
                } // SLLI, shift R1 left by  shamt ([4-0] of Imm) bits.
                0b101 => match self.op2_in >> 5 {
                    0b0000000 => self.op1_in >> self.op2_in, //SRLI, shift R1 right logically by shamt bits
                    0b0100000 => {
                        ((self.op1_in as i32) >> ((self.op2_in - 0b010000000000) as i32)) as u32
                    } //SRAI, shift R1 right arithmetically by shamt bits
                    _ => panic!("Invalid upper Imm. bits for Right Shift Instruction!"),
                },
                _ => panic!("funct3-code is bigger than 3 bits! this shouldnt happen!!!"),
            },
            0b0110011 => match self.funct3_in {
                //Register-Register instructions
                0b000 => match self.funct7_in {
                    0b0000000 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, //ADD
                    0b0100000 => ((self.op1_in as i32) - (self.op2_in as i32)) as u32, //SUB
                    _ => panic!("Invalid funct7 for ADD/SUB instruction"),
                },
                0b001 => self.op1_in << (self.op2_in & 0b11111), //SLL, shift left logical. Shift R1 left by the lowest 5 bits of R2
                0b010 => {
                    if (self.op1_in as i32) < (self.op2_in as i32) {
                        1
                    } else {
                        0
                    }
                } //SLT,  signed less than
                0b011 => {
                    if self.op1_in < self.op2_in {
                        1
                    } else {
                        0
                    }
                } //SLTU, unsigned less than
                0b100 => self.op1_in ^ self.op2_in,              //XOR, bitwise exclusive or
                0b101 => match self.funct7_in {
                    0b0000000 => self.op1_in >> (self.op2_in & 0b11111), //SRL, shift right logical. Shift R1 logically right by the lowest 5 bits or R2
                    0b0100000 => ((self.op1_in as i32) >> (self.op2_in & 0b11111)) as u32, //SRA, shift right arithmetic.
                    _ => panic!("Invalid upper Imm. bits for Right Shift Instruction!"),
                },
                0b110 => self.op1_in | self.op2_in, //OR, bitwise or
                0b111 => self.op1_in & self.op2_in, //AND, bitwise and
                _ => panic!("funct3-code is bigger than 3 bits! this shouldnt happen!!!"),
            },
            _ => panic!("Invalid or Unimplemented Instruction!"),
        };
        self.result_out_chk = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::*;

    #[test]
    fn idex_latch() {
        let ifidlatch = IFIDLatch {
            base_pc_in: 0,
            base_pc_out: 32,
            base_pc_out_chk: true,

            added_pc_in: 0,
            added_pc_out: 36,
            added_pc_out_chk: true,

            instruction_in: 0,
            instruction_out: 0b00000010000100001001000010000001,
            instruction_out_chk: true,

            pc_ptr: None,
            pc_adder_ptr: None,
            instr_mem_ptr: None,
        };

        let instrdec = InstrDecoder {
            instruction_in: 0,
            ifid_latch_ptr: None,

            opcode_out: 3,
            opcode_out_chk: true,

            r1_index_out: 6,
            r1_index_out_chk: true,

            r2_index_out: 7,
            r2_index_out_chk: true,

            rd_index_out: 8,
            rd_index_out_chk: true,

            //won't be displayed!
            funct3_out: 4,
            funct3_out_chk: true,
            funct7_out: 5,
            funct7_out_chk: true,
        };

        let regmem = RegMem {
            r1_index_in: 0,
            r2_index_in: 0,
            rd_index_in: 0,
            wb_data_in: 0,

            registers: Vec::<u32>::from([
                0, 7, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]),

            r1_data_out: 42,
            r2_data_out: 420,
            r1_data_out_chk: true,
            r2_data_out_chk: true,

            instr_dec_ptr: Some(&instrdec),
            memwb_latch_ptr: None,
            wb_mux_ptr: None,
        };

        let immdec = ImmDecoder {
            opcode_in: 0,
            instr_dec_ptr: Some(&instrdec),

            instruction_in: 0,
            ifid_latch_ptr: Some(&ifidlatch),

            immediates_out: 4200,
            immediates_out_chk: true,
        };

        let mut idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 0,
            base_pc_out_chk: false,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            r1_data_in: 0,
            r1_data_out: 0,
            r1_data_out_chk: false,

            r2_data_in: 0,
            r2_data_out: 0,
            r2_data_out_chk: false,

            immediates_in: 0,
            immediates_out: 0,
            immediates_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            ifid_latch_ptr: Some(&ifidlatch),
            reg_mem_ptr: Some(&regmem),
            imm_dec_ptr: Some(&immdec),
            instr_dec_ptr: Some(&instrdec),

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: false,

            funct7_in: 0,
            funct7_out: 0,
            funct7_out_chk: false,

            r1_index_in: 0,
            r1_index_out: 0,
            r1_index_out_chk: false,

            r2_index_in: 0,
            r2_index_out: 0,
            r2_index_out_chk: false,
        };

        idexlatch.grab_input();
        idexlatch.open_latch();

        assert!(idexlatch.base_pc_out_chk);
        assert!(idexlatch.added_pc_out_chk);
        assert!(idexlatch.r1_data_out_chk);
        assert!(idexlatch.r2_data_out_chk);
        assert!(idexlatch.immediates_out_chk);
        assert!(idexlatch.rd_index_out_chk);
        assert!(idexlatch.opcode_out_chk);
        assert!(idexlatch.funct3_out_chk);
        assert!(idexlatch.funct7_out_chk);
        assert!(idexlatch.r1_index_out_chk);
        assert!(idexlatch.r2_index_out_chk);

        assert_eq!(idexlatch.base_pc_out, 32);
        assert_eq!(idexlatch.added_pc_out, 36);
        assert_eq!(idexlatch.r1_data_out, 42);
        assert_eq!(idexlatch.r2_data_out, 420);
        assert_eq!(idexlatch.immediates_out, 4200);
        assert_eq!(idexlatch.rd_index_out, 8);
        assert_eq!(idexlatch.opcode_out, 3);
        assert_eq!(idexlatch.funct3_out, 4);
        assert_eq!(idexlatch.funct7_out, 5);
        assert_eq!(idexlatch.r1_index_out, 6);
        assert_eq!(idexlatch.r2_index_out, 7);
    }

    #[test]
    fn forwardmuxes() {
        let idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 0,
            base_pc_out_chk: false,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            r1_data_in: 0,
            r1_data_out: 1,
            r1_data_out_chk: true,

            r2_data_in: 0,
            r2_data_out: 4,
            r2_data_out_chk: true,

            immediates_in: 0,
            immediates_out: 0,
            immediates_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            ifid_latch_ptr: None,
            reg_mem_ptr: None,
            imm_dec_ptr: None,
            instr_dec_ptr: None,

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: false,

            funct7_in: 0,
            funct7_out: 0,
            funct7_out_chk: false,

            r1_index_in: 0,
            r1_index_out: 10,
            r1_index_out_chk: true,

            r2_index_in: 0,
            r2_index_out: 11,
            r2_index_out_chk: true,
        };

        let exmemlatch = EXMEMLatch {
            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            result_in: 0,
            result_out: 2,
            result_out_chk: true,

            mem_data_in: 0,
            mem_data_out: 0,
            mem_data_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 14,
            rd_index_out_chk: true,

            idex_latch_ptr: None,
            alu_ptr: None,
            r2for_mux_ptr: None,

            //won't be officially shown
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: false,
        };

        let memwblatch = MEMWBLatch {
            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            result_in: 0,
            result_out: 0,
            result_out_chk: false,

            mem_read_in: 0,
            mem_read_out: 0,
            mem_read_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 15,
            rd_index_out_chk: true,

            exmem_latch_ptr: None,
            data_mem_ptr: None,

            //won't be officially shown
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,
        };

        let wbmux = WBMux {
            added_pc_in: 0,
            result_in: 0,
            mem_read_in: 0,

            wb_data_out: 3,
            wb_data_out_chk: true,

            memwb_latch_ptr: None,
            //not in GUI
            opcode_in: 0,
        };

        let mut r1formux = R1ForMux {
            normal_r1_in: 0, //from IDEX latch, 1
            exex_r1_in: 0,   //from EX-MEM latch, 2
            memex_r1_in: 0,  //from WB Mux, 3

            r1_out: 0,
            r1_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            exmem_latch_ptr: Some(&exmemlatch),
            memwb_latch_ptr: Some(&memwblatch),
            wb_mux_ptr: Some(&wbmux),

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch, vary per test
            memex_rd_in: 0, //from MEM-WB latch, vary per test
            r1_index_in: 0, //from ID-EX latch, 10
        };

        let mut r2formux = R2ForMux {
            normal_r2_in: 0, //from IDEX latch, 4
            exex_r2_in: 0,   //from EX-MEM latch, 2
            memex_r2_in: 0,  //from WB Mux, 3

            r2_out: 0,
            r2_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            exmem_latch_ptr: Some(&exmemlatch),
            memwb_latch_ptr: Some(&memwblatch),
            wb_mux_ptr: Some(&wbmux),

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch, vary per test
            memex_rd_in: 0, //from MEM-WB latch, vary per test
            r2_index_in: 0, //from ID-EX latch, 11
        };

        //test input-grabbing
        r1formux.grab_input();
        r2formux.grab_input();

        assert_eq!(r1formux.normal_r1_in, 1);
        assert_eq!(r1formux.exex_r1_in, 2);
        assert_eq!(r1formux.memex_r1_in, 3);
        assert_eq!(r1formux.exex_rd_in, 14);
        assert_eq!(r1formux.memex_rd_in, 15);
        assert_eq!(r1formux.r1_index_in, 10);

        assert_eq!(r2formux.normal_r2_in, 4);
        assert_eq!(r2formux.exex_r2_in, 2);
        assert_eq!(r2formux.memex_r2_in, 3);
        assert_eq!(r2formux.exex_rd_in, 14);
        assert_eq!(r2formux.memex_rd_in, 15);
        assert_eq!(r2formux.r2_index_in, 11);

        //test the no-forwarding scenario.

        r1formux.decide(); //reg. addresses are all diff, so should pick normal_r1 and normal_r2
        r2formux.decide();

        assert!(r1formux.r1_out_chk);
        assert!(r2formux.r2_out_chk);

        assert_eq!(r1formux.r1_out, 1); //normal_r1
        assert_eq!(r2formux.r2_out, 4); //normal_r2

        //test  EX-EX for R1,  MEM-EX for R2

        r1formux.exex_rd_in = 10;
        r2formux.memex_rd_in = 11;

        r1formux.decide(); //r1_index and exex_rd match, so should ex-ex
        r2formux.decide(); //r2_index and memex_rd match, so should mem-ex

        assert_eq!(r1formux.r1_out, 2); //exex_r1
        assert_eq!(r2formux.r2_out, 3); //memex_r2

        //test MEM-EX for R1, EX-EX for R2

        r1formux.exex_rd_in = 20;
        r1formux.memex_rd_in = 10;
        r2formux.exex_rd_in = 11;
        r2formux.memex_rd_in = 20;

        r1formux.decide(); //r1_index, memex_rd match.
        r2formux.decide(); //r2_index, exex_rd match.

        assert_eq!(r1formux.r1_out, 3); //memex_r1
        assert_eq!(r2formux.r2_out, 2); //exex_r2

        //test for if ALL addresses match. should do EX-EX for both, most recent one.

        r1formux.exex_rd_in = 10;
        r2formux.memex_rd_in = 11;

        r1formux.decide(); //r1_index, all match
        r2formux.decide(); //r2_index, all match

        assert_eq!(r1formux.r1_out, 2); //exex_r1
        assert_eq!(r2formux.r2_out, 2); //exex_r2
    }

    #[test]
    fn r1pc_mux() {
        let idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 32,
            base_pc_out_chk: true,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            r1_data_in: 0,
            r1_data_out: 1,
            r1_data_out_chk: false,

            r2_data_in: 0,
            r2_data_out: 4,
            r2_data_out_chk: false,

            immediates_in: 0,
            immediates_out: 0,
            immediates_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            ifid_latch_ptr: None,
            reg_mem_ptr: None,
            imm_dec_ptr: None,
            instr_dec_ptr: None,

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0b0110111, //an opcode that wants the PC, not R1.
            opcode_out_chk: true,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: false,

            funct7_in: 0,
            funct7_out: 0,
            funct7_out_chk: false,

            r1_index_in: 0,
            r1_index_out: 10,
            r1_index_out_chk: true,

            r2_index_in: 0,
            r2_index_out: 11,
            r2_index_out_chk: true,
        };

        let r1formux = R1ForMux {
            normal_r1_in: 0, //from IDEX latch,
            exex_r1_in: 0,   //from EX-MEM latch,
            memex_r1_in: 0,  //from WB Mux,

            r1_out: 1,
            r1_out_chk: true,

            idex_latch_ptr: None,
            exmem_latch_ptr: None,
            memwb_latch_ptr: None,
            wb_mux_ptr: None,

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch,
            memex_rd_in: 0, //from MEM-WB latch,
            r1_index_in: 0, //from ID-EX latch,
        };

        let mut r1pcmux = R1PCMux {
            r1_in: 0, //will be 1
            pc_in: 0, //wiill be 32
            opcode_in: 0,

            op1_out: 0,
            op1_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            r1for_mux_ptr: Some(&r1formux),
        };
        //test if gets PC from a U-type instruction.
        r1pcmux.grab_input();
        r1pcmux.decide();

        assert!(r1pcmux.op1_out_chk);
        assert_eq!(r1pcmux.op1_out, 32);

        //test if gets R1 from an I-type instruction.

        r1pcmux.opcode_in = 0b0010011; //opcode for an immediate-register arithmetic op.
        r1pcmux.decide();

        assert_eq!(r1pcmux.op1_out, 1);
    }

    #[test]
    fn r2imm_mux() {
        let idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 32,
            base_pc_out_chk: true,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            r1_data_in: 0,
            r1_data_out: 1,
            r1_data_out_chk: false,

            r2_data_in: 0,
            r2_data_out: 4,
            r2_data_out_chk: false,

            immediates_in: 2,
            immediates_out: 2,
            immediates_out_chk: true,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            ifid_latch_ptr: None,
            reg_mem_ptr: None,
            imm_dec_ptr: None,
            instr_dec_ptr: None,

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0b0010011, //an I-type opcode that wants the Immediates, not the R2
            opcode_out_chk: true,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: false,

            funct7_in: 0,
            funct7_out: 0,
            funct7_out_chk: false,

            r1_index_in: 0,
            r1_index_out: 10,
            r1_index_out_chk: true,

            r2_index_in: 0,
            r2_index_out: 11,
            r2_index_out_chk: true,
        };

        let r2formux = R2ForMux {
            normal_r2_in: 0,
            exex_r2_in: 0,
            memex_r2_in: 0,

            r2_out: 1,
            r2_out_chk: true,

            idex_latch_ptr: None,
            exmem_latch_ptr: None,
            memwb_latch_ptr: None,
            wb_mux_ptr: None,

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch, vary per test
            memex_rd_in: 0, //from MEM-WB latch, vary per test
            r2_index_in: 0, //from ID-EX latch
        };

        let mut r2immmux = R2ImmMux {
            r2_in: 0,         //will be 1
            immediates_in: 0, //will be 2
            opcode_in: 0,

            op2_out: 0,
            op2_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            r2for_mux_ptr: Some(&r2formux),
        };

        r2immmux.grab_input();

        assert!(!(r2immmux.op2_out_chk));

        r2immmux.decide(); //Test Immediates choice.

        assert!(r2immmux.op2_out_chk);
        assert_eq!(r2immmux.op2_out, 2);

        r2immmux.opcode_in = 0b0110011; //set opcode to R-type code that wants R2, not Immediates
        r2immmux.decide(); // Test R2 choice.

        assert_eq!(r2immmux.op2_out, 1);
    }

    #[test]
    fn branchcomp() {
        let idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 32,
            base_pc_out_chk: true,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            r1_data_in: 0,
            r1_data_out: 1,
            r1_data_out_chk: false,

            r2_data_in: 0,
            r2_data_out: 4,
            r2_data_out_chk: false,

            immediates_in: 2,
            immediates_out: 2,
            immediates_out_chk: true,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            ifid_latch_ptr: None,
            reg_mem_ptr: None,
            imm_dec_ptr: None,
            instr_dec_ptr: None,

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0b0010011,
            opcode_out_chk: true,

            funct3_in: 0,
            funct3_out: 0b111,
            funct3_out_chk: true,

            funct7_in: 0,
            funct7_out: 0,
            funct7_out_chk: false,

            r1_index_in: 0,
            r1_index_out: 10,
            r1_index_out_chk: true,

            r2_index_in: 0,
            r2_index_out: 11,
            r2_index_out_chk: true,
        };
        let r1formux = R1ForMux {
            normal_r1_in: 0, //from IDEX latch, 1
            exex_r1_in: 0,   //from EX-MEM latch, 2
            memex_r1_in: 0,  //from WB Mux, 3

            r1_out: 0,
            r1_out_chk: true,

            idex_latch_ptr: None,
            exmem_latch_ptr: None,
            memwb_latch_ptr: None,
            wb_mux_ptr: None,

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch, vary per test
            memex_rd_in: 0, //from MEM-WB latch, vary per test
            r1_index_in: 0, //from ID-EX latch, 10
        };

        let r2formux = R2ForMux {
            normal_r2_in: 0,
            exex_r2_in: 0,
            memex_r2_in: 0,

            r2_out: 1,
            r2_out_chk: true,

            idex_latch_ptr: None,
            exmem_latch_ptr: None,
            memwb_latch_ptr: None,
            wb_mux_ptr: None,

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch, vary per test
            memex_rd_in: 0, //from MEM-WB latch, vary per test
            r2_index_in: 0, //from ID-EX latch
        };

        let mut bcomp = BranchComparator {
            r1_in: 13,
            r2_in: 13,

            branches_out: false,
            branches_out_chk: false,

            r1for_mux_ptr: Some(&r1formux),
            r2for_mux_ptr: Some(&r2formux),

            funct3_in: 0,
            idex_latch_ptr: Some(&idexlatch),
        };

        bcomp.grab_input();
        assert_eq!(bcomp.r1_in, 0);
        assert_eq!(bcomp.r2_in, 1);
        assert_eq!(bcomp.funct3_in, 0b111);
        assert!(bcomp.branches_out_chk);

        //Test BEQ, false then true
        bcomp.funct3_in = 0b000;
        bcomp.compare();
        assert!(!bcomp.branches_out);

        bcomp.r2_in = 0;
        bcomp.compare();
        assert!(bcomp.branches_out);

        //Test BNE, false then true
        bcomp.funct3_in = 0b001;
        bcomp.compare();
        assert!(!bcomp.branches_out);

        bcomp.r2_in = (i32::from(-10)) as u32; //very big unsigned int, very low signed
        bcomp.compare();
        assert!(bcomp.branches_out);

        //test BLT, false then true, signed
        bcomp.funct3_in = 0b100;
        bcomp.compare();
        assert!(!bcomp.branches_out);

        bcomp.r1_in = (i32::from(-20)) as u32;
        bcomp.compare();
        assert!(bcomp.branches_out);

        //test BGE, signed
        bcomp.funct3_in = 0b101;
        bcomp.compare();
        assert!(!bcomp.branches_out);

        bcomp.r1_in = (i32::from(-5)) as u32;
        bcomp.compare();
        assert!(bcomp.branches_out);

        //test BLTU, unsigned
        bcomp.funct3_in = 0b110;
        bcomp.compare();
        assert!(!bcomp.branches_out);

        bcomp.r1_in = 3;
        bcomp.compare();
        //println!("{}, {}", bcomp.r1_in, bcomp.r2_in);
        assert!(bcomp.branches_out);

        //test BGEU, unsigned
        bcomp.funct3_in = 0b111;
        bcomp.compare();
        assert!(!bcomp.branches_out);

        bcomp.r1_in = (i32::from(-20)) as u32;
        bcomp.r2_in = 40;
        bcomp.compare();
        assert!(bcomp.branches_out);
    }

    #[test]
    fn alu() {
        let idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 32,
            base_pc_out_chk: true,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            r1_data_in: 0,
            r1_data_out: 1,
            r1_data_out_chk: false,

            r2_data_in: 0,
            r2_data_out: 4,
            r2_data_out_chk: false,

            immediates_in: 2,
            immediates_out: 2,
            immediates_out_chk: true,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            ifid_latch_ptr: None,
            reg_mem_ptr: None,
            imm_dec_ptr: None,
            instr_dec_ptr: None,

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0b0110111, //the first opcode to check, for LUI
            opcode_out_chk: true,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: true,

            funct7_in: 0,
            funct7_out: 0,
            funct7_out_chk: true,

            r1_index_in: 0,
            r1_index_out: 10,
            r1_index_out_chk: true,

            r2_index_in: 0,
            r2_index_out: 11,
            r2_index_out_chk: true,
        };

        let r1pcmux = R1PCMux {
            r1_in: 0,
            pc_in: 0,
            opcode_in: 0,

            op1_out: 1,
            op1_out_chk: true,

            idex_latch_ptr: None,
            r1for_mux_ptr: None,
        };

        let r2immmux = R2ImmMux {
            r2_in: 0,
            immediates_in: 0b00111001110011100111000000000000,
            opcode_in: 0,

            op2_out: 0b00111001110011100111000000000000,
            op2_out_chk: true,

            idex_latch_ptr: None,
            r2for_mux_ptr: None,
        };

        let mut alu = ALUnit {
            op1_in: 0,
            op2_in: 0,

            result_out: 0,
            result_out_chk: false,

            r1pc_mux_ptr: Some(&r1pcmux),
            r2imm_mux_ptr: Some(&r2immmux),

            //not listed on GUI!
            opcode_in: 0,
            funct3_in: 0,
            funct7_in: 0,
            idex_latch_ptr: Some(&idexlatch),
        };

        alu.grab_input();

        assert!(!(alu.result_out_chk));

        //LUI: Load Upper Immediate.
        alu.compute();

        assert_eq!(alu.result_out, 0b00111001110011100111000000000000);

        //AUIPC: add PC to Upper Immediate
        alu.opcode_in = 0b0010111;
        alu.op1_in = 0b000100010001;
        alu.compute();

        assert_eq!(alu.result_out, 0b00111001110011100111000100010001);

        //JAL: jump. calculates target address. Branches, Stores, and Reads all do this same thing.
        alu.opcode_in = 0b1101111;
        alu.op1_in = (i32::from(32)) as u32;
        alu.op2_in = (i32::from(-16)) as u32;

        alu.compute();

        assert_eq!(alu.result_out, 16);

        //JALR: indirect jump
        alu.opcode_in = 0b1100111;
        alu.op1_in = 0b11;
        alu.op2_in = 0b100;

        alu.compute();

        assert_eq!(alu.result_out, 0b110);

        //Register-Immediate Instructions
        alu.opcode_in = 0b0010011;
        alu.op1_in = (i32::from(5)) as u32; //r1 = 5
        alu.op2_in = (i32::from(3)) as u32; // i = 3

        //ADDI
        alu.funct3_in = 0b000;
        alu.compute();
        assert_eq!(alu.result_out, (i32::from(8)) as u32);

        //SLTI: Set Less Than Imm. 1 if r1 < i.
        alu.funct3_in = 0b010;
        alu.compute();
        assert_eq!(alu.result_out, 0);

        alu.op1_in = (i32::from(-5)) as u32;
        alu.compute();
        assert_eq!(alu.result_out, 1);

        //SLTIU: Same thing, but unsigned.
        alu.funct3_in = 0b011;
        alu.op1_in = 0b11111111;
        alu.op2_in = 0b11;
        alu.compute();
        assert_eq!(alu.result_out, 0);

        alu.op1_in = 0b1;
        alu.compute();
        assert_eq!(alu.result_out, 1);

        //XORI: Exclusiv OR immediate.
        alu.funct3_in = 0b100;
        alu.op1_in = 0b11111;
        alu.op2_in = 0b01010;
        alu.compute();

        assert_eq!(alu.result_out, 0b10101);

        //ORI:
        alu.funct3_in = 0b110;
        alu.op1_in = 0b0111;
        alu.op2_in = 0b1110;
        alu.compute();

        assert_eq!(alu.result_out, 0b1111);

        //ANDI:
        alu.funct3_in = 0b111;
        alu.op1_in = 0b10101;
        alu.op2_in = 0b11110;
        alu.compute();

        assert_eq!(alu.result_out, 0b10100);

        //SLLI:
        alu.funct3_in = 0b001;
        alu.op1_in = 0b10101;
        alu.op2_in = 0b11; //shamt = 3
        alu.compute();

        assert_eq!(alu.result_out, 0b10101000);

        //SRLI:
        alu.funct3_in = 0b101;
        alu.op1_in = 0b11110000111100001111000011110000;
        alu.op2_in = 0b10; //shamt = 2
        alu.compute();

        assert_eq!(alu.result_out, 0b00111100001111000011110000111100);
        //SRAI:
        alu.op1_in = 0b11110000111100001111000011110000;
        alu.op2_in = 0b010000000011; //shamt = 3
        alu.compute();

        assert_eq!(alu.result_out, 0b11111110000111100001111000011110);

        //Onto RR instructions!
        //ADD is same as branches and Loads/Stores

        //SUB
        alu.opcode_in = 0b0110011;
        alu.funct3_in = 0b000;
        alu.funct7_in = 0b0100000;
        alu.op1_in = (i32::from(32)) as u32;
        alu.op2_in = (i32::from(-16)) as u32;
        alu.compute();

        assert_eq!(alu.result_out, 48);

        alu.op1_in = (i32::from(32)) as u32;
        alu.op2_in = (i32::from(128)) as u32;
        alu.compute();

        assert_eq!(alu.result_out, (i32::from(-96)) as u32);

        //SLL: shift R1 left by lowest 5 bits of R2

        alu.funct3_in = 0b001;
        alu.funct7_in = 0;
        alu.op1_in = 0b10101;
        alu.op2_in = 0b11100100; //shamt should be 0b00100, 4
        alu.compute();

        assert_eq!(alu.result_out, 0b101010000);

        //SLT: Signed Less Than
        alu.funct3_in = 0b010;
        alu.op1_in = (i32::from(-32)) as u32;
        alu.op2_in = (i32::from(1)) as u32;
        alu.compute();

        assert_eq!(alu.result_out, 1);

        alu.op1_in = (i32::from(1)) as u32;
        alu.op2_in = (i32::from(-16)) as u32;
        alu.compute();

        assert_eq!(alu.result_out, 0);

        //SLTU: Unsigned Less Than
        alu.funct3_in = 0b011;
        alu.op1_in = (i32::from(-32)) as u32; //unsigned, this is a very large integer
        alu.op2_in = (i32::from(1)) as u32;
        alu.compute();

        assert_eq!(alu.result_out, 0);

        alu.op1_in = (i32::from(1)) as u32;
        alu.op2_in = (i32::from(-16)) as u32; //when unsigned, this is a very large integer
        alu.compute();

        assert_eq!(alu.result_out, 1);

        //XOR is same as XORI
        //SRL, SRA are same as SRLI and SRAI, just with 5 bit limit.
        // OR, AND same as ORI, ANDI
    }
}
