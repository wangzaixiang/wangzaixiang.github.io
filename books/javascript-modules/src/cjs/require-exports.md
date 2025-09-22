# require与module.exports

本章深入探讨CommonJS模块系统的两个核心机制：`require()`函数和`module.exports`对象。理解它们的工作原理和最佳实践对于编写高质量的Node.js代码至关重要。

## require()函数深入解析

### require()的工作流程

```javascript
// require()的内部工作流程示例

// 1. 路径解析
const resolvedPath = require.resolve('./my-module');

// 2. 检查缓存
if (require.cache[resolvedPath]) {
    return require.cache[resolvedPath].exports;
}

// 3. 创建新模块对象
const module = {
    id: resolvedPath,
    filename: resolvedPath,
    loaded: false,
    parent: currentModule,
    children: [],
    exports: {}
};

// 4. 加载并执行模块
const moduleWrapper = NativeModule.wrap(moduleContent);
const compiledWrapper = vm.runInThisContext(moduleWrapper);
compiledWrapper.call(module.exports, module.exports, require, module, __filename, __dirname);

// 5. 标记模块已加载
module.loaded = true;

// 6. 缓存模块
require.cache[resolvedPath] = module;

// 7. 返回exports
return module.exports;
```

### require()的不同用法

```javascript
// require-usage.js

// 1. 基本用法
const fs = require('fs');
const path = require('path');
const myModule = require('./my-module');

// 2. 条件require
let config;
if (process.env.NODE_ENV === 'production') {
    config = require('./config/production.json');
} else {
    config = require('./config/development.json');
}

// 3. 动态require
function loadModule(moduleName) {
    try {
        return require(moduleName);
    } catch (error) {
        console.log(`Module ${moduleName} not found`);
        return null;
    }
}

// 4. require表达式计算
const moduleName = './modules/' + process.argv[2];
const dynamicModule = require(moduleName);

// 5. 解构require
const { readFile, writeFile } = require('fs/promises');
const { join, dirname } = require('path');

// 6. require JSON文件
const packageInfo = require('./package.json');
const config = require('./config.json');

// 7. require.resolve - 仅解析路径不加载
const modulePath = require.resolve('lodash');
console.log('Lodash path:', modulePath);

// 8. 检查模块是否存在
function moduleExists(name) {
    try {
        require.resolve(name);
        return true;
    } catch (e) {
        return false;
    }
}
```

### require缓存机制

```javascript
// require-cache.js

// 演示模块缓存
console.log('=== 模块缓存演示 ===');

// 第一次require - 执行模块代码
console.log('First require:');
const module1 = require('./counter-module');
console.log('Counter value:', module1.getValue()); // 0

// 增加计数
module1.increment();
module1.increment();
console.log('After increment:', module1.getValue()); // 2

// 第二次require - 返回缓存
console.log('Second require:');
const module2 = require('./counter-module');
console.log('Counter value:', module2.getValue()); // 2 (不是0!)

// 验证是同一个对象
console.log('Same object?', module1 === module2); // true

// counter-module.js
console.log('Counter module executing...');

let counter = 0;

module.exports = {
    increment() {
        counter++;
    },
    getValue() {
        return counter;
    }
};

// 缓存操作
console.log('\n=== 缓存操作 ===');

// 查看缓存的模块
console.log('Cached modules:');
Object.keys(require.cache).forEach(key => {
    console.log(key);
});

// 删除缓存并重新加载
const modulePath = require.resolve('./counter-module');
delete require.cache[modulePath];

console.log('After clearing cache:');
const module3 = require('./counter-module');
console.log('Counter value:', module3.getValue()); // 0 (重新执行了模块)
```

### require()的错误处理

