
#[allow(dead_code)]
pub struct FileReader {
    filename: String
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_file_reader() {
        let _file_reader = FileReader {
            filename: String::from("filename")
        };
    }
}
