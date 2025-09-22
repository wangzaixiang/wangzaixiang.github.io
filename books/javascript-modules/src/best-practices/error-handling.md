# 错误处理

在模块化开发中，良好的错误处理策略对于应用的稳定性和可维护性至关重要。本章将探讨JavaScript模块系统中的各种错误处理最佳实践。

## 模块加载错误处理

### 动态导入错误处理

```javascript
// 基础错误处理
async function loadModule(modulePath) {
  try {
    const module = await import(modulePath);
    return module;
  } catch (error) {
    console.error(`Failed to load module ${modulePath}:`, error);
    throw new Error(`Module loading failed: ${error.message}`);
  }
}

// 带重试机制的模块加载
async function loadModuleWithRetry(modulePath, maxRetries = 3) {
  let lastError;
  
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const module = await import(modulePath);
      return module;
    } catch (error) {
      lastError = error;
      console.warn(`Module load attempt ${attempt} failed:`, error);
      
      if (attempt < maxRetries) {
        // 指数退避策略
        await new Promise(resolve => 
          setTimeout(resolve, Math.pow(2, attempt) * 1000)
        );
      }
    }
  }
  
  throw new Error(`Module ${modulePath} failed to load after ${maxRetries} attempts: ${lastError.message}`);
}

// 优雅降级处理
async function loadModuleWithFallback(primaryPath, fallbackPath) {
  try {
    return await import(primaryPath);
  } catch (primaryError) {
    console.warn(`Primary module ${primaryPath} failed, trying fallback:`, primaryError);
    
    try {
      return await import(fallbackPath);
    } catch (fallbackError) {
      console.error(`Both primary and fallback modules failed:`, {
        primary: primaryError,
        fallback: fallbackError
      });
      
      // 返回最小功能实现
      return {
        default: () => console.log('Using minimal fallback implementation')
      };
    }
  }
}
```

### 错误分类处理

```javascript
// 错误类型定义
class ModuleError extends Error {
  constructor(message, code, modulePath) {
    super(message);
    this.name = 'ModuleError';
    this.code = code;
    this.modulePath = modulePath;
  }
}

class NetworkError extends ModuleError {
  constructor(message, modulePath) {
    super(message, 'NETWORK_ERROR', modulePath);
    this.name = 'NetworkError';
  }
}

class ParseError extends ModuleError {
  constructor(message, modulePath) {
    super(message, 'PARSE_ERROR', modulePath);
    this.name = 'ParseError';
  }
}

// 错误处理策略
class ModuleLoader {
  constructor() {
    this.errorHandlers = new Map();
    this.setupDefaultHandlers();
  }

  setupDefaultHandlers() {
    this.errorHandlers.set('NETWORK_ERROR', this.handleNetworkError.bind(this));
    this.errorHandlers.set('PARSE_ERROR', this.handleParseError.bind(this));
    this.errorHandlers.set('PERMISSION_ERROR', this.handlePermissionError.bind(this));
  }

  async load(modulePath) {
    try {
      return await import(modulePath);
    } catch (error) {
      const categorizedError = this.categorizeError(error, modulePath);
      return this.handleError(categorizedError);
    }
  }

  categorizeError(error, modulePath) {
    if (error.message.includes('fetch')) {
      return new NetworkError(error.message, modulePath);
    }
    
    if (error.message.includes('Unexpected token')) {
      return new ParseError(error.message, modulePath);
    }
    
    return new ModuleError(error.message, 'UNKNOWN_ERROR', modulePath);
  }

  async handleError(error) {
    const handler = this.errorHandlers.get(error.code);
    if (handler) {
      return await handler(error);
    }
    
    console.error('Unhandled module error:', error);
    throw error;
  }

  async handleNetworkError(error) {
    console.warn('Network error detected, attempting offline fallback');
    // 尝试从缓存加载
    return this.loadFromCache(error.modulePath);
  }

  async handleParseError(error) {
    console.warn('Parse error detected, attempting alternative version');
    // 尝试加载兼容版本
    const fallbackPath = error.modulePath.replace('.js', '.compat.js');
    return this.load(fallbackPath);
  }

  async handlePermissionError(error) {
    console.warn('Permission error, requesting user authorization');
    // 请求用户授权或提供替代方案
    return this.requestPermissionAndRetry(error.modulePath);
  }
}
```

## 运行时错误处理

### 模块级错误边界

