# Web Workers中的模块

Web Workers为JavaScript提供了多线程能力，使得我们可以在后台线程中执行计算密集型任务。本章将探讨如何在Web Workers中使用ES模块系统。

## Web Workers基础

### Worker类型与模块支持

```javascript
// 主线程 - main.js
// 1. 经典Worker (不支持ES模块)
const classicWorker = new Worker('./classic-worker.js');

// 2. 模块Worker (支持ES模块)
const moduleWorker = new Worker('./module-worker.js', { 
  type: 'module' 
});

// 3. 内联Worker
const inlineWorkerScript = `
  import { heavyComputation } from './utils.js';
  
  self.onmessage = function(e) {
    const result = heavyComputation(e.data);
    self.postMessage(result);
  };
`;

const blob = new Blob([inlineWorkerScript], { type: 'application/javascript' });
const inlineWorker = new Worker(URL.createObjectURL(blob), { 
  type: 'module' 
});

// Worker能力检测
function supportsModuleWorkers() {
  try {
    new Worker('data:text/javascript,', { type: 'module' }).terminate();
    return true;
  } catch (e) {
    return false;
  }
}

if (supportsModuleWorkers()) {
  console.log('Browser supports module workers');
} else {
  console.log('Falling back to classic workers');
}
```

### 模块Worker示例

```javascript
// module-worker.js - 支持ES模块的Worker
import { calculatePrimes } from './math-utils.js';
import { formatResult } from './formatters.js';
import { Logger } from './logger.js';

const logger = new Logger('MathWorker');

// Worker全局作用域中的self
self.onmessage = async function(event) {
  const { type, data, id } = event.data;
  
  try {
    let result;
    
    switch (type) {
      case 'CALCULATE_PRIMES':
        logger.info(`Calculating primes up to ${data.limit}`);
        result = await calculatePrimes(data.limit);
        break;
        
      case 'PROCESS_ARRAY':
        logger.info(`Processing array of ${data.array.length} items`);
        result = await processLargeArray(data.array, data.operation);
        break;
        
      default:
        throw new Error(`Unknown task type: ${type}`);
    }
    
    // 发送格式化的结果
    self.postMessage({
      id,
      type: 'SUCCESS',
      data: formatResult(result),
      timestamp: Date.now()
    });
    
  } catch (error) {
    logger.error('Worker task failed:', error);
    
    self.postMessage({
      id,
      type: 'ERROR',
      error: {
        message: error.message,
        stack: error.stack
      },
      timestamp: Date.now()
    });
  }
};

// 动态导入模块
async function processLargeArray(array, operation) {
  // 根据操作类型动态加载处理模块
  const { default: processor } = await import(`./processors/${operation}.js`);
  return processor(array);
}

// 错误处理
self.onerror = function(error) {
  logger.error('Uncaught error in worker:', error);
};

self.onunhandledrejection = function(event) {
  logger.error('Unhandled promise rejection in worker:', event.reason);
};

logger.info('Math worker initialized');
```

## Worker模块管理器

### 模块化Worker管理系统

