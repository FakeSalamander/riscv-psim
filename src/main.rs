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
