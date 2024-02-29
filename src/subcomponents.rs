

pub mod subc {
    use crate::isa::isa;
    use crate::isa::isa::InstrT;

pub struct PCMux<'a> {
    pub added_pc_in : u32,
    pub result_in : u32,

    pub count_out : u32,
    pub count_out_chk : bool,

    pub pc_adder_ptr :  Option<&'a PCAdder<'a>>,
    pub alu_ptr :  Option<&'a ALUnit<'a>>,

    //decider not implemented yet.
}



pub struct ProgramCounter<'a> {
    pub count_in : u32,

    pub count_out : u32,
    pub count_out_chk : bool,

    pub pc_mux_ptr :  Option<&'a PCMux<'a>>
}

impl ProgramCounter<'_> {
    pub fn grab_input(&mut self) {
        if self.pc_mux_ptr.unwrap().count_out_chk {
            self.count_in = self.pc_mux_ptr.unwrap().count_out;
        } else {
            panic!("ProgramCounter tried to update its count before count-ouput from the PCMux could be sent");
        }
    }
    pub fn update_count(&mut self) {
        self.count_out = self.count_in;   //doesn't do much more than store the PC.
        self.count_out_chk = true;
    }
}


pub struct PCAdder<'a> {
    pub count_in : u32,

    pub count_out : u32,
    pub count_out_chk : bool,

    pub pc_ptr :  Option<&'a ProgramCounter<'a>>,
}

impl PCAdder<'_> {
    pub fn grab_input(&mut self) {
        if self.pc_ptr.unwrap().count_out_chk {
            self.count_in = self.pc_ptr.unwrap().count_out;
        } else {
            panic!("PCAdder tried to update its count before count-output from the ProgramCounter could be sent");
        }
    }
    pub fn add_count(&mut self) {
        self.count_out = self.count_in + 4;   //simply increments the program count by 4, so it points to the next instruction.
        self.count_out_chk = true;
    }
}


pub struct InstrMem<'a> {
    pub ins_addr_in : u32,

    pub ins_array : Vec<u32>,

    pub instruction_out : u32,
    pub instruction_out_chk : bool,

    pub pc_ptr :  Option<&'a ProgramCounter<'a>>
}

impl InstrMem<'_> {
    pub fn grab_input(&mut self) {
        if self.pc_ptr.unwrap().count_out_chk {
            self.ins_addr_in = self.pc_ptr.unwrap().count_out;
        } else {
            panic!("PCAdder tried to update its count before count-output from the ProgramCounter could be sent");        
        }
    }
    pub fn fetch_instruction(&mut self) { //fetches the 32-bit instruction the PC refers to,
        self.instruction_out = self.ins_array[(self.ins_addr_in as usize)/4]; //the first [0] instruction is at address 0, the next [1] is at address 4, the next [2] at address 8, so on.
        self.instruction_out_chk = true;
    }
}

//The  in the previous objects is dependent on the instruction memory size that the user chooses. Unsigned Int of varying size.

pub struct IFIDLatch<'a> {
    pub base_pc_in : u32,
    pub base_pc_out : u32,
    pub base_pc_out_chk : bool, 

    pub added_pc_in : u32,
    pub added_pc_out : u32,
    pub added_pc_out_chk : bool,

    pub instruction_in : u32, 
    pub instruction_out : u32,
    pub instruction_out_chk : bool,

    pub pc_ptr :  Option<&'a ProgramCounter<'a>>,
    pub pc_adder_ptr :  Option<&'a PCAdder<'a>>,
    pub instr_mem_ptr :  Option<&'a InstrMem<'a>>,
}

impl IFIDLatch<'_> {
    pub fn grab_input (&mut self) {
        if !(self.pc_ptr.unwrap().count_out_chk) {
            panic!("IF-ID Latch tried to update its base_pc before count-output from the ProgramCounter could be sent");  
        } else if !(self.pc_adder_ptr.unwrap().count_out_chk) {
            panic!("IF-ID Latch tried to update its added_pc before count-output from the PCAdder could be sent");  
        } else if !(self.instr_mem_ptr.unwrap().instruction_out_chk) {
            panic!("IF-ID Latch tried to update its instruction before instr-output from the InstrMem could be sent");
        } else {
            self.base_pc_in = self.pc_ptr.unwrap().count_out;
            self.added_pc_in = self.pc_adder_ptr.unwrap().count_out;
            self.instruction_in = self.instr_mem_ptr.unwrap().instruction_out;
        }
    }
    pub fn open_latch (&mut self) {  //opens the latch to let new values pass through.
        self.base_pc_out = self.base_pc_in;
        self.added_pc_out = self.added_pc_in;
        self.instruction_out = self.instruction_in;

        self.base_pc_out_chk = true;
        self.added_pc_out_chk = true;
        self.instruction_out_chk =true;
    }
}

