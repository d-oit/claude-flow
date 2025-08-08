---
description: Create GitHub issues with support for main issues and sub-issues using text or markdown files
argument-hint: <text-or-file-path> [sub-issues-directory]
---

# GitHub Issue Creator

Create GitHub issues with intelligent support for main issues and sub-issues using text input or markdown files.

## Usage Examples

### Basic Issue Creation from Text
```
/github-issue-create Memory leak in logger causing performance degradation
```

### Issue from Markdown File
```
/github-issue-create analysis-reports/github-issues/logging-main-issue.md
```

### Main Issue with Sub-Issues (Recommended)
```
/github-issue-create analysis-reports/github-issues/logging-main-issue.md analysis-reports/github-issues/
```

### Using the Robust Script (Alternative)
```
# Create main issue with sub-issues
.roo/scripts/github-issue-manager.sh create analysis-reports/github-issues/logging-main-issue.md analysis-reports/github-issues/

# Link existing sub-issue to parent
.roo/scripts/github-issue-manager.sh link 6 7

# Add parent reference to existing issue
.roo/scripts/github-issue-manager.sh update 7 6
```

## Features

- **Text Input**: Create issues from simple text descriptions
- **Markdown Files**: Create issues from structured markdown files
- **Main + Sub-Issues**: Create epic issues with linked sub-tasks
- **Intelligent Labeling**: Auto-detect labels based on content
- **Template Support**: Use structured templates for consistent issues
- **Batch Creation**: Create multiple issues from directory

## How It Works

### 1. Text Input Mode
When you provide text input, it creates a simple issue with:
- Title: The provided text
- Body: Basic issue structure with your description
- Labels: Auto-detected based on content