```javascript
// worker-manager.js - Worker模块管理器
class WorkerManager {
  constructor() {
    this.workers = new Map();
    this.taskQueue = new Map();
    this.taskId = 0;
    this.workerPool = new Map();
    this.maxWorkers = navigator.hardwareConcurrency || 4;
  }

  // 注册Worker类型
  registerWorker(name, config) {
    this.workers.set(name, {
      name,
      script: config.script,
      type: config.type || 'module',
      maxInstances: config.maxInstances || 1,
      pool: [],
      activeCount: 0,
      options: config.options || {}
    });
  }

  // 执行任务
  async executeTask(workerName, taskType, data, options = {}) {
    const taskId = ++this.taskId;
    const { timeout = 30000, priority = 'normal' } = options;
    
    return new Promise((resolve, reject) => {
      const task = {
        id: taskId,
        workerName,
        taskType,
        data,
        resolve,
        reject,
        priority,
        timestamp: Date.now(),
        timeout: setTimeout(() => {
          this._handleTaskTimeout(taskId);
        }, timeout)
      };

      this.taskQueue.set(taskId, task);
      this._processTaskQueue();
    });
  }

  // 获取或创建Worker实例
  async _getWorkerInstance(workerName) {
    const workerConfig = this.workers.get(workerName);
    if (!workerConfig) {
      throw new Error(`Worker ${workerName} not registered`);
    }

    // 尝试从池中获取空闲的Worker
    const availableWorker = workerConfig.pool.find(w => !w.busy);
    if (availableWorker) {
      return availableWorker;
    }

    // 如果池未满，创建新的Worker
    if (workerConfig.pool.length < workerConfig.maxInstances) {
      const worker = await this._createWorkerInstance(workerConfig);
      workerConfig.pool.push(worker);
      return worker;
    }

    // 等待Worker变为可用
    return this._waitForAvailableWorker(workerName);
  }

  async _createWorkerInstance(config) {
    const worker = new Worker(config.script, {
      type: config.type,
      ...config.options
    });

    const workerInstance = {
      worker,
      busy: false,
      tasks: new Set(),
      created: Date.now(),
      taskCount: 0
    };

    // 设置消息处理
    worker.onmessage = (event) => {
      this._handleWorkerMessage(workerInstance, event);
    };

    worker.onerror = (error) => {
      this._handleWorkerError(workerInstance, error);
    };

    // 等待Worker初始化完成
    await this._waitForWorkerReady(worker);

    return workerInstance;
  }

  _handleWorkerMessage(workerInstance, event) {
    const { id, type, data, error } = event.data;
    const task = this.taskQueue.get(id);
    
    if (!task) {
      console.warn(`Received message for unknown task ${id}`);
      return;
    }

    clearTimeout(task.timeout);
    this.taskQueue.delete(id);
    workerInstance.tasks.delete(id);

    if (type === 'SUCCESS') {
      task.resolve(data);
    } else if (type === 'ERROR') {
      task.reject(new Error(error.message));
    }

    // 标记Worker为空闲
    if (workerInstance.tasks.size === 0) {
      workerInstance.busy = false;
      this._processTaskQueue();
    }
  }

  _handleWorkerError(workerInstance, error) {
    console.error('Worker error:', error);
    
    // 拒绝所有待处理的任务
    for (const taskId of workerInstance.tasks) {
      const task = this.taskQueue.get(taskId);
      if (task) {
        clearTimeout(task.timeout);
        task.reject(new Error(`Worker error: ${error.message}`));
        this.taskQueue.delete(taskId);
      }
    }

    // 从池中移除损坏的Worker
    this._removeWorkerFromPool(workerInstance);
  }

  _processTaskQueue() {
    const pendingTasks = Array.from(this.taskQueue.values())
      .filter(task => !task.assigned)
      .sort((a, b) => {
        // 按优先级和时间戳排序
        const priorityOrder = { high: 3, normal: 2, low: 1 };
        const aPriority = priorityOrder[a.priority] || 2;
        const bPriority = priorityOrder[b.priority] || 2;
        
        if (aPriority !== bPriority) {
          return bPriority - aPriority;
        }
        
        return a.timestamp - b.timestamp;
      });

    for (const task of pendingTasks) {
      this._assignTaskToWorker(task);
    }
  }

  async _assignTaskToWorker(task) {
    try {
      const workerInstance = await this._getWorkerInstance(task.workerName);
      
      workerInstance.busy = true;
      workerInstance.tasks.add(task.id);
      workerInstance.taskCount++;
      task.assigned = true;

      // 发送任务到Worker
      workerInstance.worker.postMessage({
        id: task.id,
        type: task.taskType,
        data: task.data
      });

    } catch (error) {
      clearTimeout(task.timeout);
      task.reject(error);
      this.taskQueue.delete(task.id);
    }
  }

  _handleTaskTimeout(taskId) {
    const task = this.taskQueue.get(taskId);
    if (task) {
      task.reject(new Error('Task timeout'));
      this.taskQueue.delete(taskId);
    }
  }

  async _waitForWorkerReady(worker) {
    return new Promise((resolve) => {
      const checkReady = () => {
        worker.postMessage({ type: 'PING' });
        
        const handlePong = (event) => {
          if (event.data.type === 'PONG') {
            worker.removeEventListener('message', handlePong);
            resolve();
          }
        };
        
        worker.addEventListener('message', handlePong);
      };
      
      checkReady();
    });
  }

  // 终止所有Worker
  terminateAllWorkers() {
    for (const workerConfig of this.workers.values()) {
      for (const workerInstance of workerConfig.pool) {
        workerInstance.worker.terminate();
      }
      workerConfig.pool.length = 0;
    }
  }

  // 获取统计信息
  getStats() {
    const stats = {
      totalWorkers: 0,
      busyWorkers: 0,
      pendingTasks: 0,
      workerTypes: {}
    };

    for (const [name, config] of this.workers) {
      const busyCount = config.pool.filter(w => w.busy).length;
      stats.totalWorkers += config.pool.length;
      stats.busyWorkers += busyCount;
      
      stats.workerTypes[name] = {
        total: config.pool.length,
        busy: busyCount,
        maxInstances: config.maxInstances
      };
    }

    stats.pendingTasks = this.taskQueue.size;
    return stats;
  }
}

// 使用示例
const workerManager = new WorkerManager();

// 注册不同类型的Worker
workerManager.registerWorker('math', {
  script: './workers/math-worker.js',
  type: 'module',
  maxInstances: 2
});

workerManager.registerWorker('image', {
  script: './workers/image-worker.js',
  type: 'module',
  maxInstances: 1
});

// 执行任务
async function calculatePrimes(limit) {
  try {
    const result = await workerManager.executeTask('math', 'CALCULATE_PRIMES', 
      { limit }, 
      { timeout: 10000, priority: 'high' }
    );
    console.log('Primes calculated:', result);
  } catch (error) {
    console.error('Failed to calculate primes:', error);
  }
}
```

