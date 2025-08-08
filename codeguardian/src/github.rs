use crate::cli::GhMode;
use crate::types::AnalysisResults;
use anyhow::Result;

pub async fn create_or_update_issue(
    results: &AnalysisResults,
    repo: &str,
    mode: &GhMode,
    labels: &str,
    dry_run: bool,
) -> Result<()> {
    // This is a convenience wrapper that creates GhIssueArgs and calls the gh_issue module
    let args = crate::cli::GhIssueArgs {
        from: "results.json".into(),
        repo: repo.to_string(),
        mode: mode.clone(),
        title: "CodeGuardian: ".to_string(),
        labels: labels.to_string(),
        summary_from: None,
        summary_auto: None,
        summary_max_chars: 800,
        summary_max_issues: 10,
        dry_run,
    };
    
    crate::cli::gh_issue::create_or_update_issue(results, &args).await
}