pub struct InstrDecoder<'a> {
    pub instruction_in : u32,
    pub ifid_latch_ptr :  Option<&'a IFIDLatch<'a>>,

    pub opcode_out : u8, //7-bit opcode
    pub opcode_out_chk : bool,

    pub r1_index_out : u8, //5-bit register index
    pub r1_index_out_chk : bool,

    pub r2_index_out : u8,
    pub r2_index_out_chk : bool,

    pub rd_index_out : u8,
    pub rd_index_out_chk : bool,

    //won't be displayed!
    pub funct3_out : u8,
    pub funct3_out_chk : bool,
    pub funct7_out : u8,
    pub funct7_out_chk : bool,
}

impl InstrDecoder<'_> {
    pub fn grab_input(&mut self) {
        if self.ifid_latch_ptr.unwrap().instruction_out_chk {
            self.instruction_in = self.ifid_latch_ptr.unwrap().instruction_out;
        } else {
            panic!("InstrDecoder tried to update its instruction before IF-ID Latch could send it.");
        }
    }
    pub fn decode(&mut self) { //gets the opcode, r1 index, r2 index, and destination register index out of the instruction, even if they end up being unused.

        //uses bit-wise AND operation on a mask in order to get the desired bits, dividing to rem

        // need to get lowest 7 bits out, just use a mask to get (6-0)
        self.opcode_out = (self.instruction_in & 0b1111111 ) as u8;
        self.opcode_out_chk =true;

        // need to get bits (19-15) out. use mask, then shift right all the zero'd bits
        self.r1_index_out = ((self.instruction_in & 0b11111000000000000000) >> 15) as u8;
        self.r1_index_out_chk = true;

        // need to get bits (24-20) out.  
        self.r2_index_out = ((self.instruction_in & 0b1111100000000000000000000) >> 20) as u8;
        self.r2_index_out_chk = true;

        // need to get bits (11-7) out. 
        self.rd_index_out = ((self.instruction_in & 0b111110000000) >> 7) as u8;
        self.rd_index_out_chk = true;

        // need to get the bits (14-12) out.
        self.funct3_out = ((self.instruction_in & 0b111000000000000) >> 12) as u8;
        self.funct3_out_chk = true;

        //need to get the bits (31-25) out.
        self.funct7_out = ((self.instruction_in & 0b11111110000000000000000000000000) >> 25) as u8;
        self.funct7_out_chk = true;
    }
}


pub struct RegMem<'a> {
    pub r1_index_in : u8,
    pub r2_index_in : u8,

    pub rd_index_in : u8,

    pub wb_data_in : u32,  //this is the writeback data from 3 steps ago, getting commited to the register

    pub registers : Vec<u32>, //the value of each corresponds to what that register name points to. 0 always points to 0 constant, and is unused.

    //real_registers : Vec //each of these correspodns to a real physical register! NOT IMPLEMENTED YET

    pub r1_data_out : u32,
    pub r1_data_out_chk : bool,
    pub r2_data_out : u32,
    pub r2_data_out_chk : bool,
    
    pub instr_dec_ptr :  Option<&'a InstrDecoder<'a>>,
    pub memwb_latch_ptr :  Option<&'a MEMWBLatch<'a>>,
    pub wb_mux_ptr :  Option<&'a WBMux<'a>>,
}

impl RegMem<'_> {
    pub fn grab_input(&mut self) {
        if !(self.instr_dec_ptr.unwrap().r1_index_out_chk && self.instr_dec_ptr.unwrap().r2_index_out_chk) {
            panic!("RegMem tried to grab R1 and R2 indices from Instruction Decoder before it was ready.");
        } else if !(self.memwb_latch_ptr.unwrap().rd_index_out_chk) {
            panic!("RegMem tried to grab destination register index from the MEM-WB Latch before it was ready.");
        } else if !(self.wb_mux_ptr.unwrap().wb_data_out_chk) {
            panic!("RegMem tried to grab the writeback data from the WB Mux before it was ready.");
        } else {
            self.r1_index_in = self.instr_dec_ptr.unwrap().r1_index_out;
            self.r2_index_in = self.instr_dec_ptr.unwrap().r2_index_out;
            
            self.rd_index_in = self.memwb_latch_ptr.unwrap().rd_index_out;
            
            self.wb_data_in = self.wb_mux_ptr.unwrap().wb_data_out;
        }
    }

    pub fn write_data(&mut self) {
        if self.rd_index_in == 0 {
            panic!("The $0 register holds a constant zero value! It cannot be written to.");
        }
        self.registers[self.rd_index_in as usize] = self.wb_data_in;
    }

    pub fn fetch_registers(&mut self) {
        self.r1_data_out = self.registers[self.r1_index_in as usize];
        self.r1_data_out_chk = true;

        self.r2_data_out = self.registers[self.r2_index_in as usize];
        self.r2_data_out_chk = true;
    }
}

