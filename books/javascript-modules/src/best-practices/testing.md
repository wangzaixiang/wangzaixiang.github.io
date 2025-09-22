# 测试模块化代码

模块化代码的测试策略与传统代码有所不同，需要考虑模块间的依赖关系、异步加载、隔离性等因素。本章将详细介绍如何有效测试JavaScript模块。

## 模块测试基础

### 单元测试策略

```javascript
// userService.js - 被测试的模块
export class UserService {
  constructor(httpClient, cache) {
    this.httpClient = httpClient;
    this.cache = cache;
  }

  async getUser(id) {
    const cacheKey = `user:${id}`;
    
    // 先检查缓存
    const cached = await this.cache.get(cacheKey);
    if (cached) {
      return cached;
    }

    // 从API获取
    const user = await this.httpClient.get(`/users/${id}`);
    
    // 缓存结果
    await this.cache.set(cacheKey, user, { ttl: 300 });
    
    return user;
  }

  async updateUser(id, data) {
    const user = await this.httpClient.put(`/users/${id}`, data);
    
    // 清除相关缓存
    await this.cache.delete(`user:${id}`);
    
    return user;
  }
}

export const createUserService = (httpClient, cache) => {
  return new UserService(httpClient, cache);
};
```

```javascript
// userService.test.js - 单元测试
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { UserService } from '../userService.js';

describe('UserService', () => {
  let userService;
  let mockHttpClient;
  let mockCache;

  beforeEach(() => {
    // 创建模拟依赖
    mockHttpClient = {
      get: vi.fn(),
      put: vi.fn()
    };
    
    mockCache = {
      get: vi.fn(),
      set: vi.fn(),
      delete: vi.fn()
    };

    userService = new UserService(mockHttpClient, mockCache);
  });

  describe('getUser', () => {
    it('should return cached user when available', async () => {
      const userId = '123';
      const cachedUser = { id: userId, name: 'John' };
      
      mockCache.get.mockResolvedValue(cachedUser);

      const result = await userService.getUser(userId);

      expect(result).toBe(cachedUser);
      expect(mockCache.get).toHaveBeenCalledWith('user:123');
      expect(mockHttpClient.get).not.toHaveBeenCalled();
    });

    it('should fetch from API and cache when not in cache', async () => {
      const userId = '123';
      const apiUser = { id: userId, name: 'John' };
      
      mockCache.get.mockResolvedValue(null);
      mockHttpClient.get.mockResolvedValue(apiUser);

      const result = await userService.getUser(userId);

      expect(result).toBe(apiUser);
      expect(mockHttpClient.get).toHaveBeenCalledWith('/users/123');
      expect(mockCache.set).toHaveBeenCalledWith('user:123', apiUser, { ttl: 300 });
    });
  });

  describe('updateUser', () => {
    it('should update user and clear cache', async () => {
      const userId = '123';
      const updateData = { name: 'Jane' };
      const updatedUser = { id: userId, name: 'Jane' };
      
      mockHttpClient.put.mockResolvedValue(updatedUser);

      const result = await userService.updateUser(userId, updateData);

      expect(result).toBe(updatedUser);
      expect(mockHttpClient.put).toHaveBeenCalledWith('/users/123', updateData);
      expect(mockCache.delete).toHaveBeenCalledWith('user:123');
    });
  });
});
```

### 模块隔离测试

```javascript
// testUtils.js - 测试工具
export class ModuleTestEnvironment {
  constructor() {
    this.originalModules = new Map();
    this.mockModules = new Map();
  }

  // 备份原始模块
  backupModule(modulePath, moduleObject) {
    this.originalModules.set(modulePath, moduleObject);
  }

  // 创建模块模拟
  mockModule(modulePath, mockImplementation) {
    this.mockModules.set(modulePath, mockImplementation);
  }

  // 恢复所有模块
  restoreAll() {
    this.originalModules.clear();
    this.mockModules.clear();
  }

  // 获取模拟的模块
  getMockedModule(modulePath) {
    return this.mockModules.get(modulePath);
  }
}

// 模块测试装饰器
export function testModule(testFn) {
  return async () => {
    const env = new ModuleTestEnvironment();
    
    try {
      await testFn(env);
    } finally {
      env.restoreAll();
    }
  };
}

// 使用示例
import { testModule } from './testUtils.js';

describe('Module Integration Tests', () => {
  it('should handle module dependencies correctly', testModule(async (env) => {
    // 模拟依赖模块
    env.mockModule('./httpClient.js', {
      get: vi.fn().mockResolvedValue({ data: 'mocked' }),
      post: vi.fn().mockResolvedValue({ success: true })
    });

    env.mockModule('./logger.js', {
      info: vi.fn(),
      error: vi.fn()
    });

    // 导入并测试目标模块
    const { DataService } = await import('./dataService.js');
    const service = new DataService();
    
    const result = await service.fetchData();
    expect(result.data).toBe('mocked');
  }));
});
```

