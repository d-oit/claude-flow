# 📊 GitHub Projects Setup Guide

## 🎯 Enterprise Project Board Configuration

### Step 1: Create Project Boards

#### 🚨 **High Priority Board**
1. Go to: `https://github.com/d-oit/claude-flow/projects`
2. Click "New project" → "Board"
3. Name: "🚨 High Priority Issues"
4. Add columns:
   - 🔥 **Urgent** (for `urgent` label)
   - ⚡ **High Priority** (for `high-priority` label)
   - 🔧 **In Progress** (for assigned issues)
   - ✅ **Done** (for closed issues)

#### 🌟 **New Contributor Board**
1. Create new project: "🌟 Good First Issues"
2. Add columns:
   - 📋 **Available** (unassigned `good first issue`)
   - 👋 **Claimed** (assigned `good first issue`)
   - 🔄 **In Review** (with PR linked)
   - 🎉 **Completed** (closed)

#### 📈 **Sprint Planning Board**
1. Create new project: "📈 Sprint Planning"
2. Add columns:
   - 🎯 **Backlog** (all open issues)
   - ⚡ **Quick Wins** (`effort/small` label)
   - 🏗️ **Large Tasks** (`effort/large` label)
   - 🚀 **This Sprint** (current sprint items)
   - ✅ **Done** (completed)

### Step 2: Configure Automation Rules

For each project board, add these automation rules:

#### Auto-add Issues
- **When**: Issue is opened
- **Then**: Add to project
- **Filter**: Based on labels

#### Auto-move Cards
- **When**: Issue is assigned → Move to "In Progress"
- **When**: Issue is closed → Move to "Done"
- **When**: PR is opened → Move to "In Review"

### Step 3: Custom Fields Setup

Add these custom fields to track workflow metrics:

#### Priority Field
- Type: Single select
- Options: Urgent, High, Medium, Low
- Auto-populate from labels

#### Effort Field  
- Type: Single select
- Options: Small, Medium, Large, Unknown
- Auto-populate from `effort/*` labels

#### AI Assisted Field
- Type: Checkbox
- Auto-check when commented by github-actions

#### Processing Time Field
- Type: Number
- Track time from creation to first response

## 🔧 Project URLs (Replace with your actual project URLs)

Once created, update these in the workflow:

```yaml
env:
  HIGH_PRIORITY_PROJECT_URL: "https://github.com/d-oit/claude-flow/projects/1"
  GOOD_FIRST_ISSUES_PROJECT_URL: "https://github.com/d-oit/claude-flow/projects/2"  
  SPRINT_PLANNING_PROJECT_URL: "https://github.com/d-oit/claude-flow/projects/3"
```

## 📊 Instant Analytics Queries

### High Priority Dashboard
```
https://github.com/d-oit/claude-flow/issues?q=is:open+label:urgent,high-priority+sort:created-desc
```

### Good First Issues Pipeline
```
https://github.com/d-oit/claude-flow/issues?q=is:open+label:"good+first+issue"+sort:created-desc
```

### AI Workflow Performance
```
https://github.com/d-oit/claude-flow/issues?q=is:issue+commented-by:github-actions+sort:updated-desc
```

### Sprint Planning View
```
https://github.com/d-oit/claude-flow/issues?q=is:open+label:effort/small,effort/large+sort:created-desc
```

## 🎯 Quick Setup Commands

Run these GitHub CLI commands to create projects programmatically:

```bash
# Install GitHub CLI if not already installed
# gh auth login

# Create High Priority Project
gh project create --title "🚨 High Priority Issues" --body "Automated high-priority issue tracking"

# Create Good First Issues Project  
gh project create --title "🌟 Good First Issues" --body "New contributor onboarding pipeline"

# Create Sprint Planning Project
gh project create --title "📈 Sprint Planning" --body "Sprint planning and effort tracking"
```

## 🔍 Monitoring Setup Verification

After setup, verify these work:

1. ✅ Create a test issue with `urgent` label → Should auto-add to High Priority board
2. ✅ Create a test issue with `good first issue` label → Should auto-add to Good First Issues board  
3. ✅ Assign an issue → Should auto-move to "In Progress"
4. ✅ Close an issue → Should auto-move to "Done"
5. ✅ Check Actions tab → Should see workflow metrics in logs