```javascript
// 模块错误边界包装器
class ModuleErrorBoundary {
  constructor(moduleName) {
    this.moduleName = moduleName;
    this.errorCount = 0;
    this.maxErrors = 5;
    this.resetInterval = 60000; // 1分钟
    this.setupErrorHandling();
  }

  setupErrorHandling() {
    // 捕获未处理的Promise拒绝
    window.addEventListener('unhandledrejection', (event) => {
      if (this.isModuleError(event.reason)) {
        this.handleError(event.reason);
        event.preventDefault();
      }
    });

    // 捕获未处理的错误
    window.addEventListener('error', (event) => {
      if (this.isModuleError(event.error)) {
        this.handleError(event.error);
      }
    });
  }

  isModuleError(error) {
    return error && error.stack && 
           error.stack.includes(this.moduleName);
  }

  handleError(error) {
    this.errorCount++;
    
    console.error(`Error in module ${this.moduleName}:`, error);
    
    // 错误频率限制
    if (this.errorCount >= this.maxErrors) {
      console.warn(`Module ${this.moduleName} has exceeded error limit, disabling`);
      this.disableModule();
      return;
    }

    // 发送错误报告
    this.reportError(error);
    
    // 重置错误计数
    setTimeout(() => {
      this.errorCount = Math.max(0, this.errorCount - 1);
    }, this.resetInterval);
  }

  disableModule() {
    // 禁用有问题的模块
    window[`${this.moduleName}_disabled`] = true;
  }

  reportError(error) {
    // 发送到错误监控服务
    if (typeof errorReporting !== 'undefined') {
      errorReporting.captureException(error, {
        module: this.moduleName,
        errorCount: this.errorCount
      });
    }
  }
}

// 使用示例
const chartModuleBoundary = new ModuleErrorBoundary('chart-module');
```

### 函数级错误处理

```javascript
// 高阶函数：添加错误处理
function withErrorHandling(fn, options = {}) {
  const {
    retries = 0,
    timeout = 5000,
    fallback = null,
    onError = console.error
  } = options;

  return async function(...args) {
    let lastError;
    
    for (let attempt = 0; attempt <= retries; attempt++) {
      try {
        // 添加超时控制
        const timeoutPromise = new Promise((_, reject) =>
          setTimeout(() => reject(new Error('Function timeout')), timeout)
        );
        
        const resultPromise = Promise.resolve(fn.apply(this, args));
        
        return await Promise.race([resultPromise, timeoutPromise]);
      } catch (error) {
        lastError = error;
        onError(`Attempt ${attempt + 1} failed:`, error);
        
        if (attempt < retries) {
          await new Promise(resolve => setTimeout(resolve, 1000 * attempt));
        }
      }
    }
    
    // 如果所有重试都失败，使用fallback
    if (fallback) {
      console.warn('Using fallback function due to repeated failures');
      return fallback.apply(this, args);
    }
    
    throw lastError;
  };
}

// 使用示例
const safeApiCall = withErrorHandling(
  async (url) => {
    const response = await fetch(url);
    return response.json();
  },
  {
    retries: 3,
    timeout: 10000,
    fallback: () => ({ error: 'Service unavailable' }),
    onError: (msg, error) => console.warn(`API call failed: ${msg}`, error)
  }
);
```

## 异步错误处理

### Promise错误处理模式

```javascript
// Promise链错误处理
class AsyncModuleManager {
  constructor() {
    this.pendingOperations = new Map();
    this.errorQueue = [];
  }

  async executeWithErrorHandling(operationId, asyncFn) {
    // 防止重复执行
    if (this.pendingOperations.has(operationId)) {
      return this.pendingOperations.get(operationId);
    }

    const operation = this.createOperation(operationId, asyncFn);
    this.pendingOperations.set(operationId, operation);

    try {
      const result = await operation;
      this.pendingOperations.delete(operationId);
      return result;
    } catch (error) {
      this.pendingOperations.delete(operationId);
      this.handleAsyncError(operationId, error);
      throw error;
    }
  }

  createOperation(operationId, asyncFn) {
    return Promise.resolve()
      .then(() => asyncFn())
      .catch(error => {
        // 增强错误信息
        error.operationId = operationId;
        error.timestamp = new Date().toISOString();
        throw error;
      });
  }

  handleAsyncError(operationId, error) {
    this.errorQueue.push({
      operationId,
      error,
      timestamp: Date.now()
    });

    // 限制错误队列大小
    if (this.errorQueue.length > 100) {
      this.errorQueue.shift();
    }

    this.reportAsyncError(operationId, error);
  }

  reportAsyncError(operationId, error) {
    console.error(`Async operation ${operationId} failed:`, error);
    
    // 批量发送错误报告
    this.batchReportErrors();
  }

  batchReportErrors() {
    if (this.errorQueue.length === 0) return;

    const errors = this.errorQueue.splice(0);
    
    // 发送到监控服务
    if (typeof analytics !== 'undefined') {
      analytics.track('Batch Error Report', {
        errors: errors.map(e => ({
          operationId: e.operationId,
          message: e.error.message,
          timestamp: e.timestamp
        }))
      });
    }
  }
}
```

