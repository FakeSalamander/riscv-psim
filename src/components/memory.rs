use crate::components::*;

pub struct EXMEMLatch<'a> {
    pub added_pc_in: u32,
    pub added_pc_out: u32,
    pub added_pc_out_chk: bool,

    pub result_in: u32,
    pub result_out: u32,
    pub result_out_chk: bool,

    pub mem_data_in: u32,
    pub mem_data_out: u32,
    pub mem_data_out_chk: bool,

    pub rd_index_in: u8,
    pub rd_index_out: u8,
    pub rd_index_out_chk: bool,

    pub idex_latch_ptr: Option<&'a IDEXLatch<'a>>,
    pub alu_ptr: Option<&'a ALUnit<'a>>,
    pub r2for_mux_ptr: Option<&'a R2ForMux<'a>>,

    //won't be officially shown
    pub opcode_in: u8,
    pub opcode_out: u8,
    pub opcode_out_chk: bool,

    pub funct3_in: u8,
    pub funct3_out: u8,
    pub funct3_out_chk: bool,
}

impl EXMEMLatch<'_> {
    pub fn grab_input(&mut self) {
        if !(self.idex_latch_ptr.unwrap().added_pc_out_chk
            && self.idex_latch_ptr.unwrap().rd_index_out_chk
            && self.idex_latch_ptr.unwrap().opcode_out_chk
                & self.idex_latch_ptr.unwrap().funct3_out_chk)
        {
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

pub struct DataMem<'a> {
    //just a shell object for the data-memory subcomponents to interact with rest of Processor. Not implemented yet.
    pub address_in: u32,
    pub data_in: u32,

    pub mem_read_out: u32,
    pub mem_read_out_chk: bool,

    pub exmem_latch_ptr: Option<&'a EXMEMLatch<'a>>,

    //wont be officially shown!
    pub opcode_in: u8,
    pub funct3_in: u8,
}

///
///
///

pub struct MEMWBLatch<'a> {
    pub added_pc_in: u32,
    pub added_pc_out: u32,
    pub added_pc_out_chk: bool,

    pub result_in: u32,
    pub result_out: u32,
    pub result_out_chk: bool,

    pub mem_read_in: u32,
    pub mem_read_out: u32,
    pub mem_read_out_chk: bool,

    pub rd_index_in: u8,
    pub rd_index_out: u8,
    pub rd_index_out_chk: bool,

    pub exmem_latch_ptr: Option<&'a EXMEMLatch<'a>>,
    pub data_mem_ptr: Option<&'a DataMem<'a>>,

    //won't be officially shown
    pub opcode_in: u8,
    pub opcode_out: u8,
    pub opcode_out_chk: bool,
}

#[cfg(test)]
mod tests {
    use crate::components::*;

    #[test]
    fn exmem_latch() {
        let idexlatch = IDEXLatch {
            base_pc_in: 0,
            base_pc_out: 32,
            base_pc_out_chk: true,

            added_pc_in: 0,
            added_pc_out: 64,
            added_pc_out_chk: true,

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
            rd_index_out: 10,
            rd_index_out_chk: true,

            ifid_latch_ptr: None,
            reg_mem_ptr: None,
            imm_dec_ptr: None,
            instr_dec_ptr: None,

            //these won't be displayed on interface!
            opcode_in: 0,
            opcode_out: 0b0110111, //the first opcode
            opcode_out_chk: true,

            funct3_in: 0,
            funct3_out: 3,
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

        let alu = ALUnit {
            op1_in: 0,
            op2_in: 0,

            result_out: 13,
            result_out_chk: true,

            r1pc_mux_ptr: None,
            r2imm_mux_ptr: None,

            //not listed on GUI!
            opcode_in: 0,
            funct3_in: 0,
            funct7_in: 0,
            idex_latch_ptr: None,
        };

        let r2formux = R2ForMux {
            normal_r2_in: 0,
            exex_r2_in: 0,
            memex_r2_in: 0,

            r2_out: 1204,
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

        let mut exmemlatch = EXMEMLatch {
            added_pc_in: 0,
            added_pc_out: 0,
            added_pc_out_chk: false,

            result_in: 0,
            result_out: 0,
            result_out_chk: false,

            mem_data_in: 0,
            mem_data_out: 0,
            mem_data_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 0,
            rd_index_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            alu_ptr: Some(&alu),
            r2for_mux_ptr: Some(&r2formux),

            //won't be officially shown
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,

            funct3_in: 0,
            funct3_out: 0,
            funct3_out_chk: false,
        };

        exmemlatch.grab_input();
        exmemlatch.open_latch();

        assert!(exmemlatch.added_pc_out_chk);
        assert!(exmemlatch.result_out_chk);
        assert!(exmemlatch.mem_data_out_chk);
        assert!(exmemlatch.rd_index_out_chk);
        assert!(exmemlatch.funct3_out_chk);
        assert!(exmemlatch.opcode_out_chk);

        assert_eq!(exmemlatch.added_pc_out, 64);
        assert_eq!(exmemlatch.result_out, 13);
        assert_eq!(exmemlatch.mem_data_out, 1204);
        assert_eq!(exmemlatch.rd_index_out, 10);
        assert_eq!(exmemlatch.funct3_out, 3);
        assert_eq!(exmemlatch.opcode_out, 0b0110111);
    }
}
