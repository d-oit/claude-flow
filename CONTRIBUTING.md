# Contributing to Claude Flow

Thank you for your interest in contributing to Claude Flow! This guide will help you get started with contributing to our AI orchestration platform using Claude Code Flow best practices.

## Ways to Contribute

- **Bug Reports**: Help us identify and fix issues
- **Feature Requests**: Suggest new capabilities
- **Documentation**: Improve guides and examples
- **Code Contributions**: Fix bugs or add features
- **AI Agents**: Create specialized AI agents
- **Commands**: Develop custom slash commands
- **MCPs**: Build Model Context Protocol integrations
- **Testing**: Add tests and improve coverage

## Quick Start

### Prerequisites

- **Node.js**: Version 20 or higher (recently migrated from Deno)
- **npm**: Version 9 or higher
- **Git**: For version control
- **Claude Code**: For AI-assisted development (required)
- **SQLite**: For memory persistence

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/claude-flow.git
   cd claude-flow
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Initialize Claude Code Flow**
   ```bash
   # Initialize Claude Code integration
   npx claude-flow init --template=development
   
   # Verify your setup
   npm run test:setup
   ```

4. **Start Development**
   ```bash
   npm run dev
   ```

## Claude Code Flow Development Workflow

### Core Commands and Best Practices

Claude Flow provides several specialized commands for AI-assisted development:

#### 1. Swarm Coordination
```bash
# Execute multi-agent swarms for complex tasks
claude-flow swarm "Build a REST API with authentication"
claude-flow swarm "Research cloud architecture patterns" --research
claude-flow swarm "Analyze system performance" --analysis --parallel

# Options:
# --strategy: auto, research, development, analysis
# --max-agents: Maximum number of agents (default: 5)
# --timeout: Timeout in minutes (default: 60)
# --parallel: Enable parallel execution
# --background: Run in background mode
```

#### 2. Hive Mind Agent Management
```bash
# Spawn specialized agents with specific capabilities
claude-flow hive-mind spawn coordinator --auto-assign
claude-flow hive-mind spawn researcher --capabilities "web-search,data-collection"
claude-flow hive-mind spawn coder --batch 3

# Interactive mode for complex agent configuration
claude-flow hive-mind spawn --interactive
```

#### 3. SPARC Development Methodology
```bash
# List available SPARC modes
claude-flow sparc modes

# Run specific SPARC mode for development
claude-flow sparc run code "implement user authentication system"
claude-flow sparc run architect "design microservices architecture"
claude-flow sparc run security-review "audit payment processing"

# Full TDD workflow
claude-flow sparc tdd "build a real-time chat application"

# Custom workflow execution
claude-flow sparc workflow project-workflow.json
```

#### 4. Maestro Specs-Driven Development
```bash
# Create feature specifications with collective intelligence
claude-flow maestro create-spec user-auth -r "Implement OAuth2 login with social providers"

# Generate technical designs from requirements
claude-flow maestro generate-design user-auth

# Generate implementation tasks
claude-flow maestro generate-tasks user-auth

# Implement specific tasks
claude-flow maestro implement-task user-auth 1
claude-flow maestro implement-task user-auth 2

# Review and approve phases
claude-flow maestro review-tasks user-auth
claude-flow maestro approve-phase user-auth

# Check workflow status
claude-flow maestro status user-auth --detailed
```

### Memory and State Management

Claude Flow uses a hybrid memory system (SQLite + Markdown) for persistence:

```bash
# Store development progress and findings
claude-flow memory store auth_progress "Completed OAuth2 implementation"
claude-flow memory store design_decisions "Chose JWT over session tokens"

# Retrieve previous work
claude-flow memory query auth
claude-flow memory query "authentication system"

# Use namespaces for different projects
claude-flow memory store project_alpha "Current status" --namespace alpha
claude-flow memory query --namespace alpha
```

### Testing with Claude Code Flow

#### Core Testing Suite
```bash
# Unit and integration tests
npm test
npm run test:integration

# AI-specific tests
npm run test:agents
npm run test:mcp
npm run test:claude-code

# Performance and memory tests
npm run test:performance
npm run test:memory
```

#### Agent Testing
```bash
# Test specific agent functionality
npm run test:agent coordinator
npm run test:agent researcher

# Test agent interactions
npm run test:agent-interactions

# Test command execution
npm run test:commands
```

### AI-Assisted Development Guidelines

#### Using Claude Code with Claude Flow

1. **Initialize Development Environment**
   ```bash
   # Set up Claude Code integration
   npx claude-flow init --template=development
   
   # Configure memory persistence
   claude-flow memory configure --storage sqlite
   ```

2. **Follow SPARC Methodology**
   - Use `spec-pseudocode` mode for requirements and planning
   - Use `architect` mode for system design
   - Use `code` mode for implementation
   - Use `tdd` mode for test-driven development
   - Use `integration` mode for system integration

