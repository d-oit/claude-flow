# GitHub Issue Creator

## Purpose
Create GitHub issues with support for main issues and sub-issues using text input or markdown files, with intelligent labeling and organization.

## Capabilities
- **Issue creation from text or markdown files**
- **Main issue with sub-issues support**
- **Intelligent labeling and categorization**
- **Template-based issue generation**
- **Batch issue creation**
- **Cross-repository issue management**

## Tools Available
- `Bash` - Execute GitHub CLI commands
- `Read` - Read markdown files for issue content
- `Write` - Create issue templates and documentation
- `TodoWrite` - Track issue creation tasks
- `TodoRead` - Review existing issue tasks

## Usage Patterns

### 1. Create Issue from Text Input
```bash
# Create a simple issue from text
Bash('gh issue create \
  --title "Fix memory leak in logger" \
  --body "Memory leak detected in the logging system causing 70% performance degradation" \
  --label "bug,performance,critical"')

# Create issue with detailed description
Bash('gh issue create \
  --title "Implement structured logging" \
  --body "## Description
  The current logging system lacks structured logging capabilities.

  ## Requirements
  - JSON-based log format
  - Context propagation
  - Correlation IDs

  ## Acceptance Criteria
  - [ ] All logs use JSON format
  - [ ] Context is properly propagated
  - [ ] Correlation IDs are generated" \
  --label "feature,logging,enhancement"')
```

### 2. Create Issue from Markdown File
```bash
# Create issue from markdown file
Read('analysis-reports/github-issues/logging-main-issue.md')

# Extract title and body from markdown
Bash('gh issue create \
  --title "$(head -n 1 analysis-reports/github-issues/logging-main-issue.md | sed "s/^# //")" \
  --body "$(tail -n +3 analysis-reports/github-issues/logging-main-issue.md)" \
  --label "epic,logging,refactor"')

# Create issue from specific markdown section
Bash('gh issue create \
  --title "Performance Issues with Synchronous Logging" \
  --body "$(sed -n "1,80p" analysis-reports/github-issues/logging-sub-issue-1-performance.md)" \
  --label "bug,performance,critical"')
```

### 3. Create Main Issue with Sub-Issues
```bash
# Create main epic issue
MAIN_BODY=$(cat <<'EOF'
# 🔧 Main Issue: Comprehensive Logging System Overhaul

**Title**: [Critical] Complete Logging System Refactor for Performance and Reliability

**Priority**: High  
**Type**: Epic  
**Component**: Logging System  
**Labels**: `logging`, `performance`, `refactor`, `critical`, `epic`

## Description
This is the main tracking issue for a comprehensive overhaul of the Claude-Flow logging system.

## Sub-Issues
- [ ] [Sub-Issue #1] High Performance Impact from Synchronous Logging Operations
- [ ] [Sub-Issue #2] Missing Structured Logging and Context Propagation  
- [ ] [Sub-Issue #3] Inconsistent Log Level Usage and Configuration

## Implementation Plan
- Phase 1: Performance Optimization (Weeks 1-2)
- Phase 2: Structured Logging (Weeks 3-4)
- Phase 3: Configuration and Consistency (Weeks 5-6)

## Success Metrics
- Performance impact reduced to <5%
- 100% structured logging compliance
- No data loss during system crashes
EOF
)

Bash("gh issue create \
  --title '🔧 Main Issue: Comprehensive Logging System Overhaul' \
  --body '$MAIN_BODY' \
  --label 'epic,logging,refactor,critical'")

# Create sub-issues
SUB_ISSUES=(
  "logging-sub-issue-1-performance.md"
  "logging-sub-issue-2-structured-logging.md" 
  "logging-sub-issue-3-log-levels.md"
)

for issue_file in "${SUB_ISSUES[@]}"; do
  # Extract title from first line
  TITLE=$(head -n 1 "analysis-reports/github-issues/$issue_file" | sed "s/^# //")
  
  # Extract body from rest of file
  BODY=$(tail -n +3 "analysis-reports/github-issues/$issue_file")
  
  # Create sub-issue
  Bash("gh issue create \
    --title '$TITLE' \
    --body '$BODY' \
    --label 'subtask,logging' \
    --body 'Parent issue: #\$(gh issue list --search \"Comprehensive Logging System Overhaul\" --limit 1 --json number --jq '.[0].number')")
done
```

