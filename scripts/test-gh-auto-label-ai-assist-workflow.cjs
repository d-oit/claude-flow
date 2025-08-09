#!/usr/bin/env node

/**
 * Comprehensive Test Suite for GitHub Workflow: auto-label-ai-assist.yml
 * Tests workflow logic, configuration, and integration scenarios
 */

const fs = require('fs');
const path = require('path');
let yaml;

class WorkflowTester {
  constructor() {
    this.workflowPath = '.github/workflows/auto-label-ai-assist.yml';
    this.labelsPath = '.github/labels.yml';
    this.labelerPath = '.github/labeler.yml';
    this.testResults = [];
    this.workflow = null;
    this.labels = null;
    this.labeler = null;
  }

  async runAllTests() {
    console.log('🧪 Starting Comprehensive Workflow Tests\n');
    
    try {
      await this.loadConfigurations();
      await this.testWorkflowStructure();
      await this.testJobDependencies();
      await this.testEnvironmentVariables();
      await this.testLabelingLogic();
      await this.testAIAssistLogic();
      await this.testTriageLogic();
      await this.testSecurityConfiguration();
      await this.testPerformanceSettings();
      await this.testErrorHandling();
      
      this.printResults();
    } catch (error) {
      console.error('❌ Test suite failed:', error.message);
      process.exit(1);
    }
  }

  async loadConfigurations() {
    console.log('📁 Loading workflow configurations...');
    
    try {
      // Load workflow YAML
      const workflowContent = fs.readFileSync(this.workflowPath, 'utf8');
      this.workflow = yaml.load(workflowContent);
      
      // Load labels configuration
      const labelsContent = fs.readFileSync(this.labelsPath, 'utf8');
      this.labels = yaml.load(labelsContent);
      
      // Load labeler configuration
      const labelerContent = fs.readFileSync(this.labelerPath, 'utf8');
      this.labeler = yaml.load(labelerContent);
      
      this.addResult('✅ Configuration Loading', 'All configuration files loaded successfully');
    } catch (error) {
      this.addResult('❌ Configuration Loading', `Failed to load configurations: ${error.message}`);
      throw error;
    }
  }

  async testWorkflowStructure() {
    console.log('🏗️ Testing workflow structure...');
    
    // Test basic structure
    this.assert(this.workflow.name, 'Workflow has a name');
    this.assert(this.workflow.on, 'Workflow has triggers');
    this.assert(this.workflow.jobs, 'Workflow has jobs');
    this.assert(this.workflow.permissions, 'Workflow has permissions');
    
    // Test required jobs
    const requiredJobs = ['validate', 'auto-label', 'ai-assist', 'triage-assist', 'monitoring'];
    requiredJobs.forEach(job => {
      this.assert(this.workflow.jobs[job], `Job '${job}' exists`);
    });
    
    // Test triggers
    this.assert(this.workflow.on.issues, 'Issues trigger configured');
    this.assert(this.workflow.on.pull_request, 'Pull request trigger configured');
    
    const issueTypes = this.workflow.on.issues.types;
    const prTypes = this.workflow.on.pull_request.types;
    
    ['opened', 'edited', 'reopened'].forEach(type => {
      this.assert(issueTypes.includes(type), `Issue trigger includes '${type}'`);
      this.assert(prTypes.includes(type), `PR trigger includes '${type}'`);
    });
    
    this.addResult('✅ Workflow Structure', 'All structural tests passed');
  }

  async testJobDependencies() {
    console.log('🔗 Testing job dependencies...');
    
    const jobs = this.workflow.jobs;
    
    // Test dependency chain
    this.assert(!jobs.validate.needs, 'Validate job has no dependencies (runs first)');
    this.assert(jobs['auto-label'].needs.includes('validate'), 'Auto-label depends on validate');
    this.assert(jobs['ai-assist'].needs.includes('validate'), 'AI-assist depends on validate');
    this.assert(jobs['ai-assist'].needs.includes('auto-label'), 'AI-assist depends on auto-label');
    this.assert(jobs['triage-assist'].needs.includes('validate'), 'Triage-assist depends on validate');
    this.assert(jobs['triage-assist'].needs.includes('auto-label'), 'Triage-assist depends on auto-label');
    
    // Test monitoring job depends on all others
    const monitoringNeeds = jobs.monitoring.needs;
    ['validate', 'auto-label', 'ai-assist', 'triage-assist'].forEach(job => {
      this.assert(monitoringNeeds.includes(job), `Monitoring depends on '${job}'`);
    });
    
    this.addResult('✅ Job Dependencies', 'All dependency tests passed');
  }