pub struct ImmDecoder<'a> {
    pub opcode_in : u8,
    pub instr_dec_ptr :  Option<&'a InstrDecoder<'a>>,

    pub instruction_in : u32,
    pub ifid_latch_ptr :  Option<&'a IFIDLatch<'a>>,

    pub immediates_out : u32,
    pub immediates_out_chk : bool,
}

impl ImmDecoder<'_> {
    pub fn grab_input (&mut self) {
        if !(self.instr_dec_ptr.unwrap().opcode_out_chk) {
            panic!("ImmDecoder tried to grab opcode before InstrDecoder was ready!");
        } else if !(self.ifid_latch_ptr.unwrap().instruction_out_chk) {
            panic!("ImmDecoder tried to grab instruction before IF-ID Latch was ready!");
        } else {
            self.opcode_in = self.instr_dec_ptr.unwrap().opcode_out;
            self.instruction_in = self.ifid_latch_ptr.unwrap().instruction_out;
        }
    }

    pub fn decode (&mut self) { //rearranges the immediates of an instruction by type, so they're where the ALU expects them.
        let instr_type : InstrT = isa::get_instruction_type(self.opcode_in);
        if matches!(instr_type, InstrT::Rtype) {
            self.immediates_out = 0xdeadbeef; //Outputs a useless value. R-Type has no immediates.
        } else if matches!(instr_type, InstrT::Itype) { //in this one, simply take the 31st thru 12th bits! they're already where they want to be.
            self.immediates_out = ((self.instruction_in as i32) >> 20) as u32 ;
        } else if matches!(instr_type, InstrT::Stype) { //(31-25) goes to [11-5],  (11-7) goes to [4-0]. do each separately, then bitwise OR

            //                       the (31-25) is converted to signed so that it does an arithmetic right shift
            self.immediates_out = ((((self.instruction_in & 0b11111110000000000000000000000000) as i32) >> 20)as u32) | ((self.instruction_in & 0b111110000000) >> 7);
        } else if matches!(instr_type, InstrT::Btype) { //A (31) to [12] ,B (30-25) to [10-5], C (11-8) to [4-1], D (7) to [11]
            let imm_a : u32 = (self.instruction_in & 0b10000000000000000000000000000000) >> 19;
            let imm_b : u32 = (self.instruction_in & 0b01111110000000000000000000000000) >> 20;
            let imm_c : u32 = (self.instruction_in & 0b00000000000000000000111100000000) >> 7;
            let imm_d : u32 = (self.instruction_in & 0b00000000000000000000000010000000) << 4;
            //println!("{:#b}",imm_a);
            //println!("{:#b}",imm_b);
            //println!("{:#b}",imm_c);
            //println!("{:#b}",imm_d);

            self.immediates_out = ((((imm_a | imm_b | imm_c | imm_d) << 19) as i32) >> 19) as u32 ; //the wonky shifting just sign-extends the 12-bit Imm preemptively
        } else if matches!(instr_type, InstrT::Utype) { //(31-12) goes to [31-12]... so just mask the rest!
            self.immediates_out = self.instruction_in & 0b11111111111111111111000000000000;
        } else { //only J-type left!  E  (31) to [20], F  (30-21) to [10-1], G  (20) to [11],  H  (19-12) to [19-12]
             let imm_e : u32 = (self.instruction_in & 0b10000000000000000000000000000000) >> 11;
             let imm_f : u32 = (self.instruction_in & 0b01111111111000000000000000000000) >> 20;
             let imm_g : u32 = (self.instruction_in & 0b00000000000100000000000000000000) >> 9;
             let imm_h : u32 =  self.instruction_in & 0b00000000000011111111000000000000;
            println!("{:#b}",imm_e);
            println!("{:#b}",imm_f);
            println!("{:#b}",imm_g);
            println!("{:#b}",imm_h);

             self.immediates_out = ((((imm_e | imm_f | imm_g | imm_h) << 11) as i32) >> 11) as u32 ; //same here, sign-shifts the 20-bit Imm
        }

        self.immediates_out_chk = true;
    }
}

