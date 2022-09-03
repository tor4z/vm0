use std::ops::{Index, IndexMut};


#[allow(dead_code)]
pub struct Memory {
    cap: usize,
    data: Vec<u8>
}


#[allow(dead_code)]
impl Memory {

    /// Create a virtual memory instance.
    /// 
    /// Argument:
    /// 
    /// * cap: The capacity of the virtual memory
    /// 
    /// Return: The virtual memory instalce
    /// 
    /// Example
    /// 
    /// ```
    /// use vm0::memory::Memory;
    /// // Create a virtual memory instance with 1024 bytes.
    /// // The initial data is 0.
    /// #[allow(unused_variables)]
    /// let mut mem = Memory::new(1024);
    /// ```
    pub fn new(cap: usize) -> Self {
        Memory{cap: cap, data: vec![0u8; cap]}
    }
}


impl Index<usize> for Memory {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}


impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}


#[cfg(test)]
mod tests {
    use super::Memory;


    #[test]
    #[allow(unused_variables)]
    fn test_memory_init() {
        let mem = Memory::new(1024);
    }

    #[test]
    fn test_memory_store() {
        let mut mem = Memory::new(1024);
        mem[0] = 0x1;
        assert_eq!(mem[0], 0x01);
    }
}