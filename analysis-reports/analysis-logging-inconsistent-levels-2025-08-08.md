# Issue Report

**Title**: Inconsistent Log Level Usage and Configuration Across Components

## Description
The Claude-Flow codebase suffers from inconsistent log level usage and configuration across different components. While some components properly use debug, info, warn, and error levels, others use inconsistent logging patterns or hardcoded log levels. This inconsistency makes it difficult to control logging verbosity, debug issues effectively, and maintain a consistent logging strategy across the system.

## Steps to Reproduce
1. Examine different components in the codebase
2. Observe inconsistent log level usage patterns
3. Note that some components use hardcoded log levels
4. Experience difficulty in controlling overall logging verbosity
5. Struggle to debug issues due to inconsistent logging

## Expected Behavior
- Consistent log level usage across all components
- Centralized log level configuration
- Appropriate log level selection for different types of messages
- Easy control of logging verbosity through configuration
- Clear separation between debug, info, warn, and error messages

## Actual Behavior
- Inconsistent log level usage across components
- Hardcoded log levels in some components
- No centralized log level configuration
- Difficulty controlling overall logging verbosity
- Unclear separation between different log levels

## Environment Details
- **Version**: Claude-Flow development version
- **OS**: Linux 6.8
- **Browser**: N/A (Node.js application)
- **Additional Components**: Logging system, configuration management

## Additional Context
### Current Implementation Issues:

#### Inconsistent Log Level Usage (manager.ts:172-176)
```typescript
this.logger.debug('Storing memory entry', {
  id: entry.id,
  type: entry.type,
  agentId: entry.agentId,
});
```

#### Hardcoded Log Levels (cli-core.ts:124-130)
```typescript
} catch (error) {
  console.error(
    chalk.red(`Error executing command '${commandName}':`),
    (error as Error).message,
  );
  if (flags.verbose) {
    console.error(error); // Always uses console.error
  }
  process.exit(1);
}
```

#### Mixed Logging Patterns (api/claude-api-errors.ts:226)
```typescript
let errorInfo = ERROR_MESSAGES.INTERNAL_SERVER_ERROR; // Default
if (error.status === 500) {
  errorInfo = ERROR_MESSAGES.INTERNAL_SERVER_ERROR;
} else if (error.status === 503) {
  errorInfo = ERROR_MESSAGES.SERVICE_UNAVAILABLE;
}
```

### Key Inconsistencies:
1. **Log Level Selection**: Some components use debug for everything, others use info
2. **Hardcoded Levels**: Some components have hardcoded log levels
3. **Configuration**: No centralized log level configuration
4. **Message Types**: No clear separation between different types of messages
5. **Verbosity Control**: Difficulty controlling overall logging verbosity

### Detailed Analysis:

#### Current Logging Patterns:
```typescript
// Component 1 - Uses debug for everything
this.logger.debug('Processing request', { requestId });

// Component 2 - Uses info for routine operations
this.logger.info('Memory operation completed', { operation: 'store' });

// Component 3 - Uses warn for non-critical issues
this.logger.warn('Memory limit approaching', { current: 80, limit: 100 });

// Component 4 - Uses error for everything
this.logger.error('Operation failed', { error });
```

### Impact on System:
- **Debugging Difficulty**: Hard to find relevant information in logs
- **Performance Issues**: Excessive logging impacts performance
- **Configuration Complexity**: Difficult to manage logging across components
- **Maintenance Burden**: High maintenance due to inconsistent patterns
- **User Experience**: Poor log visibility and control

### Affected Components:
- Memory Manager (`manager.ts`)
- CLI (`cli-core.ts`)
- API Layer (`api/`)
- Event Bus (`event-bus.ts`)
- Database Operations (`db/`)
- File System Operations (`fs/`)

## Priority Levels
- **Medium**: Affects debugging and maintenance
- **Impact**: All logging operations in the system
- **Risk**: Increased maintenance and debugging complexity

## Recommended Fixes

