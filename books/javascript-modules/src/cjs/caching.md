# CommonJS模块缓存机制

CommonJS模块系统的一个重要特性是模块缓存（Module Caching）。理解缓存机制对于优化应用性能、避免重复执行代码以及管理模块状态至关重要。

## 模块缓存原理

### 1. 缓存机制概述

```javascript
// math.js
console.log('math.js被执行');

let counter = 0;

function add(a, b) {
    counter++;
    return a + b;
}

function getCounter() {
    return counter;
}

module.exports = { add, getCounter };
```

```javascript
// app.js
const math1 = require('./math'); // 输出: math.js被执行
const math2 = require('./math'); // 不会再次输出

console.log(math1 === math2); // true - 同一个对象
console.log(math1.add(2, 3)); // 5
console.log(math1.getCounter()); // 1
console.log(math2.getCounter()); // 1 - 共享状态
```

### 2. require.cache对象

Node.js将所有加载的模块存储在`require.cache`对象中：

```javascript
// cache-demo.js
console.log('加载前的缓存:', Object.keys(require.cache));

const fs = require('fs');
const path = require('path');
const myModule = require('./my-module');

console.log('加载后的缓存:', Object.keys(require.cache));

// 查看特定模块的缓存信息
const modulePath = require.resolve('./my-module');
console.log('模块路径:', modulePath);
console.log('缓存对象:', require.cache[modulePath]);
```

### 3. 缓存键规则

缓存的键是模块的**绝对路径**：

```javascript
// 相同模块的不同引用方式
const mod1 = require('./utils');
const mod2 = require('./utils.js');
const mod3 = require(path.resolve(__dirname, 'utils.js'));

// 这些都指向同一个缓存条目（如果解析为相同的绝对路径）
console.log(mod1 === mod2); // true
console.log(mod2 === mod3); // true
```

## 缓存操作

### 1. 查看缓存

```javascript
// cache-inspector.js
function showCache() {
    console.log('当前缓存的模块:');
    Object.keys(require.cache).forEach(path => {
        console.log(`  ${path}`);
    });
}

function showModuleInfo(modulePath) {
    const resolvedPath = require.resolve(modulePath);
    const cachedModule = require.cache[resolvedPath];
    
    if (cachedModule) {
        console.log(`模块信息: ${modulePath}`);
        console.log(`  绝对路径: ${resolvedPath}`);
        console.log(`  是否已加载: true`);
        console.log(`  导出对象:`, cachedModule.exports);
        console.log(`  子模块:`, cachedModule.children.map(child => child.id));
        console.log(`  父模块:`, cachedModule.parent ? cachedModule.parent.id : 'none');
    }
}

showCache();
require('./utils');
showModuleInfo('./utils');
```

### 2. 清除缓存

```javascript
// cache-cleaner.js
function clearModuleCache(modulePath) {
    const resolvedPath = require.resolve(modulePath);
    
    // 删除缓存条目
    delete require.cache[resolvedPath];
    
    console.log(`已清除模块缓存: ${resolvedPath}`);
}

function clearAllCache() {
    Object.keys(require.cache).forEach(path => {
        delete require.cache[path];
    });
    console.log('已清除所有模块缓存');
}

// 使用示例
const math1 = require('./math'); // 首次加载
clearModuleCache('./math');
const math2 = require('./math'); // 重新加载

console.log(math1 === math2); // false - 不同的对象实例
```

### 3. 重新加载模块

```javascript
// hot-reload.js
function reloadModule(modulePath) {
    // 清除缓存
    const resolvedPath = require.resolve(modulePath);
    delete require.cache[resolvedPath];
    
    // 重新加载
    return require(modulePath);
}

// 使用示例
let config = require('./config');
console.log('原始配置:', config);

// 模拟配置文件更改后重新加载
setTimeout(() => {
    config = reloadModule('./config');
    console.log('重新加载的配置:', config);
}, 1000);
```

## 缓存模式和最佳实践

### 1. 单例模式

```javascript
// database.js - 数据库连接单例
let dbConnection = null;

function createConnection() {
    console.log('创建新的数据库连接');
    return {
        query: (sql) => console.log(`执行SQL: ${sql}`),
        close: () => console.log('关闭数据库连接')
    };
}

function getConnection() {
    if (!dbConnection) {
        dbConnection = createConnection();
    }
    return dbConnection;
}

module.exports = { getConnection };
```

```javascript
// app.js
const db1 = require('./database').getConnection();
const db2 = require('./database').getConnection();

console.log(db1 === db2); // true - 同一个连接实例
```

### 2. 工厂模式

```javascript
// logger-factory.js
const loggers = new Map();

function createLogger(name) {
    return {
        name,
        log: (message) => console.log(`[${name}] ${message}`),
        error: (message) => console.error(`[${name}] ERROR: ${message}`)
    };
}

function getLogger(name) {
    if (!loggers.has(name)) {
        loggers.set(name, createLogger(name));
    }
    return loggers.get(name);
}

module.exports = { getLogger };
```

### 3. 配置模块

