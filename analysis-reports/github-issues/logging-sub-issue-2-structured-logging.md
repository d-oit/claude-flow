# 🔴 Sub-Issue #2: Missing Structured Logging and Context Propagation

**Title**: [Critical] Implement Structured Logging and Context Propagation

**Priority**: High  
**Type**: Feature  
**Component**: Logging System  
**Labels**: `logging`, `structured`, `context`, `feature`, `critical`

**Parent Issue**: [`logging-main-issue.md`](./logging-main-issue.md)

## Description

The Claude-Flow logging system lacks structured logging capabilities and proper context propagation. While the logger supports basic JSON formatting, it doesn't provide consistent structured logging across all components, making it difficult to parse, filter, and analyze logs programmatically. Additionally, context information is not properly propagated through the system, leading to logs that lack the necessary context for effective debugging and monitoring.

## Problem Statement

The current logging implementation suffers from inconsistent structured logging and missing context propagation, which severely impacts debugging and monitoring capabilities. Without proper structured logging and context management, it's difficult to correlate log entries across components and analyze system behavior effectively.

## Steps to Trace

1. Examine logging calls across different components
2. Observe inconsistent use of structured logging
3. Note missing context information in log messages
4. Experience difficulty parsing and filtering logs
5. Struggle to correlate log entries across components

## Expected Behavior

- Consistent structured logging across all components
- Proper context propagation through the system
- Machine-readable log formats (JSON)
- Rich metadata in all log messages
- Easy correlation of log entries across components

## Actual Behavior

- Inconsistent structured logging implementation
- Missing context information in log messages
- Mix of JSON and text log formats
- Limited metadata in log entries
- Difficult correlation of log entries across components

## Environment Details

- **Version**: Claude-Flow development version
- **OS**: Linux 6.8
- **Browser**: N/A (Node.js application)
- **Additional Components**: Logging system, monitoring, debugging

## Current Implementation Issues

### Basic JSON Logging (`src/logger/logger.ts:174-186`)

```typescript
private format(entry: LogEntry): string {
  if (this.config.format === 'json') {
    // Handle error serialization for JSON format
    const jsonEntry = { ...entry };
    if (jsonEntry.error instanceof Error) {
      jsonEntry.error = {
        name: jsonEntry.error.name,
        message: jsonEntry.error.message,
        stack: jsonEntry.error.stack,
      };
    }
    return JSON.stringify(jsonEntry);
  }
  
  // Text format - lacks structure
  const contextStr = Object.keys(entry.context).length > 0 ? ` ${JSON.stringify(entry.context)}` : '';
  const dataStr = entry.data !== undefined ? ` ${JSON.stringify(entry.data)}` : '';
  const errorStr = entry.error !== undefined ? ` Error: ${JSON.stringify(entry.error)}` : '';
  
  return `[${entry.timestamp}] ${entry.level} ${entry.message}${contextStr}${dataStr}${errorStr}`;
}
```

### Inconsistent Context Usage (`src/manager.ts:172-176`)

```typescript
this.logger.debug('Storing memory entry', {
  id: entry.id,
  type: entry.type,
  agentId: entry.agentId,
});
```

### Missing Trace Information (`src/api/claude-api-errors.ts:226`)

```typescript
let errorInfo = ERROR_MESSAGES.INTERNAL_SERVER_ERROR; // No trace context
```

### Key Problems

1. **Inconsistent Structure**: Some components use JSON, others use text
2. **Missing Context**: No consistent context propagation
3. **No Trace IDs**: No correlation IDs across components
4. **Limited Metadata**: Insufficient metadata in log entries
5. **No Structured Fields**: No consistent field names and types

### Current Logging Structure

```typescript
// Component 1 - Basic JSON
{
  "timestamp": "2025-01-08T13:45:00.000Z",
  "level": "DEBUG",
  "message": "Storing memory entry",
  "context": {
    "id": "mem_123",
    "type": "observation",
    "agentId": "agent_456"
  }
}

// Component 2 - Text format
[2025-01-08T13:45:00.000Z] INFO Memory operation completed operation=store

// Component 3 - Mixed structure
{
  "timestamp": "2025-01-08T13:45:00.000Z",
  "level": "ERROR",
  "message": "Failed to process request",
  "error": {
    "name": "Error",
    "message": "Request failed",
    "stack": "..."
  }
}
```