  async testEnvironmentVariables() {
    console.log('🌍 Testing environment variables...');
    
    const env = this.workflow.env;
    
    // Test required environment variables
    const requiredEnvVars = [
      'MAX_COMMENT_LENGTH',
      'MIN_BODY_LENGTH',
      'API_RETRY_COUNT',
      'API_RETRY_DELAY',
      'ENABLE_AI_SUGGESTIONS',
      'ENABLE_PRIORITY_LABELING',
      'ENABLE_GOOD_FIRST_ISSUE_COMMENTS',
      'WORKFLOW_VERSION',
      'TELEMETRY_ENABLED'
    ];
    
    requiredEnvVars.forEach(envVar => {
      this.assert(env[envVar] !== undefined, `Environment variable '${envVar}' is defined`);
    });
    
    // Test environment variable values
    this.assert(parseInt(env.MAX_COMMENT_LENGTH) > 0, 'MAX_COMMENT_LENGTH is a positive number');
    this.assert(parseInt(env.MIN_BODY_LENGTH) > 0, 'MIN_BODY_LENGTH is a positive number');
    this.assert(parseInt(env.API_RETRY_COUNT) > 0, 'API_RETRY_COUNT is a positive number');
    this.assert(['true', 'false', true, false].includes(env.ENABLE_AI_SUGGESTIONS), 'ENABLE_AI_SUGGESTIONS is boolean');
    this.assert(['true', 'false', true, false].includes(env.TELEMETRY_ENABLED), 'TELEMETRY_ENABLED is boolean');
    
    this.addResult('✅ Environment Variables', 'All environment variable tests passed');
  }

  async testLabelingLogic() {
    console.log('🏷️ Testing labeling logic...');
    
    // Test labels configuration
    this.assert(typeof this.labels === 'object', 'Labels configuration is valid');
    
    // Test labeler configuration
    this.assert(Array.isArray(this.labeler), 'Labeler configuration is an array');
    
    // Test required labels exist
    const requiredLabels = ['bug', 'enhancement', 'documentation', 'good first issue'];
    requiredLabels.forEach(label => {
      this.assert(this.labels[label] !== undefined, `Required label '${label}' exists`);
    });
    
    // Test labeler rules
    const labelRules = this.labeler;
    const ruleLabels = labelRules.map(rule => rule.label);
    
    requiredLabels.forEach(label => {
      this.assert(ruleLabels.includes(label), `Labeler rule exists for '${label}'`);
    });
    
    // Test regex patterns
    labelRules.forEach(rule => {
      this.assert(rule.any || rule.all, `Rule for '${rule.label}' has conditions`);
      if (rule.any) {
        rule.any.forEach(condition => {
          if (typeof condition === 'object' && condition.title) {
            console.log(`Debug: Checking title regex for '${rule.label}': ${JSON.stringify(condition.title)}`);
            this.assert(this.isValidRegex(condition.title), `Title regex for '${rule.label}' is valid`);
          }
          if (typeof condition === 'object' && condition.body) {
            this.assert(this.isValidRegex(condition.body), `Body regex for '${rule.label}' is valid`);
          }
        });
      }
    });
    
    this.addResult('✅ Labeling Logic', 'All labeling tests passed');
  }