3. **Leverage Hive Mind for Complex Tasks**
   ```bash
   # Spawn multiple specialized agents
   claude-flow hive-mind spawn architect --name "system-architect"
   claude-flow hive-mind spawn coder --batch 2
   claude-flow hive-mind spawn tester --auto-assign
   ```

4. **Use Swarm for Large-Scale Development**
   ```bash
   # Coordinate multiple agents for complex features
   claude-flow swarm "Implement real-time notification system" \
     --strategy development \
     --max-agents 8 \
     --parallel \
     --review
   ```

#### Example Development Workflow

```bash
# 1. Create feature specification
claude-flow maestro create-spec notifications \
  -r "Implement real-time push notifications for mobile and web"

# 2. Generate technical design
claude-flow maestro generate-design notifications

# 3. Create development plan
claude-flow maestro generate-tasks notifications

# 4. Implement using SPARC TDD
claude-flow sparc tdd "build notification service"

# 5. Use swarm for parallel implementation
claude-flow swarm "Implement notification API and database schema" \
  --max-agents 4 \
  --parallel

# 6. Review and integrate
claude-flow maestro review-tasks notifications
claude-flow maestro approve-phase notifications

# 7. Test and validate
npm run test:notifications
npm run test:integration
```

### Code Style and Quality

#### General Principles

- **Modular Architecture**: Keep files under 500 lines
- **TypeScript Strict Mode**: Use strict types with comprehensive JSDoc
- **AI-Friendly Code**: Structure code for AI tool understanding
- **Memory Integration**: Store key decisions and findings in memory

#### JavaScript/TypeScript Best Practices

```typescript
// Use descriptive names for AI tool understanding
const processUserNotification = async (userId: string, message: string) => {
  // Clear, testable methods
  const notification = await createNotification(userId, message);
  await sendToUser(notification);
  return notification;
};

// Include comprehensive JSDoc
/**
 * Processes user notifications with AI-powered routing
 * @param userId - Target user identifier
 * @param message - Notification content
 * @param priority - Notification priority level (1-5)
 * @returns Promise<Notification> Processed notification object
 */
async function processUserNotification(
  userId: string,
  message: string,
  priority: number = 3
): Promise<Notification> {
  // Implementation with AI-assisted routing
}
```

#### Agent Implementation Patterns

```typescript
// Agent implementations should be modular and testable
class NotificationAgent {
  constructor(private config: NotificationConfig) {}
  
  async sendNotification(userId: string, message: string): Promise<void> {
    // Clear, testable methods
    const user = await this.getUser(userId);
    const notification = await this.createNotification(user, message);
    await this.deliverNotification(notification);
  }
}
```

### Contribution Types

#### Bug Reports

Use our [bug report template](.github/ISSUE_TEMPLATE/bug_report.md):

```markdown
**Bug Description**
Clear description of the issue

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Node.js: [e.g., 20.9.0]
- Claude Flow: [e.g., 2.0.0]
- Claude Code: [e.g., latest]

**Reproduction Steps**
1. Run: claude-flow swarm "test task"
2. Observe: error in agent coordination
3. Expected: successful task completion

**AI Tool Context**
- Which swarm agents were involved
- Memory namespace used
- Any error messages from Claude Code integration
```

#### Feature Requests

Use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.md):

- Describe the problem you're solving
- Propose a solution using Claude Flow patterns
- Consider integration with existing commands
- Provide use cases with command examples

#### Contributing Agents

Agents are specialized AI assistants. To contribute an agent:

1. **Create Agent Structure**
   ```
   src/agents/your-agent/
   ├── agent.ts          # Agent implementation
   ├── prompts/          # Specialized prompts
   ├── examples/         # Usage examples
   └── tests/           # Agent tests
   ```

2. **Register with Hive Mind**
   ```typescript
   // Add to AGENT_TYPES in src/cli/commands/hive-mind/spawn.ts
   const AGENT_TYPES: AgentType[] = [
     // ... existing types
     'your-agent',
   ];
   
   const CAPABILITY_MAP: Record<AgentType, AgentCapability[]> = {
     your_agent: ['capability1', 'capability2'],
   };
   ```

3. **Testing**
   ```bash
   # Test agent functionality
   npm run test:agent your-agent
   
   # Test integration with swarm
   claude-flow swarm "test task" --max-agents 1
   ```

#### Contributing Commands

Commands extend Claude Flow's capabilities:

1. **Command Structure**
   ```
   src/cli/commands/your-command/
   ├── command.ts        # Command implementation
   ├── README.md        # Usage documentation
   └── tests/          # Command tests
   ```

2. **Integration with CLI**
   ```typescript
   // Add to src/cli/index.ts
   import { yourCommand } from './commands/your-command';
   
   program.addCommand(yourCommand);
   ```

### Development Workflow

#### 1. Planning with Claude Code Flow

