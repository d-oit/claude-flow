# 🏢 Enterprise GitHub Monitoring - Complete Setup Guide

## 🚀 One-Click Setup

**The fastest way to get enterprise monitoring running:**

```bash
# Run the master setup script
bash .github/scripts/setup-enterprise-monitoring.sh
```

This interactive script will guide you through:
- **Quick Setup** (5 minutes) - Basic projects and automation
- **Complete Enterprise Setup** (15 minutes) - Full configuration with custom fields
- **Test Mode** - Safe experimentation environment
- **Dashboard Only** - Just monitoring links

## 📋 Available Setup Scripts

### 🎯 Master Script (Recommended)
```bash
.github/scripts/setup-enterprise-monitoring.sh
```
- Interactive setup with multiple modes
- Complete verification and testing
- Beautiful output with progress tracking
- Handles all edge cases and errors

### 🚀 Quick Setup
```bash
.github/scripts/setup-projects.sh
```
- Creates basic projects in 5 minutes
- Minimal configuration
- Perfect for getting started quickly

### 🏢 Complete Enterprise Setup
```bash
.github/scripts/setup-projects-complete.sh
```
- Full enterprise configuration
- Custom fields and automation
- Comprehensive documentation generation
- Production-ready setup

### 🤖 Automation Configuration
```bash
.github/scripts/configure-automation.sh
```
- Configures project automation rules
- Creates helper functions
- Generates test scenarios
- Post-setup optimization

## 🎯 What Gets Created

### 📊 GitHub Projects
1. **🚨 High Priority Issues**
   - Urgent and high-priority items
   - Auto-assignment based on labels
   - Custom priority fields

2. **🌟 Good First Issues**
   - New contributor pipeline
   - Automatic newcomer guidance
   - Progress tracking

3. **📈 Sprint Planning**
   - Effort estimation
   - Velocity tracking
   - Sprint organization

### 🤖 Workflow Automation
- **Enterprise Auto Label & AI Assist** - Smart labeling and AI suggestions
- **Project Automation** - Automatic project board management
- **Monitoring & Metrics** - Performance tracking and analytics

### 📊 Monitoring Dashboards
- **Priority Management** - Urgent and high-priority issue tracking
- **New Contributor Pipeline** - Onboarding and mentorship
- **Sprint Planning** - Agile development support
- **AI Performance** - Workflow effectiveness metrics

### 📚 Documentation
- **Setup Guides** - Complete configuration instructions
- **Automation Rules** - Project board automation
- **Test Scenarios** - Verification and testing
- **Dashboard Links** - Quick access bookmarks

## 🔧 Manual Configuration Steps

After running the automated setup, complete these manual steps:

### 1. Project Automation Rules
Visit each project and configure automation rules:

#### High Priority Project:
- **When**: Item added with `urgent` label → **Then**: Set Priority to "🔥 Urgent"
- **When**: Item added with `high-priority` label → **Then**: Set Priority to "⚡ High"
- **When**: Issue closed → **Then**: Move to "✅ Done"

#### Good First Issues Project:
- **When**: Item added with `good first issue` label → **Then**: Set Status to "📋 Available"
- **When**: Issue assigned → **Then**: Set Status to "👋 Claimed"
- **When**: Issue closed → **Then**: Set Status to "🎉 Completed"

#### Sprint Planning Project:
- **When**: Item added with `effort/small` label → **Then**: Set Effort to "⚡ Small"
- **When**: Item added with `effort/large` label → **Then**: Set Effort to "🏗️ Large"
- **When**: Issue closed → **Then**: Move to "✅ Done"

### 2. Team Permissions
Configure team access to projects:
- **Maintainers**: Admin access to all projects
- **Contributors**: Write access to relevant projects
- **Community**: Read access to Good First Issues project

### 3. Notification Settings
Set up notifications for:
- High priority issue creation
- Good first issue completion
- Workflow failures
- Project board updates

## 🧪 Testing Your Setup