```javascript
// require-error-handling.js

// 基本错误处理
function safeRequire(modulePath, defaultValue = null) {
    try {
        return require(modulePath);
    } catch (error) {
        console.log(`Failed to require ${modulePath}:`, error.message);
        return defaultValue;
    }
}

// 区分不同类型的错误
function detailedRequire(modulePath) {
    try {
        return require(modulePath);
    } catch (error) {
        switch (error.code) {
            case 'MODULE_NOT_FOUND':
                console.log(`Module not found: ${modulePath}`);
                break;
            case 'ENOENT':
                console.log(`File not found: ${modulePath}`);
                break;
            default:
                console.log(`Error loading module: ${error.message}`);
                throw error; // 重新抛出未知错误
        }
        return null;
    }
}

// 可选依赖加载
function loadOptionalDependencies(dependencies) {
    const loaded = {};
    
    dependencies.forEach(dep => {
        try {
            loaded[dep] = require(dep);
            console.log(`✅ Loaded: ${dep}`);
        } catch (error) {
            console.log(`⚠️  Optional dependency not found: ${dep}`);
            loaded[dep] = null;
        }
    });
    
    return loaded;
}

// 使用示例
const optionalDeps = loadOptionalDependencies([
    'colors',
    'moment',
    'non-existent-package'
]);

// 带重试的require
function requireWithRetry(modulePath, maxRetries = 3) {
    for (let i = 0; i < maxRetries; i++) {
        try {
            return require(modulePath);
        } catch (error) {
            if (i === maxRetries - 1) {
                throw error;
            }
            console.log(`Retry ${i + 1} for ${modulePath}`);
        }
    }
}
```

## module.exports详解

### module.exports vs exports

```javascript
// exports-comparison.js

// 理解module.exports和exports的关系
console.log('初始状态:');
console.log('exports === module.exports:', exports === module.exports); // true

// exports是module.exports的引用
exports.method1 = function() {
    return 'method1 from exports';
};

module.exports.method2 = function() {
    return 'method2 from module.exports';
};

console.log('添加方法后:');
console.log('exports === module.exports:', exports === module.exports); // 仍然是true

// 危险操作：重新赋值exports
exports = {
    method3: function() {
        return 'method3';
    }
};

console.log('重新赋值exports后:');
console.log('exports === module.exports:', exports === module.exports); // false!

// 此时module.exports仍然包含method1和method2
// 但exports指向了新对象，包含method3

// 正确的重新赋值方式
module.exports = {
    method4: function() {
        return 'method4';
    }
};

// 现在module.exports只包含method4了
```

### 不同的导出模式

```javascript
// export-patterns.js

// 1. 对象导出模式
module.exports = {
    name: 'MyModule',
    version: '1.0.0',
    
    init() {
        console.log(`${this.name} v${this.version} initialized`);
    },
    
    process(data) {
        return data.toUpperCase();
    }
};

// 2. 类导出模式
class Calculator {
    add(a, b) { return a + b; }
    subtract(a, b) { return a - b; }
    multiply(a, b) { return a * b; }
    divide(a, b) { 
        if (b === 0) throw new Error('Division by zero');
        return a / b; 
    }
}

module.exports = Calculator;

// 3. 函数导出模式
function createLogger(level = 'info') {
    const levels = ['debug', 'info', 'warn', 'error'];
    const currentLevel = levels.indexOf(level);
    
    return {
        log(message, msgLevel = 'info') {
            const msgIndex = levels.indexOf(msgLevel);
            if (msgIndex >= currentLevel) {
                console.log(`[${msgLevel.toUpperCase()}] ${message}`);
            }
        }
    };
}

module.exports = createLogger;

// 4. 混合导出模式
function mainFunction() {
    return 'Main functionality';
}

// 将函数本身作为默认导出
module.exports = mainFunction;

// 添加额外的属性和方法
module.exports.helper = function() {
    return 'Helper functionality';
};

module.exports.constant = 42;

module.exports.SubClass = class {
    constructor() {
        this.name = 'SubClass';
    }
};

// 5. 条件导出模式
if (process.env.NODE_ENV === 'development') {
    module.exports = {
        log: console.log,
        warn: console.warn,
        error: console.error,
        debug: console.debug
    };
} else {
    module.exports = {
        log() {}, // 生产环境静默
        warn: console.warn,
        error: console.error,
        debug() {}
    };
}
```

### 渐进式导出