  async testAIAssistLogic() {
    console.log('🤖 Testing AI assist logic...');
    
    const aiJob = this.workflow.jobs['ai-assist'];
    
    // Test AI assist job configuration
    this.assert(aiJob.strategy, 'AI assist has strategy configuration');
    this.assert(aiJob.strategy.matrix, 'AI assist has matrix strategy');
    this.assert(aiJob.strategy['fail-fast'] === false, 'AI assist continues on failure');
    
    // Test suggestion types
    const suggestionTypes = aiJob.strategy.matrix['suggestion-type'];
    const expectedTypes = ['bug-report', 'enhancement', 'documentation'];
    
    expectedTypes.forEach(type => {
      this.assert(suggestionTypes.includes(type), `AI assist includes '${type}' suggestions`);
    });
    
    // Test conditional execution
    this.assert(aiJob.if.includes('ENABLE_AI_SUGGESTIONS'), 'AI assist respects feature flag');
    this.assert(aiJob.if.includes('skip-processing'), 'AI assist skips when appropriate');
    
    this.addResult('✅ AI Assist Logic', 'All AI assist tests passed');
  }

  async testTriageLogic() {
    console.log('🎯 Testing triage logic...');
    
    const triageJob = this.workflow.jobs['triage-assist'];
    
    // Test triage job configuration
    this.assert(triageJob.steps, 'Triage job has steps');
    this.assert(triageJob.if.includes('skip-processing'), 'Triage respects skip flag');
    
    // Test priority labeling step
    const priorityStep = triageJob.steps.find(step => 
      step.name && step.name.includes('priority labeling')
    );
    this.assert(priorityStep, 'Priority labeling step exists');
    this.assert(priorityStep.if.includes('ENABLE_PRIORITY_LABELING'), 'Priority labeling respects feature flag');
    
    // Test good first issue step
    const goodFirstIssueStep = triageJob.steps.find(step => 
      step.name && step.name.includes('Good first issue')
    );
    this.assert(goodFirstIssueStep, 'Good first issue step exists');
    this.assert(goodFirstIssueStep.if.includes('ENABLE_GOOD_FIRST_ISSUE_COMMENTS'), 'Good first issue respects feature flag');
    
    this.addResult('✅ Triage Logic', 'All triage tests passed');
  }

  async testSecurityConfiguration() {
    console.log('🔒 Testing security configuration...');
    
    const permissions = this.workflow.permissions;
    
    // Test minimal permissions
    this.assert(permissions.issues === 'write', 'Issues permission is write-only');
    this.assert(permissions['pull-requests'] === 'write', 'PR permission is write-only');
    this.assert(permissions.contents === 'read', 'Contents permission is read-only');
    this.assert(permissions.actions === 'read', 'Actions permission is read-only');
    
    // Test no excessive permissions
    const allowedPermissions = ['issues', 'pull-requests', 'contents', 'actions'];
    Object.keys(permissions).forEach(permission => {
      this.assert(allowedPermissions.includes(permission), `Permission '${permission}' is allowed`);
    });
    
    // Test checkout security
    const autoLabelJob = this.workflow.jobs['auto-label'];
    const checkoutStep = autoLabelJob.steps.find(step => step.uses && step.uses.includes('checkout'));
    
    if (checkoutStep && checkoutStep.with) {
      this.assert(checkoutStep.with['persist-credentials'] === false, 'Checkout disables credential persistence');
      this.assert(checkoutStep.with['fetch-depth'] === 1, 'Checkout uses shallow clone');
    }
    
    this.addResult('✅ Security Configuration', 'All security tests passed');
  }

  async testPerformanceSettings() {
    console.log('⚡ Testing performance settings...');
    
    const jobs = this.workflow.jobs;
    
    // Test timeout settings
    Object.entries(jobs).forEach(([jobName, job]) => {
      this.assert(job['timeout-minutes'], `Job '${jobName}' has timeout configured`);
      this.assert(parseInt(job['timeout-minutes']) <= 10, `Job '${jobName}' timeout is reasonable`);
    });
    
    // Test concurrency settings
    this.assert(this.workflow.concurrency, 'Workflow has concurrency configuration');
    this.assert(this.workflow.concurrency.group, 'Concurrency group is defined');
    this.assert(this.workflow.concurrency['cancel-in-progress'] === false, 'Concurrency preserves data consistency');
    
    // Test caching in auto-label job
    const autoLabelJob = this.workflow.jobs['auto-label'];
    const cacheStep = autoLabelJob.steps.find(step => step.uses && step.uses.includes('cache'));
    this.assert(cacheStep, 'Auto-label job uses caching');
    
    this.addResult('✅ Performance Settings', 'All performance tests passed');
  }

