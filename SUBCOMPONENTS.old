This is a basic framework for the components of RISC-V CPU simulator with 5 pipelined stages. Different subcomponents, which hook together to form a functioning simulated CPU, are represented as structs in the Rust programming language.

Here is a quick overview of what subcomponents are planned, and which are successfully implemented.
All major/complex components are implemented, save for the Data Memory. They are ordered by pipeline stage, starting at Instruction-Fetch and ending at WriteBack.


SUBCOMPONENTS.RS:

    Jump Mux - this multiplexors decides whether the next instruction should be the sequentially next one, or a non-sequential jumped one as a result of a Jump or Branch instruction, sending the right PC across. NOT YET IMPLEMENTED>
    
    ProgramCounter - keeps track of what binary instruction to execute next. IMPLEMENTED
    
    PC-Adder - a simple adder that takes the current Program Count and increments it to the next instruction. IMPLEMENTED
            - PC can be modified by user to hold 32-bit or 64-bit values, for longer programs. IMPLEMENTED.
    
    Instruction Memory - stores the program to execute, instruction-by-instruction, and fetches the instruction given by the inputted Program Count. IMPLEMENTED.
    
    IF-ID Latch - separates the Instruction-Fetch and Instruction-Decode pipeline stages. IMPLEMENTED.
    
    Instruction Decoder - takes in an instruction, and pulls the opcode, function codes, and register indexes out for other subcomponents to use. IMPLEMENTED.
    
    Register Memory - holds the array of registers, and fetches the data of registers given to it. IMPLEMENTED.
                - In final version, Register Memory may have register renaming (a set of physical registers, and then 32 register names that get continually reassigned to them) in order to avoid false data hazards. This is NOT YET IMPLEMENTED.
    
    Immediates Decoder - takes in an instruction + opcode,  and uses it to organize and output wherver the immediates (integer literals) may be using bitwise operations. IMPLEMENTED.
    
    ID-EX Latch - separates the Instruction-Decode and Execute pipeline stages. IMPLEMENTED.
    
    R1-Forwarding Mux, R2-Forwarding Mux - these multiplexors decide whether to use the R1 and R2 data values obtained from the ID-stage, or to forward them from the previously-executed EX or MEM stages. NOT YET IMPLEMENTED.
    
    R1-PC Mux, R2-Immediates Mux - depending on the type of instruction, the ALU will need to perform computation on Register data, Immediates, or the Program Count. These multiplexors decide what data should become the Operand1 and Operand2 sent respectively. IMPLEMENTED.
    
    ALU - most important component. based on the funct-codes and opcode, it performs the desires computation on the two Operands and sends the output along. IMPLEMENTED.
        - right now, only the Base Integer Instruction set, mandatory for all RISC-V processors, is implemented. More optional instruction sets of the RISC-V ISA may be included in the final work.
    
    EX-MEM Latch - separates the Execute and Memory pipleine stages. NOT YET IMPLEMENTED.
    
    Data Memory - the larger memory that programs can read and store data from. Is likely going to be multiple subcomponents. NOT YET IMPLEMENTED.
    
    MEM-WB Latch - separates the Memory and WriteBack pipeline stages. NOT YET IMPLEMENTED.
    
    Output Mux - Depending on the instruction, the output written to the destination register will be a Word from the Data Memory, the output of the ALU's computation, or the Program Count of the next instruction. This mux selects the right one and passes it back to the Register Memory. NOT YET IMPLEMENTED
    
ISA.RS:
    This small file simply establishes an enum for the different Instruction-Types that RISC-V has, as well as a function for determining the Type of an instruction by its opcode.
    
    The instruction-type determines where the Immediates are located in the instruction and what input sources the ALU needs, so this file is primarily used by the Immediates-Decoder and R1-PC and R2-Imm Multiplexors!.
    
