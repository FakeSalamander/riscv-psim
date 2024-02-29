pub mod isa {

pub enum InstrT {
    Rtype,
    Itype,
    Stype,
    Btype,
    Utype,
    Jtype,
}



pub fn get_instruction_type (opcode : u8) -> InstrT { //takes in the opcode of an instruction, and returns what instruction type it is.
    match opcode {
        0b0110111 => InstrT::Utype, //LUI
        0b0010111 => InstrT::Utype, //AUIPC
        0b1101111 => InstrT::Jtype, //JAL
        0b1100111 => InstrT::Itype, //JALR
        0b1100011 => InstrT::Btype, //BEQ, BNE, BLT, BGE, BLTU, BGEU
        0b0000011 => InstrT::Itype, //LB, LH, LW, LBU, LHU
        0b0100011 => InstrT::Stype, //SB, SH, SW
        0b0010011 => InstrT::Itype, //ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
        0b0110011 => InstrT::Rtype, //ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
        0b0001111 => InstrT::Itype, //FENCE, FENCE.I  (not usre about these two? I dont understand them)
        0b1110011 => InstrT::Itype, //ECALL, EBREAK, CSRRW, CSRRS, CSRRC, CSRRWI, CSRRSI, CSRRCI
        _ => panic! ("Invalid instruction opcode encountered!"),
    }
}

}