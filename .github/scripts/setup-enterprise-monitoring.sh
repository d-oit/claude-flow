#!/bin/bash
# 🏢 Master Enterprise Monitoring Setup Script
# One-click setup for complete GitHub native monitoring solution

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }
log_header() { echo -e "${PURPLE}🎯 $1${NC}"; }
log_step() { echo -e "${CYAN}📋 $1${NC}"; }

# ASCII Art Header
show_header() {
    echo -e "${PURPLE}"
    cat << 'EOF'
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║    🏢 ENTERPRISE GITHUB MONITORING SETUP                    ║
║                                                              ║
║    Complete automation for GitHub native monitoring         ║
║    • Projects with smart automation                         ║
║    • AI-powered workflow assistance                         ║
║    • Real-time analytics dashboards                         ║
║    • Zero external dependencies                             ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

# Check if we're in the right directory
check_directory() {
    if [[ ! -d ".github" ]]; then
        log_error "Please run this script from the repository root directory"
        exit 1
    fi
    
    if [[ ! -f ".github/workflows/auto-label-ai-assist.yml" ]]; then
        log_error "Enterprise workflow not found. Please ensure the workflow files are present."
        exit 1
    fi
}

# Setup mode selection
select_setup_mode() {
    echo ""
    log_header "Setup Mode Selection"
    echo ""
    echo "Choose your setup mode:"
    echo "  1) 🚀 Quick Setup (5 minutes) - Basic projects and automation"
    echo "  2) 🏢 Complete Enterprise Setup (15 minutes) - Full configuration with custom fields"
    echo "  3) 🧪 Test Mode - Create test projects for experimentation"
    echo "  4) 📊 Dashboard Only - Just create monitoring dashboards"
    echo ""
    read -p "Select mode (1-4): " mode
    
    case $mode in
        1) SETUP_MODE="quick" ;;
        2) SETUP_MODE="complete" ;;
        3) SETUP_MODE="test" ;;
        4) SETUP_MODE="dashboard" ;;
        *) log_error "Invalid selection"; exit 1 ;;
    esac
    
    log_info "Selected mode: $SETUP_MODE"
}

# Quick setup
quick_setup() {
    log_header "Quick Setup Mode"
    log_step "Running basic project creation..."
    
    if [[ -f ".github/scripts/setup-projects.sh" ]]; then
        bash .github/scripts/setup-projects.sh
    else
        log_error "Quick setup script not found"
        exit 1
    fi
    
    log_success "Quick setup completed!"
}

# Complete enterprise setup
complete_setup() {
    log_header "Complete Enterprise Setup Mode"
    log_step "Running full project configuration..."
    
    if [[ -f ".github/scripts/setup-projects-complete.sh" ]]; then
        bash .github/scripts/setup-projects-complete.sh
        
        log_step "Configuring automation rules..."
        if [[ -f ".github/scripts/configure-automation.sh" ]]; then
            bash .github/scripts/configure-automation.sh
        fi
    else
        log_error "Complete setup script not found"
        exit 1
    fi
    
    log_success "Complete enterprise setup finished!"
}

# Test mode setup
test_setup() {
    log_header "Test Mode Setup"
    log_step "Creating test projects for experimentation..."
    
    # Create test projects with "TEST-" prefix
    log_info "Creating test projects (these can be safely deleted later)..."
    
    gh project create --title "TEST-🚨 High Priority" --body "Test project for high priority items" || true
    gh project create --title "TEST-🌟 Good First Issues" --body "Test project for newcomers" || true
    gh project create --title "TEST-📈 Sprint Planning" --body "Test project for sprint planning" || true
    
    log_success "Test projects created! You can experiment safely."
    log_warning "Remember to delete test projects when done: gh project delete [PROJECT_NUMBER]"
}

