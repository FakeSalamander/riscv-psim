pub mod components;
pub mod isa;

fn main() {
    use crate::components::*;
    
    println!("Hello, world!");
    
    // code for obtaining instructions here. Not sure how to do it... from a file, maybe?

    let instructions = Vec::<u32>::from([0,0,0,0]);

    //===================================================
    // SETUP
    //===================================================
    // Initializes and connects together all of the CPU subcomponents.

        //generate PC Multiplexor.
        //its input subcomponents do not yet exist, will need to be hooked up later.
        let mut pcmux = PCMux {
            added_pc_in: 0,
            result_in: 0,
            count_out: 0,
            count_out_chk: false,
            pc_adder_ptr: None,
            alu_ptr: None,
            idex_latch_ptr: None,
            branch_comp_ptr: None,
            opcode_in: 0,
            branches_in: false,
        };
    
        //generate Program Counter.
    
        let pc: ProgramCounter = ProgramCounter {
            count_in: 0,
    
            count_out: 0,
            count_out_chk: false,
    
            pc_mux_ptr: Some(&pcmux),
        };
    
        //generate PC Adder
        let pcadd: PCAdder = PCAdder {
            count_in: 0,
            count_out: 0,
            count_out_chk: true,
            pc_ptr: Some(&pc),
        };

        //hook up PC Adder to PC Multiplexor
        pcmux.pc_adder_ptr = Some(&pcadd);
    
        //generate Instruction Memory
        let mut imem = InstrMem {
            ins_addr_in: 0,
    
            ins_array: instructions, //load in the given program into the instruction memory!
    
            instruction_out: 0,
            instruction_out_chk: false,
    
            pc_ptr: Some(&pc),
        };
    
        //generate IF-ID Latch
        let mut ifidlatch = IFIDLatch {
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
    
        //generate the instruction decoder
        let mut dec = InstrDecoder {
            instruction_in: 0,
            ifid_latch_ptr: Some(&ifidlatch),
    
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
    
        //generate the Register Memory.
        //The MEM-WB Latch and WB Multiplexor are missing, and will need to be hooked up later.
        let mut regmem = RegMem {
            r1_index_in: 0,
            r2_index_in: 0,
            rd_index_in: 0,
            wb_data_in: 0,
    
            registers: Vec::<u32>::from([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]),
    
            r1_data_out: 0,
            r2_data_out: 0,
            r1_data_out_chk: false,
            r2_data_out_chk: false,
    
            instr_dec_ptr: Some(&dec),
            memwb_latch_ptr: None,
            wb_mux_ptr: None,
        };    

        //Generate the Immediates Decoder.
        let mut immdec = ImmDecoder {
            opcode_in: 0,
            instr_dec_ptr: Some(&dec),

            instruction_in: 0,
            ifid_latch_ptr: Some(&ifidlatch),

            immediates_out: 0,
            immediates_out_chk: false,
        };

        //generate the ID-EX Latch.
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
            instr_dec_ptr: Some(&dec),

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

        //hook up ID-EX Latch to PC Multiplexor
        pcmux.idex_latch_ptr = Some(&idexlatch);

        //Generate the two Forwarding Multiplexors.
        //The EX-MEM Latch, MEM-WB Latch, and WB Multiplexor do not exist yet, and will need to be connected later.
        let mut r1formux = R1ForMux {
            normal_r1_in: 0, //from IDEX latch, 1
            exex_r1_in: 0,   //from EX-MEM latch, 2
            memex_r1_in: 0,  //from WB Mux, 3

            r1_out: 0,
            r1_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            exmem_latch_ptr: None,
            memwb_latch_ptr: None,
            wb_mux_ptr: None,

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
            exmem_latch_ptr: None,
            memwb_latch_ptr: None,
            wb_mux_ptr: None,

            //not shown on GUI
            exex_rd_in: 0,  //from EX-MEM latch, vary per test
            memex_rd_in: 0, //from MEM-WB latch, vary per test
            r2_index_in: 0, //from ID-EX latch, 11
        };

        //generate the R1-PC Multiplexor.
        let mut r1pcmux = R1PCMux {
            r1_in: 0, //will be 1
            pc_in: 0, //wiill be 32
            opcode_in: 0,

            op1_out: 0,
            op1_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            r1for_mux_ptr: Some(&r1formux),
        };

        //generate the R2-Immediates Multiplexor.

        let mut r2immmux = R2ImmMux {
            r2_in: 0,         
            immediates_in: 0, 
            opcode_in: 0,

            op2_out: 0,
            op2_out_chk: false,

            idex_latch_ptr: Some(&idexlatch),
            r2for_mux_ptr: Some(&r2formux),
        };

        //generate the Branch Comparator.
        let mut bcomp = BranchComparator {
            r1_in: 0,
            r2_in: 0,

            branches_out: false,
            branches_out_chk: false,

            r1for_mux_ptr: Some(&r1formux),
            r2for_mux_ptr: Some(&r2formux),

            funct3_in: 0,
            idex_latch_ptr: Some(&idexlatch),
        };

        //hook up Branch Comparator to PC Multiplexor
        pcmux.branch_comp_ptr = Some(&bcomp);

        //generate ALU.
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

        //hook up ALU to PC Multiplexor
        pcmux.alu_ptr = Some(&alu);

        //generate EX-MEM Latch.
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

        //hook up EX-MEM Latch to Forwarding Multiplexors
        r1formux.exmem_latch_ptr = Some(&exmemlatch);
        r2formux.exmem_latch_ptr = Some(&exmemlatch);


        //generate Data Memory.
        let mut datamem = DataMem{
            address_in: 0,
            data_in: 0,

            mem_read_out: 0,
            mem_read_out_chk: false,

            exmem_latch_ptr: Some(&exmemlatch),

            //won't be officially shown!
            opcode_in: 0,
            funct3_in: 0,
        };

        //generate MEM-WB latch
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

        //hook up MEM-WB Latch to Register Memory & Forwarding Multiplexors
        regmem.memwb_latch_ptr = Some(&memwblatch);
        r1formux.memwb_latch_ptr = Some(&memwblatch);
        r2formux.memwb_latch_ptr = Some(&memwblatch);

        let mut wbmux = WBMux {
            added_pc_in: 0,
            result_in: 0,
            mem_read_in: 0,

            wb_data_out : 0,
            wb_data_out_chk : false,

            memwb_latch_ptr : Some(&memwblatch),

            //not in GUI
            opcode_in : 0,
        };

        //hook up WB Multiplexor to Register Memory & Forwarding Multiplexors
        regmem.wb_mux_ptr = Some(&wbmux);
        r1formux.wb_mux_ptr = Some(&wbmux);
        r2formux.wb_mux_ptr = Some(&wbmux);

}

fn setup( instructions: Vec<u32> ) {
    


}

/*
fn setup() {

}

fn make_backup() {

}

fn load_backup() {

}

fn step_forward() {

}

*/

// also need to do...
// code assembler