### 4. Batch Issue Creation from Directory
```bash
# Create issues from all markdown files in a directory
for file in analysis-reports/github-issues/*.md; do
  if [[ "$file" != *"README.md" ]]; then
    TITLE=$(head -n 1 "$file" | sed "s/^# //")
    BODY=$(tail -n +3 "$file")
    
    Bash("gh issue create \
      --title '$TITLE' \
      --body '$BODY' \
      --label 'auto-generated,analysis-report'")
  fi
done

# Create issues with specific patterns
Bash('find analysis-reports -name "*.md" -exec grep -l "Priority.*High" {} \; | while read file; do
  TITLE=$(head -n 1 "$file" | sed "s/^# //")
  BODY=$(tail -n +3 "$file")
  
  gh issue create \
    --title "$TITLE" \
    --body "$BODY" \
    --label "high-priority,auto-generated"
done')
```

### 5. Template-Based Issue Creation
```bash
# Create issue from template
TEMPLATE=$(cat <<'EOF'
# {{title}}

**Priority**: {{priority}}  
**Type**: {{type}}  
**Component**: {{component}}  
**Labels**: `{{labels}}`

## Description
{{description}}

## Steps to Reproduce
{{re_steps}}

## Expected Behavior
{{expected}}

## Actual Behavior
{{actual}}

## Environment Details
- **Version**: {{version}}
- **OS**: {{os}}
- **Browser**: {{browser}}

## Additional Context
{{context}}
EOF
)

# Fill template and create issue
Bash('echo "$TEMPLATE" | \
  sed "s/{{title}}/Login Error/g" | \
  sed "s/{{priority}}/High/g" | \
  sed "s/{{type}}/Bug/g" | \
  sed "s/{{component}}/Authentication/g" | \
  sed "s/{{labels}}/auth,bug,critical/g" | \
  sed "s/{{description}}/Users unable to login with valid credentials/g" | \
  sed "s/{{re_steps}}/1. Go to login page\\n2. Enter valid credentials\\n3. Click login\\n4. Error occurs/g" | \
  sed "s/{{expected}}/User should be redirected to dashboard/g" | \
  sed "s/{{actual}}/Authentication error is shown/g" | \
  sed "s/{{version}}/v2.1.0/g" | \
  sed "s/{{os}}/Linux/g" | \
  sed "s/{{browser}}/Chrome v120/g" | \
  sed "s/{{context}}/Issue started occurring after latest deployment/g" > temp_issue.md')

Bash('gh issue create --title "Login Error" --body "$(cat temp_issue.md)" --label "auth,bug,critical"')
```

## Advanced Features

### 1. Intelligent Labeling
```bash
# Auto-detect labels based on content
auto_label() {
  local content="$1"
  local labels=""
  
  if echo "$content" | grep -qi "bug\|error\|fail"; then
    labels="$labels bug"
  fi
  
  if echo "$content" | grep -qi "feature\|implement\|add"; then
    labels="$labels enhancement"
  fi
  
  if echo "$content" | grep -qi "performance\|slow\|optimize"; then
    labels="$labels performance"
  fi
  
  if echo "$content" | grep -qi "documentation\|docs\|readme"; then
    labels="$labels documentation"
  fi
  
  echo "$labels" | xargs
}

# Create issue with auto-detected labels
CONTENT=$(cat "analysis-reports/github-issues/logging-sub-issue-1-performance.md")
LABELS=$(auto_label "$CONTENT")

Bash("gh issue create \
  --title 'Performance Issues with Synchronous Logging' \
  --body '$CONTENT' \
  --label '$LABELS'")
```

### 2. Issue Linking and Relationships
```bash
# Create linked issues with parent-child relationships
# Create parent issue
PARENT_NUMBER=$(gh issue create \
  --title "Parent Feature Request" \
  --body "Main feature implementation" \
  --label "feature,epic" --json number --jq '.number')

# Create child issues
CHILD_ISSUES=(
  "Design the API"
  "Implement core functionality" 
  "Write unit tests"
  "Create documentation"
)

for child in "${CHILD_ISSUES[@]}"; do
  gh issue create \
    --title "$child" \
    --body "Part of parent issue #$PARENT_NUMBER" \
    --label "subtask" \
    --body "Parent issue: #$PARENT_NUMBER"
done
```