### Immediate Action (Medium)
1. **Implement Centralized Log Level Configuration**
   ```typescript
   export class LogLevelManager {
     private static config = new Map<string, LogLevel>();
     private static defaults = new Map<string, LogLevel>();
     
     static configure(component: string, level: LogLevel): void {
       this.config.set(component, level);
     }
     
     static setDefault(component: string, level: LogLevel): void {
       this.defaults.set(component, level);
     }
     
     static getLevel(component: string): LogLevel {
       // Check if component has specific configuration
       if (this.config.has(component)) {
         return this.config.get(component)!;
       }
       
       // Check if component has default configuration
       if (this.defaults.has(component)) {
         return this.defaults.get(component)!;
       }
       
       // Fall back to global default
       return LogLevel.INFO;
     }
     
     static shouldLog(component: string, level: LogLevel): boolean {
       const componentLevel = this.getLevel(component);
       return level >= componentLevel;
     }
     
     static configureFromGlobalConfig(globalConfig: LoggingConfig): void {
       // Configure all components based on global config
       const globalLevel = LogLevel[globalConfig.level.toUpperCase() as keyof typeof LogLevel];
       
       // Set defaults for all components
       this.setDefault('memory-manager', globalLevel);
       this.setDefault('api', globalLevel);
       this.setDefault('cli', globalLevel);
       this.setDefault('event-bus', globalLevel);
       this.setDefault('database', globalLevel);
       
       // Override specific components based on environment
       if (process.env.NODE_ENV === 'development') {
         this.configure('memory-manager', LogLevel.DEBUG);
         this.configure('api', LogLevel.DEBUG);
       }
       
       if (process.env.NODE_ENV === 'production') {
         this.configure('memory-manager', LogLevel.INFO);
         this.configure('api', LogLevel.WARN);
       }
     }
   }
   
   // Usage in components
   private log(level: LogLevel, message: string, data?: unknown): void {
     if (!LogLevelManager.shouldLog('memory-manager', level)) {
       return;
     }
     
     this.logger.log(level, message, data);
   }
   ```

2. **Add Log Level Guidelines and Best Practices**
   ```typescript
   export class LogGuidelines {
     static guidelines = {
       DEBUG: {
         description: 'Detailed information for debugging',
         examples: [
           'Function entry/exit points',
           'Variable values during processing',
           'Detailed operation steps',
           'Performance metrics',
         ],
         usage: 'Use during development and troubleshooting',
       },
       INFO: {
         description: 'General information about system operation',
         examples: [
           'System startup/shutdown',
           'Configuration changes',
           'Routine operations',
           'Status updates',
         ],
         usage: 'Use for normal system operation',
       },
       WARN: {
         description: 'Potentially harmful situations',
         examples: [
           'Deprecated functionality usage',
           'Poor performance detected',
           'Resource limits approaching',
           'Configuration issues',
         ],
         usage: 'Use for non-critical issues that need attention',
       },
       ERROR: {
         description: 'Error events that might still allow continued operation',
         examples: [
           'Failed operations',
           'Network connectivity issues',
           'Data validation failures',
           'Resource exhaustion',
         ],
         usage: 'Use for errors that affect system functionality',
       },
     };
     
     static getGuidelines(level: LogLevel): typeof this.guidelines.DEBUG {
       return this.guidelines[LogLevel[level]];
     }
     
     static suggestLevel(
       messageType: string,
       context: string,
       impact: 'low' | 'medium' | 'high'
     ): LogLevel {
       const messageLower = messageType.toLowerCase();
       const contextLower = context.toLowerCase();
       
       // Error detection
       if (messageLower.includes('error') || messageLower.includes('fail') || impact === 'high') {
         return LogLevel.ERROR;
       }
       
       // Warning detection
       if (messageLower.includes('warn') || messageLower.includes('deprecated') || impact === 'medium') {
         return LogLevel.WARN;
       }
       
       // Debug detection
       if (messageLower.includes('debug') || messageLower.includes('variable') || 
           messageLower.includes('performance') || contextLower.includes('development')) {
         return LogLevel.DEBUG;
       }
       
       // Default to info
       return LogLevel.INFO;
     }
   }
   
   // Usage in components
   private createLogMessage(
     type: string,
     message: string,
     context: string,
     impact: 'low' | 'medium' | 'high'
   ): { level: LogLevel; formattedMessage: string } {
     const level = LogGuidelines.suggestLevel(type, context, impact);
     const formattedMessage = `[${LogLevel[level]}] ${message}`;
     
     return { level, formattedMessage };
   }
   ```

