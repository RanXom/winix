use std::io;
use futures::stream::{self, Stream, StreamExt};
use bytes::Bytes;

// Pipeline command trait
pub trait AsyncCommand {
    type Input;
    type Output;
    
    async fn execute(&self, input: Self::Input) -> io::Result<Self::Output>;
}

// Pipeline that chains multiple commands
pub struct Pipeline<C1, C2> {
    first: C1,
    second: C2,
}

impl<C1, C2> Pipeline<C1, C2>
where
    C1: AsyncCommand,
    C2: AsyncCommand<Input = C1::Output>,
{
    pub fn new(first: C1, second: C2) -> Self {
        Self { first, second }
    }
    
    pub async fn execute(&self, input: C1::Input) -> io::Result<C2::Output> {
        let intermediate = self.first.execute(input).await?;
        self.second.execute(intermediate).await
    }
}

// Example: Cat -> Grep pipeline
pub struct CatGrepPipeline {
    files: Vec<String>,
    pattern: String,
}

impl CatGrepPipeline {
    pub fn new(files: Vec<String>, pattern: String) -> Self {
        Self { files, pattern }
    }
}

impl AsyncCommand for CatGrepPipeline {
    type Input = ();
    type Output = String;
    
    async fn execute(&self, _input: ()) -> io::Result<String> {
        use crate::cat::cat_async_to_string;
        use crate::grep::grep_async_to_string;
        
        // First, cat the files
        let cat_output = cat_async_to_string(self.files.clone()).await?;
        
        // Write to a temporary file for grep
        let temp_file = "temp_pipeline.txt";
        tokio::fs::write(temp_file, cat_output).await?;
        
        // Then grep the pattern
        let result = grep_async_to_string(&self.pattern, vec![temp_file]).await;
        
        // Clean up
        let _ = tokio::fs::remove_file(temp_file).await;
        
        result
    }
}

// Example: Cat -> Head pipeline
pub struct CatHeadPipeline {
    files: Vec<String>,
    lines: usize,
}

impl CatHeadPipeline {
    pub fn new(files: Vec<String>, lines: usize) -> Self {
        Self { files, lines }
    }
}

impl AsyncCommand for CatHeadPipeline {
    type Input = ();
    type Output = String;
    
    async fn execute(&self, _input: ()) -> io::Result<String> {
        use crate::cat::cat_async_to_string;
        use crate::head::head_async_to_string;
        
        // First, cat the files
        let cat_output = cat_async_to_string(self.files.clone()).await?;
        
        // Write to a temporary file for head
        let temp_file = "temp_head_pipeline.txt";
        tokio::fs::write(temp_file, cat_output).await?;
        
        // Then head the lines
        let result = head_async_to_string(vec![temp_file], self.lines).await;
        
        // Clean up
        let _ = tokio::fs::remove_file(temp_file).await;
        
        result
    }
}

// Generic pipeline executor
pub async fn execute_pipeline<C: AsyncCommand<Input = ()>>(
    command: C,
) -> io::Result<C::Output> {
    command.execute(()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cat_grep_pipeline() {
        let file_path = "test_pipeline.txt";
        let content = "hello world\nthis is a test\nhello again\nbye world";

        tokio::fs::write(file_path, content).await.unwrap();

        let pipeline = CatGrepPipeline::new(
            vec![file_path.to_string()],
            "hello".to_string(),
        );

        let result = execute_pipeline(pipeline).await.unwrap();
        assert!(result.contains("hello world"));
        assert!(result.contains("hello again"));

        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_cat_head_pipeline() {
        let file_path = "test_head_pipeline.txt";
        let content = "line 1\nline 2\nline 3\nline 4\nline 5";

        tokio::fs::write(file_path, content).await.unwrap();

        let pipeline = CatHeadPipeline::new(vec![file_path.to_string()], 3);

        let result = execute_pipeline(pipeline).await.unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2");
        assert_eq!(lines[2], "line 3");

        tokio::fs::remove_file(file_path).await.unwrap();
    }
} 