#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;

    use winix::rm::rm;

    #[test]
    fn test_rm_single_file() {
        let file_path = "delete_me.txt";
        File::create(file_path).unwrap();
        assert!(Path::new(file_path).exists());

        assert!(rm(vec![file_path]).is_ok());
        assert!(!Path::new(file_path).exists());
    }

    #[test]
    fn test_rm_multiple_files() {
        let f1 = "del1.txt";
        let f2 = "del2.txt";

        File::create(f1).unwrap();
        File::create(f2).unwrap();

        assert!(rm(vec![f1, f2]).is_ok());
        assert!(!Path::new(f1).exists());
        assert!(!Path::new(f2).exists());
    }

    #[test]
    fn test_rm_nonexistent_file() {
        let result = rm(vec!["does_not_exist.txt"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_rm_directory_error() {
        let dir = "test_dir";
        std::fs::create_dir(dir).unwrap();

        let result = rm(vec![dir]);
        assert!(result.is_err());

        std::fs::remove_dir(dir).unwrap(); // Cleanup
    }
}
