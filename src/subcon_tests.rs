 
#[cfg(test)]
mod tests {
    use crate::subcomponents::subc;

    #[test]
    fn program_counter() {
        let pcmux : subc::PCMux = subc::PCMux {
            added_pc_in : 0,
            result_in: 0,

            count_out: 64,
            count_out_chk: true,

            alu_ptr : None,
            pc_adder_ptr : None,
        };

        let mut pc : subc::ProgramCounter = subc::ProgramCounter {
            count_in : 0,

            count_out : 32,
            count_out_chk : false,

            pc_mux_ptr : Some(&pcmux),
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
        let pc : subc::ProgramCounter = subc::ProgramCounter {
            count_in : 0,

            count_out : 32,
            count_out_chk : true,

            pc_mux_ptr : None,
        };

        let mut pcadd = subc::PCAdder {
            count_in : 0,

            count_out : 0,
            count_out_chk : false,

            pc_ptr : Some(&pc),
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
        let pc : subc::ProgramCounter = subc::ProgramCounter {
            count_in : 0,

            count_out : 32,
            count_out_chk : true,

            pc_mux_ptr : None,
        };

        let mut imem = subc::InstrMem {
            ins_addr_in : 0,
            
            ins_array : Vec::<u32>::from([1,2,3,4,5,6,7,8,9,10]),

            instruction_out : 100,
            instruction_out_chk : false,

            pc_ptr : Some(&pc),
        };

        imem.grab_input();

        assert_eq!(imem.ins_addr_in, 32);
        assert_eq!(imem.instruction_out, 100);
        assert_eq!(imem.instruction_out_chk, false);

        imem.fetch_instruction(); //should get the instruction at the [8] slice, so 9.

        assert_eq!(imem.instruction_out, 9);
        assert!(imem.instruction_out_chk);
    }

    #[test]
    fn ifid_latch() {
        let pc : subc::ProgramCounter = subc::ProgramCounter {
            count_in : 0,

            count_out : 32,
            count_out_chk : true,

            pc_mux_ptr : None,
        };

        let pcadd = subc::PCAdder {
            count_in : 32,

            count_out : 36,
            count_out_chk : true,

            pc_ptr : Some(&pc),
        };

        let imem = subc::InstrMem {
            ins_addr_in : 36,
            
            ins_array : Vec::<u32>::from([1,2,3,4,5,6,7,8,9,10]),

            instruction_out : 10,
            instruction_out_chk : true,

            pc_ptr : Some(&pc),
        };

        let mut latch = subc::IFIDLatch {
            base_pc_in : 0,
            base_pc_out : 0,
            base_pc_out_chk : false, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            instruction_in : 0, 
            instruction_out : 0,
            instruction_out_chk : false,
        
            pc_ptr :  Some(&pc),
            pc_adder_ptr :  Some(&pcadd),
            instr_mem_ptr :  Some(&imem),
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
        let latch = subc::IFIDLatch {
            base_pc_in : 0,
            base_pc_out : 0,
            base_pc_out_chk : false, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            instruction_in : 0, 
            instruction_out : 0b00000010000100001001000010000001,
            instruction_out_chk : true,
        
            pc_ptr :  None,
            pc_adder_ptr :  None,
            instr_mem_ptr :  None,
        };

        let mut dec = subc::InstrDecoder {
            instruction_in : 0,
            ifid_latch_ptr : Some(&latch),

            opcode_out : 0,
            opcode_out_chk : false,
            
            r1_index_out : 0,
            r1_index_out_chk : false,
        
            r2_index_out : 0,
            r2_index_out_chk : false,
        
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            //won't be displayed!
            funct3_out : 0,
            funct3_out_chk : false,
            funct7_out : 0,
            funct7_out_chk : false,
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
        let dec = subc::InstrDecoder {
            instruction_in : 0,
            ifid_latch_ptr : None,

            opcode_out : 0,
            opcode_out_chk : false,
            
            r1_index_out : 1,
            r1_index_out_chk : true,
        
            r2_index_out : 2,
            r2_index_out_chk : true,
        
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            //won't be displayed!
            funct3_out : 0,
            funct3_out_chk : false,
            funct7_out : 0,
            funct7_out_chk : false,
        };
        
        let latch = subc::MEMWBLatch {
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
            
            result_in : 0,
             result_out : 0,
             result_out_chk : false,
        
            mem_read_in : 0,
              mem_read_out : 0,
              mem_read_out_chk : false,
            
            rd_index_in : 4,
              rd_index_out : 4,
              rd_index_out_chk : true,
        
            exmem_latch_ptr :  None,
             data_mem_ptr :  None,
        
            //won't be officially shown
             opcode_in : 0,
             opcode_out : 0,
             opcode_out_chk : false,
        };

        let mux = subc::WBMux {
            added_pc_in : 0,
            result_in : 0,
            mem_read_in : 0,

            wb_data_out : 42,
            wb_data_out_chk : true,

            memwb_latch_ptr : Some(&latch),
            
            opcode_in : 0,
        };

        let mut regmem = subc::RegMem {
            r1_index_in : 0,
            r2_index_in : 0,
            rd_index_in : 0,
            wb_data_in : 0,

            registers : Vec::<u32>::from([0,7,13,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),

            r1_data_out : 0,
            r2_data_out : 0,
            r1_data_out_chk : false,
            r2_data_out_chk : false,

            instr_dec_ptr : Some(&dec),
            memwb_latch_ptr : Some(&latch),
            wb_mux_ptr : Some(&mux),
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
        let latch = subc::IFIDLatch {
            base_pc_in : 0,
            base_pc_out : 0,
            base_pc_out_chk : false, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            instruction_in : 0, 
            instruction_out : 0b00000011011100001001000010000001, //for i-type,the immediate is 55
            instruction_out_chk : true,
        
            pc_ptr :  None,
            pc_adder_ptr :  None,
            instr_mem_ptr :  None,
        };

        let instr_dec = subc::InstrDecoder {
            instruction_in : 0,
            ifid_latch_ptr : None,

            opcode_out : 0b0010011,
            opcode_out_chk : true,
            
            r1_index_out : 0,
            r1_index_out_chk : true,
        
            r2_index_out : 0,
            r2_index_out_chk : true,
        
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            //won't be displayed!
            funct3_out : 0,
            funct3_out_chk : false,
            funct7_out : 0,
            funct7_out_chk : false,
        };

        let mut immdec = subc::ImmDecoder {
            opcode_in : 0,
            instr_dec_ptr : Some(&instr_dec),

            instruction_in : 0,
            ifid_latch_ptr : Some(&latch),

            immediates_out : 0,
            immediates_out_chk : false,
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

    #[test]
    fn idex_latch() {
        let ifidlatch = subc::IFIDLatch {
            base_pc_in : 0,
            base_pc_out : 32,
            base_pc_out_chk : true, 
        
            added_pc_in : 0,
            added_pc_out : 36,
            added_pc_out_chk : true,
        
            instruction_in : 0, 
            instruction_out : 0b00000010000100001001000010000001,
            instruction_out_chk : true,
        
            pc_ptr :  None,
            pc_adder_ptr :  None,
            instr_mem_ptr :  None,
        };

        let instrdec = subc::InstrDecoder {
            instruction_in : 0,
            ifid_latch_ptr : None,

            opcode_out : 3,
            opcode_out_chk : true,
            
            r1_index_out : 6,
            r1_index_out_chk : true,
        
            r2_index_out : 7,
            r2_index_out_chk : true,
        
            rd_index_out : 8,
            rd_index_out_chk : true,
        
            //won't be displayed!
            funct3_out : 4,
            funct3_out_chk : true,
            funct7_out : 5,
            funct7_out_chk : true,
        };

        let regmem = subc::RegMem {
            r1_index_in : 0,
            r2_index_in : 0,
            rd_index_in : 0,
            wb_data_in : 0,

            registers : Vec::<u32>::from([0,7,13,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),

            r1_data_out : 42,
            r2_data_out : 420,
            r1_data_out_chk : true,
            r2_data_out_chk : true,

            instr_dec_ptr : Some(&instrdec),
            memwb_latch_ptr : None,
            wb_mux_ptr : None,
        };

        let immdec = subc::ImmDecoder {
            opcode_in : 0,
            instr_dec_ptr : Some(&instrdec),

            instruction_in : 0,
            ifid_latch_ptr : Some(&ifidlatch),

            immediates_out : 4200,
            immediates_out_chk : true,
        };

        let mut idexlatch = subc::IDEXLatch {
            base_pc_in : 0,
            base_pc_out : 0,
            base_pc_out_chk : false, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            r1_data_in : 0,
            r1_data_out : 0,
            r1_data_out_chk : false,
        
            r2_data_in : 0,
            r2_data_out : 0,
            r2_data_out_chk : false,
        
            immediates_in : 0,
            immediates_out : 0,
            immediates_out_chk : false,
        
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            ifid_latch_ptr :  Some(&ifidlatch),
            reg_mem_ptr :  Some(&regmem),
            imm_dec_ptr :  Some(&immdec),
            instr_dec_ptr :  Some(&instrdec),
        
            //these won't be displayed on interface!
            opcode_in : 0,
            opcode_out : 0,
            opcode_out_chk : false,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : false,
        
            funct7_in : 0,
            funct7_out : 0,
            funct7_out_chk : false,
        
            r1_index_in : 0,
            r1_index_out : 0,
            r1_index_out_chk : false,
        
            r2_index_in : 0,
            r2_index_out : 0,
            r2_index_out_chk : false,
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
        let idexlatch = subc::IDEXLatch {
            base_pc_in : 0,
            base_pc_out : 0,
            base_pc_out_chk : false, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            r1_data_in : 0,
            r1_data_out : 1,
            r1_data_out_chk : true,
        
            r2_data_in : 0,
            r2_data_out : 4,
            r2_data_out_chk : true,
        
            immediates_in : 0,
            immediates_out : 0,
            immediates_out_chk : false,
        
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            ifid_latch_ptr :  None,
            reg_mem_ptr :  None,
            imm_dec_ptr :  None,
            instr_dec_ptr :  None,
        
            //these won't be displayed on interface!
            opcode_in : 0,
            opcode_out : 0,
            opcode_out_chk : false,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : false,
        
            funct7_in : 0,
            funct7_out : 0,
            funct7_out_chk : false,
        
            r1_index_in : 0,
            r1_index_out : 10,
            r1_index_out_chk : true,
        
            r2_index_in : 0,
            r2_index_out : 11,
            r2_index_out_chk : true,
        };

        let exmemlatch = subc::EXMEMLatch {
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
            
            result_in : 0,
            result_out : 2,
            result_out_chk : true,
        
            mem_data_in : 0,
            mem_data_out : 0,
            mem_data_out_chk : false,
            
            rd_index_in : 0,
            rd_index_out : 14,
            rd_index_out_chk : true,
        
            idex_latch_ptr :  None,
            alu_ptr :  None,
            r2for_mux_ptr :  None,
        
            //won't be officially shown
            opcode_in : 0,
            opcode_out : 0,
            opcode_out_chk : false,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : false,
        };

        let memwblatch = subc::MEMWBLatch {
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
            
            result_in : 0,
             result_out : 0,
             result_out_chk : false,
        
            mem_read_in : 0,
              mem_read_out : 0,
              mem_read_out_chk : false,
            
            rd_index_in : 0,
              rd_index_out : 15,
              rd_index_out_chk : true,
        
            exmem_latch_ptr :  None,
             data_mem_ptr :  None,
        
            //won't be officially shown
             opcode_in : 0,
              opcode_out : 0,
             opcode_out_chk : false,
        };

        let wbmux = subc::WBMux {
            added_pc_in : 0,
            result_in : 0,
            mem_read_in : 0,  
        
            wb_data_out : 3,
            wb_data_out_chk : true,
        
            memwb_latch_ptr :  None,
            //not in GUI
            opcode_in : 0,
        };

        let mut r1formux = subc::R1ForMux {
            normal_r1_in : 0, //from IDEX latch, 1
            exex_r1_in : 0, //from EX-MEM latch, 2
            memex_r1_in : 0, //from WB Mux, 3
        
            r1_out : 0, 
            r1_out_chk : false,
        
            idex_latch_ptr :  Some(&idexlatch),
            exmem_latch_ptr :  Some(&exmemlatch),
            memwb_latch_ptr :  Some(&memwblatch),
            wb_mux_ptr :  Some(&wbmux),
        
            //not shown on GUI
            exex_rd_in : 0,  //from EX-MEM latch, vary per test
            memex_rd_in : 0, //from MEM-WB latch, vary per test
            r1_index_in : 0, //from ID-EX latch, 10
        };

        let mut r2formux = subc::R2ForMux {
            normal_r2_in : 0, //from IDEX latch, 4
            exex_r2_in : 0, //from EX-MEM latch, 2
            memex_r2_in : 0, //from WB Mux, 3
        
            r2_out : 0, 
            r2_out_chk : false,
        
            idex_latch_ptr :  Some(&idexlatch),
            exmem_latch_ptr :  Some(&exmemlatch),
            memwb_latch_ptr :  Some(&memwblatch),
            wb_mux_ptr :  Some(&wbmux),
        
            //not shown on GUI
            exex_rd_in : 0,  //from EX-MEM latch, vary per test
            memex_rd_in : 0, //from MEM-WB latch, vary per test
            r2_index_in : 0, //from ID-EX latch, 11
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
        let idexlatch = subc::IDEXLatch {
            base_pc_in : 0,
            base_pc_out : 32,
            base_pc_out_chk : true, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            r1_data_in : 0,
            r1_data_out : 1,
            r1_data_out_chk : false,
        
            r2_data_in : 0,
            r2_data_out : 4,
            r2_data_out_chk : false,
        
            immediates_in : 0,
            immediates_out : 0,
            immediates_out_chk : false,
        
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            ifid_latch_ptr :  None,
            reg_mem_ptr :  None,
            imm_dec_ptr :  None,
            instr_dec_ptr :  None,
        
            //these won't be displayed on interface!
            opcode_in : 0,
            opcode_out : 0b0110111, //an opcode that wants the PC, not R1.
            opcode_out_chk : true,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : false,
        
            funct7_in : 0,
            funct7_out : 0,
            funct7_out_chk : false,
        
            r1_index_in : 0,
            r1_index_out : 10,
            r1_index_out_chk : true,
        
            r2_index_in : 0,
            r2_index_out : 11,
            r2_index_out_chk : true,
        };

        let r1formux = subc::R1ForMux {
            normal_r1_in : 0, //from IDEX latch, 
            exex_r1_in : 0, //from EX-MEM latch, 
            memex_r1_in : 0, //from WB Mux, 
        
            r1_out : 1, 
            r1_out_chk : true,
        
            idex_latch_ptr :  None,
            exmem_latch_ptr :  None,
            memwb_latch_ptr :  None,
            wb_mux_ptr :  None,
        
            //not shown on GUI
            exex_rd_in : 0,  //from EX-MEM latch, 
            memex_rd_in : 0, //from MEM-WB latch, 
            r1_index_in : 0, //from ID-EX latch,
        };

        let mut r1pcmux = subc::R1PCMux {
            r1_in : 0, //will be 1
            pc_in : 0, //wiill be 32
            opcode_in : 0,
        
            op1_out : 0,
            op1_out_chk : false,
        
            idex_latch_ptr :  Some(&idexlatch),
            r1for_mux_ptr :  Some(&r1formux),
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
        let idexlatch = subc::IDEXLatch {
            base_pc_in : 0,
            base_pc_out : 32,
            base_pc_out_chk : true, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            r1_data_in : 0,
            r1_data_out : 1,
            r1_data_out_chk : false,
        
            r2_data_in : 0,
            r2_data_out : 4,
            r2_data_out_chk : false,
        
            immediates_in : 2,
            immediates_out : 2,
            immediates_out_chk : true,
        
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            ifid_latch_ptr :  None,
            reg_mem_ptr :  None,
            imm_dec_ptr :  None,
            instr_dec_ptr :  None,
        
            //these won't be displayed on interface!
            opcode_in : 0,
            opcode_out : 0b0010011, //an I-type opcode that wants the Immediates, not the R2
            opcode_out_chk : true,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : false,
        
            funct7_in : 0,
            funct7_out : 0,
            funct7_out_chk : false,
        
            r1_index_in : 0,
            r1_index_out : 10,
            r1_index_out_chk : true,
        
            r2_index_in : 0,
            r2_index_out : 11,
            r2_index_out_chk : true,
        };

        let r2formux = subc::R2ForMux {
            normal_r2_in : 0, 
            exex_r2_in : 0, 
            memex_r2_in : 0, 
        
            r2_out : 1, 
            r2_out_chk : true,
        
            idex_latch_ptr :  None,
            exmem_latch_ptr :  None,
            memwb_latch_ptr :  None,
            wb_mux_ptr :  None,
        
            //not shown on GUI
            exex_rd_in : 0,  //from EX-MEM latch, vary per test
            memex_rd_in : 0, //from MEM-WB latch, vary per test
            r2_index_in : 0, //from ID-EX latch
        };

        let mut r2immmux = subc::R2ImmMux {
            r2_in : 0, //will be 1
            immediates_in : 0, //will be 2
            opcode_in : 0,
        
            op2_out : 0,
            op2_out_chk : false,
        
            idex_latch_ptr :  Some(&idexlatch),
            r2for_mux_ptr :  Some(&r2formux),
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
    fn alu() {
        let idexlatch = subc::IDEXLatch {
            base_pc_in : 0,
            base_pc_out : 32,
            base_pc_out_chk : true, 
        
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
        
            r1_data_in : 0,
            r1_data_out : 1,
            r1_data_out_chk : false,
        
            r2_data_in : 0,
            r2_data_out : 4,
            r2_data_out_chk : false,
        
            immediates_in : 2,
            immediates_out : 2,
            immediates_out_chk : true,
        
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            ifid_latch_ptr :  None,
            reg_mem_ptr :  None,
            imm_dec_ptr :  None,
            instr_dec_ptr :  None,
        
            //these won't be displayed on interface!
            opcode_in : 0,
            opcode_out : 0b0110111, //the first opcode to check, for LUI
            opcode_out_chk : true,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : true,
        
            funct7_in : 0,
            funct7_out : 0,
            funct7_out_chk : true,
        
            r1_index_in : 0,
            r1_index_out : 10,
            r1_index_out_chk : true,
        
            r2_index_in : 0,
            r2_index_out : 11,
            r2_index_out_chk : true,
        };

        let r1pcmux = subc::R1PCMux {
            r1_in : 0, 
            pc_in : 0, 
            opcode_in : 0,
        
            op1_out : 1,
            op1_out_chk : true,
        
            idex_latch_ptr :  None,
            r1for_mux_ptr :  None,
        };

        let r2immmux = subc::R2ImmMux {
            r2_in : 0,
            immediates_in : 0b00111001110011100111000000000000, 
            opcode_in : 0,
        
            op2_out : 0b00111001110011100111000000000000,
            op2_out_chk : true,
        
            idex_latch_ptr :  None,
            r2for_mux_ptr :  None,
        };

        let mut alu = subc::ALUnit {
            op1_in : 0,
            op2_in : 0,
        
            result_out : 0,
            result_out_chk : false,
        
            r1pc_mux_ptr :  Some(&r1pcmux),
            r2imm_mux_ptr :  Some(&r2immmux),
        
            //not listed on GUI!
            opcode_in : 0,
            funct3_in : 0,
            funct7_in : 0,
            idex_latch_ptr :  Some(&idexlatch),
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

    #[test]
    fn exmem_latch() {
        let idexlatch = subc::IDEXLatch {
            base_pc_in : 0,
            base_pc_out : 32,
            base_pc_out_chk : true, 
        
            added_pc_in : 0,
            added_pc_out : 64,
            added_pc_out_chk : true,
        
            r1_data_in : 0,
            r1_data_out : 1,
            r1_data_out_chk : false,
        
            r2_data_in : 0,
            r2_data_out : 4,
            r2_data_out_chk : false,
        
            immediates_in : 2,
            immediates_out : 2,
            immediates_out_chk : true,
        
            rd_index_in : 0,
            rd_index_out : 10,
            rd_index_out_chk : true,

            ifid_latch_ptr :  None,
            reg_mem_ptr :  None,
            imm_dec_ptr :  None,
            instr_dec_ptr :  None,
        
            //these won't be displayed on interface!
            opcode_in : 0,
            opcode_out : 0b0110111, //the first opcode 
            opcode_out_chk : true,
        
            funct3_in : 0,
            funct3_out : 3,
            funct3_out_chk : true,
        
            funct7_in : 0,
            funct7_out : 0,
            funct7_out_chk : true,
        
            r1_index_in : 0,
            r1_index_out : 10,
            r1_index_out_chk : true,
        
            r2_index_in : 0,
            r2_index_out : 11,
            r2_index_out_chk : true,
        };

        let alu = subc::ALUnit {
            op1_in : 0,
            op2_in : 0,
        
            result_out : 13,
            result_out_chk : true,
        
            r1pc_mux_ptr :  None,
            r2imm_mux_ptr :  None,
        
            //not listed on GUI!
            opcode_in : 0,
            funct3_in : 0,
            funct7_in : 0,
            idex_latch_ptr :  None,
        };

        let r2formux = subc::R2ForMux {
            normal_r2_in : 0, 
            exex_r2_in : 0, 
            memex_r2_in : 0, 
        
            r2_out : 1204, 
            r2_out_chk : true,
        
            idex_latch_ptr :  None,
            exmem_latch_ptr :  None,
            memwb_latch_ptr :  None,
            wb_mux_ptr :  None,
        
            //not shown on GUI
            exex_rd_in : 0,  //from EX-MEM latch, vary per test
            memex_rd_in : 0, //from MEM-WB latch, vary per test
            r2_index_in : 0, //from ID-EX latch
        };

        let mut exmemlatch = subc::EXMEMLatch {
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
            
            result_in : 0,
            result_out : 0,
            result_out_chk : false,
        
            mem_data_in : 0,
            mem_data_out : 0,
            mem_data_out_chk : false,
            
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            idex_latch_ptr :  Some(&idexlatch),
            alu_ptr :  Some(&alu),
            r2for_mux_ptr :  Some(&r2formux),
        
            //won't be officially shown
            opcode_in : 0,
            opcode_out : 0,
            opcode_out_chk : false,
        
            funct3_in : 0,
            funct3_out : 0,
            funct3_out_chk : false,
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

    #[test]
    fn memwb_latch() {
        let exmemlatch = subc::EXMEMLatch {
            added_pc_in : 0,
            added_pc_out : 64,
            added_pc_out_chk : true,
            
            result_in : 0,
            result_out : 13,
            result_out_chk : true,
        
            mem_data_in : 0,
            mem_data_out : 0,
            mem_data_out_chk : false,
            
            rd_index_in : 0,
            rd_index_out : 10,
            rd_index_out_chk : true,
        
            idex_latch_ptr :  None,
            alu_ptr :  None,
            r2for_mux_ptr :  None,
        
            //won't be officially shown
            opcode_in : 0,
            opcode_out : 0b10101,
            opcode_out_chk : true,
        
            funct3_in : 0,
            funct3_out : 3,
            funct3_out_chk : true,
        };

        let datamem = subc::DataMem { //just a shell object for the data-memory subcomponents to interact with rest of Processor. Not implemented yet.
            address_in : 0,
            data_in : 0,
        
            mem_read_out : 1204,
            mem_read_out_chk : true,
        
            exmem_latch_ptr :  None,
        
            //wont be officially shown!
            opcode_in : 0,
            funct3_in : 0,
        };

        let mut memwblatch = subc::MEMWBLatch {
            added_pc_in : 0,
            added_pc_out : 0,
            added_pc_out_chk : false,
            
            result_in : 0,
            result_out : 0,
            result_out_chk : false,
        
            mem_read_in : 0,
            mem_read_out : 0,
            mem_read_out_chk : false,
            
            rd_index_in : 0,
            rd_index_out : 0,
            rd_index_out_chk : false,
        
            exmem_latch_ptr :  Some(&exmemlatch),
            data_mem_ptr :  Some(&datamem),
        
            //won't be officially shown
            opcode_in : 0,
            opcode_out : 0,
            opcode_out_chk : false,
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