## 模块共享与通信

### Worker间模块共享

```javascript
// shared-module-manager.js - 跨Worker模块共享
class SharedModuleManager {
  constructor() {
    this.sharedModules = new Map();
    this.moduleCache = new Map();
    this.workers = new Set();
  }

  // 注册可共享的模块
  registerSharedModule(name, moduleUrl, exports = []) {
    this.sharedModules.set(name, {
      name,
      url: moduleUrl,
      exports,
      cached: false,
      module: null
    });
  }

  // 注册Worker实例
  registerWorker(worker, id) {
    this.workers.add({ worker, id });
  }

  // 预加载共享模块
  async preloadSharedModules() {
    const loadPromises = Array.from(this.sharedModules.keys())
      .map(name => this.loadSharedModule(name));
    
    await Promise.allSettled(loadPromises);
  }

  // 加载共享模块
  async loadSharedModule(name) {
    const moduleConfig = this.sharedModules.get(name);
    if (!moduleConfig) {
      throw new Error(`Shared module ${name} not found`);
    }

    if (moduleConfig.cached) {
      return moduleConfig.module;
    }

    try {
      // 在主线程中加载模块
      const module = await import(moduleConfig.url);
      
      // 缓存模块
      moduleConfig.module = module;
      moduleConfig.cached = true;
      
      // 通知所有Worker模块已可用
      this._broadcastModuleAvailable(name, moduleConfig);
      
      return module;
    } catch (error) {
      console.error(`Failed to load shared module ${name}:`, error);
      throw error;
    }
  }

  // 广播模块可用性
  _broadcastModuleAvailable(name, moduleConfig) {
    const message = {
      type: 'SHARED_MODULE_AVAILABLE',
      moduleName: name,
      moduleUrl: moduleConfig.url,
      exports: moduleConfig.exports
    };

    for (const workerInfo of this.workers) {
      try {
        workerInfo.worker.postMessage(message);
      } catch (error) {
        console.warn(`Failed to notify worker ${workerInfo.id}:`, error);
      }
    }
  }

  // 模块间通信代理
  createCommunicationBridge() {
    return {
      // Worker到Worker通信
      sendToWorker: (targetWorkerId, message) => {
        const targetWorker = Array.from(this.workers)
          .find(w => w.id === targetWorkerId);
        
        if (targetWorker) {
          targetWorker.worker.postMessage({
            type: 'WORKER_MESSAGE',
            from: 'main',
            data: message
          });
        } else {
          console.warn(`Worker ${targetWorkerId} not found`);
        }
      },

      // 广播消息到所有Worker
      broadcast: (message) => {
        for (const workerInfo of this.workers) {
          workerInfo.worker.postMessage({
            type: 'BROADCAST',
            from: 'main',
            data: message
          });
        }
      }
    };
  }
}

// Worker端的共享模块处理
// shared-worker-utils.js
class WorkerSharedModuleHandler {
  constructor() {
    this.availableModules = new Map();
    this.loadedModules = new Map();
  }

  // 处理主线程消息
  handleMainThreadMessage(event) {
    const { type, moduleName, moduleUrl, exports } = event.data;
    
    switch (type) {
      case 'SHARED_MODULE_AVAILABLE':
        this.availableModules.set(moduleName, { moduleUrl, exports });
        break;
        
      case 'WORKER_MESSAGE':
        this._handleWorkerMessage(event.data);
        break;
        
      case 'BROADCAST':
        this._handleBroadcast(event.data);
        break;
    }
  }

  // 动态加载共享模块
  async loadSharedModule(name) {
    if (this.loadedModules.has(name)) {
      return this.loadedModules.get(name);
    }

    const moduleInfo = this.availableModules.get(name);
    if (!moduleInfo) {
      throw new Error(`Shared module ${name} not available`);
    }

    try {
      const module = await import(moduleInfo.moduleUrl);
      this.loadedModules.set(name, module);
      return module;
    } catch (error) {
      console.error(`Failed to load shared module ${name} in worker:`, error);
      throw error;
    }
  }

  _handleWorkerMessage(messageData) {
    // 处理来自其他Worker的消息
    console.log('Received message from other worker:', messageData);
  }

  _handleBroadcast(messageData) {
    // 处理广播消息
    console.log('Received broadcast:', messageData);
  }
}

// 在Worker中使用
// worker-with-shared-modules.js
import { WorkerSharedModuleHandler } from './shared-worker-utils.js';

const sharedHandler = new WorkerSharedModuleHandler();

self.onmessage = async function(event) {
  const { type, data } = event.data;
  
  // 处理共享模块相关消息
  if (['SHARED_MODULE_AVAILABLE', 'WORKER_MESSAGE', 'BROADCAST'].includes(type)) {
    sharedHandler.handleMainThreadMessage(event);
    return;
  }

  // 处理业务逻辑
  try {
    let result;
    
    switch (type) {
      case 'PROCESS_WITH_SHARED_UTILS':
        // 动态加载共享工具模块
        const utils = await sharedHandler.loadSharedModule('utils');
        result = utils.processData(data);
        break;
        
      case 'CALCULATE_WITH_MATH_LIB':
        // 使用共享数学库
        const mathLib = await sharedHandler.loadSharedModule('mathLib');
        result = mathLib.complexCalculation(data);
        break;
        
      default:
        throw new Error(`Unknown task type: ${type}`);
    }
    
    self.postMessage({
      id: event.data.id,
      type: 'SUCCESS',
      data: result
    });
    
  } catch (error) {
    self.postMessage({
      id: event.data.id,
      type: 'ERROR',
      error: { message: error.message }
    });
  }
};
```

