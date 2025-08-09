#!/bin/bash
# 📊 GitHub Projects Setup Script
# Run this script to automatically create and configure GitHub Projects

set -e

echo "🚀 Setting up GitHub Projects for Enterprise Workflow Monitoring..."

# Check if GitHub CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI is not installed. Please install it first:"
    echo "   https://cli.github.com/"
    exit 1
fi

# Check if user is authenticated
if ! gh auth status &> /dev/null; then
    echo "🔐 Please authenticate with GitHub CLI first:"
    echo "   gh auth login"
    exit 1
fi

REPO="d-oit/claude-flow"

echo "📋 Creating GitHub Projects for repository: $REPO"

# Create High Priority Project
echo "🚨 Creating High Priority Issues project..."
HIGH_PRIORITY_URL=$(gh project create \
    --title "🚨 High Priority Issues" \
    --body "Automated tracking of urgent and high-priority issues from the enterprise workflow" \
    --format json | jq -r '.url')

echo "✅ High Priority Project created: $HIGH_PRIORITY_URL"

# Create Good First Issues Project  
echo "🌟 Creating Good First Issues project..."
GOOD_FIRST_URL=$(gh project create \
    --title "🌟 Good First Issues" \
    --body "New contributor onboarding pipeline with automated issue assignment" \
    --format json | jq -r '.url')

echo "✅ Good First Issues Project created: $GOOD_FIRST_URL"

# Create Sprint Planning Project
echo "📈 Creating Sprint Planning project..."
SPRINT_PLANNING_URL=$(gh project create \
    --title "📈 Sprint Planning" \
    --body "Sprint planning board with automated effort estimation and priority tracking" \
    --format json | jq -r '.url')

echo "✅ Sprint Planning Project created: $SPRINT_PLANNING_URL"

echo ""
echo "🎯 Projects Created Successfully!"
echo ""
echo "📌 Next Steps:"
echo "1. Visit each project URL to configure columns and automation rules"
echo "2. Update the project IDs in .github/workflows/project-automation.yml"
echo "3. Configure project automation rules as described in .github/PROJECT_SETUP.md"
echo ""
echo "🔗 Project URLs:"
echo "   🚨 High Priority: $HIGH_PRIORITY_URL"
echo "   🌟 Good First Issues: $GOOD_FIRST_URL"  
echo "   📈 Sprint Planning: $SPRINT_PLANNING_URL"
echo ""
echo "📊 Quick Access Links:"
echo "   All Projects: https://github.com/$REPO/projects"
echo "   Repository Insights: https://github.com/$REPO/pulse"
echo "   Workflow Runs: https://github.com/$REPO/actions"
echo ""
echo "✅ Setup complete! Your enterprise monitoring is ready to use."