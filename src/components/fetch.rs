use crate::components::*;
use crate::isa::isa;
use crate::isa::isa::InstrT;

pub struct IFIDLatch<'a> {
    pub base_pc_in: u32,
    pub base_pc_out: u32,
    pub base_pc_out_chk: bool,

    pub added_pc_in: u32,
    pub added_pc_out: u32,
    pub added_pc_out_chk: bool,

    pub instruction_in: u32,
    pub instruction_out: u32,
    pub instruction_out_chk: bool,

    pub pc_ptr: Option<&'a ProgramCounter<'a>>,
    pub pc_adder_ptr: Option<&'a PCAdder<'a>>,
    pub instr_mem_ptr: Option<&'a InstrMem<'a>>,
}

impl IFIDLatch<'_> {
    pub fn grab_input(&mut self) {
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
    pub fn open_latch(&mut self) {
        //opens the latch to let new values pass through.
        self.base_pc_out = self.base_pc_in;
        self.added_pc_out = self.added_pc_in;
        self.instruction_out = self.instruction_in;

        self.base_pc_out_chk = true;
        self.added_pc_out_chk = true;
        self.instruction_out_chk = true;
    }
}

pub struct InstrDecoder<'a> {
    pub instruction_in: u32,
    pub ifid_latch_ptr: Option<&'a IFIDLatch<'a>>,

    pub opcode_out: u8, //7-bit opcode
    pub opcode_out_chk: bool,

    pub r1_index_out: u8, //5-bit register index
    pub r1_index_out_chk: bool,

    pub r2_index_out: u8,
    pub r2_index_out_chk: bool,

    pub rd_index_out: u8,
    pub rd_index_out_chk: bool,

    //won't be displayed!
    pub funct3_out: u8,
    pub funct3_out_chk: bool,
    pub funct7_out: u8,
    pub funct7_out_chk: bool,
}

impl InstrDecoder<'_> {
    pub fn grab_input(&mut self) {
        if self.ifid_latch_ptr.unwrap().instruction_out_chk {
            self.instruction_in = self.ifid_latch_ptr.unwrap().instruction_out;
        } else {
            panic!(
                "InstrDecoder tried to update its instruction before IF-ID Latch could send it."
            );
        }
    }
    pub fn decode(&mut self) {
        //gets the opcode, r1 index, r2 index, and destination register index out of the instruction, even if they end up being unused.

        //uses bit-wise AND operation on a mask in order to get the desired bits, dividing to rem

        // need to get lowest 7 bits out, just use a mask to get (6-0)
        self.opcode_out = (self.instruction_in & 0b1111111) as u8;
        self.opcode_out_chk = true;

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
    pub r1_index_in: u8,
    pub r2_index_in: u8,

    pub rd_index_in: u8,

    pub wb_data_in: u32, //this is the writeback data from 3 steps ago, getting commited to the register

    pub registers: Vec<u32>, //the value of each corresponds to what that register name points to. 0 always points to 0 constant, and is unused.

    //real_registers : Vec //each of these correspodns to a real physical register! NOT IMPLEMENTED YET
    pub r1_data_out: u32,
    pub r1_data_out_chk: bool,
    pub r2_data_out: u32,
    pub r2_data_out_chk: bool,

    pub instr_dec_ptr: Option<&'a InstrDecoder<'a>>,
    pub memwb_latch_ptr: Option<&'a MEMWBLatch<'a>>,
    pub wb_mux_ptr: Option<&'a WBMux<'a>>,
}

