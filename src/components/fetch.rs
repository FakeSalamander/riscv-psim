use crate::components::*;

//holds all the wires for the IF Stage
#[derive(Clone, Default)]
pub struct IFLogic {
    pub pcmux_out: u32,
    pub instruction_out: u32,
    pub pcadder_out: u32,
}