pub struct IDEXLatch<'a> {
    pub base_pc_in : u32,
    pub base_pc_out : u32,
    pub base_pc_out_chk : bool, 

    pub added_pc_in : u32,
    pub added_pc_out : u32,
    pub added_pc_out_chk : bool,

    pub r1_data_in : u32,
    pub r1_data_out : u32,
    pub r1_data_out_chk : bool,

    pub r2_data_in : u32,
    pub r2_data_out : u32,
    pub r2_data_out_chk : bool,

    pub immediates_in : u32,
    pub immediates_out : u32,
    pub immediates_out_chk : bool,

    pub rd_index_in : u8,
    pub rd_index_out : u8,
    pub rd_index_out_chk : bool,

    pub ifid_latch_ptr :  Option<&'a IFIDLatch<'a>>,
    pub reg_mem_ptr :  Option<&'a RegMem<'a>>,
    pub imm_dec_ptr :  Option<&'a ImmDecoder<'a>>,
    pub instr_dec_ptr :  Option<&'a InstrDecoder<'a>>,

    //these won't be displayed on interface!
    pub opcode_in : u8,
    pub opcode_out : u8,
    pub opcode_out_chk : bool,

    pub funct3_in : u8,
    pub funct3_out : u8,
    pub funct3_out_chk : bool,

    pub funct7_in : u8,
    pub funct7_out : u8,
    pub funct7_out_chk : bool,

    pub r1_index_in : u8,
    pub r1_index_out : u8,
    pub r1_index_out_chk : bool,

    pub r2_index_in : u8,
    pub r2_index_out : u8,
    pub r2_index_out_chk : bool,
}

impl IDEXLatch<'_> {
    pub fn grab_input (&mut self ) {
        if !(self.ifid_latch_ptr.unwrap().base_pc_out_chk && self.ifid_latch_ptr.unwrap().added_pc_out_chk) {
            panic!("ID-EX Latch tried to grab the program counts from IF-ID Latch before it was ready!");
        } else if !(self.reg_mem_ptr.unwrap().r1_data_out_chk && self.reg_mem_ptr.unwrap().r2_data_out_chk) {
            panic!("ID-EX Latch tried to grab the R1 and R2 data from the RegMem before it was ready!");
        } else if !(self.imm_dec_ptr.unwrap().immediates_out_chk) {
            panic!("ID-EX Latch tried to grab the immediates from ImmDecoder before it was ready!");
        } else if !(self.instr_dec_ptr.unwrap().rd_index_out_chk && self.instr_dec_ptr.unwrap().r1_index_out_chk && self.instr_dec_ptr.unwrap().r2_index_out_chk) {
            panic!("ID-EX Latch tried to grab the R1, R2, and RD index from InstrDecoder before it was ready!");
        } else if !(self.instr_dec_ptr.unwrap().opcode_out_chk && self.instr_dec_ptr.unwrap().funct3_out_chk && self.instr_dec_ptr.unwrap().funct7_out_chk) {
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
    pub normal_r1_in : u32, //from IDEX latch
    pub exex_r1_in : u32, //from EX-MEM latch
    pub memex_r1_in : u32, //from WB Mux

    pub r1_out : u32, 
    pub r1_out_chk : bool,

    pub idex_latch_ptr :  Option<&'a IDEXLatch<'a>>,
    pub exmem_latch_ptr :  Option<&'a EXMEMLatch<'a>>,
    pub memwb_latch_ptr :  Option<&'a MEMWBLatch<'a>>,
    pub wb_mux_ptr :  Option<&'a WBMux<'a>>,

    //not shown on GUI
    pub exex_rd_in : u8,  //from EX-MEM latch
    pub memex_rd_in : u8, //from MEM-WB latch
    pub r1_index_in : u8, //from ID-EX latch
}

impl R1ForMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().r1_data_out_chk && self.idex_latch_ptr.unwrap().r1_index_out_chk) {
            panic!("R1-Forwarding Mux tried grabbing normal R1 data, R1 index before ID-EX Latch was ready!");
        } else if !(self.exmem_latch_ptr.unwrap().rd_index_out_chk && self.exmem_latch_ptr.unwrap().result_out_chk) {
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
        if self.r1_index_in == self.exex_rd_in { //if the output of the previous instruction is the input of this one...
            self.r1_out = self.exex_r1_in;
        } else if self.r1_index_in == self.memex_rd_in {//if the output of the 2nd-previous instruction is the input of this one...
            self.r1_out = self.memex_r1_in;
        } else {// otherwise, proceed as normal.
            self.r1_out = self.normal_r1_in;
        }
        self.r1_out_chk = true;
    }
}