## Impact on System

- **Debugging Difficulty**: Hard to correlate log entries across components
- **Monitoring Challenges**: Difficult to parse and filter logs programmatically
- **Performance Issues**: Inefficient log parsing and analysis
- **Maintenance Burden**: High maintenance due to inconsistent formats
- **User Experience**: Poor log visibility and analysis capabilities

### Affected Components

- Memory Manager (`src/manager.ts`)
- API Layer (`src/api/`)
- CLI (`src/cli-core.ts`)
- Event Bus (`src/event-bus.ts`)
- Database Operations (`src/db/`)
- File System Operations (`src/fs/`)

## Recommended Fixes

### Immediate Action (High)

#### 1. Implement Structured Logging Framework

```typescript
export interface StructuredLogEntry {
  // Standard fields
  timestamp: string;
  level: LogLevel;
  message: string;
  logger: string;
  
  // Correlation fields
  traceId?: string;
  spanId?: string;
  parentId?: string;
  sessionId?: string;
  userId?: string;
  
  // Context fields
  component: string;
  operation: string;
  version: string;
  
  // Metadata
  metadata: Record<string, unknown>;
  tags: string[];
  
  // Error information
  error?: {
    name: string;
    message: string;
    stack?: string;
    code?: string;
    type?: string;
  };
  
  // Performance metrics
  performance?: {
    duration?: number;
    memoryUsage?: number;
    cpuUsage?: number;
  };
}

export class StructuredLogger implements ILogger {
  private static traceContext = new Map<string, StructuredLogEntry>();
  private static correlationIdGenerator = new CorrelationIdGenerator();
  
  constructor(
    private component: string,
    private config: LoggingConfig
  ) {}
  
  private createStructuredLog(
    level: LogLevel,
    message: string,
    data?: unknown,
    error?: unknown,
    operation?: string
  ): StructuredLogEntry {
    const traceId = this.getTraceId();
    const parentId = this.getParentTraceId();
    
    return {
      timestamp: new Date().toISOString(),
      level,
      message,
      logger: this.component,
      traceId,
      spanId: this.generateSpanId(),
      parentId,
      component: this.component,
      operation: operation || 'unknown',
      version: process.env.npm_package_version || 'unknown',
      metadata: this.extractMetadata(data),
      tags: this.extractTags(data),
      error: this.formatError(error),
      performance: this.extractPerformanceMetrics(data),
    };
  }
  
  private getTraceId(): string {
    // Get trace ID from current context or generate new one
    return this.getFromContext('traceId') || this.generateTraceId();
  }
  
  private generateTraceId(): string {
    return this.correlationIdGenerator.generate();
  }
  
  private generateSpanId(): string {
    return this.correlationIdGenerator.generate();
  }
  
  private extractMetadata(data: unknown): Record<string, unknown> {
    if (!data || typeof data !== 'object') {
      return {};
    }
    
    const metadata: Record<string, unknown> = {};
    
    // Extract known metadata fields
    if ('metadata' in data) {
      metadata.metadata = (data as any).metadata;
    }
    
    // Extract other fields that are not tags or performance
    for (const [key, value] of Object.entries(data)) {
      if (!['tags', 'performance', 'metadata'].includes(key)) {
        metadata[key] = value;
      }
    }
    
    return metadata;
  }
  
  private extractTags(data: unknown): string[] {
    if (!data || typeof data !== 'object') {
      return [];
    }
    
    return (data as any).tags || [];
  }
  
  private formatError(error: unknown): {
    name: string;
    message: string;
    stack?: string;
    code?: string;
    type?: string;
  } | undefined {
    if (!error) {
      return undefined;
    }
    
    if (error instanceof Error) {
      return {
        name: error.name,
        message: error.message,
        stack: error.stack,
        code: (error as any).code,
        type: error.constructor.name,
      };
    }
    
    if (typeof error === 'string') {
      return {
        name: 'Error',
        message: error,
      };
    }
    
    return {
      name: 'UnknownError',
      message: JSON.stringify(error),
    };
  }
  
  private extractPerformanceMetrics(data: unknown): {
    duration?: number;
    memoryUsage?: number;
    cpuUsage?: number;
  } | undefined {
    if (!data || typeof data !== 'object') {
      return undefined;
    }
    
    return (data as any).performance;
  }
  
  private getFromContext(key: string): string | undefined {
    // Get value from context (could be from async hooks or thread local storage)
    return undefined; // Implementation depends on context management
  }
  
  debug(message: string, meta?: unknown): void {
    this.log(LogLevel.DEBUG, message, meta);
  }
  
  info(message: string, meta?: unknown): void {
    this.log(LogLevel.INFO, message, meta);
  }
  
  warn(message: string, meta?: unknown): void {
    this.log(LogLevel.WARN, message, meta);
  }
  
  error(message: string, error?: unknown): void {
    this.log(LogLevel.ERROR, message, error);
  }
  
  private log(level: LogLevel, message: string, data?: unknown, error?: unknown): void {
    if (!this.shouldLog(level)) {
      return;
    }
    
    const structuredEntry = this.createStructuredLog(level, message, data, error);
    
    // Store in trace context
    this.storeInTraceContext(structuredEntry);
    
    // Format and output
    const formatted = this.format(structuredEntry);
    this.output(formatted, level);
  }
  
  private format(entry: StructuredLogEntry): string {
    if (this.config.format === 'json') {
      return JSON.stringify(entry);
    }
    
    // Fallback to text format with structured information
    return this.formatAsText(entry);
  }
  
  private formatAsText(entry: StructuredLogEntry): string {
    const parts = [
      `[${entry.timestamp}]`,
      `[${entry.level}]`,
      `[${entry.component}]`,
      `[${entry.operation}]`,
      entry.message,
    ];
    
    if (entry.traceId) {
      parts.push(`[trace:${entry.traceId}]`);
    }
    
    if (entry.tags.length > 0) {
      parts.push(`[tags:${entry.tags.join(',')}]`);
    }
    
    if (entry.metadata && Object.keys(entry.metadata).length > 0) {
      parts.push(`[meta:${JSON.stringify(entry.metadata)}]`);
    }
    
    return parts.join(' ');
  }
  
  private output(formatted: string, level: LogLevel): void {
    if (this.config.destination === 'console' || this.config.destination === 'both') {
      this.writeToConsole(level, formatted);
    }
    
    if (this.config.destination === 'file' || this.config.destination === 'both') {
      this.writeToFile(formatted);
    }
  }
  
  private storeInTraceContext(entry: StructuredLogEntry): void {
    if (entry.traceId) {
      this.traceContext.set(entry.traceId, entry);
    }
  }
}
```

