#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use winix::cat::cat;

    #[test]
    fn test_cat_single_file() {
        let file_path = "test_single.txt";
        let content = "Hello, world!\r\nSecond line."; // Simulate Windows CRLF

        let mut file = File::create(file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = cat(vec![file_path]).unwrap();

        assert_eq!(result, "Hello, world!\nSecond line.\n");

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_cat_multiple_files() {
        let f1 = "file1.txt";
        let f2 = "file2.txt";

        File::create(f1).unwrap().write_all(b"First Line\r\n").unwrap();
        File::create(f2).unwrap().write_all(b"Second Line\n").unwrap();

        let result = cat(vec![f1, f2]).unwrap();

        assert_eq!(result, "First Line\nSecond Line\n");

        std::fs::remove_file(f1).unwrap();
        std::fs::remove_file(f2).unwrap();
    }

    #[test]
    fn test_cat_nonexistent_file() {
        let result = cat(vec!["no_such_file.txt"]);
        assert!(result.is_err());
    }
}