## 高级Worker模式

### Worker池管理

```javascript
// worker-pool.js - 高级Worker池管理
class AdvancedWorkerPool {
  constructor(options = {}) {
    this.maxWorkers = options.maxWorkers || navigator.hardwareConcurrency || 4;
    this.minWorkers = options.minWorkers || 1;
    this.idleTimeout = options.idleTimeout || 30000;
    this.maxTasksPerWorker = options.maxTasksPerWorker || 100;
    
    this.workerScript = options.workerScript;
    this.workerType = options.workerType || 'module';
    
    this.workers = [];
    this.taskQueue = [];
    this.runningTasks = new Map();
    this.workerStats = new Map();
    
    this.setupPool();
  }

  async setupPool() {
    // 创建最小数量的Worker
    for (let i = 0; i < this.minWorkers; i++) {
      await this.createWorker();
    }
  }

  async createWorker() {
    const worker = new Worker(this.workerScript, {
      type: this.workerType
    });

    const workerInfo = {
      id: Math.random().toString(36).substr(2, 9),
      worker,
      busy: false,
      taskCount: 0,
      totalTasks: 0,
      errors: 0,
      created: Date.now(),
      lastUsed: Date.now(),
      idleTimer: null
    };

    // 设置消息处理
    worker.onmessage = (event) => {
      this.handleWorkerMessage(workerInfo, event);
    };

    worker.onerror = (error) => {
      this.handleWorkerError(workerInfo, error);
    };

    this.workers.push(workerInfo);
    this.workerStats.set(workerInfo.id, {
      tasksCompleted: 0,
      averageTaskTime: 0,
      errorRate: 0
    });

    return workerInfo;
  }

  async executeTask(taskData, options = {}) {
    return new Promise((resolve, reject) => {
      const task = {
        id: Math.random().toString(36).substr(2, 9),
        data: taskData,
        resolve,
        reject,
        created: Date.now(),
        timeout: options.timeout || 30000,
        priority: options.priority || 'normal'
      };

      this.taskQueue.push(task);
      this.assignTasks();
    });
  }

  async assignTasks() {
    // 按优先级排序任务
    this.taskQueue.sort((a, b) => {
      const priorityOrder = { high: 3, normal: 2, low: 1 };
      return (priorityOrder[b.priority] || 2) - (priorityOrder[a.priority] || 2);
    });

    while (this.taskQueue.length > 0) {
      const availableWorker = this.getAvailableWorker();
      
      if (!availableWorker) {
        // 尝试创建新Worker
        if (this.workers.length < this.maxWorkers) {
          await this.createWorker();
          continue;
        } else {
          break; // 没有可用Worker，等待
        }
      }

      const task = this.taskQueue.shift();
      this.assignTaskToWorker(availableWorker, task);
    }
  }

  getAvailableWorker() {
    // 找到最适合的Worker
    const availableWorkers = this.workers.filter(w => !w.busy);
    
    if (availableWorkers.length === 0) {
      return null;
    }

    // 选择任务数最少的Worker
    return availableWorkers.reduce((best, current) => {
      return current.totalTasks < best.totalTasks ? current : best;
    });
  }

  assignTaskToWorker(workerInfo, task) {
    workerInfo.busy = true;
    workerInfo.taskCount++;
    workerInfo.totalTasks++;
    workerInfo.lastUsed = Date.now();

    // 清除空闲定时器
    if (workerInfo.idleTimer) {
      clearTimeout(workerInfo.idleTimer);
      workerInfo.idleTimer = null;
    }

    // 设置任务超时
    const timeoutId = setTimeout(() => {
      this.handleTaskTimeout(task.id);
    }, task.timeout);

    this.runningTasks.set(task.id, {
      task,
      worker: workerInfo,
      startTime: Date.now(),
      timeoutId
    });

    // 发送任务到Worker
    workerInfo.worker.postMessage({
      id: task.id,
      data: task.data
    });
  }

  handleWorkerMessage(workerInfo, event) {
    const { id, success, result, error } = event.data;
    const taskInfo = this.runningTasks.get(id);
    
    if (!taskInfo) {
      console.warn(`Received result for unknown task ${id}`);
      return;
    }

    const { task, timeoutId } = taskInfo;
    const duration = Date.now() - taskInfo.startTime;

    // 清除超时定时器
    clearTimeout(timeoutId);
    this.runningTasks.delete(id);

    // 更新Worker状态
    workerInfo.busy = false;
    workerInfo.taskCount--;
    workerInfo.lastUsed = Date.now();

    // 更新统计信息
    this.updateWorkerStats(workerInfo.id, duration, success);

    // 处理任务结果
    if (success) {
      task.resolve(result);
    } else {
      task.reject(new Error(error));
      workerInfo.errors++;
    }

    // 检查Worker是否需要回收
    this.checkWorkerRecycling(workerInfo);

    // 设置空闲定时器
    this.setIdleTimer(workerInfo);

    // 继续分配任务
    this.assignTasks();
  }

  handleWorkerError(workerInfo, error) {
    console.error(`Worker ${workerInfo.id} error:`, error);
    
    // 标记所有运行在该Worker上的任务为失败
    for (const [taskId, taskInfo] of this.runningTasks) {
      if (taskInfo.worker === workerInfo) {
        clearTimeout(taskInfo.timeoutId);
        taskInfo.task.reject(new Error(`Worker error: ${error.message}`));
        this.runningTasks.delete(taskId);
      }
    }

    // 从池中移除错误的Worker
    this.removeWorker(workerInfo);
  }

  handleTaskTimeout(taskId) {
    const taskInfo = this.runningTasks.get(taskId);
    if (taskInfo) {
      taskInfo.task.reject(new Error('Task timeout'));
      this.runningTasks.delete(taskId);
      
      // 释放Worker
      taskInfo.worker.busy = false;
      taskInfo.worker.taskCount--;
    }
  }

  updateWorkerStats(workerId, duration, success) {
    const stats = this.workerStats.get(workerId);
    if (stats) {
      stats.tasksCompleted++;
      stats.averageTaskTime = (stats.averageTaskTime + duration) / 2;
      
      if (!success) {
        stats.errorRate = (stats.errorRate * 0.9) + 0.1; // 指数移动平均
      } else {
        stats.errorRate = stats.errorRate * 0.95;
      }
    }
  }

  checkWorkerRecycling(workerInfo) {
    // 如果Worker执行的任务数过多，回收它
    if (workerInfo.totalTasks >= this.maxTasksPerWorker) {
      this.removeWorker(workerInfo);
      
      // 如果池中Worker数量少于最小值，创建新的
      if (this.workers.length < this.minWorkers) {
        this.createWorker();
      }
    }
  }

  setIdleTimer(workerInfo) {
    if (this.workers.length > this.minWorkers) {
      workerInfo.idleTimer = setTimeout(() => {
        if (!workerInfo.busy) {
          this.removeWorker(workerInfo);
        }
      }, this.idleTimeout);
    }
  }

  removeWorker(workerInfo) {
    const index = this.workers.indexOf(workerInfo);
    if (index > -1) {
      this.workers.splice(index, 1);
      this.workerStats.delete(workerInfo.id);
      
      if (workerInfo.idleTimer) {
        clearTimeout(workerInfo.idleTimer);
      }
      
      workerInfo.worker.terminate();
    }
  }

  // 获取池状态
  getPoolStats() {
    return {
      totalWorkers: this.workers.length,
      busyWorkers: this.workers.filter(w => w.busy).length,
      queuedTasks: this.taskQueue.length,
      runningTasks: this.runningTasks.size,
      workerStats: Object.fromEntries(this.workerStats)
    };
  }

  // 优雅关闭
  async shutdown() {
    // 等待所有运行中的任务完成
    while (this.runningTasks.size > 0) {
      await new Promise(resolve => setTimeout(resolve, 100));
    }

    // 终止所有Worker
    for (const workerInfo of this.workers) {
      if (workerInfo.idleTimer) {
        clearTimeout(workerInfo.idleTimer);
      }
      workerInfo.worker.terminate();
    }

    this.workers.length = 0;
    this.workerStats.clear();
  }
}

// 使用示例
const workerPool = new AdvancedWorkerPool({
  workerScript: './advanced-worker.js',
  workerType: 'module',
  maxWorkers: 8,
  minWorkers: 2,
  idleTimeout: 30000,
  maxTasksPerWorker: 50
});

// 执行计算密集型任务
async function runHeavyComputations() {
  const tasks = Array.from({ length: 20 }, (_, i) => ({
    operation: 'fibonacci',
    input: 40 + i,
    id: i
  }));

  try {
    const results = await Promise.all(
      tasks.map(task => 
        workerPool.executeTask(task, { 
          timeout: 10000, 
          priority: task.id < 5 ? 'high' : 'normal' 
        })
      )
    );

    console.log('All computations completed:', results);
    console.log('Pool stats:', workerPool.getPoolStats());
  } catch (error) {
    console.error('Computation failed:', error);
  }
}

runHeavyComputations();
```