```javascript
// config.js
const config = {
    database: {
        host: process.env.DB_HOST || 'localhost',
        port: process.env.DB_PORT || 5432
    },
    api: {
        port: process.env.PORT || 3000,
        timeout: 30000
    }
};

// 提供更新配置的方法
function updateConfig(updates) {
    Object.assign(config, updates);
}

module.exports = {
    ...config,
    updateConfig
};
```

## 缓存相关问题和解决方案

### 1. 循环依赖中的缓存

```javascript
// a.js
console.log('a.js开始执行');
exports.name = 'module-a';

const b = require('./b');
console.log('a.js中b的值:', b);

exports.getValue = () => `a-${b.name}`;
console.log('a.js执行完成');
```

```javascript
// b.js
console.log('b.js开始执行');
exports.name = 'module-b';

const a = require('./a');
console.log('b.js中a的值:', a); // 此时a可能是部分导出的对象

exports.getValue = () => `b-${a.name || 'undefined'}`;
console.log('b.js执行完成');
```

### 2. 开发环境热重载

```javascript
// dev-hot-reload.js
const fs = require('fs');
const path = require('path');

class ModuleHotReloader {
    constructor() {
        this.watchers = new Map();
    }
    
    watch(modulePath, callback) {
        const resolvedPath = require.resolve(modulePath);
        
        if (this.watchers.has(resolvedPath)) {
            return; // 已经在监听
        }
        
        const watcher = fs.watch(resolvedPath, (eventType) => {
            if (eventType === 'change') {
                console.log(`检测到文件变化: ${resolvedPath}`);
                
                // 清除缓存
                delete require.cache[resolvedPath];
                
                // 执行回调
                if (callback) {
                    try {
                        const newModule = require(modulePath);
                        callback(newModule, null);
                    } catch (error) {
                        callback(null, error);
                    }
                }
            }
        });
        
        this.watchers.set(resolvedPath, watcher);
    }
    
    stopWatching(modulePath) {
        const resolvedPath = require.resolve(modulePath);
        const watcher = this.watchers.get(resolvedPath);
        
        if (watcher) {
            watcher.close();
            this.watchers.delete(resolvedPath);
        }
    }
    
    stopAll() {
        this.watchers.forEach(watcher => watcher.close());
        this.watchers.clear();
    }
}

// 使用示例
const hotReloader = new ModuleHotReloader();

hotReloader.watch('./config', (newConfig, error) => {
    if (error) {
        console.error('重新加载配置失败:', error);
    } else {
        console.log('配置已更新:', newConfig);
    }
});
```

### 3. 内存泄漏预防

```javascript
// memory-management.js
class ModuleCacheManager {
    constructor() {
        this.moduleUsage = new Map();
    }
    
    trackModule(modulePath) {
        const resolvedPath = require.resolve(modulePath);
        const usage = this.moduleUsage.get(resolvedPath) || 0;
        this.moduleUsage.set(resolvedPath, usage + 1);
    }
    
    cleanup() {
        // 清理很少使用的模块
        this.moduleUsage.forEach((usage, path) => {
            if (usage < 2 && require.cache[path]) {
                delete require.cache[path];
                this.moduleUsage.delete(path);
                console.log(`清理低使用率模块: ${path}`);
            }
        });
    }
    
    getStats() {
        return {
            totalCached: Object.keys(require.cache).length,
            tracked: this.moduleUsage.size,
            usage: Array.from(this.moduleUsage.entries())
        };
    }
}

const cacheManager = new ModuleCacheManager();

// 定期清理
setInterval(() => {
    cacheManager.cleanup();
}, 60000); // 每分钟清理一次
```

## 高级缓存技巧

### 1. 条件缓存

```javascript
// conditional-cache.js
function requireWithCondition(modulePath, condition) {
    if (!condition) {
        // 如果条件不满足，临时清除缓存
        const resolvedPath = require.resolve(modulePath);
        const cached = require.cache[resolvedPath];
        delete require.cache[resolvedPath];
        
        try {
            return require(modulePath);
        } finally {
            // 恢复缓存（如果之前存在）
            if (cached) {
                require.cache[resolvedPath] = cached;
            }
        }
    }
    
    return require(modulePath);
}

// 使用示例
const isDevelopment = process.env.NODE_ENV === 'development';
const config = requireWithCondition('./config', !isDevelopment);
```

### 2. 缓存代理

```javascript
// cache-proxy.js
function createCacheProxy(require) {
    const originalRequire = require;
    const loadTimes = new Map();
    
    return function proxiedRequire(modulePath) {
        const startTime = Date.now();
        const module = originalRequire(modulePath);
        const loadTime = Date.now() - startTime;
        
        const resolvedPath = originalRequire.resolve(modulePath);
        const times = loadTimes.get(resolvedPath) || [];
        times.push(loadTime);
        loadTimes.set(resolvedPath, times);
        
        // 添加性能统计方法
        if (!module.__performance) {
            Object.defineProperty(module, '__performance', {
                value: {
                    getLoadTimes: () => loadTimes.get(resolvedPath) || [],
                    getAverageLoadTime: () => {
                        const times = loadTimes.get(resolvedPath) || [];
                        return times.reduce((sum, time) => sum + time, 0) / times.length;
                    }
                },
                enumerable: false
            });
        }
        
        return module;
    };
}

// 使用代理
const proxiedRequire = createCacheProxy(require);
const utils = proxiedRequire('./utils');
console.log('平均加载时间:', utils.__performance.getAverageLoadTime(), 'ms');
```