pub struct R2ForMux<'a> {
    pub normal_r2_in : u32, //from IDEX latch
    pub exex_r2_in : u32, //from EX-MEM latch
    pub memex_r2_in : u32, //from WB Mux

    pub r2_out : u32, 
    pub r2_out_chk : bool,

    pub idex_latch_ptr :  Option<&'a IDEXLatch<'a>>,
    pub exmem_latch_ptr :  Option<&'a EXMEMLatch<'a>>,
    pub memwb_latch_ptr :  Option<&'a MEMWBLatch<'a>>,
    pub wb_mux_ptr :  Option<&'a WBMux<'a>>,

    //not shown on GUI
    pub exex_rd_in : u8,  //from EX-MEM latch
    pub memex_rd_in : u8, //from MEM-WB latch
    pub r2_index_in : u8, //from ID-EX latch
}

impl R2ForMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().r2_data_out_chk && self.idex_latch_ptr.unwrap().r2_index_out_chk) {
            panic!("R2-Forwarding Mux tried grabbing normal R2 data, R2 index before ID-EX Latch was ready!");
        } else if !(self.exmem_latch_ptr.unwrap().rd_index_out_chk && self.exmem_latch_ptr.unwrap().result_out_chk) {
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
        if self.r2_index_in == self.exex_rd_in { //if the output of the previous instruction is the input of this one...
            self.r2_out = self.exex_r2_in;
        } else if self.r2_index_in == self.memex_rd_in {//if the output of the 2nd-previous instruction is the input of this one...
            self.r2_out = self.memex_r2_in;
        } else {// otherwise, proceed as normal.
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
    pub r1_in : u32,
    pub  pc_in : u32,
    pub opcode_in : u8,

    pub op1_out : u32,
    pub op1_out_chk : bool,

    pub idex_latch_ptr :  Option<&'a IDEXLatch<'a>>,
    pub r1for_mux_ptr :  Option<&'a R1ForMux<'a>>,
}

impl R1PCMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().opcode_out_chk && self.idex_latch_ptr.unwrap().base_pc_out_chk) {
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
        let instr_type : InstrT = isa::get_instruction_type(self.opcode_in);

        if (matches!(instr_type, InstrT::Utype) || matches!(instr_type, InstrT::Jtype) || matches!(instr_type, InstrT::Btype)) {
            self.op1_out = self.pc_in;
        } else {
            self.op1_out = self.r1_in;
        }
        self.op1_out_chk = true;
    }
}

pub struct R2ImmMux<'a> {
    pub r2_in : u32,
    pub immediates_in : u32,
    pub opcode_in : u8,

    pub op2_out : u32,
    pub op2_out_chk : bool,

    pub idex_latch_ptr :  Option<&'a IDEXLatch<'a>>,
    pub r2for_mux_ptr :  Option<&'a R2ForMux<'a>>,
}

impl R2ImmMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().opcode_out_chk && self.idex_latch_ptr.unwrap().immediates_out_chk) {
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
        let instr_type : InstrT = isa::get_instruction_type(self.opcode_in);

        if matches!(instr_type, InstrT::Rtype) {
            self.op2_out = self.r2_in;
        } else {
            self.op2_out = self.immediates_in;
        }
        self.op2_out_chk = true;
    }
}



pub struct BranchComparator<'a> {
    pub r1_in : u32,
    pub r2_in : u32,

    pub branches_out : bool,
    pub branches_out_chk : bool,

    pub r1for_mux_ptr :  &'a R1ForMux<'a>,
    pub r2for_mux_ptr :  &'a R2ForMux<'a>,
    
    //wont be displayed
    pub funct3_in : u8,
    pub idex_latch_ptr :  &'a IDEXLatch<'a>,
}

impl BranchComparator<'_> {
    pub fn grab_input(&mut self) {
        if !(self.r1for_mux_ptr.r1_out_chk) {
            panic!("BranchComparator tried getting R1 value before R1-Forward Mux was ready!");
        } else if !(self.r2for_mux_ptr.r2_out_chk) {
            panic!("BranchComparator tried getting R2 value before R2-Forward Mux was ready!");
        } else if !(self.idex_latch_ptr.funct3_out_chk) {
            panic!("BranchComparator tried getting funct3-code before ID-EX Latch was ready!");
        } else {
            self.r1_in = self.r1for_mux_ptr.r1_out;
            self.r2_in = self.r2for_mux_ptr.r2_out;

            self.funct3_in = self.idex_latch_ptr.funct3_out;
        }
    }

    pub fn compare(&mut self) { // Checks based off the 3bit funct3-code and R1 & R2, if the instruction is a Branch, whether a branch happens or not.
        self.branches_out = match self.funct3_in {
            0b000 => self.r1_in == self.r2_in, //BEQ
            0b001 => self.r1_in != self.r2_in, //BNE
            0b100 => (self.r1_in as i32) <  (self.r2_in as i32), //BLT
            0b101 => (self.r1_in as i32) >  (self.r2_in as i32), //BGE
            0b110 => self.r1_in <  self.r2_in, //BLTU
            0b111 => self.r1_in >  self.r2_in, //BGEU
            _ => false, //not a branching instruction.
        };
    }
}

