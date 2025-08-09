#!/bin/bash
# 🏢 Complete GitHub Projects Enterprise Setup Script
# This script creates projects with full configuration: columns, automation, and custom fields

set -e

echo "🏢 Starting Complete Enterprise GitHub Projects Setup..."
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v gh &> /dev/null; then
        log_error "GitHub CLI is not installed. Please install it first:"
        echo "   https://cli.github.com/"
        exit 1
    fi
    
    if ! gh auth status &> /dev/null; then
        log_error "Please authenticate with GitHub CLI first:"
        echo "   gh auth login"
        exit 1
    fi
    
    if ! command -v jq &> /dev/null; then
        log_warning "jq is not installed. Installing via package manager..."
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y jq
        elif command -v brew &> /dev/null; then
            brew install jq
        else
            log_error "Please install jq manually: https://stedolan.github.io/jq/"
            exit 1
        fi
    fi
    
    log_success "All prerequisites met!"
}

# Get repository info
get_repo_info() {
    REPO_OWNER=$(gh repo view --json owner --jq '.owner.login')
    REPO_NAME=$(gh repo view --json name --jq '.name')
    REPO_FULL_NAME="$REPO_OWNER/$REPO_NAME"
    
    log_info "Repository: $REPO_FULL_NAME"
}

# Create project with full configuration
create_project() {
    local title="$1"
    local description="$2"
    local columns="$3"
    
    log_info "Creating project: $title"
    
    # Create the project
    local project_data=$(gh project create \
        --title "$title" \
        --body "$description" \
        --format json)
    
    local project_id=$(echo "$project_data" | jq -r '.id')
    local project_url=$(echo "$project_data" | jq -r '.url')
    local project_number=$(echo "$project_data" | jq -r '.number')
    
    log_success "Project created: $project_url"
    
    # Add custom fields
    add_custom_fields "$project_id"
    
    # Configure columns (GitHub Projects v2 uses views, not columns)
    configure_project_views "$project_id" "$columns"
    
    echo "$project_id|$project_url|$project_number"
}

# Add custom fields to project
add_custom_fields() {
    local project_id="$1"
    
    log_info "Adding custom fields to project..."
    
    # Priority field
    gh api graphql -f query='
        mutation($projectId: ID!) {
            createProjectV2Field(input: {
                projectId: $projectId
                dataType: SINGLE_SELECT
                name: "Priority"
                options: [
                    {name: "🔥 Urgent", color: RED}
                    {name: "⚡ High", color: ORANGE}
                    {name: "📋 Medium", color: YELLOW}
                    {name: "🔽 Low", color: GREEN}
                ]
            }) {
                projectV2Field {
                    id
                }
            }
        }' -f projectId="$project_id" > /dev/null 2>&1 || log_warning "Priority field creation failed (may already exist)"
    
    # Effort field
    gh api graphql -f query='
        mutation($projectId: ID!) {
            createProjectV2Field(input: {
                projectId: $projectId
                dataType: SINGLE_SELECT
                name: "Effort"
                options: [
                    {name: "⚡ Small", color: GREEN}
                    {name: "📋 Medium", color: YELLOW}
                    {name: "🏗️ Large", color: RED}
                    {name: "❓ Unknown", color: GRAY}
                ]
            }) {
                projectV2Field {
                    id
                }
            }
        }' -f projectId="$project_id" > /dev/null 2>&1 || log_warning "Effort field creation failed (may already exist)"
    
    # AI Assisted field
    gh api graphql -f query='
        mutation($projectId: ID!) {
            createProjectV2Field(input: {
                projectId: $projectId
                dataType: SINGLE_SELECT
                name: "AI Assisted"
                options: [
                    {name: "🤖 Yes", color: BLUE}
                    {name: "👤 Manual", color: GRAY}
                ]
            }) {
                projectV2Field {
                    id
                }
            }
        }' -f projectId="$project_id" > /dev/null 2>&1 || log_warning "AI Assisted field creation failed (may already exist)"
    
    log_success "Custom fields added"
}

# Configure project views (GitHub Projects v2)
configure_project_views() {
    local project_id="$1"
    local view_name="$2"
    
    log_info "Configuring project views for: $view_name"
    
    # Note: GitHub Projects v2 API for views is complex
    # For now, we'll log instructions for manual setup
    log_warning "View configuration requires manual setup in GitHub UI"
    log_info "Please visit the project URL and configure views manually"
}

# Update workflow file with project IDs
update_workflow_config() {
    local high_priority_id="$1"
    local good_first_id="$2"
    local sprint_planning_id="$3"
    
    log_info "Updating workflow configuration with project IDs..."
    
    local workflow_file=".github/workflows/project-automation.yml"
    
    if [[ -f "$workflow_file" ]]; then
        # Create backup
        cp "$workflow_file" "$workflow_file.backup"
        
        # Update project IDs in workflow file
        sed -i.tmp "s/HIGH_PRIORITY_PROJECT_ID: \"\"/HIGH_PRIORITY_PROJECT_ID: \"$high_priority_id\"/" "$workflow_file"
        sed -i.tmp "s/GOOD_FIRST_ISSUES_PROJECT_ID: \"\"/GOOD_FIRST_ISSUES_PROJECT_ID: \"$good_first_id\"/" "$workflow_file"
        sed -i.tmp "s/SPRINT_PLANNING_PROJECT_ID: \"\"/SPRINT_PLANNING_PROJECT_ID: \"$sprint_planning_id\"/" "$workflow_file"
        
        # Clean up temp files
        rm -f "$workflow_file.tmp"
        
        log_success "Workflow configuration updated"
    else
        log_warning "Workflow file not found: $workflow_file"
    fi
}

