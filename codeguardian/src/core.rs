use crate::analyzers::AnalyzerRegistry;
use crate::cache::FileCache;
use crate::config::Config;
use crate::ml::MLClassifier;
use crate::streaming::StreamingAnalyzer;
use crate::types::{AnalysisResults, Finding};
use crate::utils::progress::ProgressReporter;
use crate::utils::security::{should_follow_path, canonicalize_path_safe};
use anyhow::Result;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::fs;
use walkdir::WalkDir;

pub struct GuardianEngine {
    config: Config,
    analyzer_registry: AnalyzerRegistry,
    progress: ProgressReporter,
    cache: Arc<Mutex<FileCache>>,
    streaming_analyzer: StreamingAnalyzer,
    ml_classifier: MLClassifier,
    stats: AnalysisStats,
}

impl GuardianEngine {
    pub async fn new(config: Config, progress: ProgressReporter) -> Result<Self> {
        let cache = Arc::new(Mutex::new(FileCache::load().await?));
        
        // Initialize ML classifier (looks for pre-trained model)
        let ml_classifier = MLClassifier::new(Some("codeguardian-model.fann"));
        
        Ok(Self {
            config,
            analyzer_registry: AnalyzerRegistry::new(),
            progress,
            cache,
            streaming_analyzer: StreamingAnalyzer::new(),
            ml_classifier,
            stats: AnalysisStats::new(),
        })
    }

    pub async fn get_all_files(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut all_files = Vec::new();
        
        for path in paths {
            if path.is_file() {
                all_files.push(canonicalize_path_safe(path));
            } else if path.is_dir() {
                let files = self.scan_directory(path).await?;
                all_files.extend(files);
            }
        }
        
        Ok(all_files)
    }

    pub async fn get_diff_files(&self, diff_spec: &str) -> Result<Vec<PathBuf>> {
        crate::utils::git::get_diff_files(diff_spec)
    }

    pub async fn get_staged_files(&self) -> Result<Vec<PathBuf>> {
        crate::utils::git::get_staged_files()
    }

    async fn scan_directory(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(dir_path)
            .follow_links(false) // Security: don't follow symlinks by default
            .into_iter()
            .filter_entry(|e| should_follow_path(e.path(), false))
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && self.should_analyze_file(path) {
                files.push(canonicalize_path_safe(path));
            }
        }
        