pub struct ALUnit<'a> {
    pub op1_in : u32,
    pub op2_in : u32,

    pub result_out : u32,
    pub result_out_chk : bool,

    pub r1pc_mux_ptr :  Option<&'a R1PCMux<'a>>,
    pub r2imm_mux_ptr :  Option<&'a R2ImmMux<'a>>,

    //not listed on GUI!
    pub opcode_in : u8,
    pub funct3_in : u8,
    pub funct7_in : u8,
    pub  idex_latch_ptr :  Option<&'a IDEXLatch<'a>>,
}

impl ALUnit<'_> {
    pub fn grab_input(&mut self) {
        if !(self.r1pc_mux_ptr.unwrap().op1_out_chk) {
            panic!("ALU tried grabbing Op1 before R1-PC Mux was ready!");
        } else if !(self.r2imm_mux_ptr.unwrap().op2_out_chk) {
            panic!("ALU tried grabbing Op2 before R2-Imm Mux was ready!");
        } else if !(self.idex_latch_ptr.unwrap().opcode_out_chk && self.idex_latch_ptr.unwrap().funct3_out_chk && self.idex_latch_ptr.unwrap().funct7_out_chk) {
            panic!("ALU tried grabbing opcode & funct-codes before IDEX Latch was ready!");
        } else {
            self.op1_in = self.r1pc_mux_ptr.unwrap().op1_out;
            self.op2_in = self.r2imm_mux_ptr.unwrap().op2_out;

            
            
            self.opcode_in = self.idex_latch_ptr.unwrap().opcode_out;
            self.funct3_in = self.idex_latch_ptr.unwrap().funct3_out;
            self.funct7_in = self.idex_latch_ptr.unwrap().funct7_out;
        }
    }

    pub fn compute(&mut self) { //actually computes the instruction!  for signed operations, convert Ops to signed then convert result to unsigned.
        println!("{}",self.op1_in as i32);
        println!("{}",self.op2_in as i32);
        println!("{}", ((self.op1_in as i32) + (self.op2_in as i32)) as u32);
        self.result_out = match self.opcode_in {
            0b0110111 => self.op2_in, //LUI, just put in immediate as is
            0b0010111 => self.op1_in + self.op2_in,  //AUIPC, add PC and  shifted Imm, store in RD
            0b1101111 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32,  //JAL, add PC and Imm, store in RD, jump there
            0b1100111 => (((self.op2_in as i32) + (self.op1_in as i32)) as u32) & 0b11111111111111111111111111111110, //JALR, add the R1 and imm together then set lowest bit to 0.
            0b1100011 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32,//Branches. Perform signed addition between PC and Imm to figure out new PC.
            0b0000011 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32,// LB/LH/LW, add R1 and Imm offset to get source memory address.
            0b0100011 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32,// SB/SH/SW, add R1 and Imm offset to get destination memory address. R2 goes straight to Mem (thru EX-MEM latch).
            0b0010011 => match self.funct3_in { //Register-Immediate instructions
                0b000 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32,//ADDI, siggned add R1 and Imm 
                0b010 => if (self.op1_in as i32) < (self.op2_in as i32) {1} else {0}, //STLI, check if R1 < Imm
                0b011 => if self.op1_in < self.op2_in {1} else {0}, //STLIU, STLI but unsigned.
                0b100 => self.op1_in ^ self.op2_in, //XORI, bitwise exclusive-or on R1 and Imm.
                0b110 => self.op1_in | self.op2_in, //ORI, bitwise or on R1 and Imm.
                0b111 => self.op1_in & self.op2_in, //ANDI, bitwise and on R1 and Imm.
                0b001 => if self.op2_in > 31 {panic!()} else {self.op1_in << self.op2_in} // SLLI, shift R1 left by  shamt ([4-0] of Imm) bits.
                0b101 => match self.op2_in >> 5 {
                    0b0000000 => self.op1_in >> self.op2_in, //SRLI, shift R1 right logically by shamt bits
                    0b0100000 => ((self.op1_in as i32) >> ((self.op2_in - 0b010000000000) as i32)) as u32, //SRAI, shift R1 right arithmetically by shamt bits
                    _ => panic!("Invalid upper Imm. bits for Right Shift Instruction!"),
                }
                _ => panic!("funct3-code is bigger than 3 bits! this shouldnt happen!!!"),
            }
            0b0110011 => match self.funct3_in { //Register-Register instructions
                0b000 => match self.funct7_in {
                    0b0000000 => ((self.op1_in as i32) + (self.op2_in as i32)) as u32, //ADD
                    0b0100000 => ((self.op1_in as i32) - (self.op2_in as i32)) as u32, //SUB
                    _ => panic!("Invalid funct7 for ADD/SUB instruction"),
                }
                0b001 => self.op1_in << (self.op2_in & 0b11111), //SLL, shift left logical. Shift R1 left by the lowest 5 bits of R2
                0b010 => if (self.op1_in as i32) < (self.op2_in as i32) {1} else {0}, //SLT,  signed less than
                0b011 => if self.op1_in < self.op2_in {1} else {0}, //SLTU, unsigned less than
                0b100 => self.op1_in ^ self.op2_in, //XOR, bitwise exclusive or
                0b101 => match self.funct7_in {
                    0b0000000 => self.op1_in >> (self.op2_in & 0b11111), //SRL, shift right logical. Shift R1 logically right by the lowest 5 bits or R2
                    0b0100000 => ((self.op1_in as i32) >> (self.op2_in & 0b11111)) as u32, //SRA, shift right arithmetic.
                    _ => panic!("Invalid upper Imm. bits for Right Shift Instruction!"),
                }
                0b110 => self.op1_in | self.op2_in, //OR, bitwise or
                0b111 => self.op1_in & self.op2_in, //AND, bitwise and
                _ => panic!("funct3-code is bigger than 3 bits! this shouldnt happen!!!"),
            }
            _ => panic!("Invalid or Unimplemented Instruction!"),
        };
        self.result_out_chk = true;
    }
}

