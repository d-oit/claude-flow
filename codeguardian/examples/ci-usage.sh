#!/bin/bash
# CodeGuardian CI Usage Examples
# This script demonstrates best-practice usage patterns

set -euo pipefail

echo "🚀 CodeGuardian CI Examples"
echo "=========================="

# Example 1: PR workflow (diff-only, fast feedback)
echo "📝 Example 1: PR Analysis (diff-only)"
echo "--------------------------------------"
cat << 'EOF'
# In your GitHub Actions workflow:
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out results.json \
  --emit-md report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist \
  --labels "codeguardian,automated,pr-${{ github.event.number }}"
EOF

echo ""

# Example 2: Full repository scan
echo "🔍 Example 2: Full Repository Scan"
echo "-----------------------------------"
cat << 'EOF'
# For scheduled jobs or main branch pushes:
codeguardian check . \
  --format json \
  --out results.json \
  --emit-md report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist \
  --labels "codeguardian,automated,full-scan" \
  --fail-on-issues
EOF

echo ""

# Example 3: Local development
echo "💻 Example 3: Local Development"
echo "--------------------------------"
cat << 'EOF'
# Quick local check:
codeguardian check . --format json --out results.json

# Generate report:
codeguardian report --from results.json --md report.md

# Check only staged files:
codeguardian check . --only-changed --format json --out results.json
EOF

echo ""

# Example 4: Idempotent GitHub issue creation
echo "🔄 Example 4: Idempotent GitHub Issues"
echo "--------------------------------------"
cat << 'EOF'
# The following commands are idempotent - they will update existing issues
# instead of creating duplicates:

ISSUE_TITLE="CodeGuardian: $(git rev-parse --short HEAD)"
EXISTING=$(gh issue list --state open --search "$ISSUE_TITLE in:title" --json number -q '.[0].number' || true)

if [ -n "$EXISTING" ]; then
  echo "Updating existing issue #$EXISTING"
  gh issue edit "$EXISTING" --body-file report.md --add-label "codeguardian,automated"
else
  echo "Creating new issue"
  gh issue create --title "$ISSUE_TITLE" --label "codeguardian,automated" --body-file report.md
fi
EOF

echo ""

# Example 5: Configuration templates
echo "⚙️  Example 5: Configuration Templates"
echo "--------------------------------------"
cat << 'EOF'
# Initialize with different templates:
codeguardian init --default                    # Standard configuration
codeguardian init --template minimal           # Minimal setup
codeguardian init --template security          # Security-focused
codeguardian init --template ci                # CI-optimized
EOF

echo ""

# Example 6: Advanced usage patterns
echo "🔧 Example 6: Advanced Patterns"
echo "--------------------------------"
cat << 'EOF'
# Baseline management:
codeguardian check . --format json --out baseline.json
git add baseline.json && git commit -m "chore: update baseline"

# Custom GitHub issue with summary:
echo "High-level summary of changes" > summary.md
codeguardian gh-issue \
  --from results.json \
  --repo owner/repo \
  --summary-from summary.md \
  --mode checklist

# Dry run mode (print commands without executing):
codeguardian gh-issue \
  --from results.json \
  --repo owner/repo \
  --dry-run
EOF

echo ""
echo "✅ For complete documentation, see README.md"
echo "📁 For configuration examples, see examples/codeguardian.toml"
echo "🔧 For CI setup, see .github/workflows/codeguardian-ci.yml"