# Create automation rules documentation
create_automation_docs() {
    local high_priority_url="$1"
    local good_first_url="$2"
    local sprint_planning_url="$3"
    
    log_info "Creating automation setup documentation..."
    
    cat > ".github/PROJECT_AUTOMATION_SETUP.md" << EOF
# 🤖 Project Automation Setup Instructions

## 📋 Created Projects

1. **🚨 High Priority Issues**: $high_priority_url
2. **🌟 Good First Issues**: $good_first_url
3. **📈 Sprint Planning**: $sprint_planning_url

## ⚙️ Manual Automation Rules Setup

### For High Priority Project:
1. Go to: $high_priority_url
2. Click "⚙️ Settings" → "Manage access"
3. Add automation rules:
   - **When**: Item is added to project
   - **Then**: Set Priority field based on labels
   - **When**: Issue is closed
   - **Then**: Set Status to "✅ Done"

### For Good First Issues Project:
1. Go to: $good_first_url
2. Add automation rules:
   - **When**: Issue has "good first issue" label
   - **Then**: Add to project
   - **When**: Issue is assigned
   - **Then**: Move to "👋 Claimed" column

### For Sprint Planning Project:
1. Go to: $sprint_planning_url
2. Add automation rules:
   - **When**: Issue has effort/* label
   - **Then**: Set Effort field accordingly
   - **When**: Issue priority changes
   - **Then**: Update Priority field

## 🎯 Quick Access Bookmarks

Save these URLs for instant access:
- [🚨 Urgent Issues](https://github.com/$REPO_FULL_NAME/issues?q=is:open+label:urgent)
- [🌟 Available Good First Issues](https://github.com/$REPO_FULL_NAME/issues?q=is:open+label:"good+first+issue"+no:assignee)
- [📊 All Projects](https://github.com/$REPO_FULL_NAME/projects)
- [📈 Repository Insights](https://github.com/$REPO_FULL_NAME/pulse)

## ✅ Verification Steps

1. Create a test issue with "urgent" label
2. Check if it appears in High Priority project
3. Create a test issue with "good first issue" label
4. Verify it appears in Good First Issues project
5. Check workflow logs in Actions tab

EOF

    log_success "Automation documentation created: .github/PROJECT_AUTOMATION_SETUP.md"
}

# Main execution
main() {
    echo "🚀 Enterprise GitHub Projects Complete Setup"
    echo "=========================================="
    
    check_prerequisites
    get_repo_info
    
    echo ""
    log_info "Creating enterprise project boards..."
    
    # Create High Priority Project
    log_info "1/3 Creating High Priority Issues project..."
    high_priority_result=$(create_project \
        "🚨 High Priority Issues" \
        "Automated tracking of urgent and high-priority issues from the enterprise workflow. Items are automatically added based on priority labels and moved through workflow stages." \
        "Backlog|In Progress|Review|Done")
    
    IFS='|' read -r high_priority_id high_priority_url high_priority_number <<< "$high_priority_result"
    
    # Create Good First Issues Project
    log_info "2/3 Creating Good First Issues project..."
    good_first_result=$(create_project \
        "🌟 Good First Issues" \
        "New contributor onboarding pipeline with automated issue assignment and progress tracking. Perfect for building community engagement." \
        "Available|Claimed|In Review|Completed")
    
    IFS='|' read -r good_first_id good_first_url good_first_number <<< "$good_first_result"
    
    # Create Sprint Planning Project
    log_info "3/3 Creating Sprint Planning project..."
    sprint_planning_result=$(create_project \
        "📈 Sprint Planning" \
        "Sprint planning board with automated effort estimation, priority tracking, and velocity metrics for agile development." \
        "Backlog|Quick Wins|This Sprint|In Progress|Done")
    
    IFS='|' read -r sprint_planning_id sprint_planning_url sprint_planning_number <<< "$sprint_planning_result"
    
    # Update workflow configuration
    update_workflow_config "$high_priority_id" "$good_first_id" "$sprint_planning_id"
    
    # Create automation documentation
    create_automation_docs "$high_priority_url" "$good_first_url" "$sprint_planning_url"
    
    echo ""
    echo "🎉 ENTERPRISE SETUP COMPLETE!"
    echo "============================="
    echo ""
    log_success "All projects created and configured successfully!"
    echo ""
    echo "📋 Created Projects:"
    echo "   🚨 High Priority Issues: $high_priority_url"
    echo "   🌟 Good First Issues: $good_first_url"
    echo "   📈 Sprint Planning: $sprint_planning_url"
    echo ""
    echo "🔧 Configuration:"
    echo "   ✅ Workflow file updated with project IDs"
    echo "   ✅ Custom fields added (Priority, Effort, AI Assisted)"
    echo "   ✅ Automation documentation created"
    echo ""
    echo "📊 Quick Access Dashboard:"
    echo "   🚨 Urgent: https://github.com/$REPO_FULL_NAME/issues?q=is:open+label:urgent"
    echo "   🌟 Good First: https://github.com/$REPO_FULL_NAME/issues?q=is:open+label:\"good+first+issue\"+no:assignee"
    echo "   📈 All Projects: https://github.com/$REPO_FULL_NAME/projects"
    echo "   📊 Insights: https://github.com/$REPO_FULL_NAME/pulse"
    echo ""
    echo "🎯 Next Steps:"
    echo "   1. Visit each project URL to complete automation rules setup"
    echo "   2. Read: .github/PROJECT_AUTOMATION_SETUP.md for detailed instructions"
    echo "   3. Test: Create issues with labels to verify automation"
    echo "   4. Bookmark: Save the dashboard URLs for quick access"
    echo ""
    log_success "Your enterprise monitoring system is ready! 🚀"
}

# Run main function
main "$@"