# Dashboard only setup
dashboard_setup() {
    log_header "Dashboard Only Setup"
    log_step "Creating monitoring dashboards..."
    
    # Create dashboard bookmarks file
    cat > ".github/DASHBOARD_BOOKMARKS.html" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>GitHub Enterprise Monitoring Dashboard</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; }
        .dashboard { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .card { border: 1px solid #e1e4e8; border-radius: 6px; padding: 20px; }
        .card h3 { margin-top: 0; color: #0366d6; }
        .link { display: block; padding: 8px 0; text-decoration: none; color: #586069; }
        .link:hover { color: #0366d6; }
        .urgent { color: #d73a49; }
        .success { color: #28a745; }
        .info { color: #0366d6; }
    </style>
</head>
<body>
    <h1>🏢 Enterprise GitHub Monitoring Dashboard</h1>
    <div class="dashboard">
        <div class="card">
            <h3>🚨 Priority Management</h3>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:urgent" class="link urgent">Urgent Issues</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:high-priority" class="link">High Priority</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:urgent,high-priority" class="link">All Priority Items</a>
        </div>
        
        <div class="card">
            <h3>🌟 New Contributors</h3>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:\"good+first+issue\"+no:assignee" class="link success">Available Issues</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:\"good+first+issue\"+assignee:*" class="link">In Progress</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:closed+label:\"good+first+issue\"" class="link">Completed</a>
        </div>
        
        <div class="card">
            <h3>📈 Sprint Planning</h3>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:effort/small" class="link">Quick Wins</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+label:effort/large" class="link">Large Tasks</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=is:open+-label:effort/small+-label:effort/large" class="link">Unestimated</a>
        </div>
        
        <div class="card">
            <h3>🤖 AI Performance</h3>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/issues?q=commented-by:github-actions" class="link info">AI-Assisted Issues</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/actions" class="link">Workflow Runs</a>
            <a href="https://github.com/$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')/pulse" class="link">Repository Insights</a>
        </div>
    </div>
</body>
</html>
EOF
    
    log_success "Dashboard created: .github/DASHBOARD_BOOKMARKS.html"
    log_info "Open this file in your browser for quick access to all monitoring links"
}

# Post-setup verification
verify_setup() {
    log_header "Setup Verification"
    
    local repo_name=$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')
    
    echo ""
    log_step "Checking workflow files..."
    if [[ -f ".github/workflows/auto-label-ai-assist.yml" ]]; then
        log_success "Enterprise workflow found"
    else
        log_warning "Enterprise workflow missing"
    fi
    
    if [[ -f ".github/workflows/project-automation.yml" ]]; then
        log_success "Project automation workflow found"
    else
        log_warning "Project automation workflow missing"
    fi
    
    log_step "Checking projects..."
    project_count=$(gh project list --owner $(gh repo view --json owner --jq '.owner.login') --format json | jq length)
    if [[ $project_count -gt 0 ]]; then
        log_success "$project_count project(s) found"
    else
        log_warning "No projects found"
    fi
    
    log_step "Checking labels configuration..."
    if [[ -f ".github/labels.yml" ]] && [[ -f ".github/labeler.yml" ]]; then
        log_success "Label configuration files found"
    else
        log_warning "Label configuration files missing"
    fi
    
    echo ""
    log_success "Verification completed!"
}

# Show final summary
show_summary() {
    local repo_name=$(gh repo view --json owner,name --jq '.owner.login + "/" + .name')
    
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    🎉 SETUP COMPLETE!                        ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    log_success "Enterprise GitHub monitoring is now active!"
    echo ""
    echo "📊 Quick Access Dashboard:"
    echo "   🚨 Urgent Issues: https://github.com/$repo_name/issues?q=is:open+label:urgent"
    echo "   🌟 Good First Issues: https://github.com/$repo_name/issues?q=is:open+label:\"good+first+issue\"+no:assignee"
    echo "   📈 All Projects: https://github.com/$repo_name/projects"
    echo "   📊 Repository Insights: https://github.com/$repo_name/pulse"
    echo "   🔧 Workflow Runs: https://github.com/$repo_name/actions"
    echo ""
    
    echo "🧪 Test Your Setup:"
    echo "   gh issue create --title 'Test Urgent Issue' --label urgent --body 'Testing automation'"
    echo "   gh issue create --title 'Test Good First Issue' --label 'good first issue' --body 'Testing newcomer pipeline'"
    echo ""
    
    echo "📚 Documentation Created:"
    if [[ -f ".github/PROJECT_SETUP.md" ]]; then
        echo "   📋 .github/PROJECT_SETUP.md - Complete setup guide"
    fi
    if [[ -f ".github/MONITORING_DASHBOARD.md" ]]; then
        echo "   📊 .github/MONITORING_DASHBOARD.md - Dashboard links and queries"
    fi
    if [[ -f ".github/PROJECT_AUTOMATION_SETUP.md" ]]; then
        echo "   🤖 .github/PROJECT_AUTOMATION_SETUP.md - Automation configuration"
    fi
    if [[ -f ".github/TEST_AUTOMATION.md" ]]; then
        echo "   🧪 .github/TEST_AUTOMATION.md - Testing scenarios"
    fi
    if [[ -f ".github/DASHBOARD_BOOKMARKS.html" ]]; then
        echo "   🔖 .github/DASHBOARD_BOOKMARKS.html - Browser dashboard"
    fi
    
    echo ""
    log_success "Your enterprise monitoring system is ready to use! 🚀"
}

# Main execution
main() {
    show_header
    check_directory
    select_setup_mode
    
    echo ""
    case $SETUP_MODE in
        "quick")
            quick_setup
            ;;
        "complete")
            complete_setup
            ;;
        "test")
            test_setup
            ;;
        "dashboard")
            dashboard_setup
            ;;
    esac
    
    verify_setup
    show_summary
}

# Run main function
main "$@"