```javascript
// progressive-exports.js

// 可以逐步构建exports对象

// 先创建基础结构
module.exports = {};

// 添加常量
module.exports.VERSION = '2.1.0';
module.exports.DEFAULT_CONFIG = {
    timeout: 5000,
    retries: 3
};

// 添加工具函数
module.exports.utils = {};

module.exports.utils.formatDate = function(date) {
    return date.toISOString().split('T')[0];
};

module.exports.utils.generateId = function() {
    return Math.random().toString(36).substr(2, 9);
};

// 添加主要功能
module.exports.createClient = function(config = {}) {
    const finalConfig = { ...module.exports.DEFAULT_CONFIG, ...config };
    
    return {
        config: finalConfig,
        
        async request(url) {
            console.log(`Requesting ${url} with timeout ${finalConfig.timeout}ms`);
            // 模拟请求
            return { status: 200, data: 'response data' };
        }
    };
};

// 添加错误类
class ClientError extends Error {
    constructor(message, code) {
        super(message);
        this.name = 'ClientError';
        this.code = code;
    }
}

module.exports.ClientError = ClientError;

// 添加验证函数
module.exports.validate = {
    url(url) {
        try {
            new URL(url);
            return true;
        } catch {
            return false;
        }
    },
    
    config(config) {
        if (typeof config !== 'object') return false;
        if (config.timeout && typeof config.timeout !== 'number') return false;
        if (config.retries && typeof config.retries !== 'number') return false;
        return true;
    }
};
```

## 高级模式和技巧

### 1. 模块工厂模式

```javascript
// module-factory.js

// 工厂函数创建配置化的模块
function createDatabase(options = {}) {
    const {
        host = 'localhost',
        port = 5432,
        database = 'myapp',
        pool = { min: 2, max: 10 }
    } = options;
    
    let connection = null;
    
    return {
        async connect() {
            if (!connection) {
                console.log(`Connecting to ${host}:${port}/${database}`);
                connection = { 
                    host, 
                    port, 
                    database, 
                    connected: true 
                };
            }
            return connection;
        },
        
        async query(sql, params = []) {
            const conn = await this.connect();
            console.log(`Executing: ${sql}`, params);
            return { rows: [], rowCount: 0 };
        },
        
        async close() {
            if (connection) {
                console.log('Closing database connection');
                connection.connected = false;
                connection = null;
            }
        },
        
        getConfig() {
            return { host, port, database, pool };
        }
    };
}

module.exports = createDatabase;

// 使用工厂模式
// const db = require('./database-factory')({
//     host: 'prod-server',
//     port: 5432,
//     database: 'production_db'
// });
```

### 2. 单例模式

```javascript
// singleton-pattern.js

// 单例配置管理器
let instance = null;

class ConfigManager {
    constructor() {
        if (instance) {
            return instance;
        }
        
        this.config = {};
        this.loaded = false;
        instance = this;
    }
    
    load(configPath) {
        if (!this.loaded) {
            try {
                this.config = require(configPath);
                this.loaded = true;
                console.log('Configuration loaded');
            } catch (error) {
                console.error('Failed to load configuration:', error);
                this.config = {};
            }
        }
        return this;
    }
    
    get(key, defaultValue = null) {
        return this.getNestedValue(this.config, key, defaultValue);
    }
    
    set(key, value) {
        this.setNestedValue(this.config, key, value);
    }
    
    getNestedValue(obj, path, defaultValue) {
        const keys = path.split('.');
        let current = obj;
        
        for (const key of keys) {
            if (current === null || current === undefined || !(key in current)) {
                return defaultValue;
            }
            current = current[key];
        }
        
        return current;
    }
    
    setNestedValue(obj, path, value) {
        const keys = path.split('.');
        const lastKey = keys.pop();
        let current = obj;
        
        for (const key of keys) {
            if (!(key in current)) {
                current[key] = {};
            }
            current = current[key];
        }
        
        current[lastKey] = value;
    }
}

// 导出单例实例
module.exports = new ConfigManager();

// 无论在哪里require，都是同一个实例
// const config1 = require('./config-manager');
// const config2 = require('./config-manager');
// console.log(config1 === config2); // true
```

### 3. 插件系统

