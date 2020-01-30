mod processor;

use processor::Processor;


const SCREEN_HEIGHT: usize = 32; // pixels
const SCREEN_WIDTH: usize = 64; // pixels
const RAM_SIZE: usize = 4096; // bytes

fn main() {
    let processor = Processor::new();
}