### Service Worker与模块

```javascript
// service-worker.js - Service Worker中的模块使用
import { CacheManager } from './cache-manager.js';
import { NotificationManager } from './notification-manager.js';
import { SyncManager } from './sync-manager.js';

const cacheManager = new CacheManager();
const notificationManager = new NotificationManager();
const syncManager = new SyncManager();

// Service Worker安装
self.addEventListener('install', async (event) => {
  console.log('Service Worker installing...');
  
  event.waitUntil(
    cacheManager.precacheAssets([
      '/',
      '/app.js',
      '/styles.css',
      '/offline.html'
    ])
  );
});

// Service Worker激活
self.addEventListener('activate', async (event) => {
  console.log('Service Worker activating...');
  
  event.waitUntil(
    Promise.all([
      cacheManager.cleanupOldCaches(),
      self.clients.claim()
    ])
  );
});

// 网络请求拦截
self.addEventListener('fetch', (event) => {
  event.respondWith(handleFetch(event.request));
});

// 后台同步
self.addEventListener('sync', (event) => {
  if (event.tag === 'background-sync') {
    event.waitUntil(syncManager.handleBackgroundSync());
  }
});

// 推送通知
self.addEventListener('push', (event) => {
  const data = event.data ? event.data.json() : {};
  event.waitUntil(notificationManager.showNotification(data));
});

async function handleFetch(request) {
  // 动态导入处理模块
  const { default: fetchHandler } = await import('./fetch-handlers.js');
  return fetchHandler.handle(request, cacheManager);
}

// cache-manager.js - 缓存管理模块
export class CacheManager {
  constructor() {
    this.cacheName = 'app-cache-v1';
    this.dynamicCacheName = 'dynamic-cache-v1';
  }

  async precacheAssets(urls) {
    const cache = await caches.open(this.cacheName);
    return cache.addAll(urls);
  }

  async getCachedResponse(request) {
    const cache = await caches.open(this.cacheName);
    const response = await cache.match(request);
    
    if (response) {
      return response;
    }

    // 尝试动态缓存
    const dynamicCache = await caches.open(this.dynamicCacheName);
    return dynamicCache.match(request);
  }

  async cacheResponse(request, response) {
    const cache = await caches.open(this.dynamicCacheName);
    return cache.put(request, response.clone());
  }

  async cleanupOldCaches() {
    const cacheNames = await caches.keys();
    const oldCaches = cacheNames.filter(name => 
      name !== this.cacheName && name !== this.dynamicCacheName
    );

    return Promise.all(
      oldCaches.map(name => caches.delete(name))
    );
  }
}
```

通过这些技术，可以充分利用Web Workers的多线程能力，同时享受ES模块系统带来的代码组织和复用优势，构建高性能的Web应用。