## 异步模块测试

### 动态导入测试

```javascript
// moduleLoader.js - 动态模块加载器
export class ModuleLoader {
  constructor() {
    this.loadedModules = new Map();
  }

  async loadModule(modulePath) {
    if (this.loadedModules.has(modulePath)) {
      return this.loadedModules.get(modulePath);
    }

    try {
      const module = await import(modulePath);
      this.loadedModules.set(modulePath, module);
      return module;
    } catch (error) {
      throw new Error(`Failed to load module ${modulePath}: ${error.message}`);
    }
  }

  async loadModuleWithTimeout(modulePath, timeout = 5000) {
    const timeoutPromise = new Promise((_, reject) =>
      setTimeout(() => reject(new Error('Module load timeout')), timeout)
    );

    return Promise.race([
      this.loadModule(modulePath),
      timeoutPromise
    ]);
  }

  getLoadedModuleCount() {
    return this.loadedModules.size;
  }

  clearCache() {
    this.loadedModules.clear();
  }
}
```

```javascript
// moduleLoader.test.js - 异步模块测试
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { ModuleLoader } from '../moduleLoader.js';

describe('ModuleLoader', () => {
  let loader;

  beforeEach(() => {
    loader = new ModuleLoader();
  });

  describe('loadModule', () => {
    it('should load module successfully', async () => {
      // 使用实际存在的模块进行测试
      const module = await loader.loadModule('./testData/sampleModule.js');
      
      expect(module).toBeDefined();
      expect(module.default).toBeDefined();
      expect(loader.getLoadedModuleCount()).toBe(1);
    });

    it('should cache loaded modules', async () => {
      const modulePath = './testData/sampleModule.js';
      
      const module1 = await loader.loadModule(modulePath);
      const module2 = await loader.loadModule(modulePath);
      
      expect(module1).toBe(module2); // 应该是同一个对象
      expect(loader.getLoadedModuleCount()).toBe(1);
    });

    it('should handle module load failure', async () => {
      const invalidPath = './nonexistent/module.js';
      
      await expect(loader.loadModule(invalidPath))
        .rejects
        .toThrow(/Failed to load module/);
    });
  });

  describe('loadModuleWithTimeout', () => {
    it('should timeout when module takes too long to load', async () => {
      // 模拟慢加载的模块
      const slowModulePath = './testData/slowModule.js';
      
      await expect(loader.loadModuleWithTimeout(slowModulePath, 100))
        .rejects
        .toThrow('Module load timeout');
    });
  });
});
```

### Promise并发测试

```javascript
// concurrentLoader.js - 并发模块加载器
export class ConcurrentModuleLoader {
  constructor(maxConcurrency = 3) {
    this.maxConcurrency = maxConcurrency;
    this.activeLoads = 0;
    this.pendingLoads = [];
    this.results = new Map();
  }

  async loadModules(modulePaths) {
    const loadPromises = modulePaths.map(path => this.queueLoad(path));
    
    const results = await Promise.allSettled(loadPromises);
    
    return this.processResults(results, modulePaths);
  }

  async queueLoad(modulePath) {
    if (this.results.has(modulePath)) {
      return this.results.get(modulePath);
    }

    return new Promise((resolve, reject) => {
      this.pendingLoads.push({ modulePath, resolve, reject });
      this.processQueue();
    });
  }

  async processQueue() {
    if (this.activeLoads >= this.maxConcurrency || this.pendingLoads.length === 0) {
      return;
    }

    const { modulePath, resolve, reject } = this.pendingLoads.shift();
    this.activeLoads++;

    try {
      const module = await import(modulePath);
      this.results.set(modulePath, module);
      resolve(module);
    } catch (error) {
      reject(error);
    } finally {
      this.activeLoads--;
      this.processQueue();
    }
  }

  processResults(results, modulePaths) {
    const successful = [];
    const failed = [];

    results.forEach((result, index) => {
      const modulePath = modulePaths[index];
      
      if (result.status === 'fulfilled') {
        successful.push({ path: modulePath, module: result.value });
      } else {
        failed.push({ path: modulePath, error: result.reason });
      }
    });

    return { successful, failed };
  }
}
```

