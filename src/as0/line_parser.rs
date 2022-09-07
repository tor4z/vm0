#![allow(dead_code)]

pub struct LineParser {
    byte_pos: usize
}


impl LineParser {
    pub fn new() -> LineParser {
        LineParser {
            byte_pos: 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_line_parser() {
        let _parser = LineParser {
            byte_pos: 0,
        };
    }

    fn construct_line_parser_by_new() {
        let _parser = LineParser::new();
    }
}
