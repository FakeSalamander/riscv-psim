mod decode;
mod execute;
mod fetch;
mod memory;
mod writeback;

pub use decode::*;
pub use execute::*;
pub use fetch::*;
pub use memory::*;
pub use writeback::*;

trait Logic {
    fn update(&mut self);
}
trait Latch {
    fn update(&mut self);
}
