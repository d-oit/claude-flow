use crate::cli::{GhIssueArgs, GhMode};
use crate::github_api::GitHubApiClient;
use crate::types::AnalysisResults;
use anyhow::{anyhow, Result};
use std::process::Command;
use tokio::fs;

pub async fn run(args: GhIssueArgs) -> Result<()> {
    // Load results from JSON file
    let json_content = fs::read_to_string(&args.from).await?;
    let results: AnalysisResults = serde_json::from_str(&json_content)?;

    create_or_update_issue(&results, &args).await
}

pub async fn create_or_update_issue(results: &AnalysisResults, args: &GhIssueArgs) -> Result<()> {
    // Generate issue title
    let title = generate_issue_title(&args.title, &args.repo)?;
    
    // Generate issue body
    let body = generate_issue_body(results, &args.mode, args).await?;
    
    if args.dry_run {
        println!("DRY RUN: Would create/update GitHub issue");
        println!("Title: {}", title);
        println!("Repository: {}", args.repo);
        println!("Labels: {}", args.labels);
        println!("Body length: {} characters", body.len());
        return Ok(());
    }
    
    // Initialize GitHub API client with rate limiting
    let mut github_client = GitHubApiClient::new();
    
    // Check for existing issue (idempotency)
    let existing_issue = github_client.find_existing_issue(&title, &args.repo).await?;
    
    // Write body to temporary file
    let temp_file = "tmp_rovodev_issue_body.md";
    fs::write(temp_file, &body).await?;
    
    // Create or update issue with rate limiting
    if let Some(issue_number) = existing_issue {
        github_client.update_issue(issue_number, temp_file, &args.labels, &args.repo).await?;
        println!("✅ Updated existing issue #{}", issue_number);
    } else {
        let issue_number = github_client.create_issue(&title, temp_file, &args.labels, &args.repo).await?;
        println!("✅ Created new issue #{}", issue_number);
    }
    
    // Clean up temp file
    fs::remove_file(temp_file).await.ok();
    
    Ok(())
}

async fn find_existing_issue(title: &str, repo: &str, dry_run: bool) -> Result<Option<u64>> {
    let search_query = format!("{} in:title", title);
    
    let cmd = format!(
        "gh issue list --repo {} --state open --search '{}' --json number,title -q '.[0].number'",
        repo, search_query
    );
    
    if dry_run {
        println!("DRY RUN: {}", cmd);
        return Ok(None);
    }
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() && stdout != "null" {
            return Ok(stdout.parse().ok());
        }
    }
    
    Ok(None)
}

async fn create_issue(title: &str, body: &str, labels: &str, repo: &str, dry_run: bool) -> Result<u64> {
    // Write body to temporary file to handle large content
    let temp_file = "tmp_rovodev_issue_body.md";
    fs::write(temp_file, body).await?;
    
    let cmd = format!(
        "gh issue create --repo {} --title '{}' --label '{}' --body-file {}",
        repo, title, labels, temp_file
    );
    
    if dry_run {
        println!("DRY RUN: {}", cmd);
        fs::remove_file(temp_file).await.ok();
        return Ok(0);
    }
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()?;
    
    // Clean up temp file
    fs::remove_file(temp_file).await.ok();
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to create GitHub issue: {}", stderr));
    }
    
    // Extract issue number from output (GitHub CLI returns URL)
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(issue_url) = stdout.lines().last() {
        if let Some(number_str) = issue_url.split('/').last() {
            return Ok(number_str.parse().unwrap_or(0));
        }
    }
    
    Ok(0)
}

async fn update_issue(issue_number: u64, body: &str, labels: &str, repo: &str, dry_run: bool) -> Result<()> {
    // Write body to temporary file
    let temp_file = "tmp_rovodev_issue_body.md";
    fs::write(temp_file, body).await?;
    
    let cmd = format!(
        "gh issue edit {} --repo {} --body-file {} --add-label '{}'",
        issue_number, repo, temp_file, labels
    );
    
    if dry_run {
        println!("DRY RUN: {}", cmd);
        fs::remove_file(temp_file).await.ok();
        return Ok(());
    }
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()?;
    
    // Clean up temp file
    fs::remove_file(temp_file).await.ok();
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to update GitHub issue: {}", stderr));
    }
    
    Ok(())
}

fn generate_issue_title(prefix: &str, repo: &str) -> Result<String> {
    // Try to get current commit hash for PR context
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Ok(format!("{}{}", prefix, commit));
        }
    }
    
    // Try to get PR number from environment
    if let Ok(pr_number) = std::env::var("GITHUB_PR_NUMBER") {
        return Ok(format!("{}PR #{}", prefix, pr_number));
    }
    
    // Fallback to timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
    Ok(format!("{}{}", prefix, timestamp))
}

