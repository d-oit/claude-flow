# 📊 Enterprise GitHub Native Monitoring Dashboard

## 🎯 Quick Access Links

### 🚨 Priority Management
- [**Urgent Issues**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:urgent+sort:created-desc) - Critical items requiring immediate attention
- [**High Priority**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:high-priority+sort:created-desc) - Important issues for current sprint
- [**All Priority Items**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:urgent,high-priority+sort:created-desc) - Combined priority view

### 🌟 New Contributor Pipeline
- [**Available Good First Issues**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:"good+first+issue"+no:assignee+sort:created-desc) - Ready for new contributors
- [**Claimed Good First Issues**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:"good+first+issue"+assignee:*+sort:updated-desc) - Currently being worked on
- [**Completed by New Contributors**](https://github.com/d-oit/claude-flow/issues?q=is:closed+label:"good+first+issue"+sort:closed-desc) - Success stories

### ⚡ Sprint Planning
- [**Quick Wins (Small Effort)**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:effort/small+sort:created-desc) - Easy tasks for filling sprint gaps
- [**Large Tasks**](https://github.com/d-oit/claude-flow/issues?q=is:open+label:effort/large+sort:created-desc) - Major features requiring planning
- [**Unestimated Issues**](https://github.com/d-oit/claude-flow/issues?q=is:open+-label:effort/small+-label:effort/large+sort:created-desc) - Need effort estimation

### 🤖 AI Workflow Performance
- [**AI-Assisted Issues**](https://github.com/d-oit/claude-flow/issues?q=is:issue+commented-by:github-actions+sort:updated-desc) - Issues with AI suggestions
- [**Bug Reports with AI Help**](https://github.com/d-oit/claude-flow/issues?q=is:issue+label:bug+commented-by:github-actions+sort:updated-desc) - AI-improved bug reports
- [**Enhanced Feature Requests**](https://github.com/d-oit/claude-flow/issues?q=is:issue+label:enhancement+commented-by:github-actions+sort:updated-desc) - AI-enhanced feature requests

### 📈 Repository Health
- [**Recent Activity**](https://github.com/d-oit/claude-flow/pulse) - GitHub native insights
- [**Workflow Runs**](https://github.com/d-oit/claude-flow/actions) - Enterprise workflow performance
- [**All Projects**](https://github.com/d-oit/claude-flow/projects) - Project boards overview

## 📊 Key Metrics Tracking

### 🎯 Performance Indicators
- **Response Time**: Time from issue creation to first AI suggestion
- **Success Rate**: Percentage of successful workflow runs
- **Label Accuracy**: Effectiveness of auto-labeling
- **Contributor Onboarding**: Good first issue completion rate

### 📈 Trend Analysis
- **Issue Velocity**: Issues opened vs closed over time
- **Priority Distribution**: Ratio of urgent/high/normal priority items
- **AI Effectiveness**: Issues improved by AI suggestions
- **Effort Estimation Accuracy**: Small vs large task completion times

## 🔍 Advanced Search Queries

### Issue Triage
```
# Issues needing triage (no labels)
is:open no:label sort:created-desc

# Stale issues (no activity in 30 days)
is:open updated:<2024-01-01 sort:updated-asc

# Issues with multiple priority labels (needs cleanup)
is:open label:urgent label:high-priority
```

### Workflow Analysis
```
# Recently auto-labeled issues
is:issue label:bug,enhancement,documentation created:>2024-01-01

# Issues with effort estimation
is:open label:effort/small,effort/large sort:created-desc

# AI suggestions effectiveness
is:issue commented-by:github-actions created:>2024-01-01
```

### Team Performance
```
# Issues by assignee
is:open assignee:USERNAME sort:updated-desc

# Recently closed issues (team velocity)
is:closed closed:>2024-01-01 sort:closed-desc

# Pull requests linked to issues
is:pr linked:issue sort:updated-desc
```

## 🎛️ Monitoring Setup

### 1. Browser Bookmarks
Save these key dashboard links as bookmarks for instant access:
- 🚨 Priority Dashboard
- 🌟 Good First Issues
- 📊 Workflow Performance
- 📈 Repository Insights

### 2. GitHub Notifications
Configure notifications for:
- Workflow failures
- High priority issue creation
- Good first issue completion

### 3. Regular Reviews
Schedule regular reviews using these dashboards:
- **Daily**: Priority issues and workflow health
- **Weekly**: Sprint planning and effort estimation
- **Monthly**: AI effectiveness and contributor metrics

## 🚀 Getting Started

### Quick Setup (5 minutes)
1. **Run setup script**: `.github/scripts/setup-projects.sh`
2. **Bookmark dashboards**: Save the priority links above
3. **Test workflow**: Create a test issue with labels
4. **Verify automation**: Check Actions tab for workflow runs

### Full Configuration (30 minutes)
1. **Create Projects**: Follow `.github/PROJECT_SETUP.md`
2. **Configure automation rules**: Set up project board automation
3. **Customize labels**: Add any project-specific labels
4. **Train team**: Share dashboard links with team members

## 📱 Mobile Access

All dashboards work perfectly on mobile GitHub app:
- Tap "Issues" → Use filters
- Tap "Actions" → View workflow runs  
- Tap "Projects" → Monitor boards
- Tap "Insights" → Check repository health

## 🔧 Troubleshooting

### Common Issues
- **No AI suggestions**: Check workflow permissions
- **Labels not applied**: Verify labeler.yml configuration
- **Projects not updating**: Check project automation rules
- **Slow performance**: Review workflow timeout settings

### Debug Commands
```bash
# Check workflow status
gh run list --workflow="Enterprise Auto Label & AI Assist"

# View recent workflow logs
gh run view --log

# List repository projects
gh project list

# Check repository insights
gh api repos/d-oit/claude-flow/stats/contributors
```