```javascript
// concurrentLoader.test.js - 并发测试
import { describe, it, expect, beforeEach } from 'vitest';
import { ConcurrentModuleLoader } from '../concurrentLoader.js';

describe('ConcurrentModuleLoader', () => {
  let loader;

  beforeEach(() => {
    loader = new ConcurrentModuleLoader(2); // 最大并发数为2
  });

  it('should load multiple modules concurrently', async () => {
    const modulePaths = [
      './testData/module1.js',
      './testData/module2.js',
      './testData/module3.js'
    ];

    const startTime = Date.now();
    const result = await loader.loadModules(modulePaths);
    const endTime = Date.now();

    expect(result.successful).toHaveLength(3);
    expect(result.failed).toHaveLength(0);
    
    // 验证并发加载确实更快
    expect(endTime - startTime).toBeLessThan(3000); // 假设单个模块加载需要1秒
  });

  it('should handle mixed success and failure', async () => {
    const modulePaths = [
      './testData/validModule.js',
      './nonexistent/module.js',
      './testData/anotherValidModule.js'
    ];

    const result = await loader.loadModules(modulePaths);

    expect(result.successful).toHaveLength(2);
    expect(result.failed).toHaveLength(1);
    
    expect(result.failed[0].path).toBe('./nonexistent/module.js');
  });

  it('should respect concurrency limits', async () => {
    const loader = new ConcurrentModuleLoader(1); // 限制为1
    
    let concurrentCount = 0;
    let maxConcurrent = 0;

    // 创建模拟模块路径
    const modulePaths = Array.from({ length: 5 }, (_, i) => `./test${i}.js`);
    
    // 监控并发数
    const originalImport = global.import;
    global.import = async (path) => {
      concurrentCount++;
      maxConcurrent = Math.max(maxConcurrent, concurrentCount);
      
      await new Promise(resolve => setTimeout(resolve, 100));
      
      concurrentCount--;
      return { default: `module-${path}` };
    };

    try {
      await loader.loadModules(modulePaths);
      expect(maxConcurrent).toBe(1);
    } finally {
      global.import = originalImport;
    }
  });
});
```

## 集成测试

### 模块间集成测试

```javascript
// integration.test.js - 集成测试
import { describe, it, expect, beforeEach, afterEach } from 'vitest';

describe('Module Integration Tests', () => {
  let testServer;
  let testDatabase;

  beforeEach(async () => {
    // 启动测试环境
    testDatabase = await startTestDatabase();
    testServer = await startTestServer();
  });

  afterEach(async () => {
    // 清理测试环境
    await testServer.stop();
    await testDatabase.cleanup();
  });

  it('should handle complete user workflow', async () => {
    // 导入真实模块
    const { UserService } = await import('../src/services/userService.js');
    const { HttpClient } = await import('../src/http/httpClient.js');
    const { RedisCache } = await import('../src/cache/redisCache.js');

    // 创建真实的依赖实例
    const httpClient = new HttpClient({
      baseURL: testServer.url,
      timeout: 5000
    });

    const cache = new RedisCache({
      host: testDatabase.host,
      port: testDatabase.port
    });

    const userService = new UserService(httpClient, cache);

    // 测试完整的用户工作流
    const userData = { name: 'Test User', email: 'test@example.com' };
    
    // 1. 创建用户
    const createdUser = await userService.createUser(userData);
    expect(createdUser.id).toBeDefined();

    // 2. 获取用户（应该从API获取）
    const fetchedUser = await userService.getUser(createdUser.id);
    expect(fetchedUser.name).toBe(userData.name);

    // 3. 再次获取用户（应该从缓存获取）
    const cachedUser = await userService.getUser(createdUser.id);
    expect(cachedUser.name).toBe(userData.name);

    // 4. 更新用户
    const updatedData = { name: 'Updated User' };
    const updatedUser = await userService.updateUser(createdUser.id, updatedData);
    expect(updatedUser.name).toBe(updatedData.name);

    // 5. 验证缓存已被清除
    const freshUser = await userService.getUser(createdUser.id);
    expect(freshUser.name).toBe(updatedData.name);
  });

  it('should handle module loading failures gracefully', async () => {
    // 模拟模块加载失败的情况
    const { ModuleManager } = await import('../src/core/moduleManager.js');
    
    const manager = new ModuleManager();
    
    // 注册模块加载失败的处理器
    const errorHandler = vi.fn();
    manager.onModuleLoadError(errorHandler);

    // 尝试加载不存在的模块
    const result = await manager.loadOptionalModule('./nonexistent.js');
    
    expect(result).toBeNull();
    expect(errorHandler).toHaveBeenCalled();
  });
});
```