#### 2. Add Context Propagation System

```typescript
export class ContextManager {
  private static context = new Map<string, unknown>();
  private static contextStack: Array<Map<string, unknown>> = [];
  
  static setContext(key: string, value: unknown): void {
    this.context.set(key, value);
  }
  
  static getContext<T>(key: string): T | undefined {
    return this.context.get(key) as T;
  }
  
  static getTraceId(): string | undefined {
    return this.getContext<string>('traceId');
  }
  
  static setTraceId(traceId: string): void {
    this.setContext('traceId', traceId);
  }
  
  static generateAndSetTraceId(): string {
    const traceId = this.generateTraceId();
    this.setTraceId(traceId);
    return traceId;
  }
  
  static generateTraceId(): string {
    return `trace_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }
  
  static pushContext(): void {
    const newContext = new Map(this.context);
    this.contextStack.push(newContext);
    this.context.clear();
  }
  
  static popContext(): void {
    if (this.contextStack.length > 0) {
      this.context = this.contextStack.pop()!;
    }
  }
  
  static withContext<T>(context: Record<string, unknown>, fn: () => T): T {
    this.pushContext();
    try {
      for (const [key, value] of Object.entries(context)) {
        this.setContext(key, value);
      }
      return fn();
    } finally {
      this.popContext();
    }
  }
  
  static async withContextAsync<T>(context: Record<string, unknown>, fn: () => Promise<T>): Promise<T> {
    this.pushContext();
    try {
      for (const [key, value] of Object.entries(context)) {
        this.setContext(key, value);
      }
      return await fn();
    } finally {
      this.popContext();
    }
  }
}

// Usage in components
async function processRequest(request: Request): Promise<Response> {
  const traceId = ContextManager.generateAndSetTraceId();
  ContextManager.setContext('requestId', request.id);
  ContextManager.setContext('userId', request.userId);
  
  try {
    // Process request with context
    return await ContextManager.withContextAsync(
      { operation: 'process_request' },
      () => handleRequest(request)
    );
  } catch (error) {
    // Log with context
    logger.error('Request processing failed', error);
    throw error;
  }
}
```

#### 3. Add Correlation ID Generation

```typescript
export class CorrelationIdGenerator {
  private static counter = 0;
  