pub struct EXMEMLatch<'a> {
    pub added_pc_in : u32,
    pub added_pc_out : u32,
    pub added_pc_out_chk : bool,
    
    pub result_in : u32,
    pub result_out : u32,
    pub result_out_chk : bool,

    pub mem_data_in : u32,
    pub mem_data_out : u32,
    pub mem_data_out_chk : bool,
    
    pub rd_index_in : u8,
    pub rd_index_out : u8,
    pub rd_index_out_chk : bool,

    pub idex_latch_ptr :  Option<&'a IDEXLatch<'a>>,
    pub alu_ptr :  Option<&'a ALUnit<'a>>,
    pub r2for_mux_ptr :  Option<&'a R2ForMux<'a>>,

    //won't be officially shown
    pub opcode_in : u8,
    pub opcode_out : u8,
    pub opcode_out_chk : bool,

    pub funct3_in : u8,
    pub funct3_out : u8,
    pub funct3_out_chk : bool,
}

impl EXMEMLatch<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().added_pc_out_chk && self.idex_latch_ptr.unwrap().rd_index_out_chk && self.idex_latch_ptr.unwrap().opcode_out_chk & self.idex_latch_ptr.unwrap().funct3_out_chk) {
            panic!("EX-MEM Latch tried grabbing added PC, RD index, Funct3, & opcode before ID-EX Latch was ready!");
        } else if !(self.alu_ptr.unwrap().result_out_chk) {
            panic!("EX-MEM Latch tried grabbing computation result before the ALU was ready!");
        } else if !(self.r2for_mux_ptr.unwrap().r2_out_chk) {
            panic!("EX-MEM Latch tried grabbing Memory-Data-In from R2 Index ");
        } else {
            self.added_pc_in = self.idex_latch_ptr.unwrap().added_pc_out;
            self.result_in = self.alu_ptr.unwrap().result_out;
            self.mem_data_in = self.r2for_mux_ptr.unwrap().r2_out;
            self.rd_index_in = self.idex_latch_ptr.unwrap().rd_index_out;

            self.opcode_in = self.idex_latch_ptr.unwrap().opcode_out;
            self.funct3_in = self.idex_latch_ptr.unwrap().funct3_out;
        }
    }

    pub fn open_latch(&mut self) {
        self.added_pc_out = self.added_pc_in;
        self.added_pc_out_chk = true;

        self.result_out = self.result_in;
        self.result_out_chk = true;

        self.mem_data_out = self.mem_data_in;
        self.mem_data_out_chk = true;

        self.rd_index_out = self.rd_index_in;
        self.rd_index_out_chk = true;

        self.opcode_out = self.opcode_in;
        self.opcode_out_chk = true;
        self.funct3_out = self.funct3_in;
        self.funct3_out_chk = true;
    }
}


pub struct DataMem<'a> { //just a shell object for the data-memory subcomponents to interact with rest of Processor. Not implemented yet.
    pub address_in : u32,
    pub data_in : u32,

    pub mem_read_out : u32,
    pub mem_read_out_chk : bool,

    pub exmem_latch_ptr :  Option<&'a EXMEMLatch<'a>>,

    //wont be officially shown!
    pub opcode_in : u8,
    pub funct3_in : u8
}