### 端到端测试

```javascript
// e2e.test.js - 端到端测试
import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { chromium } from 'playwright';

describe('End-to-End Module Tests', () => {
  let browser;
  let context;
  let page;

  beforeAll(async () => {
    browser = await chromium.launch();
    context = await browser.newContext();
    page = await context.newPage();
  });

  afterAll(async () => {
    await browser.close();
  });

  it('should load modules dynamically in browser', async () => {
    // 访问测试页面
    await page.goto('http://localhost:3000/test.html');

    // 等待页面初始化
    await page.waitForLoadState('networkidle');

    // 触发动态模块加载
    await page.click('[data-testid="load-chart-module"]');

    // 等待模块加载完成
    await page.waitForSelector('[data-testid="chart-container"]');

    // 验证模块功能
    const chartTitle = await page.textContent('[data-testid="chart-title"]');
    expect(chartTitle).toBe('Sales Chart');

    // 验证模块间通信
    await page.click('[data-testid="update-chart-data"]');
    
    await page.waitForFunction(() => {
      const chart = document.querySelector('[data-testid="chart-container"]');
      return chart && chart.dataset.updated === 'true';
    });

    const updatedData = await page.getAttribute('[data-testid="chart-container"]', 'data-points');
    expect(parseInt(updatedData)).toBeGreaterThan(0);
  });

  it('should handle module load errors gracefully', async () => {
    await page.goto('http://localhost:3000/error-test.html');

    // 模拟网络故障
    await page.route('**/modules/broken-module.js', route => {
      route.fulfill({ status: 500, body: 'Server Error' });
    });

    // 触发有问题的模块加载
    await page.click('[data-testid="load-broken-module"]');

    // 验证错误处理
    await page.waitForSelector('[data-testid="error-message"]');
    
    const errorMessage = await page.textContent('[data-testid="error-message"]');
    expect(errorMessage).toContain('Module failed to load');

    // 验证fallback机制
    await page.waitForSelector('[data-testid="fallback-content"]');
    
    const fallbackText = await page.textContent('[data-testid="fallback-content"]');
    expect(fallbackText).toBe('Using fallback implementation');
  });
});
```

## 性能测试

### 模块加载性能测试

```javascript
// performance.test.js - 性能测试
import { describe, it, expect } from 'vitest';
import { performance } from 'perf_hooks';

describe('Module Performance Tests', () => {
  it('should load modules within acceptable time limits', async () => {
    const moduleLoads = [
      './src/modules/chart.js',
      './src/modules/table.js',
      './src/modules/form.js',
      './src/modules/validation.js'
    ];

    const startTime = performance.now();
    
    const loadPromises = moduleLoads.map(async (modulePath) => {
      const moduleStartTime = performance.now();
      const module = await import(modulePath);
      const moduleEndTime = performance.now();
      
      return {
        path: modulePath,
        loadTime: moduleEndTime - moduleStartTime,
        module
      };
    });

    const results = await Promise.all(loadPromises);
    const totalTime = performance.now() - startTime;

    // 验证总加载时间
    expect(totalTime).toBeLessThan(2000); // 2秒内

    // 验证单个模块加载时间
    results.forEach(result => {
      expect(result.loadTime).toBeLessThan(500); // 每个模块500ms内
      expect(result.module).toBeDefined();
    });

    // 验证并发加载确实比串行快
    const averageTime = totalTime / results.length;
    expect(averageTime).toBeLessThan(totalTime);
  });

  it('should handle memory usage efficiently', async () => {
    const initialMemory = process.memoryUsage();
    
    // 加载大量模块
    const modulePromises = Array.from({ length: 50 }, (_, i) => 
      import(`./testData/module${i % 5}.js`) // 重复使用5个模块
    );

    await Promise.all(modulePromises);

    const finalMemory = process.memoryUsage();
    const memoryIncrease = finalMemory.heapUsed - initialMemory.heapUsed;

    // 验证内存增长在合理范围内（考虑模块缓存）
    expect(memoryIncrease).toBeLessThan(50 * 1024 * 1024); // 50MB
  });

  it('should demonstrate efficient module caching', async () => {
    const modulePath = './src/modules/heavyModule.js';
    
    // 第一次加载
    const firstLoadStart = performance.now();
    const firstModule = await import(modulePath);
    const firstLoadTime = performance.now() - firstLoadStart;

    // 第二次加载（应该从缓存获取）
    const secondLoadStart = performance.now();
    const secondModule = await import(modulePath);
    const secondLoadTime = performance.now() - secondLoadStart;

    // 验证缓存效果
    expect(secondModule).toBe(firstModule); // 同一个对象
    expect(secondLoadTime).toBeLessThan(firstLoadTime * 0.1); // 缓存加载应该快很多
  });
});
```

