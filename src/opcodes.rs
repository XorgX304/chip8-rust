use crate::{
    SCREEN_HEIGHT,
    SCREEN_WIDTH,
    OPCODE_SIZE,
};

use processor::Processor;

/* 
 * Comments taken from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.1
 *
 *  nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
 *  n or nibble - A 4-bit value, the lowest 4 bits of the instruction
 *  x - A 4-bit value, the lower 4 bits of the high byte of the instruction
 *  y - A 4-bit value, the upper 4 bits of the low byte of the instruction
 *  kk or byte - An 8-bit value, the lowest 8 bits of the instruction
 */

impl Processor {
    // Clear the display.
    pub fn op_00E0(&mut self) {
        self.vram = [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];
        
    }
    // Return from a subroutine.
    // The interpreter sets the program counter to the address at the top
    // of the stack, then subtracts 1 from the stack pointer.
    pub fn op_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }
    // Jump to location nnn.
    // The interpreter sets the program counter to nnn.
    pub fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }
    // Call subroutine at nnn.
    // The interpreter increments the stack pointer, then puts the current
    // PC on the top of the stack. The PC is then set to nnn.
    pub fn op_2nnn(&mut self, nnn: u16) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = nnn;
    }
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk,
    // and if they are equal, increments the program counter by 2.
    pub fn op_3xkk(&mut self, x: u8, kk: u8) {
        if reg[x] == kk {
            self.pc += OPCODE_SIZE * 2;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }
    // Skip next instruction if Vx != kk.
    // The interpreter compares register Vx to kk,
    // and if they are not equal, increments the program counter by 2.
    pub fn op_4xkk(&mut self, x: u8, kk: u8) {
        if self.reg[x] != kk {
            self.pc += OPCODE_SIZE * 2;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }
    // Skip next instruction if Vx = Vy.
    // The interpreter compares register Vx to register Vy,
    // and if they are equal, increments the program counter by 2.
    pub fn op_5xy0(&mut self, x: u8, y: u8) {
        if self.reg[x] == self.reg[y] {
            self.pc += OPCODE_SIZE * 2;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }
    // Set Vx = kk.
    // The interpreter puts the value kk into register Vx.
    pub fn op_6xkk(&mut self, x: u8, kk: u8) {
        self.reg[x] = kk;
        self.pc += OPCODE_SIZE;
    }
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx,
    // then stores the result in Vx.
    pub fn op_7xkk(&mut self, x: u8, kk: u8) {
        self.reg[x] += kk;
        self.pc += OPCODE_SIZE;
        // TODO SET OVERFLOW BIT
    }
    // Set Vx = Vy.
    // Stores the value of register Vy in register Vx.
    pub fn op_8xy0(&mut self, x: u8, y: u8) {
        self.reg[x] = self.reg[y];
        self.pc += OPCODE_SIZE;
    }
    // Set Vx = Vx OR Vy.
    // Performs a bitwise OR on the values of Vx and Vy,
    // then stores the result in Vx.
    // A bitwise OR compares the corrseponding bits from two values,
    // and if either bit is 1, then the same bit in the result is also 1.
    // Otherwise, it is 0.
    pub fn op_8xy1(&mut self, x: u8, y: u8) {
        self.reg[x] = self.reg[x] | self.reg[y];
    }
}
