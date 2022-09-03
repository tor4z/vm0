use vm0::{memory::Memory, processor::Processor};


#[test]
#[allow(unused_variables)]
fn test_processor_memory_binding() {
    let mut vm = Memory::new(1024);
    let proc = Processor::new(&mut vm);
}