3. **Implement Log Level Validation**
   ```typescript
   export class LogLevelValidator {
     static validateLogUsage(
       component: string,
       level: LogLevel,
       message: string,
       context?: unknown
     ): void {
       const issues: string[] = [];
       
       // Check for debug usage in production
       if (level === LogLevel.DEBUG && process.env.NODE_ENV === 'production') {
         issues.push('Debug logging should not be used in production');
       }
       
       // Check for error usage for non-errors
       if (level === LogLevel.ERROR && !this.isError(message)) {
         issues.push('Error level should only be used for actual errors');
       }
       
       // Check for warn usage for warnings
       if (level === LogLevel.WARN && !this.isWarning(message)) {
         issues.push('Warning level should only be used for warnings');
       }
       
       // Check for log level consistency
       if (this.isInconsistentLevel(level, message)) {
         issues.push('Log level is inconsistent with message content');
       }
       
       // Report issues
       if (issues.length > 0) {
         console.warn(`Log level validation issues in ${component}:`, issues);
         console.warn('Message:', message);
         console.warn('Context:', context);
       }
     }
     
     private static isError(message: string): boolean {
       const errorKeywords = ['error', 'fail', 'exception', 'critical', 'fatal'];
       return errorKeywords.some(keyword => 
         message.toLowerCase().includes(keyword)
       );
     }
     
     private static isWarning(message: string): boolean {
       const warningKeywords = ['warn', 'deprecated', 'slow', 'limit', 'threshold'];
       return warningKeywords.some(keyword => 
         message.toLowerCase().includes(keyword)
       );
     }
     
     private static isInconsistentLevel(level: LogLevel, message: string): boolean {
       const messageLower = message.toLowerCase();
       
       if (level === LogLevel.DEBUG && !messageLower.includes('debug')) {
         return false; // Debug can be used for any detailed information
       }
       
       if (level === LogLevel.INFO && messageLower.includes('error')) {
         return true; // Info should not be used for errors
       }
       
       if (level === LogLevel.WARN && messageLower.includes('error')) {
         return true; // Warning should not be used for errors
       }
       
       return false;
     }
   }
   
   // Usage in logger
     private log(level: LogLevel, message: string, data?: unknown): void {
       LogLevelValidator.validateLogUsage(
         this.context.component || 'unknown',
         level,
         message,
         data
       );
       
       // Rest of logging logic
     }
   ```

### Medium-term Improvements
1. **Add Log Level Analytics**
2. **Implement Log Level Optimization**
3. **Add Log Level Recommendations**
4. **Implement Log Level Auto-Configuration**

### Long-term Optimizations
1. **Add Log Level Machine Learning**
2. **Implement Log Level Predictive Analytics**
3. **Add Log Level Performance Monitoring**
4. **Implement Log Level Auto-Tuning**

## Impact Assessment
- **Severity**: Medium - Affects debugging and maintenance
- **Frequency**: Always - Every logging operation is affected
- **User Impact**: Medium - Poor log visibility and control
- **Business Impact**: Medium - Increased maintenance and debugging costs
- **Performance Impact**: Low - Minimal overhead from validation

## Testing Recommendations
1. **Log Level Tests**: Verify consistent log level usage
2. **Configuration Tests**: Test centralized log level configuration
3. **Validation Tests**: Test log level validation
4. **Guideline Tests**: Test log level guidelines and suggestions
5. **Integration Tests**: Test logging across components
6. **Performance Tests**: Measure impact of log level validation

## Monitoring Requirements
1. **Log Level Usage**: Monitor log level usage patterns
2. **Configuration Compliance**: Monitor compliance with log level guidelines
3. **Validation Issues**: Monitor log level validation issues
4. **Performance Impact**: Monitor performance impact of logging
5. **Debugging Effectiveness**: Monitor debugging effectiveness
6. **Alerts**: Alert on unusual log level patterns