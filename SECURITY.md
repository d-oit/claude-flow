# Security Policy

## 🔒 Supported Versions

We actively support the following versions of Claude Flow with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 2.0.x   | ✅ Yes             |
| 1.x.x   | ⚠️ Critical fixes only |
| < 1.0   | ❌ No             |

## 🚨 Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### 1. **DO NOT** create a public GitHub issue

### 2. Report privately via one of these methods:

- **GitHub Security Advisory**: Use GitHub's [private vulnerability reporting](https://github.com/ruvnet/claude-flow/security/advisories/new)
- **Direct Contact**: Reach out to maintainers privately

### 3. Include the following information:

- **Description**: Clear description of the vulnerability
- **Impact**: What could an attacker accomplish?
- **Reproduction**: Step-by-step instructions to reproduce
- **Environment**: OS, Node.js version, Claude Flow version
- **Proof of Concept**: Code or screenshots (if applicable)

## 🛡️ Security Considerations for AI Tools

### Claude Code Integration Security

Since Claude Flow integrates with Claude Code and AI tools, please be aware of:

1. **API Key Security**
   - Never commit API keys to version control
   - Use environment variables or secure credential stores
   - Rotate keys regularly

2. **Code Execution Risks**
   - AI-generated code should be reviewed before execution
   - Use sandboxed environments for testing AI suggestions
   - Be cautious with file system operations

3. **Data Privacy**
   - Sensitive data may be sent to AI services
   - Review what data is shared with external APIs
   - Consider using local models for sensitive projects

### Agent and MCP Security

1. **Agent Validation**
   - Only install agents from trusted sources
   - Review agent code before installation
   - Monitor agent behavior and permissions

2. **MCP (Model Context Protocol) Security**
   - Validate MCP server configurations
   - Use secure connections (HTTPS/WSS)
   - Limit MCP server permissions

## 🔐 Security Best Practices

### For Contributors

1. **Code Review**
   - All code must be reviewed by at least one maintainer
   - Security-sensitive changes require additional review
   - Use static analysis tools

2. **Dependencies**
   - Keep dependencies updated
   - Use `npm audit` to check for vulnerabilities
   - Pin dependency versions in production

3. **Testing**
   - Include security tests in your contributions
   - Test with various input types and edge cases
   - Validate input sanitization

### For Users

1. **Installation Security**
   ```bash
   # Verify package integrity
   npm audit
   
   # Use specific versions
   npm install claude-flow@2.0.0
   
   # Check for known vulnerabilities
   npm audit --audit-level=moderate
   ```

2. **Configuration Security**
   - Use secure file permissions for `.claude/` directory
   - Regularly review installed agents and commands
   - Monitor system resource usage

3. **Network Security**
   - Use HTTPS for all external connections
   - Validate SSL certificates
   - Consider using VPN for sensitive work

## 🚀 AI-Specific Security Guidelines

### Working with Claude Code

1. **Prompt Injection Prevention**
   - Sanitize user inputs in prompts
   - Use structured prompts with clear boundaries
   - Validate AI responses before execution

2. **Code Generation Safety**
   - Review all AI-generated code
   - Test in isolated environments first
   - Use version control for all changes

3. **Data Handling**
   - Minimize sensitive data in prompts
   - Use data masking when possible
   - Understand data retention policies

### Agent Development Security

1. **Secure Agent Design**
   ```yaml
   # Example secure agent configuration
   agent:
     name: "secure-example"
     permissions:
       - "read:files"
       - "write:temp"
     restrictions:
       - "no-network"
       - "no-system-calls"
   ```

2. **Input Validation**
   - Validate all user inputs
   - Sanitize file paths
   - Limit resource consumption

## 📋 Security Checklist

### Before Contributing

- [ ] Code has been reviewed for security issues
- [ ] Dependencies are up to date
- [ ] No hardcoded secrets or credentials
- [ ] Input validation is implemented
- [ ] Error handling doesn't leak sensitive information

### Before Deployment

- [ ] Security audit completed
- [ ] All tests pass including security tests
- [ ] Documentation updated with security considerations
- [ ] Monitoring and logging configured

## 🔄 Response Timeline

We aim to respond to security reports according to this timeline:

- **Initial Response**: Within 48 hours
- **Assessment**: Within 1 week
- **Fix Development**: 2-4 weeks (depending on severity)
- **Public Disclosure**: After fix is released and users have time to update

## 🏆 Security Hall of Fame

We recognize security researchers who help improve Claude Flow's security:

<!-- Future contributors will be listed here -->

## 📞 Contact

For security-related questions or concerns:

- **Security Team**: Create a private security advisory on GitHub
- **General Security Questions**: Open a discussion in our GitHub Discussions
- **Documentation**: Check our [security documentation](docs/security-guide.md)

## 📜 Legal

This security policy is subject to our [Terms of Service](LICENSE) and [Code of Conduct](CODE_OF_CONDUCT.md).

---

**Remember**: Security is everyone's responsibility. When in doubt, ask questions and err on the side of caution.