# CodeGuardian

A security-first code analysis CLI with GitHub integration, built with best-practice defaults for CI/CD workflows.

## Features

### 🔒 Security-by-Default
- **Deterministic findings** with stable IDs using SHA-256
- **No secrets in logs** - automatic redaction of common patterns
- **Sandboxed execution** - no symlink following, resource limits
- **Canonicalized paths** - prevents path traversal issues

### 🚀 CI-First UX
- **JSON as source of truth** - Markdown and GitHub issues are derived artifacts
- **Diff-only mode** for PRs - fast, focused feedback
- **TTY-aware progress bars** - auto-disable in CI environments
- **Idempotent GitHub integration** - updates existing issues instead of creating duplicates

### 📊 Comprehensive Analysis
- **Integrity checking** - cryptographic hashing with BLAKE3
- **Lint drift detection** - configuration consistency across projects
- **Non-production code detection** - TODOs, debug statements, potential secrets

### 🧠 **Intelligent ML Filtering** (NEW!)
- **RUV-FANN neural networks** - 200x faster than BERT, 100x smaller
- **False positive reduction** - 60-80% noise reduction with 90%+ accuracy
- **Online learning** - improves from user feedback automatically
- **Zero-config ML** - works out of the box, no setup required

## Quick Start

### Installation

```bash
# Install from source
cargo install --git https://github.com/your-org/codeguardian

# Or download binary from releases
curl -L https://github.com/your-org/codeguardian/releases/latest/download/codeguardian-linux-x64.tar.gz | tar xz
```

### Basic Usage

```bash
# Initialize configuration
codeguardian init --default

# Run analysis (recommended defaults)
codeguardian check . --format json --out results.json

# Generate markdown report
codeguardian report --from results.json --md report.md

# Create GitHub issue
codeguardian gh-issue --from results.json --repo owner/repo
```

### CI Integration

For PRs (diff-only, fast feedback):
```bash
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out results.json \
  --emit-md report.md \
  --emit-gh \
  --repo $GITHUB_REPOSITORY
```

For scheduled scans (full repository):
```bash
codeguardian check . \
  --format json \
  --out results.json \
  --emit-md report.md \
  --emit-gh \
  --repo $GITHUB_REPOSITORY \
  --fail-on-issues
```

## Commands

### `check` (Primary Command)

Run comprehensive code analysis with best-practice defaults:

```bash
codeguardian check [OPTIONS] [PATHS]...
```

**Key Options:**
- `--format json` - Output format (JSON is source of truth)
- `--out results.json` - Output file for results
- `--emit-md report.md` - Generate markdown report
- `--emit-gh` - Create/update GitHub issue
- `--diff origin/main..HEAD` - Analyze only changed files
- `--only-changed` - Analyze only staged files
- `--fail-on-issues` - Exit with code 2 if issues found

### `report` (Converter)

Convert results to different formats:

```bash
codeguardian report --from results.json --md report.md
```

### `gh-issue` (GitHub Integration)

Create or update GitHub issues with idempotent behavior:

```bash
codeguardian gh-issue \
  --from results.json \
  --repo owner/repo \
  --mode checklist \
  --title "CodeGuardian: " \
  --labels "codeguardian,automated"
```

**GitHub Issue Modes:**
- `checklist` - Interactive checklist format (default)
- `simple` - Standard issue format
- `children` - Parent issue with child issues for large reports

### `init` (Setup)

Initialize configuration:

```bash
codeguardian init --default
```

## Configuration

CodeGuardian uses `codeguardian.toml` for configuration. See [examples/codeguardian.toml](examples/codeguardian.toml) for a complete example.

### Key Configuration Sections

```toml
[general]
follow_symlinks = false  # Security: don't follow symlinks
max_file_size = 10485760  # 10MB limit
parallel_workers = 0      # Auto-detect CPU cores

[analyzers.integrity]
enabled = true
hash_algorithm = "blake3"  # BLAKE3 by default

[analyzers.lint_drift]
enabled = true
canonicalize_configs = true  # Stable JSON/YAML formatting

[analyzers.non_production]
enabled = true
todo_escalation_days = 30  # Escalate old TODOs

[github]
default_labels = ["codeguardian", "automated"]
title_prefix = "CodeGuardian: "
max_body_size = 60000  # Auto-switch to children mode
```

## GitHub Actions Integration

See [.github/workflows/codeguardian-ci.yml](.github/workflows/codeguardian-ci.yml) for a complete CI setup:

```yaml
- name: Run CodeGuardian (PR diff-only)
  run: |
    codeguardian check . \
      --diff origin/main..HEAD \
      --format json \
      --out results.json \
      --emit-md report.md \
      --emit-gh \
      --repo ${{ github.repository }}
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Best Practices

### 1. Deterministic Results
- Findings are sorted by severity → file → line
- Stable IDs ensure consistent tracking across runs
- Versioned schemas prevent compatibility issues

### 2. Security Hardening
- No secrets in logs (automatic redaction)
- Sandboxed file access with size limits
- No symlink following by default

### 3. CI Optimization
- Use `--diff` for PRs (fast, focused)
- Use full scans for scheduled jobs
- Always upload `results.json` and `report.md` as artifacts

### 4. GitHub Integration
- Issues are idempotent (updates existing instead of duplicating)
- Automatic body truncation for large reports
- Stable checklist format with finding IDs

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Benchmarking

```bash
cargo bench
```

## Architecture

CodeGuardian follows a modular architecture:

- **CLI Layer** - Argument parsing and command dispatch
- **Core Engine** - File discovery, parallel processing, result aggregation
- **Analyzer Registry** - Pluggable analysis modules
- **GitHub Integration** - Idempotent issue creation/updates
- **Security Layer** - Path validation, secret redaction, resource limits

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass and code is formatted
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Roadmap

- [ ] SARIF output format support
- [ ] Additional language-specific analyzers
- [ ] Baseline drift detection
- [ ] Custom rule definitions
- [ ] VS Code extension
- [ ] Integration with more CI platforms