### 2. Markdown File Mode
When you provide a markdown file path, it:
- Extracts title from first line (starting with #)
- Uses remaining content as issue body
- Auto-detects labels from content
- Creates a well-structured issue

### 3. Main + Sub-Issues Mode
When you provide a directory path, it:
- Creates main issue from first markdown file
- Creates sub-issues from remaining markdown files
- **Links sub-issues to main issue using GitHub GraphQL API** (if permissions allow)
- Applies consistent labeling
- **Adds parent issue reference to sub-issue bodies** (fallback method)

## File Structure Examples

### Single Issue
```
# Issue Title

**Priority**: High  
**Type**: Bug  
**Component**: Logging System  
**Labels**: `logging`, `performance`, `critical`

## Description
Detailed description of the issue...

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen...

## Actual Behavior
What actually happens...

## Environment Details
- Version: 1.0.0
- OS: Linux
- Node.js: v18.x
```

### Main Issue with Sub-Issues
```
github-issues/
├── main-issue.md          # Main epic issue
├── sub-issue-1.md         # First sub-task
├── sub-issue-2.md         # Second sub-task
└── sub-issue-3.md         # Third sub-task
```

## Auto-Detection Logic

### Priority Detection
- **Critical**: Contains "critical", "blocking", "urgent", "showstopper"
- **High**: Contains "high", "major", "severe", "important"
- **Medium**: Contains "medium", "moderate", "normal"
- **Low**: Contains "low", "minor", "trivial"

### Type Detection
- **Bug**: Contains "bug", "error", "fail", "broken", "issue"
- **Feature**: Contains "feature", "implement", "add", "new"
- **Documentation**: Contains "docs", "documentation", "readme", "guide"
- **Performance**: Contains "performance", "slow", "optimize", "speed"
- **Security**: Contains "security", "vulnerability", "auth", "credential"

### Component Detection
- **Logging**: Contains "log", "logger", "logging"
- **Database**: Contains "db", "database", "sql", "query"
- **API**: Contains "api", "endpoint", "rest", "graphql"
- **UI**: Contains "ui", "interface", "frontend", "display"
- **CLI**: Contains "cli", "command", "terminal", "shell"

## Best Practices

### Issue Structure
1. Start with clear, descriptive title
2. Include priority and type information
3. Provide detailed description
4. Add reproduction steps
5. Include environment details
6. Link related issues

### Label Strategy
- Use consistent naming conventions
- Include priority indicators
- Add component-specific labels
- Use type labels for categorization
- Include status labels when needed

### File Organization
- Group related issues together
- Use descriptive filenames
- Include README files for context
- Maintain consistent structure
- Version control your issue templates

## Examples

### Bug Report Template
```
# Bug Title Here

**Priority**: High  
**Type**: Bug  
**Component**: System  
**Labels**: `bug`, `system`, `critical`

## Problem Description
Clear description of the bug...

## Steps to Reproduce
1. First action
2. Second action  
3. Third action
4. Bug occurs

## Expected Behavior
What should happen...

## Actual Behavior
What actually happens...

## Environment
- Version: 1.0.0
- OS: Linux
- Browser: Chrome 120
```

### Feature Request Template
```
# Feature Request: New Feature Name

**Priority**: Medium  
**Type**: Feature  
**Component**: User Interface  
**Labels**: `feature`, `ui`, `enhancement`

## Description
Detailed description of the requested feature...

## Use Cases
1. User scenario one
2. User scenario two
3. User scenario three

## Requirements
- [ ] Basic functionality
- [ ] Advanced features
- [ ] Error handling
- [ ] Documentation

## Acceptance Criteria
- [ ] Feature works as described
- [ ] All requirements met
- [ ] Proper error handling
- [ ] Documentation updated
```

### Performance Issue Template
```
# Performance: Slow Operation in Component

**Priority**: High  
**Type**: Performance  
**Component**: Database  
**Labels**: `performance`, `database`, `optimization`

## Problem
Performance degradation in [specific operation]...

## Current Performance
- Response time: 5 seconds
- Throughput: 100 requests/minute
- Memory usage: 2GB

## Expected Performance
- Response time: < 500ms
- Throughput: 1000 requests/minute  
- Memory usage: < 1GB

## Analysis
- Bottleneck identified in [specific area]
- Optimization opportunities: [list opportunities]
- Estimated improvement: [percentage]

## Implementation Plan
1. [Step one]
2. [Step two]
3. [Step three]
```

## Integration with GitHub CLI

This command uses GitHub CLI (`gh`) to create issues and GitHub GraphQL API to link sub-issues. Make sure you have:

1. **GitHub CLI installed**: `brew install gh` (macOS) or download from GitHub
2. **Authentication**: `gh auth login` to authenticate with GitHub
3. **Repository configured**: Working in a git repository with GitHub remote
4. **GraphQL API access**: Required for sub-issue linking (enabled by default)
5. **Sufficient permissions**: Write access to repository issues and GraphQL API

## Sub-Issues Linking

Since GitHub CLI doesn't natively support sub-issues, this command uses a robust approach with multiple fallback methods:

### Primary Method: GraphQL API Linking

1. **Create Parent Issue**: Uses `gh issue create` to create the main issue
2. **Create Sub-Issues**: Uses `gh issue create` for each sub-issue
3. **Link Issues**: Uses GitHub GraphQL API to establish parent-child relationships

#### GraphQL API Integration

The command automatically:
- Retrieves issue IDs using GraphQL queries
- Uses GraphQL mutation with `GraphQL-Features: sub_issues` header
- Links sub-issues to their parent issue

Example GraphQL call:
```bash
# Get issue ID
PARENT_ID=$(gh api graphql -f query='query { repository(owner:"'$REPO_OWNER'",name:"'$REPO_NAME'"){issue(number:'$PARENT_ISSUE'){id}}}' -q .data.repository.issue.id)

# Link sub-issue
gh api graphql -H "GraphQL-Features: sub_issues" -f query='
  mutation {
    addSubIssue(input: { issueId: "'"$PARENT_ID"'", subIssueId: "'"$CHILD_ID"'" }) {
      clientMutationId
    }
  }'
```

### Fallback Method: Parent Issue Reference

When GraphQL API is not available due to permissions, the command:

1. **Updates Sub-Issue Bodies**: Adds parent issue reference to each sub-issue
2. **Manual Linking**: Provides clear parent-child relationships in issue bodies
3. **Consistent Formatting**: Uses standardized reference format

Example sub-issue body update:
```bash
# Add parent reference to sub-issue body
gh issue edit $SUB_ISSUE_NUMBER --body "$(gh issue view $SUB_ISSUE_NUMBER --json body --jq .body)

---

**Parent Issue**: #$PARENT_ISSUE"
```

### Hybrid Approach

The command attempts GraphQL linking first, then falls back to body references if:

- GraphQL API returns permission errors
- Repository doesn't support sub-issues
- Rate limits are exceeded
- Authentication issues occur

This ensures sub-issues are always properly referenced, even when automated linking fails.

## Error Handling

### Common Issues
- **File not found**: Check file path and permissions
- **GitHub auth**: Run `gh auth login` to authenticate
- **Repository not found**: Ensure you're in the correct directory
- **Rate limits**: Wait and retry if hitting GitHub API limits

### Validation
- File format validation for markdown files
- Content length validation (title < 255 chars, body < 65536 chars)
- Label validation (proper format and length)
- Repository access validation

## Advanced Usage

### Custom Templates
Create custom issue templates in your project:

```
.templates/
├── bug-template.md
├── feature-template.md
├── performance-template.md
└── documentation-template.md
```

### Batch Processing
Process multiple directories or use with scripts:

```bash
# Create issues from all markdown files in directory
find . -name "*.md" -not -path "./.git/*" | while read file; do
  /github-issue-create "$file"
done

# Create main issue with sub-issues from directory
/github-issue-create main-issue.md ./sub-issues/
```

### Sub-Issues Script Example

For advanced sub-issues management, use this robust script based on the proven gh-sub-issues approach:

```bash
#!/bin/bash
# create-sub-issues.sh - Robust sub-issue creation with fallback methods

set -euo pipefail

# Configuration
REPO_OWNER="d-oit"
REPO_NAME="claude-flow"
PARENT_TITLE="Parent Issue Title"
PARENT_BODY_FILE="parent-issue.md"

# Function to get issue ID from number
get_issue_id() {
    local issue_number="$1"
    gh api graphql -f query="query { repository(owner:\"$REPO_OWNER\",name:\"$REPO_NAME\"){issue(number:$issue_number){id}}}" -q .data.repository.issue.id
}

# Function to link sub-issue to parent
link_sub_issue() {
    local parent_id="$1"
    local child_id="$2"
    
    # Try GraphQL linking first
    if gh api graphql \
        -H "GraphQL-Features: sub_issues" \
        -f query="mutation { addSubIssue(input: {issueId: \"$parent_id\", subIssueId: \"$child_id\"}) { clientMutationId } }" \
        >/dev/null 2>&1; then
        echo "✅ Linked sub-issue via GraphQL API"
        return 0
    fi
    
    # Fallback: Add parent reference to issue body
    echo "⚠️  GraphQL linking failed, using fallback method"
    
    # Get current body
    local current_body
    current_body=$(gh issue view "$child_number" --json body --jq .body)
    
    # Add parent reference
    local updated_body="$current_body

---

**Parent Issue**: #$parent_number"
    
    # Update issue body
    if gh issue edit "$child_number" --body "$updated_body" >/dev/null 2>&1; then
        echo "✅ Added parent reference to sub-issue body"
        return 0
    fi
    
    echo "❌ Failed to link sub-issue"
    return 1
}

# Create parent issue
echo "Creating parent issue..."
PARENT_RESPONSE=$(gh issue create \
    --title "$PARENT_TITLE" \
    --body "$(cat "$PARENT_BODY_FILE")" \
    --label "epic,system,high-priority")

# Extract parent number from response
PARENT_NUMBER=$(echo "$PARENT_RESPONSE" | grep -oE '[0-9]+$')
PARENT_ID=$(get_issue_id "$PARENT_NUMBER")

echo "Created parent issue #$PARENT_NUMBER (ID: $PARENT_ID)"

# Create sub-issues
SUB_ISSUES=(
    "Sub-issue 1: Task description"
    "Sub-issue 2: Task description"
    "Sub-issue 3: Task description"
)

for sub_title in "${SUB_ISSUES[@]}"; do
    echo "Creating sub-issue: $sub_title"
    
    # Create sub-issue body with parent reference
    SUB_BODY="**Parent Issue**: #$PARENT_NUMBER

## Description
Task description for sub-issue...

## Implementation
- [ ] Step 1
- [ ] Step 2
- [ ] Step 3"
    
    # Create sub-issue
    SUB_RESPONSE=$(gh issue create \
        --title "$sub_title" \
        --body "$SUB_BODY" \
        --label "subtask,system")
    
    # Extract sub-issue number
    SUB_NUMBER=$(echo "$SUB_RESPONSE" | grep -oE '[0-9]+$')
    SUB_ID=$(get_issue_id "$SUB_NUMBER")
    
    echo "Created sub-issue #$SUB_NUMBER (ID: $SUB_ID)"
    
    # Link to parent
    if link_sub_issue "$PARENT_ID" "$SUB_ID"; then
        echo "✅ Successfully linked sub-issue #$SUB_NUMBER to parent #$PARENT_NUMBER"
    else
        echo "❌ Failed to link sub-issue #$SUB_NUMBER"
    fi
    
    echo "---"
done

echo "🎉 Created parent issue #$PARENT_NUMBER with ${#SUB_ISSUES[@]} sub-issues"
```

### Key Features of This Approach

1. **Dual Method Linking**: GraphQL API first, fallback to body references
2. **Error Handling**: Comprehensive error checking and graceful fallbacks
3. **ID Retrieval**: Proper GraphQL queries to get issue IDs
4. **Status Reporting**: Clear success/failure messages for each operation
5. **Robust Parsing**: Reliable extraction of issue numbers from responses
6. **Configuration**: Easy customization for different repositories

## Tips for Better Issues

### Writing Good Titles
- Be specific and concise
- Include the affected component
- Use action-oriented language
- Avoid vague terms like "Issue" or "Problem"

### Detailed Descriptions
- Explain the problem clearly
- Provide context and background
- Include relevant technical details
- Add screenshots or logs if helpful

### Clear Steps to Reproduce
- Make them easy to follow
- Include exact commands or inputs
- Note any prerequisites
- Provide expected vs actual results

### Environment Information
- Include version numbers
- Note operating system details
- List relevant dependencies
- Mention configuration settings

## Troubleshooting

### Commands Not Working
1. Check GitHub CLI installation: `gh --version`
2. Verify authentication: `gh auth status`
3. Ensure file paths are correct
4. Check file permissions and readability

### Issues Not Creating
1. Verify GitHub repository access
2. Check GitHub CLI authentication
3. Look for error messages in terminal
4. Try creating issues manually with `gh issue create`
5. For sub-issues, ensure GraphQL API access is available

### Sub-Issues Not Linking
1. **Check Repository Support**: Sub-issues require GitHub Enterprise or recent GitHub.com
2. **Verify GraphQL Access**: Test with `gh api graphql --help`
3. **Check Permissions**: Ensure write access to issues and GraphQL API
4. **Retrieve Issue IDs**: Use proper GraphQL queries to get issue IDs
5. **Handle Rate Limits**: Wait and retry if hitting GitHub API limits
6. **Try Fallback Method**: Add parent references to issue bodies manually
7. **Test GraphQL Manually**: `gh api graphql -H "GraphQL-Features: sub_issues" -f query='mutation { addSubIssue(input: {issueId: "parentID", subIssueId: "childID"}) { clientMutationId } }'`

### Permission Issues
If you encounter "Resource not accessible by integration" errors:
1. **Check Token Scopes**: Ensure your GitHub token has `issues:write` and `graphql:write` permissions
2. **Use Personal Access Token**: Generate a new PAT with full repo access
3. **Verify Repository Access**: Ensure you have write access to the target repository
4. **Check GitHub Enterprise Settings**: Some features may be disabled by organization policies

### Labels Not Applied
1. Check label spelling and format
2. Verify labels exist in repository
3. Ensure proper case sensitivity
4. Try creating issue without labels first

---

**Note**: This command requires GitHub CLI to be installed and properly authenticated. Use `gh auth login` to set up authentication with your GitHub account.