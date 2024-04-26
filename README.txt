A simple, commandline-based RISC-V simulator that displays its own internal state.
 
IMPLEMENTED:
    - Core Simulator

    - Backup & Rewind Functions

    - GUI

    
TO IMPLEMENT:
    - Data Memory Display

    - LOAD-ALU bug-fix

    - Assembler


REQUIREMENTS:
    - The simulator is written entirely in Rust, and only needs Cargo and a working Rust environment to run.

USAGE:

Copy the github repo and CD into it.

        git clone https://github.com/FakeSalamander/riscv-psim
        cd riscv-psim

If you just want to run the program without building it, enter the following command, where "<program_file>" is the path to the file containing the machine-code program you want to run. A small set of example programs has been provided.

        cargo run -- <program_file>

If you want a compiled binary, run the following program.

        cargo build --release

The resulting binary can be found in the ./target/release/ directory, and can be run like any other program.

        ./riscv-psim <program_file>