### 并发错误处理

```javascript
// 并发操作错误处理
class ConcurrentModuleLoader {
  constructor(maxConcurrency = 5) {
    this.maxConcurrency = maxConcurrency;
    this.activeLoads = 0;
    this.queue = [];
    this.results = new Map();
    this.errors = new Map();
  }

  async loadModules(modulePaths) {
    const loadPromises = modulePaths.map(path => this.queueLoad(path));
    
    // 使用Promise.allSettled处理部分失败
    const results = await Promise.allSettled(loadPromises);
    
    return this.processResults(results, modulePaths);
  }

  async queueLoad(modulePath) {
    return new Promise((resolve, reject) => {
      this.queue.push({ modulePath, resolve, reject });
      this.processQueue();
    });
  }

  async processQueue() {
    if (this.activeLoads >= this.maxConcurrency || this.queue.length === 0) {
      return;
    }

    const { modulePath, resolve, reject } = this.queue.shift();
    this.activeLoads++;

    try {
      const module = await this.loadSingleModule(modulePath);
      this.results.set(modulePath, module);
      resolve(module);
    } catch (error) {
      this.errors.set(modulePath, error);
      reject(error);
    } finally {
      this.activeLoads--;
      this.processQueue(); // 处理下一个
    }
  }

  async loadSingleModule(modulePath) {
    try {
      return await import(modulePath);
    } catch (error) {
      console.error(`Failed to load ${modulePath}:`, error);
      throw new Error(`Module load failed: ${modulePath}`);
    }
  }

  processResults(results, modulePaths) {
    const successful = [];
    const failed = [];

    results.forEach((result, index) => {
      const modulePath = modulePaths[index];
      
      if (result.status === 'fulfilled') {
        successful.push({ modulePath, module: result.value });
      } else {
        failed.push({ modulePath, error: result.reason });
      }
    });

    // 记录加载统计
    console.log(`Module loading completed: ${successful.length} successful, ${failed.length} failed`);
    
    if (failed.length > 0) {
      console.warn('Failed modules:', failed);
    }

    return {
      successful,
      failed,
      hasErrors: failed.length > 0
    };
  }
}
```

## 错误监控与报告

### 错误监控系统

```javascript
class ModuleErrorMonitor {
  constructor() {
    this.errorBuffer = [];
    this.reportInterval = 30000; // 30秒
    this.maxBufferSize = 50;
    this.setupPeriodicReporting();
  }

  captureError(error, context = {}) {
    const errorData = {
      message: error.message,
      stack: error.stack,
      timestamp: Date.now(),
      url: window.location.href,
      userAgent: navigator.userAgent,
      context
    };

    this.errorBuffer.push(errorData);

    // 立即处理严重错误
    if (this.isCriticalError(error)) {
      this.reportImmediately(errorData);
    }

    // 缓冲区溢出时发送
    if (this.errorBuffer.length >= this.maxBufferSize) {
      this.sendErrorReport();
    }
  }

  isCriticalError(error) {
    const criticalPatterns = [
      /cannot read property/i,
      /is not a function/i,
      /network error/i,
      /security/i
    ];

    return criticalPatterns.some(pattern => pattern.test(error.message));
  }

  setupPeriodicReporting() {
    setInterval(() => {
      if (this.errorBuffer.length > 0) {
        this.sendErrorReport();
      }
    }, this.reportInterval);
  }

  async sendErrorReport() {
    if (this.errorBuffer.length === 0) return;

    const errors = this.errorBuffer.splice(0);
    
    try {
      await this.submitErrors(errors);
      console.log(`Sent ${errors.length} error reports`);
    } catch (reportError) {
      console.error('Failed to send error report:', reportError);
      // 将错误放回缓冲区
      this.errorBuffer.unshift(...errors.slice(-10)); // 只保留最新的10个
    }
  }

  async submitErrors(errors) {
    const reportData = {
      errors,
      session: this.getSessionInfo(),
      performance: this.getPerformanceMetrics()
    };

    const response = await fetch('/api/errors', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(reportData)
    });

    if (!response.ok) {
      throw new Error(`Error reporting failed: ${response.status}`);
    }
  }

  getSessionInfo() {
    return {
      sessionId: this.getSessionId(),
      timestamp: Date.now(),
      modules: this.getLoadedModules()
    };
  }

  getPerformanceMetrics() {
    if (!window.performance) return null;

    return {
      memory: window.performance.memory ? {
        used: window.performance.memory.usedJSHeapSize,
        total: window.performance.memory.totalJSHeapSize
      } : null,
      timing: window.performance.timing
    };
  }

  async reportImmediately(errorData) {
    try {
      await this.submitErrors([errorData]);
    } catch (error) {
      console.error('Failed to send immediate error report:', error);
    }
  }
}

// 全局错误监控器
const errorMonitor = new ModuleErrorMonitor();

// 集成到模块加载器
const originalImport = window.import || (() => {});
window.import = function(specifier) {
  return originalImport(specifier).catch(error => {
    errorMonitor.captureError(error, { 
      type: 'module_load', 
      specifier 
    });
    throw error;
  });
};
```