        Ok(files)
    }

    fn should_analyze_file(&self, path: &Path) -> bool {
        // Skip files based on configuration
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Skip common ignore patterns
            if name.starts_with('.') && name != ".gitignore" && name != ".dockerignore" {
                return false;
            }
        }
        
        // Check file size limits (security: prevent processing huge files)
        if let Ok(metadata) = path.metadata() {
            if metadata.len() > 10 * 1024 * 1024 { // 10MB limit
                return false;
            }
        }
        
        true
    }

    pub async fn analyze_files(&mut self, files: &[PathBuf], parallel: usize) -> Result<AnalysisResults> {
        let start_time = Instant::now();
        let config_hash = self.compute_config_hash();
        let mut results = AnalysisResults::new(config_hash.clone());
        
        // Start progress reporting
        self.progress.start_scan(files.len());
        
        // Check cache for each file first
        let (cached_files, uncached_files) = self.partition_cached_files(files, &config_hash).await?;
        
        // Add cached findings
        for (file_path, cached_findings) in cached_files {
            for finding in cached_findings {
                results.add_finding(finding);
            }
            self.stats.cache_hits += 1;
            self.progress.update(&format!("Cached: {}", file_path.display()));
        }
        
        if !uncached_files.is_empty() {
            // Determine parallelism
            let num_workers = if parallel == 0 {
                num_cpus::get()
            } else {
                parallel
            };
            
            // Process uncached files in parallel
            let analyzer_registry = Arc::new(&self.analyzer_registry);
            let cache = Arc::clone(&self.cache);
            let streaming_analyzer = Arc::new(&self.streaming_analyzer);
            
            let findings: Vec<Vec<Finding>> = uncached_files
                .par_chunks(uncached_files.len() / num_workers.max(1))
                .map(|chunk| {
                    let mut chunk_findings = Vec::new();
                    for file_path in chunk {
                        match self.analyze_single_file_optimized(
                            file_path, 
                            &analyzer_registry, 
                            &streaming_analyzer,
                            &config_hash
                        ) {
                            Ok(file_findings) => {
                                // Cache the results
                                if let Ok(mut cache_guard) = cache.lock() {
                                    let _ = tokio::task::block_in_place(|| {
                                        tokio::runtime::Handle::current().block_on(
                                            cache_guard.cache_findings(file_path, file_findings.clone(), &config_hash)
                                        )
                                    });
                                }
                                chunk_findings.extend(file_findings);
                                self.stats.cache_misses += 1;
                            }
                            Err(e) => {
                                eprintln!("Error analyzing {}: {}", file_path.display(), e);
                                self.stats.errors += 1;
                            }
                        }
                    }
                    chunk_findings
                })
                .collect();
            
            // Collect all findings and apply ML filtering
            let mut all_findings = Vec::new();
            for chunk_findings in findings {
                all_findings.extend(chunk_findings);
            }
            
            // Apply ML-based false positive reduction if enabled
            let filtered_findings = self.ml_classifier.filter_findings(all_findings, 0.3)?; // 30% confidence threshold
            
            for finding in filtered_findings {
                results.add_finding(finding);
            }
        }
        
        // Update summary
        results.summary.total_files_scanned = files.len();
        self.stats.total_duration = start_time.elapsed();
        
        // Save cache
        if let Ok(cache_guard) = self.cache.lock() {
            let _ = cache_guard.save().await;
        }
        
        // Finish progress reporting
        self.progress.finish(&format!(
            "Analyzed {} files ({} cached, {} new) in {:.2}s", 
            files.len(),
            self.stats.cache_hits,
            self.stats.cache_misses,
            self.stats.total_duration.as_secs_f64()
        ));
        
        Ok(results)
    }

    fn analyze_single_file_optimized(
        &self, 
        file_path: &Path, 
        analyzer_registry: &AnalyzerRegistry,
        streaming_analyzer: &StreamingAnalyzer,
        config_hash: &str
    ) -> Result<Vec<Finding>> {
        // Update progress
        self.progress.update(&format!("Analyzing {}", file_path.display()));
        
        // Check if we should use streaming analysis
        if StreamingAnalyzer::should_use_streaming(file_path) {
            // Use streaming analysis for large files
            self.analyze_large_file_streaming(file_path, analyzer_registry, streaming_analyzer)
        } else {
            // Standard in-memory analysis for smaller files
            let content = std::fs::read(file_path)?;
            analyzer_registry.analyze_file(file_path, &content)
        }
    }
    
    fn analyze_large_file_streaming(
        &self,
        file_path: &Path,
        analyzer_registry: &AnalyzerRegistry,
        _streaming_analyzer: &StreamingAnalyzer,
    ) -> Result<Vec<Finding>> {
        // For now, fall back to chunked reading for large files
        // In a full implementation, this would use the streaming analyzer
        // with line-by-line or chunk-by-chunk processing
        
        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut all_findings = Vec::new();
        
        // Process file line by line to save memory
        let mut line_buffer = String::new();
        let mut line_number = 1u32;
        
        for line_result in reader.lines() {
            let line = line_result?;
            line_buffer.clear();
            line_buffer.push_str(&line);
            line_buffer.push('\n');
            
            // Analyze this line
            let line_findings = analyzer_registry.analyze_file(file_path, line_buffer.as_bytes())?;
            all_findings.extend(line_findings);
            
            line_number += 1;
            
            // Yield occasionally for very large files
            if line_number % 10000 == 0 {
                std::thread::yield_now();
            }
        }
        
        Ok(all_findings)
    }
    
    async fn partition_cached_files(
        &self,
        files: &[PathBuf],
        config_hash: &str,
    ) -> Result<(Vec<(PathBuf, Vec<Finding>)>, Vec<PathBuf>)> {
        let mut cached_files = Vec::new();
        let mut uncached_files = Vec::new();
        
        if let Ok(cache_guard) = self.cache.lock() {
            for file_path in files {
                if let Some(cached_findings) = cache_guard.get_cached_findings(file_path, config_hash) {
                    cached_files.push((file_path.clone(), cached_findings));
                } else {
                    uncached_files.push(file_path.clone());
                }
            }
        } else {
            // If cache is locked, treat all files as uncached
            uncached_files.extend_from_slice(files);
        }
        
        Ok((cached_files, uncached_files))
    }

    fn compute_config_hash(&self) -> String {
        use sha2::{Digest, Sha256};
        
        // Serialize config and hash it
        let config_str = toml::to_string(&self.config).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(config_str.as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }
}

#[derive(Debug, Default)]
pub struct AnalysisStats {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub errors: usize,
    pub total_duration: std::time::Duration,
}

impl AnalysisStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}