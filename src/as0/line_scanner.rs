#![allow(dead_code)]
pub struct Line {
    line_num: usize,
    text: String
}


#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn test_line_construct() {
        let _line = Line {line_num: 0, text: String::from("text")};
    }
}
