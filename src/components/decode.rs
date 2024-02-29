use crate::components::*;

pub struct PCMux<'a> {
    pub added_pc_in: u32,
    pub result_in: u32,

    pub count_out: u32,
    pub count_out_chk: bool,

    pub pc_adder_ptr: Option<&'a PCAdder<'a>>,
    pub alu_ptr: Option<&'a ALUnit<'a>>,
    // these wont be visible
    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
    pub branch_comp_ptr: Option<&'a BranchComparator<'a>>,

    pub opcode_in: u8,
    pub branches_in: bool,
}

impl PCMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.pc_adder_ptr.unwrap().count_out_chk) {
            panic!("PC Mux tried to update its added-PC before count-output from the PCAdder could be sent");
        } else if !(self.alu_ptr.unwrap().result_out_chk) {
            panic!("PC Mux tried to update its result befoe it could be sent from ALU");
        } else if !(self.idex_latch_ptr.unwrap().opcode_out_chk) {
            panic!("PC Mux tried to update its opcode before it could be sent from ID-EX Latch");
        } else if !(self.branch_comp_ptr.unwrap().branches_out_chk) {
            panic!("PC MUx tried to update the branches-decision before it could be sent from Branch Comparator");
        } else {
            self.added_pc_in = self.pc_adder_ptr.unwrap().count_out;
            self.result_in = self.alu_ptr.unwrap().result_out;

            self.opcode_in = self.idex_latch_ptr.unwrap().opcode_out;
            self.branches_in = self.branch_comp_ptr.unwrap().branches_out;
        }
    }
    pub fn decide(&mut self) {
        self.count_out = match self.opcode_in {
            0b1101111 => self.result_in,
            0b1100111 => self.result_in,
            0b1100011 => {
                if self.branches_in {
                    self.result_in
                } else {
                    self.added_pc_in
                }
            }
            _ => self.added_pc_in,
        }
    }
}

pub struct ProgramCounter<'a> {
    pub count_in: u32,

    pub count_out: u32,
    pub count_out_chk: bool,

    pub pc_mux_ptr: Option<&'a PCMux<'a>>,
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
        self.count_out = self.count_in; //doesn't do much more than store the PC.
        self.count_out_chk = true;
    }
}

pub struct PCAdder<'a> {
    pub count_in: u32,

    pub count_out: u32,
    pub count_out_chk: bool,

    pub pc_ptr: Option<&'a ProgramCounter<'a>>,
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
        self.count_out = self.count_in + 4; //simply increments the program count by 4, so it points to the next instruction.
        self.count_out_chk = true;
    }
}

pub struct InstrMem<'a> {
    pub ins_addr_in: u32,

    pub ins_array: Vec<u32>,

    pub instruction_out: u32,
    pub instruction_out_chk: bool,

    pub pc_ptr: Option<&'a ProgramCounter<'a>>,
}

