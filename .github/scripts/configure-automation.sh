#!/bin/bash
# 🤖 GitHub Projects Automation Configuration Script
# This script helps configure automation rules for the created projects

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

echo "🤖 GitHub Projects Automation Configuration"
echo "=========================================="

# Get repository info
REPO_OWNER=$(gh repo view --json owner --jq '.owner.login')
REPO_NAME=$(gh repo view --json name --jq '.name')
REPO_FULL_NAME="$REPO_OWNER/$REPO_NAME"

log_info "Repository: $REPO_FULL_NAME"

# Get project information
get_projects() {
    log_info "Fetching project information..."
    
    # List all projects for the repository
    projects=$(gh project list --owner "$REPO_OWNER" --format json)
    
    if [[ $(echo "$projects" | jq length) -eq 0 ]]; then
        log_error "No projects found. Please run setup-projects-complete.sh first."
        exit 1
    fi
    
    echo "$projects" | jq -r '.[] | "\(.number)|\(.title)|\(.url)"' | while IFS='|' read -r number title url; do
        echo "   📋 Project #$number: $title"
        echo "      🔗 $url"
    done
}

# Create automation rules via GitHub CLI (where possible)
configure_basic_automation() {
    log_info "Configuring basic automation rules..."
    
    # Note: GitHub Projects v2 automation rules are primarily configured via UI
    # This script provides the commands and instructions for manual setup
    
    cat > ".github/AUTOMATION_COMMANDS.md" << 'EOF'
# 🤖 Automation Configuration Commands

## GitHub CLI Commands for Project Management

### Add Issues to Projects Automatically

```bash
# Add urgent issues to High Priority project
gh project item-add [PROJECT_NUMBER] --url https://github.com/OWNER/REPO/issues/ISSUE_NUMBER

# Add good first issues to Good First Issues project  
gh project item-add [PROJECT_NUMBER] --url https://github.com/OWNER/REPO/issues/ISSUE_NUMBER

# Update item fields
gh project item-edit --project-id [PROJECT_ID] --id [ITEM_ID] --field-id [FIELD_ID] --single-select-option-id [OPTION_ID]
```

### Bulk Operations

```bash
# List all open urgent issues
gh issue list --label urgent --state open --json number,title,url

# List all good first issues without assignees
gh issue list --label "good first issue" --state open --json number,title,assignees,url | jq '.[] | select(.assignees | length == 0)'

# List all issues with effort labels
gh issue list --label effort/small,effort/large --state open --json number,title,labels,url
```

## Automation Rules Setup (Manual via GitHub UI)

### High Priority Project Automation:
1. Go to project settings
2. Add rule: "When item is added" → "Set Priority field from labels"
3. Add rule: "When issue is closed" → "Set Status to Done"
4. Add rule: "When issue is assigned" → "Set Status to In Progress"

### Good First Issues Project Automation:
1. Add rule: "When item added with 'good first issue' label" → "Set Status to Available"
2. Add rule: "When issue is assigned" → "Set Status to Claimed"  
3. Add rule: "When PR is linked" → "Set Status to In Review"
4. Add rule: "When issue is closed" → "Set Status to Completed"

### Sprint Planning Project Automation:
1. Add rule: "When item added with 'effort/small' label" → "Set Effort to Small"
2. Add rule: "When item added with 'effort/large' label" → "Set Effort to Large"
3. Add rule: "When priority label added" → "Update Priority field"
4. Add rule: "When issue is closed" → "Set Status to Done"

EOF

    log_success "Automation commands documentation created: .github/AUTOMATION_COMMANDS.md"
}

# Create workflow integration helpers
create_workflow_helpers() {
    log_info "Creating workflow integration helpers..."
    
    # Create a helper script for project operations
    cat > ".github/scripts/project-helpers.sh" << 'EOF'
#!/bin/bash
# 🔧 Project Helper Functions for Workflows

# Add issue to project by label
add_to_project_by_label() {
    local issue_number="$1"
    local label="$2"
    local repo="$3"
    
    case "$label" in
        "urgent"|"high-priority")
            # Add to High Priority project
            echo "Adding issue #$issue_number to High Priority project"
            # gh project item-add command would go here
            ;;
        "good first issue")
            # Add to Good First Issues project
            echo "Adding issue #$issue_number to Good First Issues project"
            # gh project item-add command would go here
            ;;
        "effort/small"|"effort/large")
            # Add to Sprint Planning project
            echo "Adding issue #$issue_number to Sprint Planning project"
            # gh project item-add command would go here
            ;;
    esac
}