### 3. Multi-Repository Issue Creation
```bash
# Create issues across multiple repositories
REPOS=("repo1" "repo2" "repo3")

for repo in "${REPOS[@]}"; do
  # Switch to repository context
  Bash("gh repo view $repo --json nameWithOwner --jq '.nameWithOwner' > current_repo.txt")
  
  # Create repository-specific issue
  TITLE="Common Issue for $repo"
  BODY="This issue affects the $repo repository specifically."
  
  Bash("gh issue create \
    --title '$TITLE' \
    --body '$BODY' \
    --label 'cross-repo,common-issue'")
done
```

## File Organization Templates

### Main Issue with Sub-Issues Structure
```
github-issues/
├── README.md
├── main-issue.md
├── sub-issue-1-name.md
├── sub-issue-2-name.md
└── sub-issue-3-name.md
```

### Batch Processing Script
```bash
#!/bin/bash
# create-issues-from-md.sh

# Create issues from markdown files in directory
create_issues_from_directory() {
  local dir="$1"
  local label_prefix="$2"
  
  for file in "$dir"/*.md; do
    if [[ "$file" != *"README.md" ]]; then
      echo "Creating issue from: $file"
      
      # Extract metadata
      TITLE=$(head -n 1 "$file" | sed "s/^# //")
      PRIORITY=$(grep -o "Priority:.*" "$file" | sed "s/Priority: *//")
      TYPE=$(grep -o "Type:.*" "$file" | sed "s/Type: *//")
      
      # Create labels
      LABELS="$label_prefix"
      if [[ -n "$PRIORITY" ]]; then
        LABELS="$LABELS,$PRIORITY"
      fi
      if [[ -n "$TYPE" ]]; then
        LABELS="$LABELS,$TYPE"
      fi
      
      # Create issue
      gh issue create \
        --title "$TITLE" \
        --body "$(tail -n +3 "$file")" \
        --label "$LABELS"
    fi
  done
}

# Usage
create_issues_from_directory "analysis-reports/github-issues" "auto-generated"
```

## Best Practices

### 1. Issue Structure
- Use clear, descriptive titles
- Include detailed descriptions with context
- Add specific acceptance criteria
- Include environment details
- Link related issues and PRs

### 2. Label Strategy
- Use consistent label naming conventions
- Include priority indicators (low, medium, high, critical)
- Add component-specific labels
- Use type labels (bug, feature, documentation, etc.)
- Include status labels when appropriate

### 3. Template Usage
- Create issue templates for common issue types
- Include placeholders for dynamic content
- Provide clear instructions for filling templates
- Validate template completeness before creation

### 4. Batch Operations
- Group related issues together
- Use consistent naming conventions
- Include parent-child relationships when needed
- Add appropriate labels for batch-created issues

## Integration with Other Commands

### 1. Issue Tracker Integration
```bash
# Create issue and add to tracking system
Bash('gh issue create \
  --title "New Feature Request" \
  --body "Feature description" \
  --label "feature"')

# Add to todo list for tracking
TodoWrite { todos: [
  { id: "feature-request", content: "Review and implement new feature", status: "pending", priority: "medium" }
]}
```

### 2. Swarm Coordination Integration
```bash
# Create issue and initialize swarm
Bash('gh issue create \
  --title "Complex Feature Implementation" \
  --body "Requires swarm coordination" \
  --label "swarm-ready,feature"')

# Initialize swarm for the issue
Bash('npx ruv-swarm github issue-init $(gh issue list --search "Complex Feature Implementation" --limit 1 --json number --jq ".[0].number")')
```

## Error Handling

### 1. Validation
```bash
# Validate issue content before creation
validate_issue() {
  local title="$1"
  local body="$2"
  
  if [[ -z "$title" ]]; then
    echo "Error: Issue title cannot be empty"
    return 1
  fi
  
  if [[ ${#title} -gt 255 ]]; then
    echo "Error: Issue title too long (max 255 characters)"
    return 1
  fi
  
  if [[ -z "$body" ]]; then
    echo "Error: Issue body cannot be empty"
    return 1
  fi
  
  return 0
}

# Usage
if validate_issue "$TITLE" "$BODY"; then
  gh issue create --title "$TITLE" --body "$BODY"
else
  echo "Issue creation failed validation"
fi
```