impl InstrMem<'_> {
    pub fn grab_input(&mut self) {
        if self.pc_ptr.unwrap().count_out_chk {
            self.ins_addr_in = self.pc_ptr.unwrap().count_out;
        } else {
            panic!("PCAdder tried to update its count before count-output from the ProgramCounter could be sent");
        }
    }
    pub fn fetch_instruction(&mut self) {
        //fetches the 32-bit instruction the PC refers to,
        self.instruction_out = self.ins_array[(self.ins_addr_in as usize) / 4]; //the first [0] instruction is at address 0, the next [1] is at address 4, the next [2] at address 8, so on.
        self.instruction_out_chk = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::*;

    #[test]
    fn pcmux() {
        let bcomp: BranchComparator = BranchComparator {
            r1_in: 0,
            r2_in: 0,
            branches_out: true,
            branches_out_chk: true,
            r1for_mux_ptr: None,
            r2for_mux_ptr: None,
            funct3_in: 0,
            idex_latch_ptr: None,
        };

        let pcadd: PCAdder = PCAdder {
            count_in: 0,
            count_out: 36,
            count_out_chk: true,
            pc_ptr: None,
        };

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
            opcode_out: 0b0010011, //a random opcode that shouldn't cause any jumping
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

        let alu = ALUnit {
            op1_in: 0,
            op2_in: 0,

            result_out: 128, //this is where the jump will take us
            result_out_chk: true,

            r1pc_mux_ptr: None,
            r2imm_mux_ptr: None,

            //not listed on GUI!
            opcode_in: 0,
            funct3_in: 0,
            funct7_in: 0,
            idex_latch_ptr: None,
        };

        let mut pcmux = PCMux {
            added_pc_in: 0,
            result_in: 0,
            count_out: 0,
            count_out_chk: false,
            pc_adder_ptr: Some(&pcadd),
            alu_ptr: Some(&alu),
            idex_latch_ptr: Some(&idexlatch),
            branch_comp_ptr: Some(&bcomp),
            opcode_in: 0,
            branches_in: false,
        };

        pcmux.grab_input();
        assert!(pcmux.branches_in);
        assert_eq!(pcmux.added_pc_in, 36);
        assert_eq!(pcmux.result_in, 128);
        assert_eq!(pcmux.opcode_in, 0b0010011);

        //test non-jump instruction
        pcmux.decide();

        assert_eq!(pcmux.count_out, 36);

        //test YES Branch
        pcmux.opcode_in = 0b1100011; //Branch Opcode
        pcmux.decide();

        assert_eq!(pcmux.count_out, 128);

        //test NO branch
        pcmux.branches_in = false;
        pcmux.decide();

        assert_eq!(pcmux.count_out, 36);
        //test JAL/JALR
        pcmux.opcode_in = 0b1101111; //JAL Opcode
        pcmux.decide();

        assert_eq!(pcmux.count_out, 128);
        pcmux.opcode_in = 0b1100111; //JALR opcode
        pcmux.decide();

        assert_eq!(pcmux.count_out, 128);
    }

    #[test]
    fn ifid_latch() {
        let pc: ProgramCounter = ProgramCounter {
            count_in: 0,

            count_out: 32,
            count_out_chk: true,

            pc_mux_ptr: None,
        };

        let pcadd = PCAdder {
            count_in: 32,

            count_out: 36,
            count_out_chk: true,

            pc_ptr: Some(&pc),
        };

        let imem = InstrMem {
            ins_addr_in: 36,

            ins_array: Vec::<u32>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),

            instruction_out: 10,
            instruction_out_chk: true,

            pc_ptr: Some(&pc),
        };

        let mut latch = IFIDLatch {
            base_pc_in: 0,
            base_pc_out: 0,
            base_pc_out_chk: false,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            instruction_in: 0,
            instruction_out: 0,
            instruction_out_chk: false,

            pc_ptr: Some(&pc),
            pc_adder_ptr: Some(&pcadd),
            instr_mem_ptr: Some(&imem),
        };

        latch.grab_input();

        assert_eq!(latch.base_pc_out, 0);
        assert_eq!(latch.added_pc_out, 0);
        assert_eq!(latch.instruction_out, 0);

        assert!(!(latch.base_pc_out_chk));
        assert!(!(latch.added_pc_out_chk));
        assert!(!(latch.instruction_out_chk));

        latch.open_latch();

        assert_eq!(latch.base_pc_out, 32);
        assert_eq!(latch.added_pc_out, 36);
        assert_eq!(latch.instruction_out, 10);

        assert!(latch.base_pc_out_chk);
        assert!(latch.added_pc_out_chk);
        assert!(latch.instruction_out_chk);
    }

    #[test]
    fn decoder() {
        let latch = IFIDLatch {
            base_pc_in: 0,
            base_pc_out: 0,
            base_pc_out_chk: false,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            instruction_in: 0,
            instruction_out: 0b00000010000100001001000010000001,
            instruction_out_chk: true,

            pc_ptr: None,
            pc_adder_ptr: None,
            instr_mem_ptr: None,
        };

        let mut dec = InstrDecoder {
            instruction_in: 0,
            ifid_latch_ptr: Some(&latch),

            opcode_out: 0,
            opcode_out_chk: false,

            r1_index_out: 0,
            r1_index_out_chk: false,

            r2_index_out: 0,
            r2_index_out_chk: false,

            rd_index_out: 0,
            rd_index_out_chk: false,

            //won't be displayed!
            funct3_out: 0,
            funct3_out_chk: false,
            funct7_out: 0,
            funct7_out_chk: false,
        };

        dec.grab_input();

        dec.decode();

        assert_eq!(dec.opcode_out, 1);
        assert_eq!(dec.r1_index_out, 1);
        assert_eq!(dec.r2_index_out, 1);
        assert_eq!(dec.rd_index_out, 1);
        assert_eq!(dec.funct3_out, 1);
        assert_eq!(dec.funct7_out, 1);

        assert!(dec.opcode_out_chk);
        assert!(dec.r1_index_out_chk);
        assert!(dec.r2_index_out_chk);
        assert!(dec.rd_index_out_chk);
        assert!(dec.funct3_out_chk);
        assert!(dec.funct7_out_chk);

        dec.instruction_in = 0b01111110111101111011011110111111;

        dec.decode();

        assert_eq!(dec.opcode_out, 0b0111111);
        assert_eq!(dec.r1_index_out, 0b01111);
        assert_eq!(dec.r2_index_out, 0b01111);
        assert_eq!(dec.rd_index_out, 0b01111);
        assert_eq!(dec.funct3_out, 0b011);
        assert_eq!(dec.funct7_out, 0b0111111);
    }

    #[test]
    fn reg_mem() {
        let dec = InstrDecoder {
            instruction_in: 0,
            ifid_latch_ptr: None,

            opcode_out: 0,
            opcode_out_chk: false,

            r1_index_out: 1,
            r1_index_out_chk: true,

            r2_index_out: 2,
            r2_index_out_chk: true,

            rd_index_out: 0,
            rd_index_out_chk: false,

            //won't be displayed!
            funct3_out: 0,
            funct3_out_chk: false,
            funct7_out: 0,
            funct7_out_chk: false,
        };

        let latch = MEMWBLatch {
            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            result_in: 0,
            result_out: 0,
            result_out_chk: false,

            mem_read_in: 0,
            mem_read_out: 0,
            mem_read_out_chk: false,

            rd_index_in: 4,
            rd_index_out: 4,
            rd_index_out_chk: true,

            exmem_latch_ptr: None,
            data_mem_ptr: None,

            //won't be officially shown
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,
        };

        let mux = WBMux {
            added_pc_in: 0,
            result_in: 0,
            mem_read_in: 0,

            wb_data_out: 42,
            wb_data_out_chk: true,

            memwb_latch_ptr: Some(&latch),

            opcode_in: 0,
        };

        let mut regmem = RegMem {
            r1_index_in: 0,
            r2_index_in: 0,
            rd_index_in: 0,
            wb_data_in: 0,

            registers: Vec::<u32>::from([
                0, 7, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]),

            r1_data_out: 0,
            r2_data_out: 0,
            r1_data_out_chk: false,
            r2_data_out_chk: false,

            instr_dec_ptr: Some(&dec),
            memwb_latch_ptr: Some(&latch),
            wb_mux_ptr: Some(&mux),
        };

        regmem.grab_input();

        assert_eq!(regmem.r1_index_in, 1);
        assert_eq!(regmem.r2_index_in, 2);
        assert_eq!(regmem.rd_index_in, 4);
        assert_eq!(regmem.wb_data_in, 42);

        regmem.write_data();

        assert_eq!(regmem.registers[4], 42);

        regmem.fetch_registers();

        assert_eq!(regmem.r1_data_out, 7);
        assert!(regmem.r1_data_out_chk);

        assert_eq!(regmem.r2_data_out, 13);
        assert!(regmem.r2_data_out_chk);
    }

    #[test]
    fn imm_dec() {
        let latch = IFIDLatch {
            base_pc_in: 0,
            base_pc_out: 0,
            base_pc_out_chk: false,

            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            instruction_in: 0,
            instruction_out: 0b00000011011100001001000010000001, //for i-type,the immediate is 55
            instruction_out_chk: true,

            pc_ptr: None,
            pc_adder_ptr: None,
            instr_mem_ptr: None,
        };

        let instr_dec = InstrDecoder {
            instruction_in: 0,
            ifid_latch_ptr: None,

            opcode_out: 0b0010011,
            opcode_out_chk: true,

            r1_index_out: 0,
            r1_index_out_chk: true,

            r2_index_out: 0,
            r2_index_out_chk: true,

            rd_index_out: 0,
            rd_index_out_chk: false,

            //won't be displayed!
            funct3_out: 0,
            funct3_out_chk: false,
            funct7_out: 0,
            funct7_out_chk: false,
        };

        let mut immdec = ImmDecoder {
            opcode_in: 0,
            instr_dec_ptr: Some(&instr_dec),

            instruction_in: 0,
            ifid_latch_ptr: Some(&latch),

            immediates_out: 0,
            immediates_out_chk: false,
        };

        immdec.grab_input();

        assert_eq!(immdec.instruction_in, 0b00000011011100001001000010000001);
        assert_eq!(immdec.opcode_in, 0b0010011);

        immdec.decode();

        assert_eq!(immdec.immediates_out, 55);
        assert!(immdec.immediates_out_chk);

        immdec.instruction_in = 0b00000110000100001001001110000001; //immediates should be 0b1100111
        immdec.opcode_in = 0b0100011; //an S-type opcode.
        immdec.decode();

        assert_eq!(immdec.immediates_out, 0b1100111); //S-type test.

        immdec.instruction_in = 0b00000110000100001001001110000001; //immediates should be 0b0100001100110
        immdec.opcode_in = 0b1100011; //a B-type opcode.
        immdec.decode();

        assert_eq!(immdec.immediates_out, 0b0100001100110);

        immdec.instruction_in = 0b00000110000100001001001110000001; //immediates should be 0b00000110000100001001000000000000
        immdec.opcode_in = 0b0110111; //a U-type opcode
        immdec.decode();

        assert_eq!(immdec.immediates_out, 0b00000110000100001001000000000000);

        immdec.instruction_in = 0b10000110000100001001001110000001; //immediates should be 0b10000100110000110000, sign extended.
        immdec.opcode_in = 0b1101111; //the only J-type opcode.
        immdec.decode();

        assert_eq!(immdec.immediates_out, 0b11111111111100001001100001100000);
    }
}
