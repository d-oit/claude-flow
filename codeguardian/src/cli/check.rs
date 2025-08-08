use crate::cli::CheckArgs;
use crate::config::Config;
use crate::core::GuardianEngine;
use crate::types::AnalysisResults;
use crate::utils::progress::ProgressReporter;
use anyhow::Result;
use std::time::Instant;
use tokio::fs;

pub async fn run(args: CheckArgs) -> Result<()> {
    let start_time = Instant::now();
    
    // Load configuration
    let config = Config::load_from_file("codeguardian.toml").unwrap_or_else(|_| {
        eprintln!("Warning: No configuration file found, using defaults");
        Config::default()
    });

    // Initialize progress reporter (TTY-aware)
    let progress = ProgressReporter::new(!args.quiet && atty::is(atty::Stream::Stdout));

    // Initialize the Guardian engine
    let mut engine = GuardianEngine::new(config, progress).await?;

    // Determine files to scan
    let files_to_scan = if let Some(diff_spec) = &args.diff {
        engine.get_diff_files(diff_spec).await?
    } else if args.only_changed {
        engine.get_staged_files().await?
    } else {
        engine.get_all_files(&args.paths).await?
    };

    if files_to_scan.is_empty() {
        println!("No files to analyze");
        return Ok(());
    }

    // Run analysis
    let mut results = engine.analyze_files(&files_to_scan, args.parallel).await?;
    
    // Sort findings deterministically
    results.sort_findings();
    
    // Update timing
    results.summary.scan_duration_ms = start_time.elapsed().as_millis() as u64;

    // Save JSON results (source of truth)
    let json_output = serde_json::to_string_pretty(&results)?;
    fs::write(&args.out, json_output).await?;
    
    if !args.quiet {
        println!("Results saved to: {}", args.out.display());
    }

    // Emit markdown report if requested
    if let Some(md_path) = &args.emit_md {
        let markdown = crate::report::generate_markdown(&results)?;
        fs::write(md_path, markdown).await?;
        
        if !args.quiet {
            println!("Markdown report saved to: {}", md_path.display());
        }
    }

    // Emit GitHub issue if requested
    if args.emit_gh {
        if let Some(repo) = &args.repo {
            crate::github::create_or_update_issue(
                &results,
                repo,
                &args.gh_mode,
                &args.labels,
                false, // dry_run
            ).await?;
        } else {
            eprintln!("Warning: --emit-gh specified but no --repo provided");
        }
    }

    // Print summary to stdout if not quiet
    if !args.quiet {
        print_summary(&results);
    }

    // Determine exit code
    if args.fail_on_issues && results.has_issues() {
        std::process::exit(2);
    }

    Ok(())
}

fn print_summary(results: &AnalysisResults) {
    println!("\n📊 Analysis Summary");
    println!("==================");
    println!("Files scanned: {}", results.summary.total_files_scanned);
    println!("Total findings: {}", results.summary.total_findings);
    println!("Duration: {}ms", results.summary.scan_duration_ms);
    
    if !results.summary.findings_by_severity.is_empty() {
        println!("\nFindings by severity:");
        for (severity, count) in &results.summary.findings_by_severity {
            let emoji = match severity {
                crate::types::Severity::Critical => "🔴",
                crate::types::Severity::High => "🟠", 
                crate::types::Severity::Medium => "🟡",
                crate::types::Severity::Low => "🔵",
                crate::types::Severity::Info => "ℹ️",
            };
            println!("  {} {}: {}", emoji, severity, count);
        }
    }

    if !results.summary.findings_by_analyzer.is_empty() {
        println!("\nFindings by analyzer:");
        for (analyzer, count) in &results.summary.findings_by_analyzer {
            println!("  {}: {}", analyzer, count);
        }
    }
}