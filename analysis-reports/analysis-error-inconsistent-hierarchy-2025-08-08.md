# Issue Report

**Title**: Inconsistent Error Hierarchy Between ClaudeFlowError and AppError

## Description
The codebase contains two different error base classes that create confusion and inconsistency in error handling. The `ClaudeFlowError` class in `errors.ts` is well-structured with proper error codes and details, while the `AppError` class in `error-handler.ts` uses a different structure. This inconsistency makes it difficult for developers to know which error type to use and leads to unpredictable error handling behavior.

## Steps to Reproduce
1. Examine different components of the codebase
2. Notice that some components use `ClaudeFlowError` while others use `AppError`
3. Try to implement error handling in a new component
4. Confusion about which error hierarchy to follow
5. Inconsistent error handling implementation

## Expected Behavior
- Single, consistent error hierarchy throughout the codebase
- Clear guidelines on which error type to use in different scenarios
- Consistent error handling patterns across all components
- Easy to understand and maintain error handling code

## Actual Behavior
- Two different error base classes with different structures
- `ClaudeFlowError` has code, details, and proper stack trace capture
- `AppError` has optional code, statusCode, and different initialization
- Developers don't know which error type to use
- Inconsistent error handling across components

## Environment Details
- **Version**: Claude-Flow development version
- **OS**: Linux 6.8
- **Browser**: N/A (Node.js application)
- **Additional Components**: Error handling system

## Additional Context
### Current Implementation Issues:

#### ClaudeFlowError (errors.ts:8-28)
```typescript
export class ClaudeFlowError extends Error {
  constructor(
    message: string,
    public readonly code: string,
    public readonly details?: unknown,
  ) {
    super(message);
    this.name = 'ClaudeFlowError';
    Error.captureStackTrace(this, this.constructor);
  }

  toJSON() {
    return {
      name: this.name,
      message: this.message,
      code: this.code,
      details: this.details,
      stack: this.stack,
    };
  }
}
```

#### AppError (error-handler.ts:11-21)
```typescript
export class AppError extends Error {
  constructor(
    message: string,
    public code?: string,
    public statusCode?: number,
  ) {
    super(message);
    this.name = 'AppError';
    Object.setPrototypeOf(this, AppError.prototype);
  }
}
```

### Key Differences:
1. **Constructor Parameters**: `ClaudeFlowError` requires code, `AppError` makes it optional
2. **Property Types**: `ClaudeFlowError` uses `readonly` properties, `AppError` uses mutable properties
3. **Stack Trace**: `ClaudeFlowError` captures stack trace, `AppError` doesn't
4. **Serialization**: `ClaudeFlowError` has `toJSON()` method, `AppError` doesn't
5. **Inheritance**: Both extend `Error` but with different approaches

### Usage Patterns:
- `ClaudeFlowError` is used in most components (memory, API, etc.)
- `AppError` is rarely used but creates confusion
- Some components import both error types
- No clear documentation on when to use which error type

## Priority Levels
- **Critical**: High - Creates confusion and inconsistency in error handling
- **Impact**: Affects all error handling in the codebase
- **Risk**: Leads to bugs and difficult debugging

## Recommended Fixes

### Immediate Action (Critical)
1. **Consolidate Error Hierarchy**
   ```typescript
   // Replace both with a single, comprehensive error class
   export class ClaudeFlowError extends Error {
     constructor(
       message: string,
       public readonly code: string,
       public readonly statusCode?: number,
       public readonly retryable: boolean = false,
       public readonly details?: unknown,
     ) {
       super(message);
       this.name = 'ClaudeFlowError';
       Error.captureStackTrace(this, this.constructor);
     }

     toJSON() {
       return {
         name: this.name,
         message: this.message,
         code: this.code,
         statusCode: this.statusCode,
         retryable: this.retryable,
         details: this.details,
         stack: this.stack,
       };
     }
   }
   ```

2. **Deprecate AppError**
   ```typescript
   // Add deprecation warning
   export class AppError extends ClaudeFlowError {
     constructor(message: string, code?: string, statusCode?: number) {
       super(message, code || 'APP_ERROR', statusCode, false);
       this.name = 'AppError';
     }
   }
   ```

3. **Update All Components**
   - Replace all `AppError` usage with `ClaudeFlowError`
   - Update all imports to use the single error class
   - Remove `error-handler.js` and `error-handler.ts` files

### Migration Strategy
1. **Phase 1**: Update documentation to specify the single error hierarchy
2. **Phase 2**: Update all components to use `ClaudeFlowError`
3. **Phase 3**: Remove `AppError` and related files
4. **Phase 4**: Add comprehensive error handling tests

## Impact Assessment
- **Severity**: Critical - Affects all error handling in the codebase
- **Frequency**: Always - Every error handling scenario is affected
- **User Impact**: Medium - Developers experience confusion
- **Business Impact**: Medium - Development time wasted on confusion
- **Performance Impact**: None - No performance impact

## Testing Recommendations
1. Unit tests for all error types
2. Integration tests for error handling across components
3. Migration tests to ensure no breaking changes
4. Error serialization tests
5. Stack trace capture tests

## Monitoring Requirements
1. Track error type usage across components
2. Monitor for any remaining `AppError` usage
3. Log error handling performance
4. Alert on any error handling failures