impl RegMem<'_> {
    pub fn grab_input(&mut self) {
        if !(self.instr_dec_ptr.unwrap().r1_index_out_chk
            && self.instr_dec_ptr.unwrap().r2_index_out_chk)
        {
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
    pub opcode_in: u8,
    pub instr_dec_ptr: Option<&'a InstrDecoder<'a>>,

    pub instruction_in: u32,
    pub ifid_latch_ptr: Option<&'a IFIDLatch<'a>>,

    pub immediates_out: u32,
    pub immediates_out_chk: bool,
}

impl ImmDecoder<'_> {
    pub fn grab_input(&mut self) {
        if !(self.instr_dec_ptr.unwrap().opcode_out_chk) {
            panic!("ImmDecoder tried to grab opcode before InstrDecoder was ready!");
        } else if !(self.ifid_latch_ptr.unwrap().instruction_out_chk) {
            panic!("ImmDecoder tried to grab instruction before IF-ID Latch was ready!");
        } else {
            self.opcode_in = self.instr_dec_ptr.unwrap().opcode_out;
            self.instruction_in = self.ifid_latch_ptr.unwrap().instruction_out;
        }
    }

    pub fn decode(&mut self) {
        //rearranges the immediates of an instruction by type, so they're where the ALU expects them.
        let instr_type: InstrT = isa::get_instruction_type(self.opcode_in);
        if matches!(instr_type, InstrT::Rtype) {
            self.immediates_out = 0xdeadbeef; //Outputs a useless value. R-Type has no immediates.
        } else if matches!(instr_type, InstrT::Itype) {
            //in this one, simply take the 31st thru 12th bits! they're already where they want to be.
            self.immediates_out = ((self.instruction_in as i32) >> 20) as u32;
        } else if matches!(instr_type, InstrT::Stype) {
            //(31-25) goes to [11-5],  (11-7) goes to [4-0]. do each separately, then bitwise OR

            //                       the (31-25) is converted to signed so that it does an arithmetic right shift
            self.immediates_out = ((((self.instruction_in & 0b11111110000000000000000000000000)
                as i32)
                >> 20) as u32)
                | ((self.instruction_in & 0b111110000000) >> 7);
        } else if matches!(instr_type, InstrT::Btype) {
            //A (31) to [12] ,B (30-25) to [10-5], C (11-8) to [4-1], D (7) to [11]
            let imm_a: u32 = (self.instruction_in & 0b10000000000000000000000000000000) >> 19;
            let imm_b: u32 = (self.instruction_in & 0b01111110000000000000000000000000) >> 20;
            let imm_c: u32 = (self.instruction_in & 0b00000000000000000000111100000000) >> 7;
            let imm_d: u32 = (self.instruction_in & 0b00000000000000000000000010000000) << 4;
            //println!("{:#b}",imm_a);
            //println!("{:#b}",imm_b);
            //println!("{:#b}",imm_c);
            //println!("{:#b}",imm_d);

            self.immediates_out = ((((imm_a | imm_b | imm_c | imm_d) << 19) as i32) >> 19) as u32;
        //the wonky shifting just sign-extends the 12-bit Imm preemptively
        } else if matches!(instr_type, InstrT::Utype) {
            //(31-12) goes to [31-12]... so just mask the rest!
            self.immediates_out = self.instruction_in & 0b11111111111111111111000000000000;
        } else {
            //only J-type left!  E  (31) to [20], F  (30-21) to [10-1], G  (20) to [11],  H  (19-12) to [19-12]
            let imm_e: u32 = (self.instruction_in & 0b10000000000000000000000000000000) >> 11;
            let imm_f: u32 = (self.instruction_in & 0b01111111111000000000000000000000) >> 20;
            let imm_g: u32 = (self.instruction_in & 0b00000000000100000000000000000000) >> 9;
            let imm_h: u32 = self.instruction_in & 0b00000000000011111111000000000000;
            println!("{:#b}", imm_e);
            println!("{:#b}", imm_f);
            println!("{:#b}", imm_g);
            println!("{:#b}", imm_h);

            self.immediates_out = ((((imm_e | imm_f | imm_g | imm_h) << 11) as i32) >> 11) as u32;
            //same here, sign-shifts the 20-bit Imm
        }

        self.immediates_out_chk = true;
    }
}

//
#[cfg(test)]
mod tests {
    use crate::components::*;

    #[test]
    fn program_counter() {
        let pcmux: PCMux = PCMux {
            added_pc_in: 0,
            result_in: 0,

            count_out: 64,
            count_out_chk: true,

            alu_ptr: None,
            pc_adder_ptr: None,

            idex_latch_ptr: None,
            branch_comp_ptr: None,

            opcode_in: 0,
            branches_in: false,
        };

        let mut pc: ProgramCounter = ProgramCounter {
            count_in: 0,

            count_out: 32,
            count_out_chk: false,

            pc_mux_ptr: Some(&pcmux),
        };

        pc.grab_input();

        assert_eq!(pc.count_in, 64);
        assert_eq!(pc.count_out, 32);
        assert!(!(pc.count_out_chk));

        pc.update_count();

        assert_eq!(pc.count_out, 64);
        assert!(pc.count_out_chk);
    }

    #[test]
    fn pc_adder() {
        let pc: ProgramCounter = ProgramCounter {
            count_in: 0,

            count_out: 32,
            count_out_chk: true,

            pc_mux_ptr: None,
        };

        let mut pcadd = PCAdder {
            count_in: 0,

            count_out: 0,
            count_out_chk: false,

            pc_ptr: Some(&pc),
        };

        pcadd.grab_input();

        assert_eq!(pcadd.count_in, 32);
        assert_eq!(pcadd.count_out, 0);
        assert!(!(pcadd.count_out_chk));

        pcadd.add_count();

        assert_eq!(pcadd.count_out, 36);
        assert!(pcadd.count_out_chk);
    }

    #[test]
    fn instr_mem() {
        let pc: ProgramCounter = ProgramCounter {
            count_in: 0,

            count_out: 32,
            count_out_chk: true,

            pc_mux_ptr: None,
        };

        let mut imem = InstrMem {
            ins_addr_in: 0,

            ins_array: Vec::<u32>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),

            instruction_out: 100,
            instruction_out_chk: false,

            pc_ptr: Some(&pc),
        };

        imem.grab_input();

        assert_eq!(imem.ins_addr_in, 32);
        assert_eq!(imem.instruction_out, 100);
        assert_eq!(imem.instruction_out_chk, false);

        imem.fetch_instruction(); //should get the instruction at the [8] slice, so 9.

        assert_eq!(imem.instruction_out, 9);
        assert!(imem.instruction_out_chk);
    }
}
