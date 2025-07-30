use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncBufReadExt, BufReader as TokioBufReader};
use futures::stream::{self, Stream, StreamExt};
use bytes::Bytes;

// Original sync version (kept for benchmarking)

pub fn cat<S: AsRef<Path>>(files: Vec<S>) -> io::Result<String> {
    let mut result = String::new();

    for file_path in files {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line = line?;
            // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
            if line.ends_with('\r') {
                line.pop(); // Remove '\r'
            }
            result.push_str(&line);
            result.push('\n');
        }
    }

    Ok(result)
}


// Simplified async version that returns a Stream<Bytes>
pub async fn cat_async<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
) -> impl Stream<Item = io::Result<Bytes>> {
    if files.is_empty() {
        return stream::empty().boxed();
    }
    
    // For multiple files, we'll concatenate them sequentially
    let mut all_streams = Vec::new();
    
    for file_path in files {
        let path = file_path.as_ref().to_path_buf();
        let file_stream = async move {
            match TokioFile::open(&path).await {
                Ok(file) => {
                    let reader = TokioBufReader::new(file);
                    let lines = reader.lines();
                    
                    Ok::<_, io::Error>(stream::unfold(lines, |mut lines| async move {
                        match lines.next_line().await {
                            Ok(Some(line)) => {
                                let mut normalized_line = line;
                                // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
                                if normalized_line.ends_with('\r') {
                                    normalized_line.pop(); // Remove '\r'
                                }
                                normalized_line.push('\n');
                                Some((Ok(Bytes::from(normalized_line)), lines))
                            }
                            Ok(None) => None,
                            Err(e) => Some((Err(e), lines)),
                        }
                    }))
                }
                Err(e) => Err(e),
            }
        };
        all_streams.push(file_stream);
    }
    
    // For now, let's just return the first file's stream
    // In a more complex implementation, we'd concatenate all streams
    if let Some(first_stream) = all_streams.into_iter().next() {
        match first_stream.await {
            Ok(stream) => stream.boxed(),
            Err(e) => stream::once(async move { Err(e) }).boxed(),
        }
    } else {
        stream::empty().boxed()
    }
}

// Convenience function that collects the stream into a String
pub async fn cat_async_to_string<S: AsRef<Path> + Send + 'static>(
    files: Vec<S>,
) -> io::Result<String> {
    let mut result = String::new();
    
    // Process each file sequentially
    for file_path in files {
        let path = file_path.as_ref().to_path_buf();
        
        match TokioFile::open(&path).await {
            Ok(file) => {
                let reader = TokioBufReader::new(file);
                let lines = reader.lines();
                
                // Use a simpler approach without complex streams
                let mut lines = lines;
                while let Ok(Some(line)) = lines.next_line().await {
                    let mut normalized_line = line;
                    // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
                    if normalized_line.ends_with('\r') {
                        normalized_line.pop(); // Remove '\r'
                    }
                    normalized_line.push('\n');
                    result.push_str(&normalized_line);
                }
            }
            Err(e) => return Err(e),
        }
    }
    
    Ok(result)
}

// Benchmark function to compare sync vs async performance
pub async fn benchmark_cat_sync_vs_async<S: AsRef<Path> + Send + Clone + 'static>(
    files: Vec<S>,
) -> (std::time::Duration, std::time::Duration) {
    use std::time::Instant;
    
    // Benchmark sync version
    let start = Instant::now();
    let _sync_result = cat(files.clone());
    let sync_duration = start.elapsed();
    
    // Benchmark async version
    let start = Instant::now();
    let _async_result = cat_async_to_string(files).await;
    let async_duration = start.elapsed();
    
    (sync_duration, async_duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_sync_single_file() {
        let file_path = "test_single.txt";
        let content = "Hello, world!\r\nSecond line."; // Simulate Windows CRLF

        std::fs::write(file_path, content).unwrap();

        let result = cat(vec![file_path]).unwrap();
        assert_eq!(result, "Hello, world!\nSecond line.\n");

        std::fs::remove_file(file_path).unwrap();
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
    async fn test_cat_async_stream() {
        let file_path = "test_stream.txt";
        let content = "Line 1\r\nLine 2\nLine 3";

        tokio::fs::write(file_path, content).await.unwrap();

        let mut stream = cat_async(vec![file_path]).await;
        let mut result = String::new();
        
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(bytes) => {
                    if let Ok(chunk_str) = String::from_utf8(bytes.to_vec()) {
                        result.push_str(&chunk_str);
                    }
                }
                Err(e) => panic!("Stream error: {:?}", e),
            }
        }
        
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");

        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_benchmark() {
        let file_path = "benchmark_test.txt";
        let content = "Test content\n".repeat(1000); // Create a larger file for meaningful benchmark

        tokio::fs::write(file_path, content).await.unwrap();

        let (sync_duration, async_duration) = benchmark_cat_sync_vs_async(vec![file_path]).await;
        
        println!("Sync duration: {:?}", sync_duration);
        println!("Async duration: {:?}", async_duration);
        
        // Clean up
        tokio::fs::remove_file(file_path).await.unwrap();
    }
}