///
/// 
/// 

pub struct MEMWBLatch<'a> {
    pub added_pc_in : u32,
    pub added_pc_out : u32,
    pub added_pc_out_chk : bool,
    
    pub result_in : u32,
    pub  result_out : u32,
    pub  result_out_chk : bool,

    pub mem_read_in : u32,
    pub   mem_read_out : u32,
    pub   mem_read_out_chk : bool,
    
    pub rd_index_in : u8,
    pub   rd_index_out : u8,
    pub   rd_index_out_chk : bool,

    pub exmem_latch_ptr :  Option<&'a EXMEMLatch<'a>>,
    pub  data_mem_ptr :  Option<&'a DataMem<'a>>,

    //won't be officially shown
    pub  opcode_in : u8,
    pub   opcode_out : u8,
    pub  opcode_out_chk : bool,
}

impl MEMWBLatch<'_> {
    pub fn grab_input(&mut self) {
        if !(self.exmem_latch_ptr.unwrap().added_pc_out_chk && self.exmem_latch_ptr.unwrap().result_out_chk && self.exmem_latch_ptr.unwrap().rd_index_out_chk && self.exmem_latch_ptr.unwrap().opcode_out_chk) {
            panic!("MEM-WB Latch tried grabbing Added PC, Computation Result, Destination Index, or Opcode before EX-MEM Latch was ready!");
        } else if !(self.data_mem_ptr.unwrap().mem_read_out_chk) {
            panic!("MEM-WB Latch tried grabbing Memory Read output before the Data Memory was ready!");
        } else {
            self.added_pc_in = self.exmem_latch_ptr.unwrap().added_pc_out;
            self.result_in = self.exmem_latch_ptr.unwrap().result_out;
            self.mem_read_in = self.data_mem_ptr.unwrap().mem_read_out;
            self.rd_index_in = self.exmem_latch_ptr.unwrap().rd_index_out;

            self.opcode_in = self.exmem_latch_ptr.unwrap().opcode_out; 
        }
    }

    pub fn open_latch(&mut self) {
        self.added_pc_out = self.added_pc_in;
        self.added_pc_out_chk = true;

        self.result_out = self.result_in;
        self.result_out_chk = true;

        self.mem_read_out = self.mem_read_in;
        self.mem_read_out_chk = true;

        self.rd_index_out = self.rd_index_in;
        self.rd_index_out_chk = true;

        self.opcode_out = self.opcode_in;
        self.opcode_out_chk = true;
    }
}

pub struct WBMux<'a> {
    pub  added_pc_in : u32,
    pub  result_in : u32,
    pub   mem_read_in : u32,  

    pub  wb_data_out : u32,
    pub   wb_data_out_chk : bool,

    pub   memwb_latch_ptr :  Option<&'a MEMWBLatch<'a>>,
    //not in GUI
    pub  opcode_in : u8,
}

impl WBMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.memwb_latch_ptr.unwrap().added_pc_out_chk && self.memwb_latch_ptr.unwrap().result_out_chk && self.memwb_latch_ptr.unwrap().mem_read_out_chk && self.memwb_latch_ptr.unwrap().opcode_out_chk) {
            panic!("WB Multiplexor tried to grab Added PC, Computation Result, Memory Read Data, and Opcode from MEM-WB Latch before it was ready!");
        } else {
            self.added_pc_in = self.memwb_latch_ptr.unwrap().added_pc_out;
            self.result_in = self.memwb_latch_ptr.unwrap().result_in;
            self.mem_read_in = self.memwb_latch_ptr.unwrap().mem_read_out;
            
            self.opcode_in = self.memwb_latch_ptr.unwrap().opcode_out;
        }
    }

    pub fn decide(&mut self) {
        if self.opcode_in == 0b11011111 || self.opcode_in == 0b1100111 { //JAL, JALR, store (pc+4) into RD
            self.wb_data_out = self.added_pc_in;
        } else if self.opcode_in == 0b0000011 { // LB, LH, LW, LBU, LHU. The load instructions all load the memory read data into RD
            self.wb_data_out = self.mem_read_in;
        } else if self.opcode_in == 0b1100011 || self.opcode_in == 0b0100011 { // Branches & Stores. These write nothing to RD at all!
            self.wb_data_out = 0xdeadbeef; //special value representing null
        } else { //all other opcodes return the ALU's result to the RD
            self.wb_data_out = self.result_in;
        }
        self.wb_data_out_chk = true;
    }
}


}