## 测试中的缓存管理

### 1. 测试隔离

```javascript
// test-helper.js
class TestCacheManager {
    constructor() {
        this.originalCache = null;
    }
    
    // 备份当前缓存
    backup() {
        this.originalCache = { ...require.cache };
    }
    
    // 恢复备份的缓存
    restore() {
        if (this.originalCache) {
            // 清除当前缓存
            Object.keys(require.cache).forEach(key => {
                delete require.cache[key];
            });
            
            // 恢复原始缓存
            Object.assign(require.cache, this.originalCache);
        }
    }
    
    // 清除特定模块的缓存
    clearModule(modulePath) {
        const resolvedPath = require.resolve(modulePath);
        delete require.cache[resolvedPath];
    }
    
    // 模拟新鲜环境
    isolate(callback) {
        this.backup();
        try {
            // 清除所有用户模块缓存（保留Node.js内置模块）
            Object.keys(require.cache).forEach(path => {
                if (!path.includes('node_modules') && !path.includes('internal')) {
                    delete require.cache[path];
                }
            });
            
            return callback();
        } finally {
            this.restore();
        }
    }
}

// 在测试中使用
const testCache = new TestCacheManager();

describe('Module tests', () => {
    beforeEach(() => {
        testCache.backup();
    });
    
    afterEach(() => {
        testCache.restore();
    });
    
    it('should load module fresh', () => {
        testCache.clearModule('./my-module');
        const module = require('./my-module');
        // 测试逻辑
    });
});
```

## 性能优化

### 1. 缓存预热

```javascript
// cache-warmup.js
function warmupCache(modules) {
    console.log('开始缓存预热...');
    const startTime = Date.now();
    
    modules.forEach(modulePath => {
        try {
            require(modulePath);
            console.log(`✓ 预热完成: ${modulePath}`);
        } catch (error) {
            console.error(`✗ 预热失败: ${modulePath}`, error.message);
        }
    });
    
    const warmupTime = Date.now() - startTime;
    console.log(`缓存预热完成，耗时: ${warmupTime}ms`);
}

// 应用启动时预热
const criticalModules = [
    './database',
    './cache',
    './logger',
    './config',
    './utils'
];

warmupCache(criticalModules);
```

### 2. 智能缓存清理

```javascript
// smart-cache-cleaner.js
class SmartCacheManager {
    constructor(options = {}) {
        this.maxCacheSize = options.maxCacheSize || 100;
        this.maxAge = options.maxAge || 3600000; // 1小时
        this.accessTimes = new Map();
        this.loadTimes = new Map();
    }
    
    trackAccess(modulePath) {
        const resolvedPath = require.resolve(modulePath);
        this.accessTimes.set(resolvedPath, Date.now());
    }
    
    shouldEvict(modulePath) {
        const resolvedPath = require.resolve(modulePath);
        const lastAccess = this.accessTimes.get(resolvedPath);
        
        if (!lastAccess) return true;
        
        // 超过最大年龄
        if (Date.now() - lastAccess > this.maxAge) {
            return true;
        }
        
        return false;
    }
    
    cleanup() {
        const cachedModules = Object.keys(require.cache);
        
        // 如果缓存数量超过限制
        if (cachedModules.length > this.maxCacheSize) {
            const candidates = cachedModules
                .filter(path => this.shouldEvict(path))
                .sort((a, b) => {
                    const aTime = this.accessTimes.get(a) || 0;
                    const bTime = this.accessTimes.get(b) || 0;
                    return aTime - bTime; // 最少最近使用
                });
            
            // 清理最老的模块
            const toRemove = candidates.slice(0, Math.floor(this.maxCacheSize * 0.1));
            toRemove.forEach(path => {
                delete require.cache[path];
                this.accessTimes.delete(path);
                console.log(`智能清理模块: ${path}`);
            });
        }
    }
    
    getStats() {
        return {
            cacheSize: Object.keys(require.cache).length,
            trackedAccess: this.accessTimes.size,
            maxCacheSize: this.maxCacheSize,
            oldestAccess: Math.min(...this.accessTimes.values()),
            newestAccess: Math.max(...this.accessTimes.values())
        };
    }
}
```

## 总结

CommonJS模块缓存机制是Node.js性能优化的重要组成部分：

- ✅ **自动缓存**: 模块首次加载后自动缓存，提高性能
- ✅ **状态共享**: 支持模块间状态共享和单例模式
- ✅ **灵活控制**: 可以手动清除和重新加载模块
- ✅ **开发友好**: 支持热重载和测试隔离
- ⚠️ **内存管理**: 需要注意内存泄漏和缓存清理
- ⚠️ **循环依赖**: 在循环依赖中需要特别小心缓存时机

理解和善用缓存机制能够显著提升Node.js应用的性能和开发体验。

---

**下一章**: [模块互操作性](./interop.md) →
