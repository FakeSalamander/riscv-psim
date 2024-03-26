use crate::components::*;

impl MEMWBLatch<'_> {
    pub fn grab_input(&mut self) {
        if !(self.exmem_latch_ptr.unwrap().added_pc_out_chk
            && self.exmem_latch_ptr.unwrap().result_out_chk
            && self.exmem_latch_ptr.unwrap().rd_index_out_chk
            && self.exmem_latch_ptr.unwrap().opcode_out_chk)
        {
            panic!("MEM-WB Latch tried grabbing Added PC, Computation Result, Destination Index, or Opcode before EX-MEM Latch was ready!");
        } else if !(self.data_mem_ptr.unwrap().mem_read_out_chk) {
            panic!(
                "MEM-WB Latch tried grabbing Memory Read output before the Data Memory was ready!"
            );
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
    pub added_pc_in: u32,
    pub result_in: u32,
    pub mem_read_in: u32,

    pub wb_data_out: u32,
    pub wb_data_out_chk: bool,

    pub memwb_latch_ptr: Option<&'a MEMWBLatch<'a>>,
    //not in GUI
    pub opcode_in: u8,
}

impl WBMux<'_> {
    pub fn grab_input(&mut self) {
        if !(self.memwb_latch_ptr.unwrap().added_pc_out_chk
            && self.memwb_latch_ptr.unwrap().result_out_chk
            && self.memwb_latch_ptr.unwrap().mem_read_out_chk
            && self.memwb_latch_ptr.unwrap().opcode_out_chk)
        {
            panic!("WB Multiplexor tried to grab Added PC, Computation Result, Memory Read Data, and Opcode from MEM-WB Latch before it was ready!");
        } else {
            self.added_pc_in = self.memwb_latch_ptr.unwrap().added_pc_out;
            self.result_in = self.memwb_latch_ptr.unwrap().result_in;
            self.mem_read_in = self.memwb_latch_ptr.unwrap().mem_read_out;

            self.opcode_in = self.memwb_latch_ptr.unwrap().opcode_out;
        }
    }

    pub fn decide(&mut self) {
        if self.opcode_in == 0b11011111 || self.opcode_in == 0b1100111 {
            //JAL, JALR, store (pc+4) into RD
            self.wb_data_out = self.added_pc_in;
        } else if self.opcode_in == 0b0000011 {
            // LB, LH, LW, LBU, LHU. The load instructions all load the memory read data into RD
            self.wb_data_out = self.mem_read_in;
        } else if self.opcode_in == 0b1100011 || self.opcode_in == 0b0100011 {
            // Branches & Stores. These write nothing to RD at all!
            self.wb_data_out = 0xdeadbeef; //special value representing null
        } else {
            //all other opcodes return the ALU's result to the RD
            self.wb_data_out = self.result_in;
        }
        self.wb_data_out_chk = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::*;

    #[test]
    fn memwb_latch() {
        let exmemlatch = EXMEMLatch {
            added_pc_in: 0,
            added_pc_out: 64,
            added_pc_out_chk: true,

            result_in: 0,
            result_out: 13,
            result_out_chk: true,

            mem_data_in: 0,
            mem_data_out: 0,
            mem_data_out_chk: false,

            rd_index_in: 0,
            rd_index_out: 10,
            rd_index_out_chk: true,

            idex_latch_ptr: None,
            alu_ptr: None,
            r2for_mux_ptr: None,

            //won't be officially shown
            opcode_in: 0,
            opcode_out: 0b10101,
            opcode_out_chk: true,

            funct3_in: 0,
            funct3_out: 3,
            funct3_out_chk: true,
        };

        let datamem = DataMem {
            //just a shell object for the data-memory subcomponents to interact with rest of Processor. Not implemented yet.
            address_in: 0,
            data_in: 0,

            mem_read_out: 1204,
            mem_read_out_chk: true,

            exmem_latch_ptr: None,

            //wont be officially shown!
            opcode_in: 0,
            funct3_in: 0,
        };

        let mut memwblatch = MEMWBLatch {
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
            rd_index_out: 0,
            rd_index_out_chk: false,

            exmem_latch_ptr: Some(&exmemlatch),
            data_mem_ptr: Some(&datamem),

            //won't be officially shown
            opcode_in: 0,
            opcode_out: 0,
            opcode_out_chk: false,
        };

        memwblatch.grab_input();
        memwblatch.open_latch();

        assert_eq!(memwblatch.opcode_out, 0b10101);
        assert_eq!(memwblatch.rd_index_out, 10);
        assert_eq!(memwblatch.result_out, 13);
        assert_eq!(memwblatch.mem_read_out, 1204);
        assert_eq!(memwblatch.added_pc_out, 64);
    }
}