### 2. Retry Logic
```bash
# Retry issue creation on failure
create_issue_with_retry() {
  local title="$1"
  local body="$2"
  local labels="$3"
  local max_attempts=3
  local attempt=1
  
  while [[ $attempt -le $max_attempts ]]; do
    if gh issue create --title "$title" --body "$body" --label "$labels" >/dev/null 2>&1; then
      echo "Issue created successfully"
      return 0
    else
      echo "Attempt $attempt failed, retrying..."
      sleep 2
      ((attempt++))
    fi
  done
  
  echo "Failed to create issue after $max_attempts attempts"
  return 1
}
```

## Examples

### 1. Simple Bug Report
```bash
Bash('gh issue create \
  --title "Memory leak in logger causing 70% performance degradation" \
  --body "## Problem
  Memory leak detected in the logging system during high-frequency operations.

  ## Steps to Reproduce
  1. Enable file logging
  2. Generate high-frequency log messages
  3. Monitor memory usage
  4. Observe memory growth without proper cleanup

  ## Expected Behavior
  Memory usage should remain stable during logging operations.

  ## Actual Behavior
  Memory usage grows continuously until system becomes unresponsive.

  ## Environment
  - Version: Claude-Flow development version
  - OS: Linux 6.8
  - Node.js: v18.x" \
  --label "bug,performance,critical,logging"')
```

### 2. Feature Request with Sub-Issues
```bash
# Create main epic
MAIN_EPIC=$(cat <<'EOF'
# 🚀 Feature Request: Advanced Analytics Dashboard

**Priority**: High  
**Type**: Feature  
**Component**: Analytics  
**Labels**: `feature,analytics,enhancement,epic`

## Description
Implement a comprehensive analytics dashboard for system monitoring and insights.

## Sub-Issues
- [ ] [Sub-Issue #1] Real-time Performance Metrics
- [ ] [Sub-Issue #2] User Activity Analytics  
- [ ] [Sub-Issue #3] System Health Monitoring
- [ ] [Sub-Issue #4] Custom Report Generation

## Success Criteria
- [ ] Dashboard loads in <2 seconds
- [ ] Real-time data updates every 30 seconds
- [ ] Export functionality for all reports
- [ ] Mobile-responsive design
EOF
)

Bash("gh issue create \
  --title '🚀 Feature Request: Advanced Analytics Dashboard' \
  --body '$MAIN_EPIC' \
  --label 'feature,analytics,enhancement,epic'")

# Create sub-issues
SUB_ISSUES=(
  "Real-time Performance Metrics"
  "User Activity Analytics"
  "System Health Monitoring" 
  "Custom Report Generation"
)

for sub_issue in "${SUB_ISSUES[@]}"; do
  Bash("gh issue create \
    --title '$sub_issue' \
    --body 'Part of epic: Advanced Analytics Dashboard' \
    --label 'subtask,analytics'")
done
```

### 3. Documentation Update from File
```bash
# Create issue from existing documentation file
Bash('gh issue create \
  --title "Update API Documentation for v2.0" \
  --body "$(cat docs/api/v2.0-changes.md)" \
  --label "documentation,api,update" \
  --assignee @ruvnet')
```

## Security Considerations

1. **Input Validation**: Validate all issue content before creation
2. **Permission Checks**: Ensure user has permission to create issues
3. **Rate Limiting**: Handle GitHub API rate limits gracefully
4. **Content Sanitization**: Sanitize issue content to prevent XSS
5. **Access Control**: Restrict access to sensitive issue content

## Performance Optimization

1. **Batch Operations**: Use batch operations for multiple issue creation
2. **Parallel Processing**: Process multiple issues in parallel when possible
3. **Caching**: Cache frequently used issue templates
4. **Lazy Loading**: Load issue content only when needed
5. **Connection Pooling**: Reuse GitHub CLI connections

---

**Related Commands**: [issue-tracker.md](./issue-tracker.md), [swarm-issue.md](./swarm-issue.md), [repo-analyze.md](./repo-analyze.md)