#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use winix::cat::{cat, cat_async_to_string, benchmark_cat_sync_vs_async};

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

    #[tokio::test]
    async fn test_cat_async_single_file() {
        let file_path = "test_async_single.txt";
        let content = "Hello, world!\r\nSecond line."; // Simulate Windows CRLF

        tokio::fs::write(file_path, content).await.unwrap();

        let result = cat_async_to_string(vec![file_path]).await.unwrap();

        assert_eq!(result, "Hello, world!\nSecond line.\n");

        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_cat_async_multiple_files() {
        let f1 = "async_file1.txt";
        let f2 = "async_file2.txt";

        tokio::fs::write(f1, "First Line\r\n").await.unwrap();
        tokio::fs::write(f2, "Second Line\n").await.unwrap();

        let result = cat_async_to_string(vec![f1, f2]).await.unwrap();

        assert_eq!(result, "First Line\nSecond Line\n");

        tokio::fs::remove_file(f1).await.unwrap();
        tokio::fs::remove_file(f2).await.unwrap();
    }

    #[tokio::test]
    async fn test_cat_async_nonexistent_file() {
        let result = cat_async_to_string(vec!["no_such_async_file.txt"]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_benchmark_sync_vs_async() {
        let file_path = "benchmark_test.txt";
        let content = "Test content\n".repeat(1000); // Create a larger file for meaningful benchmark

        tokio::fs::write(file_path, content).await.unwrap();

        let (sync_duration, async_duration) = benchmark_cat_sync_vs_async(vec![file_path]).await;
        
        println!("Benchmark Results:");
        println!("  Sync duration: {:?}", sync_duration);
        println!("  Async duration: {:?}", async_duration);
        println!("  Speedup: {:.2}x", sync_duration.as_micros() as f64 / async_duration.as_micros() as f64);
        
        // Clean up
        tokio::fs::remove_file(file_path).await.unwrap();
    }
}
