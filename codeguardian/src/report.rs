use crate::types::AnalysisResults;
use anyhow::Result;

pub fn generate_markdown(results: &AnalysisResults) -> Result<String> {
    crate::cli::report::generate_markdown(results)
}