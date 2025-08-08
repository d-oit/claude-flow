use crate::cli::InitArgs;
use crate::config::Config;
use anyhow::Result;
use tokio::fs;

pub async fn run(args: InitArgs) -> Result<()> {
    let config_path = "codeguardian.toml";
    
    // Check if config already exists
    if fs::metadata(config_path).await.is_ok() {
        println!("Configuration file already exists at {}", config_path);
        println!("Use --force to overwrite (not implemented yet)");
        return Ok(());
    }
    
    // Create default configuration
    let config = if args.default {
        Config::default()
    } else if let Some(template) = &args.template {
        create_from_template(template)?
    } else {
        create_interactive_config().await?
    };
    
    // Save configuration
    let toml_content = toml::to_string_pretty(&config)?;
    fs::write(config_path, toml_content).await?;
    
    println!("✅ Created configuration file: {}", config_path);
    println!("\nNext steps:");
    println!("1. Review and customize the configuration");
    println!("2. Run: codeguardian check .");
    println!("3. Set up CI integration with GitHub Actions");
    
    Ok(())
}

fn create_from_template(template: &str) -> Result<Config> {
    match template {
        "minimal" => Ok(Config::minimal()),
        "security" => Ok(Config::security_focused()),
        "ci" => Ok(Config::ci_optimized()),
        _ => {
            println!("Unknown template: {}", template);
            println!("Available templates: minimal, security, ci");
            Ok(Config::default())
        }
    }
}

async fn create_interactive_config() -> Result<Config> {
    println!("🚀 CodeGuardian Configuration Setup");
    println!("===================================\n");
    
    use std::io::{self, Write};
    
    // Helper function to read user input
    fn read_input(prompt: &str, default: &str) -> Result<String> {
        print!("{} [{}]: ", prompt, default);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            Ok(default.to_string())
        } else {
            Ok(input.to_string())
        }
    }
    
    // Helper function to read yes/no input
    fn read_bool(prompt: &str, default: bool) -> Result<bool> {
        let default_str = if default { "y" } else { "n" };
        let input = read_input(&format!("{} (y/n)", prompt), default_str)?;
        Ok(input.to_lowercase().starts_with('y'))
    }
    
    println!("This wizard will help you create a customized CodeGuardian configuration.\n");
    
    // Project type selection
    println!("1. What type of project are you analyzing?");
    println!("   1) Rust project");
    println!("   2) Python project");
    println!("   3) JavaScript/TypeScript project");
    println!("   4) Multi-language project");
    println!("   5) Other");
    
    let project_type = read_input("Select project type", "4")?;
    
    // Security focus
    let security_focused = read_bool("Is this a security-critical project?", false)?;
    
    // CI usage
    let ci_usage = read_bool("Will this be used in CI/CD pipelines?", true)?;
    
    // Performance preferences
    let max_file_size_mb = read_input("Maximum file size to analyze (MB)", "50")?
        .parse::<u64>()
        .unwrap_or(50);
    
    let max_depth = read_input("Maximum directory depth to scan", "15")?
        .parse::<u32>()
        .unwrap_or(15);
    
    // Build configuration based on answers
    let mut config = Config::default();
    
    // Adjust based on project type
    match project_type.as_str() {
        "1" => {
            // Rust project
            config.general.include_patterns = vec![
                "**/*.rs".to_string(),
                "**/Cargo.toml".to_string(),
                "**/Cargo.lock".to_string(),
            ];
            config.lint_drift.config_files = vec![
                "clippy.toml".to_string(),
                ".rustfmt.toml".to_string(),
                "rustfmt.toml".to_string(),
            ];
        },
        "2" => {
            // Python project
            config.general.include_patterns = vec![
                "**/*.py".to_string(),
                "**/pyproject.toml".to_string(),
                "**/requirements*.txt".to_string(),
                "**/setup.py".to_string(),
            ];
            config.lint_drift.config_files = vec![
                ".pylintrc".to_string(),
                "pyproject.toml".to_string(),
                "setup.cfg".to_string(),
            ];
        },
        "3" => {
            // JavaScript/TypeScript project
            config.general.include_patterns = vec![
                "**/*.js".to_string(),
                "**/*.ts".to_string(),
                "**/*.jsx".to_string(),
                "**/*.tsx".to_string(),
                "**/package.json".to_string(),
                "**/tsconfig.json".to_string(),
            ];
            config.lint_drift.config_files = vec![
                ".eslintrc*".to_string(),
                "tsconfig.json".to_string(),
                ".prettierrc*".to_string(),
            ];
        },
        _ => {
            // Multi-language or other - keep defaults
        }
    }
    
    // Adjust for security focus
    if security_focused {
        config.security.max_memory_mb = 512;
        config.security.operation_timeout = 180;
        config.integrity.verify_signatures = true;
        config.integrity.baseline_file = Some("security-baseline.json".to_string());
        
        // Add security-focused patterns
        config.non_production.patterns.extend(vec![
            crate::config::NonProdPattern {
                pattern: r"(?i)(password|secret|key|token)\s*=\s*[\"'][^\"']+[\"']".to_string(),
                description: "Hardcoded credentials".to_string(),
                severity: "critical".to_string(),
                exclude_paths: vec!["**/tests/**".to_string(), "**/examples/**".to_string()],
            },
            crate::config::NonProdPattern {
                pattern: r"(?i)api[_-]?key\s*[:=]\s*[\"'][^\"']+[\"']".to_string(),
                description: "Hardcoded API key".to_string(),
                severity: "critical".to_string(),
                exclude_paths: vec!["**/tests/**".to_string()],
            },
        ]);
    }
    
    // Adjust for CI usage
    if ci_usage {
        config.performance.worker_threads = 0; // Auto-detect
        config.performance.memory_pool_mb = 512;
        config.security.operation_timeout = 600; // 10 minutes for CI
        config.lint_drift.baseline_file = Some("ci-baseline.json".to_string());
    }
    
    // Apply user preferences
    config.general.max_file_size = max_file_size_mb * 1024 * 1024;
    config.general.max_depth = Some(max_depth);
    
    println!("\n✅ Configuration created successfully!");
    println!("📝 Review the generated codeguardian.toml file and customize as needed.");
    
    Ok(config)
}