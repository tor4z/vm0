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

    pub fn write(&mut self, data: &Vec<u8>, mut index: usize) -> Result<(), &str> {
        for v in data.iter() {
            self.data[index] = *v;
            index += 1;
            if index >= self.cap {
                panic!("Memory out of range")
            }
        }
        Ok(())
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

    #[test]
    fn test_memory_write_mem_from_vec() {
        let mut mem = Memory::new(1024);
        let data : Vec<u8> = vec![0, 1, 2];
        mem.write(&data, 0).unwrap();
        for i in 0..2 {
            assert_eq!(mem[i], i as u8)
        }
    }

    #[test]
    #[should_panic]
    fn test_memory_write_mem_from_vec_overflow() {
        let mut mem = Memory::new(64);
        let data : Vec<u8> = vec![1; 100];
        mem.write(&data, 0).unwrap();
    }
}