  async testErrorHandling() {
    console.log('🛡️ Testing error handling...');
    
    const jobs = this.workflow.jobs;
    
    // Test continue-on-error settings
    const autoLabelJob = jobs['auto-label'];
    const labelSteps = autoLabelJob.steps.filter(step => 
      step['continue-on-error'] === true
    );
    this.assert(labelSteps.length > 0, 'Auto-label has error-tolerant steps');
    
    // Test retry configuration
    const aiJob = jobs['ai-assist'];
    const scriptStep = aiJob.steps.find(step => 
      step.uses && step.uses.includes('github-script')
    );
    if (scriptStep && scriptStep.with) {
      this.assert(scriptStep.with.retries, 'AI assist has retry configuration');
    }
    
    // Test monitoring always runs
    const monitoringJob = jobs.monitoring;
    this.assert(monitoringJob.if.includes('always()'), 'Monitoring always runs for observability');
    
    this.addResult('✅ Error Handling', 'All error handling tests passed');
  }

  // Helper methods
  assert(condition, message) {
    if (!condition) {
      throw new Error(`Assertion failed: ${message}`);
    }
  }

  addResult(status, message) {
    this.testResults.push({ status, message });
    console.log(`  ${status} ${message}`);
  }

  isValidRegex(pattern) {
    try {
      // Handle different regex formats
      if (typeof pattern === 'string') {
        // Extract regex from pattern (remove /.../ wrapper if present)
        const regexMatch = pattern.match(/^\/(.+)\/([gimuy]*)$/);
        if (regexMatch) {
          let regexBody = regexMatch[1];
          let flags = regexMatch[2];
          
          // Handle (?i) inline case-insensitive flag (convert to 'i' flag)
          if (regexBody.startsWith('(?i)')) {
            regexBody = regexBody.substring(4);
            if (!flags.includes('i')) {
              flags += 'i';
            }
          }
          
          new RegExp(regexBody, flags);
          return true;
        } else {
          // Try as plain string (might be a simple pattern)
          new RegExp(pattern);
          return true;
        }
      }
      return false;
    } catch (error) {
      return false;
    }
  }

  printResults() {
    console.log('\n📊 Test Results Summary');
    console.log('========================');
    
    const passed = this.testResults.filter(r => r.status.includes('✅')).length;
    const failed = this.testResults.filter(r => r.status.includes('❌')).length;
    
    console.log(`✅ Passed: ${passed}`);
    console.log(`❌ Failed: ${failed}`);
    console.log(`📊 Total: ${this.testResults.length}`);
    
    if (failed > 0) {
      console.log('\n❌ Failed Tests:');
      this.testResults
        .filter(r => r.status.includes('❌'))
        .forEach(r => console.log(`  ${r.message}`));
      process.exit(1);
    } else {
      console.log('\n🎉 All tests passed! Workflow is ready for production.');
    }
  }
}

// Check if js-yaml is available, if not provide a simple YAML parser
try {
  yaml = require('js-yaml');
} catch (error) {
  console.log('📦 js-yaml not found, using simple YAML parser...');
  // Simple YAML parser fallback
  const simpleYaml = {
    load: (content) => {
      // Simple YAML parser for basic structures
      const lines = content.split('\n');
      const result = {};
      let currentKey = null;
      let currentObject = result;
      const stack = [result];
      
      for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) continue;
        
        const indent = line.length - line.trimLeft().length;
        const colonIndex = trimmed.indexOf(':');
        
        if (colonIndex > 0) {
          const key = trimmed.substring(0, colonIndex).trim();
          const value = trimmed.substring(colonIndex + 1).trim();
          
          if (value) {
            // Simple value
            currentObject[key] = value === 'true' ? true : value === 'false' ? false : 
                               !isNaN(value) ? Number(value) : value;
          } else {
            // Object start
            currentObject[key] = {};
            currentObject = currentObject[key];
            currentKey = key;
          }
        }
      }
      
      return result;
    }
  };
  yaml = simpleYaml;
}

// Run tests if this file is executed directly
if (require.main === module) {
  const tester = new WorkflowTester();
  tester.runAllTests().catch(error => {
    console.error('Test execution failed:', error);
    process.exit(1);
  });
}

module.exports = WorkflowTester;