## 错误恢复策略

### 自动恢复机制

```javascript
class ModuleRecoveryManager {
  constructor() {
    this.failedModules = new Map();
    this.recoveryStrategies = new Map();
    this.setupRecoveryStrategies();
  }

  setupRecoveryStrategies() {
    this.recoveryStrategies.set('reload', this.reloadModule.bind(this));
    this.recoveryStrategies.set('fallback', this.useFallbackModule.bind(this));
    this.recoveryStrategies.set('retry', this.retryModuleLoad.bind(this));
    this.recoveryStrategies.set('degrade', this.degradeGracefully.bind(this));
  }

  async handleModuleFailure(modulePath, error, context = {}) {
    const failureInfo = {
      error,
      timestamp: Date.now(),
      attempts: 0,
      context
    };

    this.failedModules.set(modulePath, failureInfo);

    // 选择恢复策略
    const strategy = this.selectRecoveryStrategy(error, context);
    
    try {
      return await this.executeRecoveryStrategy(strategy, modulePath, failureInfo);
    } catch (recoveryError) {
      console.error(`Recovery failed for ${modulePath}:`, recoveryError);
      return this.handleRecoveryFailure(modulePath, recoveryError);
    }
  }

  selectRecoveryStrategy(error, context) {
    if (error.message.includes('network')) {
      return 'retry';
    }
    
    if (error.message.includes('parse')) {
      return 'fallback';
    }
    
    if (context.isCritical) {
      return 'reload';
    }
    
    return 'degrade';
  }

  async executeRecoveryStrategy(strategy, modulePath, failureInfo) {
    const handler = this.recoveryStrategies.get(strategy);
    if (!handler) {
      throw new Error(`Unknown recovery strategy: ${strategy}`);
    }

    console.log(`Attempting recovery for ${modulePath} using strategy: ${strategy}`);
    return await handler(modulePath, failureInfo);
  }

  async reloadModule(modulePath, failureInfo) {
    // 强制重新加载模块
    const cacheBustedPath = `${modulePath}?v=${Date.now()}`;
    return await import(cacheBustedPath);
  }

  async useFallbackModule(modulePath, failureInfo) {
    // 尝试加载备用模块
    const fallbackPaths = [
      modulePath.replace('.js', '.fallback.js'),
      modulePath.replace('.js', '.min.js'),
      './fallbacks/default-module.js'
    ];

    for (const fallbackPath of fallbackPaths) {
      try {
        return await import(fallbackPath);
      } catch (fallbackError) {
        console.warn(`Fallback ${fallbackPath} also failed:`, fallbackError);
      }
    }

    throw new Error('All fallback options exhausted');
  }

  async retryModuleLoad(modulePath, failureInfo) {
    failureInfo.attempts++;
    const maxRetries = 3;
    
    if (failureInfo.attempts >= maxRetries) {
      throw new Error(`Max retries (${maxRetries}) exceeded for ${modulePath}`);
    }

    // 指数退避
    const delay = Math.pow(2, failureInfo.attempts) * 1000;
    await new Promise(resolve => setTimeout(resolve, delay));

    return await import(modulePath);
  }

  async degradeGracefully(modulePath, failureInfo) {
    // 返回最小功能实现
    console.warn(`Gracefully degrading functionality for ${modulePath}`);
    
    return {
      default: function() {
        console.warn(`Module ${modulePath} is unavailable, using degraded functionality`);
        return null;
      },
      __degraded: true,
      __originalPath: modulePath
    };
  }

  async handleRecoveryFailure(modulePath, recoveryError) {
    console.error(`All recovery attempts failed for ${modulePath}`);
    
    // 移除失败的模块记录
    this.failedModules.delete(modulePath);
    
    // 发送失败报告
    errorMonitor.captureError(recoveryError, {
      type: 'recovery_failure',
      modulePath,
      originalError: this.failedModules.get(modulePath)?.error
    });

    // 返回空实现避免应用崩溃
    return this.degradeGracefully(modulePath, {});
  }
}

// 全局恢复管理器
const recoveryManager = new ModuleRecoveryManager();
```

通过实施这些错误处理策略，可以大大提高模块化应用的健壮性和用户体验，确保即使在出现错误时应用也能继续运行。
