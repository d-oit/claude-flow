use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main configuration structure for CodeGuardian
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// General settings
    pub general: GeneralConfig,
    
    /// File integrity checker configuration
    pub integrity: IntegrityConfig,
    
    /// Lint configuration drift analyzer settings
    pub lint_drift: LintDriftConfig,
    
    /// Non-production code detector settings
    pub non_production: NonProductionConfig,
    
    /// Security settings
    pub security: SecurityConfig,
    
    /// Performance settings
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: u64,
    
    /// File patterns to include
    pub include_patterns: Vec<String>,
    
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
    
    /// Whether to follow symbolic links
    pub follow_symlinks: bool,
    
    /// Maximum directory depth to traverse
    pub max_depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityConfig {
    /// Hash algorithm to use (blake3, sha256, sha512)
    pub algorithm: HashAlgorithm,
    
    /// Whether to store hashes in extended attributes
    pub use_xattr: bool,
    
    /// Baseline file for integrity checking
    pub baseline_file: Option<PathBuf>,
    
    /// Whether to verify file signatures
    pub verify_signatures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintDriftConfig {
    /// Lint configuration files to monitor
    pub config_files: Vec<String>,
    
    /// Git repository path for baseline comparison
    pub git_repo: Option<PathBuf>,
    
    /// Git reference for baseline (branch, tag, or commit)
    pub baseline_ref: String,
    
    /// Whether to check for missing configurations
    pub check_missing: bool,
    
    /// Custom drift detection rules
    pub custom_rules: HashMap<String, DriftRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProductionConfig {
    /// Patterns to detect non-production code
    pub patterns: Vec<NonProdPattern>,
    
    /// File extensions to analyze
    pub file_extensions: Vec<String>,
    
    /// Whether to check test directories
    pub include_test_dirs: bool,
    
    /// Custom detection rules
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable sandboxed file access
    pub sandbox_enabled: bool,
    
    /// Maximum memory usage per operation (in MB)
    pub max_memory_mb: usize,
    
    /// Timeout for operations (in seconds)
    pub operation_timeout: u64,
    
    /// Whether to redact sensitive data in logs
    pub redact_sensitive_data: bool,
    
    /// Allowed file system paths
    pub allowed_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Number of worker threads (0 = auto)
    pub worker_threads: usize,
    
    /// Buffer size for file operations (in KB)
    pub buffer_size_kb: usize,
    
    /// Enable memory mapping for large files
    pub use_mmap: bool,
    
    /// Memory pool size (in MB)
    pub memory_pool_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Blake3,
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftRule {
    pub name: String,
    pub pattern: String,
    pub severity: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProdPattern {
    pub pattern: String,
    pub description: String,
    pub severity: String,
    pub exclude_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub name: String,
    pub pattern: String,
    pub file_types: Vec<String>,
    pub severity: String,
    pub message: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            integrity: IntegrityConfig::default(),
            lint_drift: LintDriftConfig::default(),
            non_production: NonProductionConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100MB
            include_patterns: vec![
                "**/*.rs".to_string(),
                "**/*.py".to_string(),
                "**/*.js".to_string(),
                "**/*.ts".to_string(),
                "**/*.go".to_string(),
                "**/*.java".to_string(),
                "**/*.c".to_string(),
                "**/*.cpp".to_string(),
                "**/*.h".to_string(),
                "**/*.hpp".to_string(),
            ],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/build/**".to_string(),
                "**/dist/**".to_string(),
            ],
            follow_symlinks: false,
            max_depth: Some(20),
        }
    }
}

impl Default for IntegrityConfig {
    fn default() -> Self {
        Self {
            algorithm: HashAlgorithm::Blake3,
            use_xattr: false,
            baseline_file: None,
            verify_signatures: false,
        }
    }
}

impl Default for LintDriftConfig {
    fn default() -> Self {
        Self {
            config_files: vec![
                ".eslintrc*".to_string(),
                ".pylintrc".to_string(),
                "pyproject.toml".to_string(),
                "clippy.toml".to_string(),
                ".rustfmt.toml".to_string(),
                "rustfmt.toml".to_string(),
            ],
            git_repo: None,
            baseline_ref: "main".to_string(),
            check_missing: true,
            custom_rules: HashMap::new(),
        }
    }
}

impl Default for NonProductionConfig {
    fn default() -> Self {
        Self {
            patterns: vec![
                NonProdPattern {
                    pattern: r"#\[cfg\(test\)\]".to_string(),
                    description: "Test configuration attribute".to_string(),
                    severity: "medium".to_string(),
                    exclude_paths: vec!["**/tests/**".to_string(), "**/*test*.rs".to_string()],
                },
                NonProdPattern {
                    pattern: r"println!\s*\(".to_string(),
                    description: "Debug print statement".to_string(),
                    severity: "low".to_string(),
                    exclude_paths: vec!["**/examples/**".to_string()],
                },
                NonProdPattern {
                    pattern: r"todo!\s*\(".to_string(),
                    description: "TODO macro".to_string(),
                    severity: "high".to_string(),
                    exclude_paths: vec![],
                },
                NonProdPattern {
                    pattern: r"unimplemented!\s*\(".to_string(),
                    description: "Unimplemented macro".to_string(),
                    severity: "high".to_string(),
                    exclude_paths: vec![],
                },
                NonProdPattern {
                    pattern: r"panic!\s*\(".to_string(),
                    description: "Panic macro".to_string(),
                    severity: "high".to_string(),
                    exclude_paths: vec!["**/tests/**".to_string()],
                },
            ],
            file_extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "go".to_string(),
                "java".to_string(),
            ],
            include_test_dirs: false,
            custom_rules: vec![],
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sandbox_enabled: true,
            max_memory_mb: 1024, // 1GB
            operation_timeout: 300, // 5 minutes
            redact_sensitive_data: true,
            allowed_paths: vec![],
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: 0, // Auto-detect
            buffer_size_kb: 64,
            use_mmap: true,
            memory_pool_mb: 256,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| GuardianError::io("Failed to read config file", e))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| GuardianError::config(
                format!("Failed to parse config file: {}", e),
                Some(path.to_path_buf()),
            ))?;

        config.validate()?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)
            .map_err(|e| GuardianError::io("Failed to write config file", e))?;
        Ok(())
    }

    /// Create default configuration file
    pub fn create_default_config() -> Result<()> {
        let config = Self::default();
        config.save(Path::new("codeguardian.toml"))?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate general config
        if self.general.max_file_size == 0 {
            return Err(GuardianError::config(
                "max_file_size must be greater than 0",
                None,
            ));
        }

        // Validate security config
        if self.security.max_memory_mb == 0 {
            return Err(GuardianError::config(
                "max_memory_mb must be greater than 0",
                None,
            ));
        }

        if self.security.operation_timeout == 0 {
            return Err(GuardianError::config(
                "operation_timeout must be greater than 0",
                None,
            ));
        }

        // Validate performance config
        if self.performance.buffer_size_kb == 0 {
            return Err(GuardianError::config(
                "buffer_size_kb must be greater than 0",
                None,
            ));
        }

        // Validate patterns
        for pattern in &self.non_production.patterns {
            if pattern.pattern.is_empty() {
                return Err(GuardianError::config(
                    "Non-production pattern cannot be empty",
                    None,
                ));
            }
        }

        Ok(())
    }

    /// Get effective worker thread count
    pub fn effective_worker_threads(&self) -> usize {
        if self.performance.worker_threads == 0 {
            num_cpus::get()
        } else {
            self.performance.worker_threads
        }
    }

    /// Create minimal configuration for basic usage
    pub fn minimal() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: 10 * 1024 * 1024, // 10MB
                include_patterns: vec![
                    "**/*.rs".to_string(),
                    "**/*.py".to_string(),
                    "**/*.js".to_string(),
                ],
                exclude_patterns: vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                ],
                follow_symlinks: false,
                max_depth: Some(10),
            },
            integrity: IntegrityConfig {
                algorithm: HashAlgorithm::Blake3,
                use_xattr: false,
                baseline_file: None,
                verify_signatures: false,
            },
            lint_drift: LintDriftConfig {
                config_files: vec![
                    ".eslintrc*".to_string(),
                    "pyproject.toml".to_string(),
                ],
                git_repo: None,
                baseline_ref: "main".to_string(),
                check_missing: false,
                custom_rules: HashMap::new(),
            },
            non_production: NonProductionConfig {
                patterns: vec![
                    NonProdPattern {
                        pattern: r"todo!\s*\(".to_string(),
                        description: "TODO macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec![],
                    },
                ],
                file_extensions: vec!["rs".to_string(), "py".to_string()],
                include_test_dirs: false,
                custom_rules: vec![],
            },
            security: SecurityConfig::default(),
            performance: PerformanceConfig {
                worker_threads: 2,
                buffer_size_kb: 32,
                use_mmap: false,
                memory_pool_mb: 128,
            },
        }
    }

    /// Create security-focused configuration
    pub fn security_focused() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: 50 * 1024 * 1024, // 50MB
                include_patterns: vec![
                    "**/*.rs".to_string(),
                    "**/*.py".to_string(),
                    "**/*.js".to_string(),
                    "**/*.ts".to_string(),
                    "**/*.go".to_string(),
                    "**/*.java".to_string(),
                    "**/*.c".to_string(),
                    "**/*.cpp".to_string(),
                    "**/*.sh".to_string(),
                    "**/*.yml".to_string(),
                    "**/*.yaml".to_string(),
                    "**/*.json".to_string(),
                ],
                exclude_patterns: vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                    "**/build/**".to_string(),
                ],
                follow_symlinks: false,
                max_depth: Some(15),
            },
            integrity: IntegrityConfig {
                algorithm: HashAlgorithm::Blake3,
                use_xattr: true,
                baseline_file: Some("security-baseline.json".to_string()),
                verify_signatures: true,
            },
            lint_drift: LintDriftConfig::default(),
            non_production: NonProductionConfig {
                patterns: vec![
                    NonProdPattern {
                        pattern: r"todo!\s*\(".to_string(),
                        description: "TODO macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec![],
                    },
                    NonProdPattern {
                        pattern: r"unimplemented!\s*\(".to_string(),
                        description: "Unimplemented macro".to_string(),
                        severity: "critical".to_string(),
                        exclude_paths: vec![],
                    },
                    NonProdPattern {
                        pattern: r"panic!\s*\(".to_string(),
                        description: "Panic macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string()],
                    },
                    NonProdPattern {
                        pattern: r"(?i)(password|secret|key|token)\s*=\s*[\"'][^\"']+[\"']".to_string(),
                        description: "Hardcoded credentials".to_string(),
                        severity: "critical".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string(), "**/examples/**".to_string()],
                    },
                    NonProdPattern {
                        pattern: r"(?i)api[_-]?key\s*[:=]\s*[\"'][^\"']+[\"']".to_string(),
                        description: "Hardcoded API key".to_string(),
                        severity: "critical".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string()],
                    },
                ],
                file_extensions: vec![
                    "rs".to_string(),
                    "py".to_string(),
                    "js".to_string(),
                    "ts".to_string(),
                    "go".to_string(),
                    "java".to_string(),
                    "sh".to_string(),
                ],
                include_test_dirs: false,
                custom_rules: vec![],
            },
            security: SecurityConfig {
                sandbox_enabled: true,
                max_memory_mb: 512,
                operation_timeout: 180,
                redact_sensitive_data: true,
                allowed_paths: vec![],
            },
            performance: PerformanceConfig::default(),
        }
    }

    /// Create CI-optimized configuration
    pub fn ci_optimized() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: 20 * 1024 * 1024, // 20MB
                include_patterns: vec![
                    "**/*.rs".to_string(),
                    "**/*.py".to_string(),
                    "**/*.js".to_string(),
                    "**/*.ts".to_string(),
                    "**/*.go".to_string(),
                    "**/*.java".to_string(),
                ],
                exclude_patterns: vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                    "**/build/**".to_string(),
                    "**/dist/**".to_string(),
                    "**/coverage/**".to_string(),
                    "**/.nyc_output/**".to_string(),
                ],
                follow_symlinks: false,
                max_depth: Some(12),
            },
            integrity: IntegrityConfig {
                algorithm: HashAlgorithm::Blake3,
                use_xattr: false,
                baseline_file: Some("ci-baseline.json".to_string()),
                verify_signatures: false,
            },
            lint_drift: LintDriftConfig {
                config_files: vec![
                    ".eslintrc*".to_string(),
                    ".pylintrc".to_string(),
                    "pyproject.toml".to_string(),
                    "clippy.toml".to_string(),
                    ".rustfmt.toml".to_string(),
                    "rustfmt.toml".to_string(),
                    ".github/workflows/*.yml".to_string(),
                    ".github/workflows/*.yaml".to_string(),
                ],
                git_repo: None,
                baseline_ref: "main".to_string(),
                check_missing: true,
                custom_rules: HashMap::new(),
            },
            non_production: NonProductionConfig {
                patterns: vec![
                    NonProdPattern {
                        pattern: r"todo!\s*\(".to_string(),
                        description: "TODO macro".to_string(),
                        severity: "medium".to_string(),
                        exclude_paths: vec![],
                    },
                    NonProdPattern {
                        pattern: r"unimplemented!\s*\(".to_string(),
                        description: "Unimplemented macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec![],
                    },
                    NonProdPattern {
                        pattern: r"console\.log\s*\(".to_string(),
                        description: "Console log statement".to_string(),
                        severity: "low".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string()],
                    },
                    NonProdPattern {
                        pattern: r"print\s*\(".to_string(),
                        description: "Print statement".to_string(),
                        severity: "low".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string(), "**/examples/**".to_string()],
                    },
                ],
                file_extensions: vec![
                    "rs".to_string(),
                    "py".to_string(),
                    "js".to_string(),
                    "ts".to_string(),
                ],
                include_test_dirs: false,
                custom_rules: vec![],
            },
            security: SecurityConfig {
                sandbox_enabled: true,
                max_memory_mb: 2048,
                operation_timeout: 600, // 10 minutes for CI
                redact_sensitive_data: true,
                allowed_paths: vec![],
            },
            performance: PerformanceConfig {
                worker_threads: 0, // Auto-detect for CI
                buffer_size_kb: 128,
                use_mmap: true,
                memory_pool_mb: 512,
            },
        }
    }
}