```javascript
// plugin-system.js

class PluginManager {
    constructor() {
        this.plugins = new Map();
        this.hooks = new Map();
    }
    
    register(name, plugin) {
        if (typeof plugin !== 'object') {
            throw new Error('Plugin must be an object');
        }
        
        if (typeof plugin.init !== 'function') {
            throw new Error('Plugin must have an init method');
        }
        
        this.plugins.set(name, plugin);
        console.log(`Plugin registered: ${name}`);
        
        // 初始化插件
        plugin.init(this);
        
        // 注册插件的钩子
        if (plugin.hooks) {
            Object.entries(plugin.hooks).forEach(([hookName, handler]) => {
                this.addHook(hookName, handler);
            });
        }
    }
    
    addHook(name, handler) {
        if (!this.hooks.has(name)) {
            this.hooks.set(name, []);
        }
        this.hooks.get(name).push(handler);
    }
    
    async executeHook(name, ...args) {
        const handlers = this.hooks.get(name) || [];
        const results = [];
        
        for (const handler of handlers) {
            try {
                const result = await handler(...args);
                results.push(result);
            } catch (error) {
                console.error(`Hook ${name} error:`, error);
            }
        }
        
        return results;
    }
    
    getPlugin(name) {
        return this.plugins.get(name);
    }
    
    unregister(name) {
        const plugin = this.plugins.get(name);
        if (plugin && typeof plugin.destroy === 'function') {
            plugin.destroy();
        }
        this.plugins.delete(name);
        console.log(`Plugin unregistered: ${name}`);
    }
}

// 导出插件管理器
const pluginManager = new PluginManager();

// 提供便捷的插件加载函数
function loadPlugin(pluginPath) {
    try {
        const plugin = require(pluginPath);
        const name = plugin.name || require('path').basename(pluginPath);
        pluginManager.register(name, plugin);
        return plugin;
    } catch (error) {
        console.error(`Failed to load plugin ${pluginPath}:`, error);
        return null;
    }
}

module.exports = {
    pluginManager,
    loadPlugin
};
```

### 4. 延迟初始化

```javascript
// lazy-initialization.js

// 延迟初始化的数据库连接
let _connection = null;
let _connecting = false;
let _connectionPromise = null;

async function getConnection() {
    if (_connection) {
        return _connection;
    }
    
    if (_connecting) {
        return _connectionPromise;
    }
    
    _connecting = true;
    _connectionPromise = initializeConnection();
    
    try {
        _connection = await _connectionPromise;
        return _connection;
    } finally {
        _connecting = false;
        _connectionPromise = null;
    }
}

async function initializeConnection() {
    console.log('Initializing database connection...');
    
    // 模拟异步连接过程
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    const connection = {
        id: Math.random().toString(36),
        connected: true,
        
        async query(sql) {
            if (!this.connected) {
                throw new Error('Connection closed');
            }
            console.log(`Executing: ${sql}`);
            return { rows: [], count: 0 };
        },
        
        async close() {
            this.connected = false;
            _connection = null;
            console.log('Connection closed');
        }
    };
    
    console.log('Database connection established');
    return connection;
}

// 导出接口
module.exports = {
    async query(sql) {
        const conn = await getConnection();
        return conn.query(sql);
    },
    
    async close() {
        if (_connection) {
            await _connection.close();
        }
    },
    
    isConnected() {
        return _connection && _connection.connected;
    }
};
```

## 性能优化技巧

### 1. 条件require

```javascript
// conditional-require.js

// 避免不必要的require
let heavyModule = null;

function getHeavyModule() {
    if (!heavyModule) {
        console.log('Loading heavy module...');
        heavyModule = require('./heavy-computation');
    }
    return heavyModule;
}

// 只有在需要时才加载
function performHeavyOperation(data) {
    const heavy = getHeavyModule();
    return heavy.process(data);
}

// 基于功能检测的条件require
let cryptoModule = null;

function getCrypto() {
    if (!cryptoModule) {
        try {
            cryptoModule = require('crypto');
        } catch (error) {
            // 回退到其他实现
            cryptoModule = require('./crypto-fallback');
        }
    }
    return cryptoModule;
}

// 基于环境的条件require
const debugMode = process.env.NODE_ENV === 'development';

let debugUtils = null;
if (debugMode) {
    debugUtils = require('./debug-utils');
}

function debug(message) {
    if (debugMode && debugUtils) {
        debugUtils.log(message);
    }
}

module.exports = {
    performHeavyOperation,
    getCrypto,
    debug
};
```

### 2. require缓存优化

