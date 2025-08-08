use crate::cli::ReportArgs;
use crate::types::AnalysisResults;
use anyhow::Result;
use std::collections::HashMap;
use tokio::fs;

pub async fn run(args: ReportArgs) -> Result<()> {
    // Load results from JSON file
    let json_content = fs::read_to_string(&args.from).await?;
    let results: AnalysisResults = serde_json::from_str(&json_content)?;

    // Generate report based on format
    let report_content = match args.format {
        crate::cli::ReportFormat::Markdown => generate_markdown(&results)?,
        crate::cli::ReportFormat::Html => generate_html(&results)?,
        crate::cli::ReportFormat::Text => generate_text(&results)?,
    };

    // Output to file or stdout
    if let Some(output_path) = &args.md {
        fs::write(output_path, report_content).await?;
        println!("Report saved to: {}", output_path.display());
    } else {
        print!("{}", report_content);
    }

    Ok(())
}

pub fn generate_markdown(results: &AnalysisResults) -> Result<String> {
    let mut md = String::new();
    
    // Header with metadata
    md.push_str(&format!("# CodeGuardian Analysis Report\n\n"));
    md.push_str(&format!("**Generated:** {}\n", results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
    md.push_str(&format!("**Tool Version:** {} v{}\n", results.tool_metadata.name, results.tool_metadata.version));
    md.push_str(&format!("**Schema Version:** {}\n", results.schema_version));
    md.push_str(&format!("**Config Hash:** `{}`\n\n", results.config_hash[..8].to_string()));

    // Summary section
    md.push_str("## 📊 Summary\n\n");
    md.push_str(&format!("- **Files Scanned:** {}\n", results.summary.total_files_scanned));
    md.push_str(&format!("- **Total Findings:** {}\n", results.summary.total_findings));
    md.push_str(&format!("- **Scan Duration:** {}ms\n\n", results.summary.scan_duration_ms));

    // Findings by severity
    if !results.summary.findings_by_severity.is_empty() {
        md.push_str("### Findings by Severity\n\n");
        md.push_str("| Severity | Count | Emoji |\n");
        md.push_str("|----------|-------|-------|\n");
        
        let severity_order = [
            crate::types::Severity::Critical,
            crate::types::Severity::High,
            crate::types::Severity::Medium,
            crate::types::Severity::Low,
            crate::types::Severity::Info,
        ];
        
        for severity in &severity_order {
            if let Some(count) = results.summary.findings_by_severity.get(severity) {
                let emoji = match severity {
                    crate::types::Severity::Critical => "🔴",
                    crate::types::Severity::High => "🟠",
                    crate::types::Severity::Medium => "🟡",
                    crate::types::Severity::Low => "🔵",
                    crate::types::Severity::Info => "ℹ️",
                };
                md.push_str(&format!("| {} | {} | {} |\n", severity, count, emoji));
            }
        }
        md.push_str("\n");
    }

    // Findings by analyzer
    if !results.summary.findings_by_analyzer.is_empty() {
        md.push_str("### Findings by Analyzer\n\n");
        md.push_str("| Analyzer | Count |\n");
        md.push_str("|----------|-------|\n");
        
        let mut analyzers: Vec<_> = results.summary.findings_by_analyzer.iter().collect();
        analyzers.sort_by_key(|(name, _)| *name);
        
        for (analyzer, count) in analyzers {
            md.push_str(&format!("| {} | {} |\n", analyzer, count));
        }
        md.push_str("\n");
    }

    // Detailed findings
    if !results.findings.is_empty() {
        md.push_str("## 🔍 Detailed Findings\n\n");
        
        // Group findings by severity
        let mut findings_by_severity: HashMap<_, Vec<_>> = HashMap::new();
        for finding in &results.findings {
            findings_by_severity.entry(&finding.severity).or_default().push(finding);
        }
        
        let severity_order = [
            crate::types::Severity::Critical,
            crate::types::Severity::High,
            crate::types::Severity::Medium,
            crate::types::Severity::Low,
            crate::types::Severity::Info,
        ];
        
        for severity in &severity_order {
            if let Some(findings) = findings_by_severity.get(severity) {
                let emoji = match severity {
                    crate::types::Severity::Critical => "🔴",
                    crate::types::Severity::High => "🟠",
                    crate::types::Severity::Medium => "🟡",
                    crate::types::Severity::Low => "🔵",
                    crate::types::Severity::Info => "ℹ️",
                };
                
                md.push_str(&format!("### {} {} Issues\n\n", emoji, severity));
                
                for finding in findings {
                    md.push_str(&format!("#### {}\n\n", finding.message));
                    md.push_str(&format!("- **ID:** `{}`\n", finding.id));
                    md.push_str(&format!("- **File:** `{}`\n", finding.file.display()));
                    md.push_str(&format!("- **Line:** {}\n", finding.line));
                    if let Some(column) = finding.column {
                        md.push_str(&format!("- **Column:** {}\n", column));
                    }
                    md.push_str(&format!("- **Analyzer:** {}\n", finding.analyzer));
                    md.push_str(&format!("- **Rule:** {}\n", finding.rule));
                    
                    if let Some(description) = &finding.description {
                        md.push_str(&format!("- **Description:** {}\n", description));
                    }
                    
                    if let Some(suggestion) = &finding.suggestion {
                        md.push_str(&format!("- **Suggestion:** {}\n", suggestion));
                    }
                    
                    md.push_str("\n");
                }
            }
        }
    } else {
        md.push_str("## ✅ No Issues Found\n\nGreat job! No issues were detected in the analyzed code.\n\n");
    }

    // Footer
    md.push_str("---\n");
    md.push_str("*Generated by CodeGuardian - Security-first code analysis*\n");

    Ok(md)
}

pub fn generate_html(results: &AnalysisResults) -> Result<String> {
    // Basic HTML wrapper around markdown content
    let markdown_content = generate_markdown(results)?;
    
    Ok(format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>CodeGuardian Analysis Report</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; }}
        .summary {{ background: #f6f8fa; padding: 20px; border-radius: 6px; margin: 20px 0; }}
        .finding {{ border-left: 4px solid #0969da; padding: 16px; margin: 16px 0; background: #f6f8fa; }}
        .critical {{ border-left-color: #d1242f; }}
        .high {{ border-left-color: #fb8500; }}
        .medium {{ border-left-color: #f1c40f; }}
        .low {{ border-left-color: #0969da; }}
        .info {{ border-left-color: #6c757d; }}
        code {{ background: #f3f4f6; padding: 2px 4px; border-radius: 3px; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #d0d7de; padding: 8px 12px; text-align: left; }}
        th {{ background: #f6f8fa; }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        markdown_to_html(&markdown_content)
    ))
}

pub fn generate_text(results: &AnalysisResults) -> Result<String> {
    let mut text = String::new();
    
    text.push_str("CODEGUARDIAN ANALYSIS REPORT\n");
    text.push_str("============================\n\n");
    
    text.push_str(&format!("Generated: {}\n", results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
    text.push_str(&format!("Tool: {} v{}\n", results.tool_metadata.name, results.tool_metadata.version));
    text.push_str(&format!("Schema: {}\n", results.schema_version));
    text.push_str(&format!("Config Hash: {}\n\n", &results.config_hash[..8]));
    
    text.push_str("SUMMARY\n");
    text.push_str("-------\n");
    text.push_str(&format!("Files Scanned: {}\n", results.summary.total_files_scanned));
    text.push_str(&format!("Total Findings: {}\n", results.summary.total_findings));
    text.push_str(&format!("Scan Duration: {}ms\n\n", results.summary.scan_duration_ms));
    
    if !results.findings.is_empty() {
        text.push_str("FINDINGS\n");
        text.push_str("--------\n\n");
        
        for (i, finding) in results.findings.iter().enumerate() {
            text.push_str(&format!("{}. [{}] {}\n", i + 1, finding.severity.to_string().to_uppercase(), finding.message));
            text.push_str(&format!("   File: {}\n", finding.file.display()));
            text.push_str(&format!("   Line: {}\n", finding.line));
            text.push_str(&format!("   Analyzer: {} ({})\n", finding.analyzer, finding.rule));
            text.push_str(&format!("   ID: {}\n\n", finding.id));
        }
    } else {
        text.push_str("No issues found.\n\n");
    }
    
    Ok(text)
}

// Simple markdown to HTML conversion (basic implementation)
fn markdown_to_html(markdown: &str) -> String {
    markdown
        .replace("# ", "<h1>")
        .replace("\n", "</h1>\n")
        .replace("## ", "<h2>")
        .replace("### ", "<h3>")
        .replace("#### ", "<h4>")
        .replace("**", "<strong>")
        .replace("*", "</strong>")
        .replace("`", "<code>")
}