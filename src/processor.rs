#![allow(dead_code)]
use core::panic;
use std::usize;
use crate::memory::Memory;
use crate::constant::*;

pub struct Processor {
    pub reg: Vec<i32>, // TODO remove pub later.
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
    /// let m = Memory::new(1024);
    /// let proc = Processor::new(m);
    /// ```
    pub fn new(m: Memory) -> Self {
        Processor {reg: vec![0_i32; NUM_REG],
                   freg: vec![0.0_f32; NUM_FREG],
                   dreg: vec![0.0_f64; NUM_DREG],
                   mem: m
        }
    }

    fn r(&self, index: usize) -> i32 {
        self.reg[index]
    }

    fn write_r(&mut self, index: usize, v: i32) {
        self.reg[index] = v;
    }

    fn fr (&mut self, index: usize) -> &mut f32 {
        &mut self.freg[index]
    }

    fn dr(&mut self, index: usize) -> &mut f64 {
        &mut self.dreg[index]
    }

    pub fn exec(&mut self) -> Result<(), &str> {
        loop {
            let ip = self.r(IP);
            match self.mem[ip as usize] {
                LBI => {
                    let r = self.mem[(ip + 1) as usize];
                    let b = self.mem[(ip + 2) as usize];
                    self.write_r(r as usize, b as i32);
                    self.write_r(IP, ip + 3);
                }
                HALT => {
                    break;
                }
                _ => {
                    panic!("Unknown instruction")
                }
            }
        }
        Ok(())
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
        let m = Memory::new(1024);
        let mut proc = Processor::new(m);
        proc.write_r(IP, 1);
        assert_eq!(proc.r(IP), 1);

        proc.write_r(IP, proc.r(IP) + 1);
        assert_eq!(proc.r(IP), 2);
    }

    #[test]
    fn test_processor_freg_manipulate() {
        let m = Memory::new(1024);
        let mut proc = Processor::new(m);
        *proc.fr(F1) = 1_f32;
        assert_eq!(*proc.fr(IP), 1_f32);

        *proc.fr(F1) += 1_f32;
        assert_eq!(*proc.fr(IP), 2_f32);
    }

    #[test]
    fn test_processor_dreg_manipulate() {
        let m = Memory::new(1024);
        let mut proc = Processor::new(m);
        *proc.dr(D1) = 1_f64;
        assert_eq!(*proc.dr(D1), 1_f64);

        *proc.dr(D1) += 1_f64;
        assert_eq!(*proc.dr(D1), 2_f64);
    }
}
