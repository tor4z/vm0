pub struct Processor {

}


#[allow(dead_code)]
impl Processor {
    fn new() -> Self {
        Processor {  }
    }
}



#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    #[allow(unused_variables)]
    fn test_new_processor() {
        let proc = Processor::new();
    }

}