  static generate(): string {
    const timestamp = Date.now().toString(36);
    const random = Math.random().toString(36).substr(2, 9);
    const counter = (this.counter++).toString(36);
    
    return `${timestamp}_${random}_${counter}`;
  }
  
  static generateShort(): string {
    return this.generate().substr(0, 8);
  }
}

// Usage in logger
private generateTraceId(): string {
  return CorrelationIdGenerator.generate();
}
```

## Implementation Plan

### Phase 1: Core Structured Logging (Week 3)
1. **Implement Structured Logger**
   - Create structured log entry interface
   - Implement JSON formatting
   - Add metadata and tags support

2. **Add Context Management**
   - Implement context propagation
   - Add trace ID generation
   - Create context stack management

### Phase 2: Advanced Features (Week 4)
1. **Add Correlation System**
   - Implement correlation ID generation
   - Add span and parent IDs
   - Create trace context management

2. **Add Schema Validation**
   - Validate log entry structure
   - Ensure consistent field names
   - Add type checking

## Testing Requirements

### Unit Tests
- Structured logging format validation
- Context propagation across components
- Correlation ID generation and usage
- Schema validation tests

### Integration Tests
- End-to-end structured logging flow
- Cross-component correlation
- Context propagation in async operations
- Error handling with structured logging

### Performance Tests
- Structured logging performance impact
- Memory usage during structured logging
- JSON serialization overhead
- Context management performance

## Success Metrics

### Quality Metrics
- **Structured Logging**: 100% consistent JSON format across all components
- **Context Propagation**: 100% of log entries include correlation IDs
- **Schema Compliance**: 100% of log entries pass schema validation
- **Correlation Accuracy**: 100% traceability across components

### Performance Metrics
- **Serialization Overhead**: Keep JSON serialization overhead minimal
- **Memory Usage**: Manage memory usage for context and metadata
- **Processing Time**: Maintain fast log processing times

## Monitoring Requirements

### Quality Monitoring
- Log format compliance
- Context propagation effectiveness
- Correlation accuracy
- Schema validation results

### Performance Monitoring
- JSON serialization performance
- Context management overhead
- Memory usage patterns
- Processing time metrics

### Alerting
- Format violations
- Missing context warnings
- Correlation failures
- Schema validation errors

## Risk Assessment

### Medium Risk Items
- **Performance Impact**: Risk of performance degradation from structured logging
- **Memory Usage**: Risk of increased memory usage from context and metadata
- **Compatibility**: Risk of breaking existing integrations

### Mitigation Strategies
- Implement performance optimizations
- Add memory management controls
- Provide migration path for existing components
- Maintain backward compatibility during transition

## Dependencies

### Internal Dependencies
- Configuration management system
- Error handling system
- Performance monitoring system
- Context management system

### External Dependencies
- JSON serialization
- Date and time formatting
- Random number generation
- Map and data structure management

## Timeline

| Task | Duration | Dependencies |
|------|----------|--------------|
| Structured logger implementation | 3 days | None |
| Context management system | 2 days | Structured logger |
| Correlation ID generation | 1 day | Context management |
| Schema validation | 2 days | All above |
| Testing and validation | 3 days | All above |

## Success Criteria

### Technical Success
- [ ] 100% consistent structured logging format
- [ ] Proper context propagation across components
- [ ] No performance regression from structured logging
- [ ] Comprehensive test coverage (>90%)

### Business Success
- [ ] Improved debugging capabilities
- [ ] Better monitoring and observability
- [ ] Enhanced log analysis capabilities
- [ ] Reduced debugging time

## Additional Context

This critical feature enhancement will significantly improve the Claude-Flow system's debugging and monitoring capabilities. Proper structured logging and context propagation are essential for effective system observability and troubleshooting.

### Related Issues
- Main logging system overhaul epic
- Performance optimization issues
- Debugging improvements
- Monitoring system enhancements

### Future Considerations
- Log analytics and machine learning
- Advanced correlation and tracing
- Log enrichment and transformation
- Integration with monitoring platforms

---

**This issue must be resolved to enable effective debugging and monitoring of the Claude-Flow system.**