## 测试工具与配置

### 测试环境配置

```javascript
// vitest.config.js - 测试配置
import { defineConfig } from 'vitest/config';
import path from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./tests/setup.js'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      exclude: [
        'node_modules/',
        'tests/',
        '**/*.test.js',
        '**/*.config.js'
      ]
    },
    // 模块解析配置
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@tests': path.resolve(__dirname, './tests')
    },
    // 超时配置
    testTimeout: 10000,
    hookTimeout: 10000
  },
  // 模块转换配置
  esbuild: {
    target: 'es2020'
  }
});
```

```javascript
// tests/setup.js - 测试设置
import { vi } from 'vitest';

// 全局模拟
global.fetch = vi.fn();

// 模拟浏览器API
Object.defineProperty(window, 'performance', {
  value: {
    now: vi.fn(() => Date.now()),
    mark: vi.fn(),
    measure: vi.fn()
  }
});

// 模拟动态导入
const originalImport = global.import;
global.import = vi.fn().mockImplementation((path) => {
  // 对测试模块使用真实导入
  if (path.startsWith('./testData/') || path.startsWith('../')) {
    return originalImport(path);
  }
  
  // 对其他模块返回模拟
  return Promise.resolve({
    default: `mocked-${path}`,
    __esModule: true
  });
});

// 清理函数
afterEach(() => {
  vi.clearAllMocks();
});
```

### 测试数据生成

```javascript
// tests/testDataGenerator.js - 测试数据生成器
export class TestDataGenerator {
  static createMockModule(name, exports = {}) {
    return {
      default: exports.default || (() => `Mock ${name}`),
      ...exports,
      __moduleName: name,
      __isMock: true
    };
  }

  static createModuleTree(depth = 3, breadth = 3) {
    const tree = {
      modules: new Map(),
      dependencies: new Map()
    };

    for (let i = 0; i < breadth; i++) {
      const moduleName = `level0-module${i}`;
      const module = this.createMockModule(moduleName);
      
      tree.modules.set(moduleName, module);
      
      if (depth > 1) {
        const children = this.createModuleTree(depth - 1, breadth);
        tree.dependencies.set(moduleName, Array.from(children.modules.keys()));
        
        // 合并子模块
        children.modules.forEach((module, name) => {
          tree.modules.set(name, module);
        });
        
        children.dependencies.forEach((deps, name) => {
          tree.dependencies.set(name, deps);
        });
      }
    }

    return tree;
  }

  static async createAsyncModules(count = 10) {
    const modules = [];
    
    for (let i = 0; i < count; i++) {
      const delay = Math.random() * 1000; // 0-1秒随机延迟
      
      const module = await new Promise(resolve => {
        setTimeout(() => {
          resolve(this.createMockModule(`async-module-${i}`, {
            loadTime: delay,
            id: i
          }));
        }, delay);
      });
      
      modules.push(module);
    }

    return modules;
  }
}
```

通过这些全面的测试策略和工具，可以确保模块化代码的质量和可靠性，从单元测试到集成测试，从功能验证到性能监控，形成完整的测试体系。