```bash
# Create feature specification
claude-flow maestro create-spec new-feature \
  -r "Detailed feature description"

# Generate technical design
claude-flow maestro generate-design new-feature

# Create implementation tasks
claude-flow maestro generate-tasks new-feature
```

#### 2. Development with SPARC

```bash
# Use SPARC methodology for implementation
claude-flow sparc tdd "implement core functionality"

# Or use specific modes
claude-flow sparc run code "implement feature X"
claude-flow sparc run architect "design component Y"
```

#### 3. Testing and Validation

```bash
# Run comprehensive tests
npm test
npm run test:integration
npm run test:agents

# Test with Claude Code integration
npm run test:claude-code
```

#### 4. Documentation and Review

```bash
# Update documentation
claude-flow sparc run docs-writer "update feature documentation"

# Review implementation
claude-flow maestro review-tasks new-feature
```

### Code Quality Tools

```bash
# Linting and formatting
npm run lint
npm run lint:fix

# Type checking
npm run typecheck

# Security audit
npm audit
npm run security:check

# Performance analysis
npm run test:performance
```

### Memory and State Management Best Practices

```bash
# Store development progress
claude-flow memory store feature_progress "Current implementation status"

# Store design decisions
claude-flow memory store architecture_decisions "Key design choices made"

# Retrieve previous work
claude-flow memory query "feature implementation"
```

### Integration with External Tools

#### MCP (Model Context Protocol) Integration

```bash
# Test MCP integrations
npm run test:mcp

# Validate MCP protocol compliance
npm run test:mcp-protocol

# Test with different AI models
npm run test:mcp-models
```

#### Git Integration

```bash
# Commit with Claude Code assistance
claude-code commit --message "feat: add notification system"

# Review changes with AI
claude-code diff --review
```

### Troubleshooting Common Issues

#### Claude Code Integration Issues

```bash
# Reinitialize Claude Code integration
npx claude-flow init --template=development

# Check memory configuration
claude-flow memory status

# Test Claude Code connectivity
claude-flow test:claude-code
```

#### Agent Coordination Issues

```bash
# Check swarm status
claude-flow swarm status <swarm-id>

# Reset hive mind
claude-flow hive-mind reset

# Test individual agents
npm run test:agent <agent-type>
```

#### Memory Persistence Issues

```bash
# Check memory database
claude-flow memory status

# Repair memory database
claude-flow memory repair

# Backup memory
claude-flow memory backup
```

### Security Considerations

#### AI-Specific Security

- **Input Validation**: Always validate AI inputs and outputs
- **Prompt Injection**: Protect against malicious prompts
- **Data Privacy**: Be mindful of sensitive data in AI interactions
- **Code Execution**: Sandbox AI-generated code execution

#### General Security

- Follow our [Security Policy](SECURITY.md)
- Use secure coding practices
- Validate all user inputs
- Handle errors gracefully without exposing sensitive information

### Release Process

#### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

#### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Security review completed
- [ ] AI integration tested
- [ ] Performance benchmarks run
- [ ] Changelog updated

### Contribution Recognition

We recognize contributors in several ways:

- **Contributors List**: Added to README and documentation
- **Release Notes**: Mentioned in release announcements
- **Special Recognition**: For significant contributions
- **Maintainer Invitation**: For consistent, high-quality contributions

### Getting Help

#### Community Support

- **GitHub Discussions**: For questions and ideas
- **Issues**: For bugs and feature requests
- **Discord**: Real-time community chat

#### Maintainer Contact

- **Code Reviews**: Tag `@maintainers` in PRs
- **Security Issues**: Follow our [Security Policy](SECURITY.md)
- **Major Changes**: Create an RFC (Request for Comments)

### Best Practices for AI-Assisted Development

#### Effective AI Collaboration

1. **Clear Prompts**: Write specific, contextual prompts
2. **Iterative Refinement**: Use AI for initial drafts, then refine
3. **Human Oversight**: Always review AI-generated code
4. **Testing**: Thoroughly test AI-assisted contributions

#### Tool Integration Examples

```bash
# Complete workflow with Claude Code
claude-flow maestro create-spec new-feature
claude-flow sparc tdd "implement feature"
claude-flow swarm "parallel implementation" --max-agents 4
claude-flow maestro review-tasks new-feature
claude-flow sparc run docs-writer "update documentation"
```

### Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md). We're committed to providing a welcoming and inclusive environment for all contributors.

### License

By contributing to Claude Flow, you agree that your contributions will be licensed under the same license as the project (MIT License).

---

## Thank You!

Your contributions help make Claude Flow better for everyone. Whether you're fixing a typo, adding a feature, or creating a new AI agent, every contribution matters!

**Happy coding with Claude Code Flow!**

---

*This document is living and will be updated as our project evolves. If you have suggestions for improving this guide, please open an issue or submit a PR.*