### Quick Verification
```bash
# Test urgent issue automation
gh issue create --title "🚨 Critical Bug" --label urgent --body "Testing high priority automation"

# Test good first issue pipeline
gh issue create --title "📚 Documentation Fix" --label "good first issue" --body "Testing newcomer pipeline"

# Test effort estimation
gh issue create --title "⚡ Quick Feature" --label effort/small --body "Testing sprint planning"

# Check workflow runs
gh run list --workflow="Enterprise Auto Label & AI Assist"
```

### Comprehensive Testing
Follow the test scenarios in `.github/TEST_AUTOMATION.md` for complete verification.

## 📊 Monitoring Your Setup

### Daily Monitoring
- [🚨 Urgent Issues](https://github.com/d-oit/claude-flow/issues?q=is:open+label:urgent)
- [📊 Workflow Runs](https://github.com/d-oit/claude-flow/actions)
- [📈 Repository Pulse](https://github.com/d-oit/claude-flow/pulse)

### Weekly Reviews
- [🌟 Good First Issue Progress](https://github.com/d-oit/claude-flow/issues?q=label:"good+first+issue")
- [📈 Sprint Planning Board](https://github.com/d-oit/claude-flow/projects)
- [🤖 AI Assistance Effectiveness](https://github.com/d-oit/claude-flow/issues?q=commented-by:github-actions)

### Monthly Analytics
- Issue velocity and resolution time
- New contributor onboarding success
- AI suggestion acceptance rate
- Project automation effectiveness

## 🔧 Troubleshooting

### Common Issues

#### Projects Not Updating
```bash
# Check project permissions
gh api repos/d-oit/claude-flow --jq '.permissions'

# Verify automation rules in project settings
echo "Visit: https://github.com/d-oit/claude-flow/projects"
```

#### Workflows Not Triggering
```bash
# Check workflow permissions
gh api repos/d-oit/claude-flow/actions/permissions

# View recent workflow runs
gh run list --limit 10
```

#### AI Suggestions Not Appearing
```bash
# Check workflow conditions
gh run view --log | grep -i "ai"

# Verify label configuration
cat .github/labeler.yml
```

### Debug Commands
```bash
# Repository health check
gh repo view --json permissions,hasIssuesEnabled,hasProjectsEnabled

# Workflow status
gh workflow list

# Project list
gh project list --owner $(gh repo view --json owner --jq '.owner.login')

# Recent issues with automation
gh issue list --limit 20 --json number,title,labels,comments
```

## 🎯 Advanced Configuration

### Custom Labels
Add project-specific labels to `.github/labels.yml`:
```yaml
custom-priority: Custom priority level
team-backend: Backend team responsibility
team-frontend: Frontend team responsibility
```

### Workflow Customization
Modify environment variables in `.github/workflows/auto-label-ai-assist.yml`:
```yaml
env:
  ENABLE_AI_SUGGESTIONS: true
  ENABLE_PRIORITY_LABELING: true
  MAX_COMMENT_LENGTH: 2000
```

### Integration with External Tools
- **Slack**: Use GitHub Actions to send notifications
- **Jira**: Link issues using automation rules
- **Analytics**: Export metrics to external dashboards

## 📚 Additional Resources

### Documentation Files
- `.github/PROJECT_SETUP.md` - Detailed project configuration
- `.github/MONITORING_DASHBOARD.md` - Dashboard links and queries
- `.github/AUTOMATION_COMMANDS.md` - CLI commands for automation
- `.github/TEST_AUTOMATION.md` - Testing scenarios and verification

### GitHub Resources
- [GitHub Projects Documentation](https://docs.github.com/en/issues/planning-and-tracking-with-projects)
- [GitHub Actions Workflow Syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions)
- [GitHub CLI Manual](https://cli.github.com/manual/)

### Best Practices
- Regular backup of project configurations
- Periodic review of automation rules
- Team training on new features
- Continuous monitoring and optimization

---

**🎉 Your enterprise GitHub monitoring system is now ready for production use!**

For support or questions, check the troubleshooting section or create an issue with the `question` label.