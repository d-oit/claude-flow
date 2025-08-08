use crate::types::Finding;
use anyhow::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader as AsyncBufReader};

/// Threshold for switching to streaming analysis (5MB)
const STREAMING_THRESHOLD: u64 = 5 * 1024 * 1024;

pub struct StreamingAnalyzer {
    chunk_size: usize,
}

impl StreamingAnalyzer {
    pub fn new() -> Self {
        Self {
            chunk_size: 64 * 1024, // 64KB chunks
        }
    }

    pub fn should_use_streaming(file_path: &Path) -> bool {
        file_path
            .metadata()
            .map(|m| m.len() > STREAMING_THRESHOLD)
            .unwrap_or(false)
    }

    pub async fn analyze_large_file<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = File::open(file_path).await?;
        let reader = AsyncBufReader::new(file);
        let mut lines = reader.lines();
        let mut all_findings = Vec::new();
        let mut line_number = 1;

        while let Some(line) = lines.next_line().await? {
            // Analyze line by line for memory efficiency
            let mut line_findings = analyzer_fn(&line, line_number)?;
            all_findings.append(&mut line_findings);
            line_number += 1;

            // Yield control periodically to prevent blocking
            if line_number % 1000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(all_findings)
    }

    pub async fn analyze_binary_chunks<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&[u8], u64) -> Result<Vec<Finding>>,
    {
        use tokio::io::AsyncReadExt;

        let mut file = File::open(file_path).await?;
        let mut buffer = vec![0u8; self.chunk_size];
        let mut offset = 0u64;
        let mut all_findings = Vec::new();

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            let chunk = &buffer[..bytes_read];
            let mut chunk_findings = analyzer_fn(chunk, offset)?;
            all_findings.append(&mut chunk_findings);

            offset += bytes_read as u64;

            // Yield control to prevent blocking
            tokio::task::yield_now().await;
        }

        Ok(all_findings)
    }

    /// Memory-efficient line-by-line analysis for text files
    pub fn analyze_text_streaming<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = std::fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut all_findings = Vec::new();

        for (line_number, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let mut line_findings = analyzer_fn(&line, line_number + 1)?;
            all_findings.append(&mut line_findings);
        }

        Ok(all_findings)
    }
}

/// Adaptive chunk size based on file size and available memory
pub struct AdaptiveChunking {
    base_chunk_size: usize,
    max_chunk_size: usize,
    available_memory: usize,
}

impl AdaptiveChunking {
    pub fn new() -> Self {
        Self {
            base_chunk_size: 64 * 1024,     // 64KB
            max_chunk_size: 1024 * 1024,    // 1MB
            available_memory: Self::estimate_available_memory(),
        }
    }

    pub fn optimal_chunk_size(&self, file_size: u64) -> usize {
        // Adaptive chunking based on file size and available memory
        let memory_based_chunk = self.available_memory / 10; // Use 10% of available memory
        let file_based_chunk = (file_size / 100).max(self.base_chunk_size as u64) as usize;
        
        memory_based_chunk
            .min(file_based_chunk)
            .min(self.max_chunk_size)
            .max(self.base_chunk_size)
    }

    fn estimate_available_memory() -> usize {
        // Simple heuristic - in production, use proper memory detection
        #[cfg(target_os = "linux")]
        {
            if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
                for line in meminfo.lines() {
                    if line.starts_with("MemAvailable:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return kb * 1024; // Convert to bytes
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback: assume 1GB available
        1024 * 1024 * 1024
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_streaming_analysis() {
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Write test content
        let content = "line 1\nline 2\nline 3\n".repeat(1000);
        temp_file.write_all(content.as_bytes()).unwrap();
        
        let analyzer = StreamingAnalyzer::new();
        let findings = analyzer
            .analyze_large_file(temp_file.path(), |line, line_num| {
                if line.contains("line 2") {
                    Ok(vec![Finding::new(
                        "test",
                        "test_rule",
                        crate::types::Severity::Info,
                        temp_file.path().to_path_buf(),
                        line_num as u32,
                        "Found line 2".to_string(),
                    )])
                } else {
                    Ok(vec![])
                }
            })
            .await
            .unwrap();

        assert_eq!(findings.len(), 1000); // Should find "line 2" in each repetition
    }

    #[test]
    fn test_adaptive_chunking() {
        let chunking = AdaptiveChunking::new();
        
        // Small file should use base chunk size
        let small_chunk = chunking.optimal_chunk_size(1024);
        assert_eq!(small_chunk, chunking.base_chunk_size);
        
        // Large file should use larger chunks but not exceed max
        let large_chunk = chunking.optimal_chunk_size(100 * 1024 * 1024);
        assert!(large_chunk > chunking.base_chunk_size);
        assert!(large_chunk <= chunking.max_chunk_size);
    }
}