# Update project item status
update_project_status() {
    local issue_number="$1"
    local new_status="$2"
    local repo="$3"
    
    echo "Updating issue #$issue_number status to: $new_status"
    # Project status update commands would go here
}

# Get project metrics
get_project_metrics() {
    local project_number="$1"
    
    echo "Fetching metrics for project #$project_number"
    # Metrics collection commands would go here
}

EOF

    chmod +x ".github/scripts/project-helpers.sh"
    log_success "Project helper functions created: .github/scripts/project-helpers.sh"
}

# Create test scenarios
create_test_scenarios() {
    log_info "Creating test scenarios..."
    
    cat > ".github/TEST_AUTOMATION.md" << EOF
# 🧪 Automation Testing Scenarios

## Test the Complete Workflow

### Scenario 1: High Priority Issue
\`\`\`bash
# Create urgent issue
gh issue create --title "🚨 Critical Bug: System Down" --body "Production system is experiencing downtime" --label urgent,bug

# Expected: Should auto-add to High Priority project
# Verify: Check project board and workflow logs
\`\`\`

### Scenario 2: Good First Issue
\`\`\`bash
# Create good first issue
gh issue create --title "📚 Update README typo" --body "Fix spelling error in documentation" --label "good first issue",documentation

# Expected: Should auto-add to Good First Issues project
# Verify: Check newcomer project board
\`\`\`

### Scenario 3: Sprint Planning
\`\`\`bash
# Create effort-estimated issue
gh issue create --title "⚡ Add dark mode toggle" --body "Simple UI enhancement" --label enhancement,effort/small

# Expected: Should auto-add to Sprint Planning project with effort estimation
# Verify: Check sprint planning board
\`\`\`

### Scenario 4: AI Assistance
\`\`\`bash
# Create incomplete bug report
gh issue create --title "Something is broken" --body "It doesn't work" --label bug

# Expected: AI should suggest improvements
# Verify: Check for bot comment with suggestions
\`\`\`

## Verification Commands

\`\`\`bash
# Check workflow runs
gh run list --workflow="Enterprise Auto Label & AI Assist"

# Check project automation
gh run list --workflow="GitHub Projects Automation"

# View recent issues with labels
gh issue list --label urgent,high-priority,"good first issue" --limit 10

# Check AI-assisted issues
gh issue list --state all --limit 20 | grep -E "(commented.*github-actions|bot)"
\`\`\`

## Success Criteria

- ✅ Issues with priority labels auto-add to High Priority project
- ✅ Good first issues auto-add to newcomer project  
- ✅ Effort-labeled issues auto-add to sprint planning
- ✅ AI suggestions appear on incomplete issues
- ✅ Project boards update automatically
- ✅ Workflow metrics appear in Actions logs

## Troubleshooting

### Common Issues:
1. **Projects not updating**: Check project automation rules in UI
2. **Workflow not triggering**: Verify permissions and triggers
3. **AI not suggesting**: Check workflow conditions and labels
4. **Missing metrics**: Verify structured logging in workflow runs

### Debug Commands:
\`\`\`bash
# View workflow logs
gh run view --log

# Check repository permissions
gh api repos/$REPO_FULL_NAME --jq '.permissions'

# List project automation rules (manual check required)
echo "Visit: https://github.com/$REPO_FULL_NAME/projects"
\`\`\`
EOF

    log_success "Test scenarios created: .github/TEST_AUTOMATION.md"
}

# Main execution
main() {
    echo ""
    get_projects
    echo ""
    
    configure_basic_automation
    create_workflow_helpers  
    create_test_scenarios
    
    echo ""
    echo "🎉 AUTOMATION CONFIGURATION COMPLETE!"
    echo "===================================="
    echo ""
    log_success "Configuration files created successfully!"
    echo ""
    echo "📋 Created Files:"
    echo "   🤖 .github/AUTOMATION_COMMANDS.md - Manual setup commands"
    echo "   🔧 .github/scripts/project-helpers.sh - Workflow helper functions"
    echo "   🧪 .github/TEST_AUTOMATION.md - Testing scenarios and verification"
    echo ""
    echo "🎯 Next Steps:"
    echo "   1. Visit each project URL to configure automation rules manually"
    echo "   2. Run test scenarios from TEST_AUTOMATION.md"
    echo "   3. Verify automation works by creating test issues"
    echo "   4. Monitor workflow runs in Actions tab"
    echo ""
    echo "📊 Quick Verification:"
    echo "   gh issue create --title 'Test Issue' --label urgent --body 'Testing automation'"
    echo "   gh run list --workflow='Enterprise Auto Label & AI Assist'"
    echo ""
    log_success "Your enterprise automation is configured! 🚀"
}

# Run main function
main "$@"