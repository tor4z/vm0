use vm0::{memory::Memory, processor::Processor, constant::* };
mod common;

#[test]
#[allow(unused_variables)]
fn test_processor_memory_binding() {
    let proc = Processor::new(Memory::new(1024));
}


#[test]
fn test_write_mem() {
    let mut proc = create_proc!(1024);
    proc.mem[0] = 0x1;
    assert_eq!(proc.mem[0], 0x1)
}


#[test]
fn test_write_memory_with_vec() {
    let mut proc = create_proc!(1024);
    let data: Vec<u8> = vec![0, 1, 2];
    proc.mem.write(&data, 0).unwrap();
    for i in 0..2 {
        assert_eq!(proc.mem[i], i as u8);
    }
}


#[test]
fn test_processor_inst_lbi() {
    let mut proc = create_proc!(1024);
    let instr: Vec<u8> = vec![LBI, R1 as u8, 0x1, HALT];
    proc.mem.write(&instr, 0).unwrap();
    proc.exec().unwrap();
    assert_eq!(proc.reg[R1], 0x1);
}