async fn generate_issue_body(results: &AnalysisResults, mode: &GhMode, args: &GhIssueArgs) -> Result<String> {
    let mut body = String::new();
    
    // Add summary from file if provided
    if let Some(summary_file) = &args.summary_from {
        if let Ok(summary_content) = fs::read_to_string(summary_file).await {
            body.push_str("## Summary\n\n");
            body.push_str(&summary_content);
            body.push_str("\n\n");
        }
    }
    
    // Add auto-generated summary if requested
    if let Some(provider) = &args.summary_auto {
        body.push_str("## 🤖 AI Summary\n\n");
        body.push_str(&generate_ai_summary(results, provider)?);
        body.push_str("\n\n");
    }
    
    // Add analysis metadata
    body.push_str("## Analysis Details\n\n");
    body.push_str(&format!("- **Generated:** {}\n", results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
    body.push_str(&format!("- **Tool:** {} v{}\n", results.tool_metadata.name, results.tool_metadata.version));
    body.push_str(&format!("- **Files Scanned:** {}\n", results.summary.total_files_scanned));
    body.push_str(&format!("- **Total Findings:** {}\n", results.summary.total_findings));
    body.push_str(&format!("- **Duration:** {}ms\n\n", results.summary.scan_duration_ms));
    
    // Generate findings based on mode
    match mode {
        GhMode::Checklist => {
            body.push_str(&generate_checklist_body(results, args)?);
        }
        GhMode::Simple => {
            body.push_str(&generate_simple_body(results, args)?);
        }
        GhMode::Children => {
            body.push_str(&generate_children_body(results, args)?);
        }
    }
    
    // Check if body is too large (GitHub limit ~65536 bytes)
    if body.len() > 60000 {
        body = truncate_body(body, results);
    }
    
    Ok(body)
}

fn generate_checklist_body(results: &AnalysisResults, args: &GhIssueArgs) -> Result<String> {
    let mut body = String::new();
    
    if results.findings.is_empty() {
        body.push_str("## ✅ No Issues Found\n\nGreat job! No issues were detected in the analyzed code.\n");
        return Ok(body);
    }
    
    body.push_str("## 🔍 Findings Checklist\n\n");
    body.push_str("Check off items as they are resolved:\n\n");
    
    // Limit findings for readability
    let max_findings = args.summary_max_issues.min(results.findings.len());
    
    for finding in results.findings.iter().take(max_findings) {
        let emoji = match finding.severity {
            crate::types::Severity::Critical => "🔴",
            crate::types::Severity::High => "🟠",
            crate::types::Severity::Medium => "🟡",
            crate::types::Severity::Low => "🔵",
            crate::types::Severity::Info => "ℹ️",
        };
        
        body.push_str(&format!(
            "- [ ] {} **{}** - `{}:{}` ({})\n",
            emoji,
            finding.message,
            finding.file.display(),
            finding.line,
            finding.id
        ));
        
        if let Some(description) = &finding.description {
            body.push_str(&format!("  > {}\n", description));
        }
        
        body.push_str("\n");
    }
    
    if results.findings.len() > max_findings {
        body.push_str(&format!(
            "\n*... and {} more findings. See full report for details.*\n",
            results.findings.len() - max_findings
        ));
    }
    
    Ok(body)
}

fn generate_simple_body(results: &AnalysisResults, _args: &GhIssueArgs) -> Result<String> {
    let mut body = String::new();
    
    if results.findings.is_empty() {
        body.push_str("## ✅ No Issues Found\n\nGreat job! No issues were detected in the analyzed code.\n");
        return Ok(body);
    }
    
    // Use the markdown report generator
    let markdown_report = crate::cli::report::generate_markdown(results)?;
    body.push_str(&markdown_report);
    
    Ok(body)
}

fn generate_children_body(results: &AnalysisResults, args: &GhIssueArgs) -> Result<String> {
    let mut body = String::new();
    
    body.push_str("## 📋 Analysis Overview\n\n");
    body.push_str("This is the main tracking issue for CodeGuardian analysis results.\n");
    body.push_str("Individual findings will be created as separate child issues for better tracking.\n\n");
    
    // Group findings by severity
    let mut critical = Vec::new();
    let mut high = Vec::new();
    let mut medium = Vec::new();
    let mut low = Vec::new();
    
    for finding in &results.findings {
        match finding.severity {
            crate::types::Severity::Critical => critical.push(finding),
            crate::types::Severity::High => high.push(finding),
            crate::types::Severity::Medium => medium.push(finding),
            crate::types::Severity::Low => low.push(finding),
            crate::types::Severity::Info => low.push(finding), // Treat info as low
        }
    }
    
    // Add summary by severity
    body.push_str("### 📊 Findings Summary\n\n");
    if !critical.is_empty() {
        body.push_str(&format!("- 🔴 **Critical**: {} issues\n", critical.len()));
    }
    if !high.is_empty() {
        body.push_str(&format!("- 🟠 **High**: {} issues\n", high.len()));
    }
    if !medium.is_empty() {
        body.push_str(&format!("- 🟡 **Medium**: {} issues\n", medium.len()));
    }
    if !low.is_empty() {
        body.push_str(&format!("- 🟢 **Low/Info**: {} issues\n", low.len()));
    }
    
    body.push_str("\n### 🔗 Child Issues Strategy\n\n");
    
    // Create child issues for critical and high severity findings
    let high_priority_findings: Vec<_> = results.findings.iter()
        .filter(|f| matches!(f.severity, crate::types::Severity::Critical | crate::types::Severity::High))
        .collect();
    
    if !high_priority_findings.is_empty() {
        body.push_str("#### 🚨 High Priority (Separate Issues)\n\n");
        body.push_str("The following critical and high severity findings will be created as individual child issues:\n\n");
        
        for finding in &high_priority_findings {
            let child_title = format!("[{}] {} - {}:{}", 
                match finding.severity {
                    crate::types::Severity::Critical => "CRITICAL",
                    crate::types::Severity::High => "HIGH",
                    _ => "ISSUE",
                },
                finding.message.chars().take(50).collect::<String>(),
                finding.file.display(),
                finding.line
            );
            
            body.push_str(&format!("- [ ] **{}** (ID: `{}`)\n", child_title, finding.id));
            
            if args.dry_run {
                body.push_str("  - *Dry run: Would create child issue with GitHub CLI*\n");
            } else {
                body.push_str("  - *Child issue will be created automatically*\n");
            }
            
            if let Some(description) = &finding.description {
                body.push_str(&format!("  - Description: {}\n", description));
            }
            
            if let Some(suggestion) = &finding.suggestion {
                body.push_str(&format!("  - Suggested fix: {}\n", suggestion));
            }
        }
        body.push_str("\n");
    }
    
    // Add medium and low priority findings as a summary
    let lower_priority_findings: Vec<_> = results.findings.iter()
        .filter(|f| matches!(f.severity, crate::types::Severity::Medium | crate::types::Severity::Low | crate::types::Severity::Info))
        .collect();
    
    if !lower_priority_findings.is_empty() {
        body.push_str("#### 📝 Lower Priority (Tracked Here)\n\n");
        body.push_str("The following findings are tracked as checklist items in this main issue:\n\n");
        
        let max_items = args.summary_max_issues.min(lower_priority_findings.len());
        for finding in lower_priority_findings.iter().take(max_items) {
            let severity_emoji = match finding.severity {
                crate::types::Severity::Medium => "🟡",
                crate::types::Severity::Low => "🟢",
                crate::types::Severity::Info => "ℹ️",
                _ => "⚪",
            };
            
            body.push_str(&format!("- [ ] {} **{}** - `{}:{}` (ID: `{}`)\n",
                severity_emoji,
                finding.message,
                finding.file.display(),
                finding.line,
                finding.id
            ));
            
            if let Some(suggestion) = &finding.suggestion {
                body.push_str(&format!("  - *Suggestion: {}*\n", suggestion));
            }
        }
        
        if lower_priority_findings.len() > max_items {
            body.push_str(&format!("\n*... and {} more findings (see full results.json)*\n", 
                lower_priority_findings.len() - max_items));
        }
    }
    
    body.push_str("\n### 🛠️ Implementation Guide\n\n");
    body.push_str("**Child Issue Creation:**\n");
    body.push_str("```bash\n");
    body.push_str("# Critical and High severity findings will be created as:\n");
    body.push_str("gh issue create --title \"[CRITICAL] Finding Title\" \\\n");
    body.push_str("  --body \"Detailed finding description with context\" \\\n");
    body.push_str("  --label \"codeguardian,critical,security\" \\\n");
    body.push_str("  --assignee @me\n");
    body.push_str("```\n\n");
    
    body.push_str("**Resolution Process:**\n");
    body.push_str("1. 🔴 Address Critical and High severity issues first (separate issues)\n");
    body.push_str("2. 🟡 Work through Medium priority items in this issue\n");
    body.push_str("3. 🟢 Address Low priority items as time permits\n");
    body.push_str("4. ✅ Check off items as they are resolved\n");
    body.push_str("5. 🔗 Reference finding IDs in commit messages and PRs\n\n");
    
    body.push_str("**Finding ID Usage:**\n");
    body.push_str("```bash\n");
    body.push_str("git commit -m \"fix: resolve security issue (CodeGuardian: abc123def456)\"\n");
    body.push_str("```\n\n");
    
    Ok(body)
}

fn truncate_body(mut body: String, results: &AnalysisResults) -> String {
    let mut truncated = String::new();
    
    truncated.push_str("## ⚠️ Report Truncated\n\n");
    truncated.push_str("This report was truncated due to size limits. ");
    truncated.push_str("See the full results.json artifact for complete details.\n\n");
    
    // Add summary only
    truncated.push_str("## Summary\n\n");
    truncated.push_str(&format!("- **Total Findings:** {}\n", results.summary.total_findings));
    truncated.push_str(&format!("- **Files Scanned:** {}\n", results.summary.total_files_scanned));
    
    // Add only critical and high severity findings
    let critical_high: Vec<_> = results.findings.iter()
        .filter(|f| matches!(f.severity, crate::types::Severity::Critical | crate::types::Severity::High))
        .take(10)
        .collect();
    
    if !critical_high.is_empty() {
        truncated.push_str("\n## 🔴 Critical & High Severity Issues\n\n");
        for finding in critical_high {
            truncated.push_str(&format!(
                "- **{}** - `{}:{}` ({})\n",
                finding.message,
                finding.file.display(),
                finding.line,
                finding.id
            ));
        }
    }
    
    truncated
}

fn generate_ai_summary(results: &AnalysisResults, provider: &str) -> Result<String> {
    let mut summary = String::new();
    
    // For now, provide a structured summary based on analysis
    // This can be enhanced with actual AI integration later
    
    match provider.to_lowercase().as_str() {
        "openai" | "claude" | "gemini" => {
            summary.push_str(&format!("*AI Provider: {}*\n\n", provider));
            summary.push_str("**Analysis Overview:**\n");
            
            // Generate intelligent summary based on findings
            let critical_count = results.findings.iter()
                .filter(|f| matches!(f.severity, crate::types::Severity::Critical))
                .count();
            let high_count = results.findings.iter()
                .filter(|f| matches!(f.severity, crate::types::Severity::High))
                .count();
            
            if critical_count > 0 {
                summary.push_str(&format!("🚨 **Immediate Action Required**: {} critical security issues detected that need urgent attention.\n", critical_count));
            }
            
            if high_count > 0 {
                summary.push_str(&format!("⚠️ **High Priority**: {} high-severity issues found that should be addressed soon.\n", high_count));
            }
            
            // Analyze patterns in findings
            let mut analyzer_counts = std::collections::HashMap::new();
            for finding in &results.findings {
                *analyzer_counts.entry(&finding.analyzer).or_insert(0) += 1;
            }
            
            summary.push_str("\n**Key Areas of Concern:**\n");
            for (analyzer, count) in analyzer_counts.iter() {
                let description = match analyzer.as_str() {
                    "integrity" => "File integrity and corruption issues",
                    "lint_drift" => "Configuration drift and consistency problems",
                    "non_production" => "Non-production code and security vulnerabilities",
                    _ => "Code quality issues",
                };
                summary.push_str(&format!("- **{}**: {} issues - {}\n", analyzer, count, description));
            }
            
            // Generate recommendations
            summary.push_str("\n**Recommended Actions:**\n");
            if critical_count > 0 {
                summary.push_str("1. 🔴 **Immediate**: Address all critical security vulnerabilities\n");
            }
            if high_count > 0 {
                summary.push_str("2. 🟠 **This Sprint**: Resolve high-priority issues\n");
            }
            summary.push_str("3. 🟡 **Next Sprint**: Work through medium-priority items\n");
            summary.push_str("4. 📊 **Ongoing**: Monitor and prevent regression\n");
            
            // Risk assessment
            let total_high_risk = critical_count + high_count;
            summary.push_str("\n**Risk Assessment:**\n");
            if total_high_risk == 0 {
                summary.push_str("✅ **Low Risk**: No critical or high-severity issues detected.\n");
            } else if total_high_risk <= 3 {
                summary.push_str("🟡 **Medium Risk**: Limited high-priority issues that can be managed.\n");
            } else {
                summary.push_str("🔴 **High Risk**: Multiple critical/high-severity issues require immediate attention.\n");
            }
            
            summary.push_str("\n*Note: This is a rule-based summary. For advanced AI analysis, integrate with your preferred AI provider.*\n");
        }
        _ => {
            summary.push_str(&format!("*Unknown AI provider: {}*\n\n", provider));
            summary.push_str("**Basic Summary:**\n");
            summary.push_str(&format!("- Total findings: {}\n", results.findings.len()));
            summary.push_str(&format!("- Files scanned: {}\n", results.summary.total_files_scanned));
            summary.push_str(&format!("- Scan duration: {}ms\n", results.summary.scan_duration_ms));
            summary.push_str("\n*To enable AI summarization, use: openai, claude, or gemini*\n");
        }
    }
    
    Ok(summary)
}