# Issue Report

**Title**: Unhandled Promise Rejections in Event Bus waitFor Method

## Description
The event bus `waitFor` method contains a critical flaw where event handlers that throw exceptions are not properly caught, leading to unhandled promise rejections. This can crash the application or cause unpredictable behavior when event handlers fail. The current implementation only handles the timeout case but doesn't wrap the handler execution in try-catch, leaving the application vulnerable to handler exceptions.

## Steps to Reproduce
1. Register an event handler that throws an exception
2. Use the `waitFor` method to wait for that event
3. Observe that the exception is not caught and causes an unhandled promise rejection
4. Note that the application may crash or behave unpredictably
5. Check that no error logging occurs for the handler exception

## Expected Behavior
- Event handler exceptions should be caught and logged
- The `waitFor` method should reject with the handler exception
- The application should remain stable despite handler failures
- Proper error logging should occur for all handler exceptions
- Timeout and handler exceptions should be handled consistently

## Actual Behavior
- Event handler exceptions are not caught
- Unhandled promise rejections occur
- Application can crash or behave unpredictably
- No error logging for handler exceptions
- Inconsistent error handling between timeout and handler exceptions

## Environment Details
- **Version**: Claude-Flow development version
- **OS**: Linux 6.8
- **Browser**: N/A (Node.js application)
- **Additional Components**: Event bus system, async operations

## Additional Context
### Current Implementation Issue:

#### Event Bus waitFor Method (event-bus.ts:130-147)
```typescript
async waitFor(event: string, timeoutMs?: number): Promise<unknown> {
  return new Promise((resolve, reject) => {
    const handler = (data: unknown) => {
      if (timer) clearTimeout(timer);
      resolve(data);
    };
    
    let timer: number | undefined;
    if (timeoutMs) {
      timer = setTimeout(() => {
        this.off(event, handler);
        reject(new Error(`Timeout waiting for event: ${event}`));
      }, timeoutMs);
    }
    
    this.once(event, handler);
  });
}
```

### Problems with Current Implementation:
1. **No Error Handling**: The handler function doesn't wrap `resolve(data)` in try-catch
2. **Unhandled Rejections**: If `data` processing throws, it becomes an unhandled rejection
3. **Inconsistent Error Handling**: Timeout errors are handled but handler errors are not
4. **No Logging**: Handler exceptions are not logged
5. **Application Instability**: Can crash the application if exceptions are unhandled

### Example of the Problem:
```typescript
// Event handler that throws an exception
eventBus.on('data-process', (data) => {
  if (!data.valid) {
    throw new Error('Invalid data format');
  }
  return process(data);
});

// waitFor will crash if the handler throws
try {
  await eventBus.waitFor('data-process', 5000);
} catch (error) {
  // This won't catch the handler exception
  console.error('Error waiting for event:', error);
}
```

### Impact on System:
- **Application Stability**: Can cause crashes due to unhandled rejections
- **Debugging**: Difficult to trace where exceptions come from
- **Error Tracking**: Missing error logs for handler exceptions
- **User Experience**: Unpredictable application behavior
- **System Reliability**: Reduced reliability due to unhandled exceptions

### Affected Components:
- Event Bus (`event-bus.ts`)
- Any component using `waitFor` method
- Async operations that depend on events
- Error handling chains that use events

## Priority Levels
- **Critical**: High - Can cause application crashes
- **Impact**: Affects all async operations using event bus
- **Risk**: Application instability and unpredictable behavior

## Recommended Fixes

### Immediate Action (Critical)
1. **Add Error Handling to Handler**
   ```typescript
   async waitFor(event: string, timeoutMs?: number): Promise<unknown> {
     return new Promise((resolve, reject) => {
       const handler = (data: unknown) => {
         try {
           if (timer) clearTimeout(timer);
           resolve(data);
         } catch (handlerError) {
           this.logger.error('Error in event handler for event:', event, handlerError);
           reject(handlerError);
         }
       };
       
       let timer: number | undefined;
       if (timeoutMs) {
         timer = setTimeout(() => {
           this.off(event, handler);
           reject(new Error(`Timeout waiting for event: ${event}`));
         }, timeoutMs);
       }
       
       this.once(event, handler);
     });
   }
   ```

2. **Add Global Error Handler for Events**
   ```typescript
   private setupGlobalErrorHandler(): void {
     // Add a global error handler for all event emissions
     this.typedBus.on('error', (error) => {
       this.logger.error('Unhandled error in event handler:', error);
     });
   }
   ```

3. **Add Safe Event Emission**
   ```typescript
   override emit<K extends keyof EventMap>(event: K, data: EventMap[K]): void {
     if (this.debug) {
       console.debug(`[EventBus] Emitting event: ${String(event)}`, data);
     }
     
     // Track event metrics
     const count = this.eventCounts.get(event) || 0;
     this.eventCounts.set(event, count + 1);
     this.lastEventTimes.set(event, Date.now());
     
     // Wrap handler execution in try-catch
     try {
       super.emit(event, data);
     } catch (error) {
       this.logger.error(`Error in event handler for ${String(event)}:`, error);
       // Don't re-throw to prevent application crash
     }
   }
   ```

### Medium-term Improvements
1. **Add Event Handler Validation**
2. **Implement Event Handler Timeout**
3. **Add Event Handler Metrics**
4. **Implement Event Handler Recovery**

### Long-term Optimizations
1. **Add Event Handler Sandboxing**
2. **Implement Event Handler Isolation**
3. **Add Event Handler Performance Monitoring**
4. **Implement Event Handler Auto-restart**

## Impact Assessment
- **Severity**: Critical - Can cause application crashes
- **Frequency**: Common - Occurs when any event handler throws
- **User Impact**: High - Application can crash unexpectedly
- **Business Impact**: High - Can cause downtime and data loss
- **Performance Impact**: Low - Minimal overhead from error handling

## Testing Recommendations
1. **Error Injection Tests**: Inject exceptions into event handlers
2. **Rejection Tests**: Verify that promise rejections are handled
3. **Stability Tests**: Test application stability with handler failures
4. **Logging Tests**: Verify that handler exceptions are logged
5. **Timeout Tests**: Test timeout and handler exception interaction
6. **Integration Tests**: Test with multiple event handlers and exceptions

## Monitoring Requirements
1. **Unhandled Rejection Rate**: Monitor rate of unhandled promise rejections
2. **Handler Exception Rate**: Track rate of event handler exceptions
3. **Application Crashes**: Monitor for application crashes
4. **Error Logging**: Verify that all handler exceptions are logged
5. **Event Success Rate**: Track successful vs. failed event emissions
6. **Alerts**: Alert on high exception rates or application crashes