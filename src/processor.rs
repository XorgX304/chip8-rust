use crate::{
    RAM_SIZE,
    SCREEN_HEIGHT,
    SCREEN_WIDTH,
};

pub const OPCODE_SIZE: usize = 2; // bytes

pub struct Processor {
    ram: [u8; RAM_SIZE],
    reg: [u8; 16], // main register
    vram: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], // hold display info
    index: usize, // 0x000 to 0xFFF also ref'ed as 'i'
    pc: usize, // 0x000 to 0xFFF also ref'ed as 'program count'
    /*
     * 0x000 - 0x1FF -> Chip-8 Interpreter
     * 0x050 - 0x0A0 -> in-built font set
     * 0x200 - 0xFFF -> RAM and ROM
     */
    delay_timer: usize, // 60hz timer, from point called
    sound_timer: usize, // 60hz timer, from point called
    stack: [u8; 16], // store prog_count, when subroutine gets called
    sp: usize, // stack pointer, shows on which level the pc is stored
    keypad: [bool; 16],
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            ram: [0; RAM_SIZE],
            reg: [0; 16],
            vram: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            index: 0,
            pc: 0x200, // points to first instruction at the beginning
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [false; 16],
        }
    }
    fn get_opcode(&mut self) -> u16 { // u16 can store 5 digits
        // somehow fetch code
    }

    fn exec_opcode(opcode: u16) {
        // TODO match opcode to an abstracted function
        // 
        let next_instruction = match FOO {

        }
    }

    // opcodes come here
    // write some abstracted functions, which get called and perform the
    // actual action
    // -> move to opcodes.rs
}
