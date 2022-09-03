use vm0::{memory::Memory, processor::Processor};
mod common;


#[test]
#[allow(unused_variables)]
fn test_processor_memory_binding() {
    let proc = Processor::new(Memory::new(1024));
}


#[test]
fn test_write_vm() {
    let mut proc = create_proc!(1024);
    proc.vm[0] = 0x1;
    assert_eq!(proc.vm[0], 0x1)
}



#[test]
#[allow(unused_variables)]
fn test_processor_inst_lbi() {
    let proc = create_proc!(10);
    // TODO implement me.
}