```javascript
// cache-optimization.js

// 手动管理require缓存
class RequireCache {
    static clear(modulePath) {
        const resolvedPath = require.resolve(modulePath);
        delete require.cache[resolvedPath];
    }
    
    static clearPattern(pattern) {
        const regex = new RegExp(pattern);
        Object.keys(require.cache).forEach(key => {
            if (regex.test(key)) {
                delete require.cache[key];
            }
        });
    }
    
    static preload(modules) {
        modules.forEach(modulePath => {
            try {
                require(modulePath);
                console.log(`Preloaded: ${modulePath}`);
            } catch (error) {
                console.warn(`Failed to preload: ${modulePath}`);
            }
        });
    }
    
    static getStats() {
        const cached = Object.keys(require.cache);
        return {
            count: cached.length,
            modules: cached.map(path => ({
                path,
                loaded: require.cache[path].loaded
            }))
        };
    }
}

// 智能require封装
function smartRequire(modulePath, options = {}) {
    const { cache = true, reload = false } = options;
    
    if (reload) {
        RequireCache.clear(modulePath);
    }
    
    const module = require(modulePath);
    
    if (!cache) {
        // 立即清除缓存
        setImmediate(() => {
            RequireCache.clear(modulePath);
        });
    }
    
    return module;
}

module.exports = {
    RequireCache,
    smartRequire
};
```

## 调试和监控

### 1. require跟踪

```javascript
// require-tracer.js

// 跟踪require调用
const originalRequire = module.constructor.prototype.require;

module.constructor.prototype.require = function(id) {
    const start = Date.now();
    console.log(`📦 Requiring: ${id}`);
    
    try {
        const result = originalRequire.call(this, id);
        const duration = Date.now() - start;
        console.log(`✅ Loaded: ${id} (${duration}ms)`);
        return result;
    } catch (error) {
        const duration = Date.now() - start;
        console.log(`❌ Failed: ${id} (${duration}ms) - ${error.message}`);
        throw error;
    }
};

// 使用示例
const fs = require('fs');        // 📦 Requiring: fs / ✅ Loaded: fs (1ms)
const path = require('path');    // 📦 Requiring: path / ✅ Loaded: path (0ms)

// 恢复原始require
function restoreRequire() {
    module.constructor.prototype.require = originalRequire;
}

module.exports = { restoreRequire };
```

### 2. 模块依赖分析

```javascript
// dependency-analyzer.js

class DependencyAnalyzer {
    constructor() {
        this.dependencies = new Map();
        this.loadTimes = new Map();
    }
    
    analyze() {
        Object.entries(require.cache).forEach(([path, moduleObj]) => {
            const deps = moduleObj.children.map(child => child.filename);
            this.dependencies.set(path, deps);
        });
        
        return this.generateReport();
    }
    
    generateReport() {
        const report = {
            totalModules: this.dependencies.size,
            dependencies: {},
            circularDependencies: this.findCircularDependencies(),
            heaviestModules: this.findHeaviestModules()
        };
        
        this.dependencies.forEach((deps, module) => {
            report.dependencies[module] = {
                dependsOn: deps,
                dependentCount: deps.length
            };
        });
        
        return report;
    }
    
    findCircularDependencies() {
        // 简化的循环依赖检测
        const visited = new Set();
        const visiting = new Set();
        const cycles = [];
        
        const dfs = (module, path = []) => {
            if (visiting.has(module)) {
                const cycleStart = path.indexOf(module);
                cycles.push(path.slice(cycleStart).concat(module));
                return;
            }
            
            if (visited.has(module)) return;
            
            visiting.add(module);
            path.push(module);
            
            const deps = this.dependencies.get(module) || [];
            deps.forEach(dep => dfs(dep, [...path]));
            
            visiting.delete(module);
            visited.add(module);
        };
        
        this.dependencies.forEach((_, module) => {
            if (!visited.has(module)) {
                dfs(module);
            }
        });
        
        return cycles;
    }
    
    findHeaviestModules(top = 10) {
        const modules = Array.from(this.dependencies.entries())
            .map(([module, deps]) => ({
                module,
                dependencyCount: deps.length
            }))
            .sort((a, b) => b.dependencyCount - a.dependencyCount)
            .slice(0, top);
        
        return modules;
    }
}

module.exports = DependencyAnalyzer;
```

## 总结

`require()`和`module.exports`是CommonJS模块系统的核心：

- ✅ **require()**: 同步加载、缓存机制、路径解析
- ✅ **module.exports**: 灵活的导出方式、与exports的关系
- ✅ **高级模式**: 工厂模式、单例模式、插件系统
- ✅ **性能优化**: 条件加载、缓存管理、延迟初始化
- ✅ **调试工具**: 依赖跟踪、性能监控、循环依赖检测

理解这些机制有助于编写更高效、更可维护的Node.js应用。

---

**下一章**: [模块缓存机制](./caching.md) →