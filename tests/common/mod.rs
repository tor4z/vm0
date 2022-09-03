#[macro_export]
macro_rules! create_proc {
    ($cap: literal) => {
        Processor::new(Memory::new($cap))
    }
}
