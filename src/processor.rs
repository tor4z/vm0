//! ## Instruction table
//! LBI R, byte
//! 
//! * Meaning: Store byte constant in R
//! * Encoding: BBB
//! 
//! 
#![allow(dead_code)]
use std::usize;
use crate::memory::Memory;


const NUM_REG: usize = 32;
const NUM_FREG: usize = 10;
const NUM_DREG: usize = 10;

const IP: usize = 0;
const SP: usize = 1;
const FP: usize = 2;
const BE: usize = 3;

const F1: usize = 0;

const D1: usize = 0;

const LBI: u8 = 0;


pub struct Processor {
    reg: Vec<i32>,
    freg: Vec<f32>,
    dreg: Vec<f64>,
    pub mem: Memory
}


#[allow(dead_code)]
impl Processor {
    /// Create a Processor instance
    /// 
    /// Argument:
    /// 
    /// * m: the virtual memory instance
    /// 
    /// Return: the Proccessor instance
    /// 
    /// Example:
    /// ```
    /// #![allow(dead_code)]
    /// use vm0::{processor::Processor, memory::Memory};
    /// let vm = Memory::new(1024);
    /// let proc = Processor::new(vm);
    /// ```
    pub fn new(m: Memory) -> Self {
        Processor {reg: vec![0_i32; NUM_REG],
                   freg: vec![0.0_f32; NUM_FREG],
                   dreg: vec![0.0_f64; NUM_DREG],
                   mem: m
        }
    }

    fn r(&mut self, index: usize) -> &mut i32 {
        &mut self.reg[index]
    }

    fn fr (&mut self, index: usize) -> &mut f32 {
        &mut self.freg[index]
    }

    fn dr(&mut self, index: usize) -> &mut f64 {
        &mut self.dreg[index]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_processor() {
        let _ = Processor::new(Memory::new(1024));
    }

    #[test]
    fn test_processor_reg_manipulate() {
        let vm = Memory::new(1024);
        let mut proc = Processor::new(vm);
        *proc.r(IP) = 1;
        assert_eq!(*proc.r(IP), 1);

        *proc.r(IP) += 1;
        assert_eq!(*proc.r(IP), 2);
    }

    #[test]
    fn test_processor_freg_manipulate() {
        let vm = Memory::new(1024);
        let mut proc = Processor::new(vm);
        *proc.fr(F1) = 1_f32;
        assert_eq!(*proc.fr(IP), 1_f32);

        *proc.fr(F1) += 1_f32;
        assert_eq!(*proc.fr(IP), 2_f32);
    }

    #[test]
    fn test_processor_dreg_manipulate() {
        let vm = Memory::new(1024);
        let mut proc = Processor::new(vm);
        *proc.dr(D1) = 1_f64;
        assert_eq!(*proc.dr(D1), 1_f64);

        *proc.dr(D1) += 1_f64;
        assert_eq!(*proc.dr(D1), 2_f64);
    }
}
