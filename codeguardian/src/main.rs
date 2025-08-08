use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod core;
mod types;
mod analyzers;
mod github;
mod utils;
mod report;

use cli::*;

#[derive(Parser)]
#[command(name = "codeguardian")]
#[command(about = "A security-first code analysis CLI with GitHub integration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run code analysis (primary command)
    Check(CheckArgs),
    /// Convert results to different formats
    Report(ReportArgs),
    /// Create or update GitHub issues
    GhIssue(GhIssueArgs),
    /// Initialize configuration
    Init(InitArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => cli::check::run(args).await,
        Commands::Report(args) => cli::report::run(args).await,
        Commands::GhIssue(args) => cli::gh_issue::run(args).await,
        Commands::Init(args) => cli::init::run(args).await,
    }
}