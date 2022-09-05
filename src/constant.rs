//! ## Instruction table
//! LBI R, byte
//! 
//! * Meaning: Store byte constant in R
//! * Encoding: BBB
//! 
//! 

// Instructions
pub const LBI: u8 = 0;
pub const HALT: u8 = 42;


pub const NUM_REG: usize = 32;
pub const NUM_FREG: usize = 10;
pub const NUM_DREG: usize = 10;

// Functional registers
pub const IP: usize = 0;
pub const SP: usize = 1;
pub const FP: usize = 2;
pub const BE: usize = 3;

// General purpose registers
pub const R1: usize = 8;

// Float-point holding registers
pub const F1: usize = 0;

// Double precision holding register
pub const D1: usize = 0;
