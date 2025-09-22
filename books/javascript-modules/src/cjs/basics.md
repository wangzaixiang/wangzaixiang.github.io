# CommonJS基础

CommonJS是Node.js采用的模块系统，也是JavaScript模块化历史上最重要的规范之一。虽然ES模块是现代标准，但理解CommonJS仍然至关重要，因为大量的Node.js代码和npm包仍在使用这种模块系统。

## 什么是CommonJS

### 背景和历史

CommonJS规范于2009年诞生，目标是为JavaScript提供一个服务器端的模块系统。在ES6模块出现之前，CommonJS是JavaScript模块化的事实标准。

```javascript
// CommonJS的核心理念
// 1. 每个文件都是一个模块
// 2. 模块内的变量和函数默认是私有的
// 3. 通过module.exports导出
// 4. 通过require()导入
// 5. 同步加载模块
```

### 核心特性

- **同步加载**: 模块在require时同步加载和执行
- **缓存机制**: 模块只执行一次，后续require返回缓存结果
- **动态加载**: 可以在运行时动态require模块
- **简单易用**: 语法简洁，学习成本低
- **Node.js原生支持**: Node.js内置支持，无需额外配置

## 基本语法

### 1. 模块导出

```javascript
// math.js - 基本导出示例

// 方式1: 直接给exports添加属性
exports.add = function(a, b) {
    return a + b;
};

exports.subtract = function(a, b) {
    return a - b;
};

exports.PI = 3.14159;

// 方式2: 使用module.exports
module.exports.multiply = function(a, b) {
    return a * b;
};

// 方式3: 整体替换module.exports
module.exports = {
    divide: function(a, b) {
        if (b === 0) {
            throw new Error('Division by zero');
        }
        return a / b;
    },
    power: function(base, exponent) {
        return Math.pow(base, exponent);
    }
};

// 注意: 一旦整体替换module.exports，之前的exports.*都会失效
```

### 2. 模块导入

```javascript
// app.js - 基本导入示例

// 导入整个模块
const math = require('./math');
console.log(math.add(2, 3)); // 5

// 解构导入特定函数
const { add, subtract, PI } = require('./math');
console.log(add(5, 3)); // 8
console.log(PI); // 3.14159

// 导入并重命名
const { add: sum, subtract: diff } = require('./math');
console.log(sum(10, 5)); // 15
console.log(diff(10, 5)); // 5

// 导入内置模块
const fs = require('fs');
const path = require('path');
const http = require('http');

// 导入npm包
const lodash = require('lodash');
const express = require('express');
```

### 3. 不同的导出模式

```javascript
// user.js - 类导出示例
class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
    }
    
    getInfo() {
        return `${this.name} <${this.email}>`;
    }
}

// 导出类
module.exports = User;

// config.js - 对象导出示例
const config = {
    database: {
        host: 'localhost',
        port: 5432,
        name: 'myapp'
    },
    server: {
        port: 3000,
        env: process.env.NODE_ENV || 'development'
    },
    secrets: {
        jwtSecret: process.env.JWT_SECRET || 'default-secret'
    }
};

module.exports = config;

// utils.js - 函数集合导出
function formatDate(date) {
    return date.toISOString().split('T')[0];
}

function isValidEmail(email) {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
}

function generateId() {
    return Math.random().toString(36).substr(2, 9);
}

// 批量导出
module.exports = {
    formatDate,
    isValidEmail,
    generateId
};

// logger.js - 单函数导出
function createLogger(level = 'info') {
    return {
        info: (message) => console.log(`[INFO] ${message}`),
        warn: (message) => console.warn(`[WARN] ${message}`),
        error: (message) => console.error(`[ERROR] ${message}`)
    };
}

module.exports = createLogger;
```

## exports vs module.exports

### 理解两者的关系

```javascript
// 理解exports和module.exports的关系

// Node.js内部的模块包装器大致如下：
function wrapModule(moduleCode) {
    return `
    (function(exports, require, module, __filename, __dirname) {
        ${moduleCode}
        return module.exports;
    });
    `;
}

// 初始状态下：exports === module.exports
console.log(exports === module.exports); // true

// exports是module.exports的引用
// 所以下面两种写法是等价的：
exports.hello = 'world';
module.exports.hello = 'world';
```

### 常见陷阱

```javascript
// trap.js - 常见陷阱示例

// ❌ 错误用法1: 直接给exports赋值
exports = {
    name: 'John',
    age: 30
};
// 这样做只是改变了exports的指向，不会影响module.exports

// ❌ 错误用法2: 混用exports和module.exports
exports.method1 = function() { return 'method1'; };
module.exports = {
    method2: function() { return 'method2'; }
};
// module.exports会覆盖exports的设置

// ✅ 正确用法1: 始终使用exports添加属性
exports.method1 = function() { return 'method1'; };
exports.method2 = function() { return 'method2'; };

// ✅ 正确用法2: 始终使用module.exports
module.exports = {
    method1: function() { return 'method1'; },
    method2: function() { return 'method2'; }
};

// ✅ 正确用法3: 先设置module.exports，再使用exports
module.exports = {};
exports.method1 = function() { return 'method1'; };
exports.method2 = function() { return 'method2'; };
```

## 模块加载机制

### 1. 同步加载

```javascript
// sync-loading.js

console.log('Before require');

// require是同步的，会阻塞代码执行
const largeModule = require('./large-module');

console.log('After require');

// large-module.js
console.log('Large module loading...');
// 模拟大量计算
for (let i = 0; i < 1000000; i++) {
    // 大量计算
}
console.log('Large module loaded');

module.exports = {
    data: 'some data'
};

// 输出顺序：
// Before require
// Large module loading...
// Large module loaded
// After require
```

### 2. 模块缓存

```javascript
// cache-demo.js

// 第一次require会执行模块代码
const module1 = require('./counter');
console.log('First require:', module1.getCount()); // 0

module1.increment();
console.log('After increment:', module1.getCount()); // 1

// 第二次require返回缓存的模块
const module2 = require('./counter');
console.log('Second require:', module2.getCount()); // 1 (不是0！)

console.log(module1 === module2); // true (同一个对象)

// counter.js
console.log('Counter module executing...');

let count = 0;

module.exports = {
    increment() {
        count++;
    },
    getCount() {
        return count;
    }
};
```

### 3. 清除模块缓存

```javascript
// cache-management.js

// 查看模块缓存
console.log('Cached modules:', Object.keys(require.cache));

// 加载模块
const myModule = require('./my-module');

// 清除特定模块缓存
delete require.cache[require.resolve('./my-module')];

// 重新加载模块（会重新执行模块代码）
const reloadedModule = require('./my-module');

// 清除所有缓存（不建议在生产环境使用）
function clearAllCache() {
    Object.keys(require.cache).forEach(key => {
        delete require.cache[key];
    });
}

// 安全的模块重载函数
function safeReload(modulePath) {
    try {
        // 解析模块路径
        const resolvedPath = require.resolve(modulePath);
        
        // 删除缓存
        delete require.cache[resolvedPath];
        
        // 重新加载
        return require(modulePath);
    } catch (error) {
        console.error('Failed to reload module:', error);
        return null;
    }
}
```

## 模块解析算法

### 1. 解析规则

```javascript
// Node.js模块解析算法示例

// 1. 核心模块（优先级最高）
const fs = require('fs');          // 直接加载Node.js内置模块
const path = require('path');      // 内置模块不需要路径

// 2. 相对路径模块
const myModule = require('./my-module');     // ./my-module.js
const subModule = require('./sub/module');   // ./sub/module.js
const parentModule = require('../parent');   // ../parent.js

// 3. 绝对路径模块
const absoluteModule = require('/home/user/app/module');

// 4. node_modules中的模块
const lodash = require('lodash');           // node_modules/lodash
const express = require('express');         // node_modules/express

// Node.js查找node_modules的顺序：
// ./node_modules/module-name
// ../node_modules/module-name
// ../../node_modules/module-name
// ... 一直到文件系统根目录
```

### 2. 文件扩展名解析

```javascript
// extension-resolution.js

// Node.js会按以下顺序尝试扩展名：

// require('./module') 会依次尝试：
// 1. ./module.js
// 2. ./module.json
// 3. ./module.node

// 如果上述都不存在，会尝试目录：
// 4. ./module/package.json (查找main字段)
// 5. ./module/index.js
// 6. ./module/index.json
// 7. ./module/index.node

// 演示不同类型的模块加载
const jsModule = require('./utils');        // utils.js
const jsonConfig = require('./config');     // config.json
const packageModule = require('./my-package'); // my-package/index.js

// config.json
{
    "name": "my-app",
    "version": "1.0.0",
    "database": {
        "host": "localhost",
        "port": 5432
    }
}

// my-package/package.json
{
    "name": "my-package",
    "main": "lib/index.js"
}
```

### 3. package.json的作用

```javascript
// package-resolution.js

// package.json示例
{
    "name": "my-library",
    "version": "1.0.0",
    "main": "dist/index.js",        // CommonJS入口
    "module": "dist/index.esm.js",  // ES模块入口
    "exports": {                     // 新的导出字段
        ".": {
            "require": "./dist/index.js",
            "import": "./dist/index.esm.js"
        },
        "./utils": "./dist/utils.js"
    },
    "files": ["dist/"]
}

// 当require('my-library')时，Node.js会：
// 1. 查找node_modules/my-library/package.json
// 2. 读取main字段值：dist/index.js
// 3. 加载node_modules/my-library/dist/index.js
```

## CommonJS的高级特性

### 1. 动态require

```javascript
// dynamic-require.js

// 基于条件的动态加载
const environment = process.env.NODE_ENV || 'development';

let config;
if (environment === 'production') {
    config = require('./config/production');
} else if (environment === 'test') {
    config = require('./config/test');
} else {
    config = require('./config/development');
}

// 基于用户输入的动态加载
function loadPlugin(pluginName) {
    try {
        const plugin = require(`./plugins/${pluginName}`);
        return plugin;
    } catch (error) {
        console.error(`Failed to load plugin: ${pluginName}`, error);
        return null;
    }
}

// 批量加载模块
function loadModules(moduleNames) {
    const modules = {};
    
    moduleNames.forEach(name => {
        try {
            modules[name] = require(`./modules/${name}`);
        } catch (error) {
            console.warn(`Failed to load module: ${name}`);
        }
    });
    
    return modules;
}

// 使用示例
const plugins = ['auth', 'logger', 'database'];
const loadedModules = loadModules(plugins);
```

### 2. 条件导出

```javascript
// conditional-exports.js

// 基于环境的条件导出
if (process.env.NODE_ENV === 'development') {
    // 开发环境：导出详细的调试版本
    module.exports = {
        log: console.log,
        debug: console.debug,
        warn: console.warn,
        error: console.error,
        trace: console.trace
    };
} else {
    // 生产环境：导出简化版本
    module.exports = {
        log: () => {}, // 空操作
        debug: () => {},
        warn: console.warn,
        error: console.error,
        trace: () => {}
    };
}

// 基于功能检测的条件导出
const hasFileSystem = (() => {
    try {
        require('fs');
        return true;
    } catch {
        return false;
    }
})();

if (hasFileSystem) {
    module.exports = require('./file-storage');
} else {
    module.exports = require('./memory-storage');
}
```

### 3. 模块工厂模式

```javascript
// factory-pattern.js

// 模块工厂：根据参数创建不同的实例
function createLogger(options = {}) {
    const {
        level = 'info',
        prefix = '',
        timestamp = true
    } = options;
    
    const levels = {
        debug: 0,
        info: 1,
        warn: 2,
        error: 3
    };
    
    const currentLevel = levels[level] || 1;
    
    function log(targetLevel, message) {
        if (levels[targetLevel] >= currentLevel) {
            const time = timestamp ? new Date().toISOString() : '';
            const fullMessage = `${time} ${prefix} [${targetLevel.toUpperCase()}] ${message}`;
            console.log(fullMessage);
        }
    }
    
    return {
        debug: (msg) => log('debug', msg),
        info: (msg) => log('info', msg),
        warn: (msg) => log('warn', msg),
        error: (msg) => log('error', msg)
    };
}

module.exports = createLogger;

// 使用工厂模式
const logger = require('./logger-factory')({
    level: 'debug',
    prefix: '[MyApp]',
    timestamp: true
});

logger.info('Application started');
```

### 4. 模块单例模式

```javascript
// singleton-pattern.js

// 单例数据库连接
class Database {
    constructor() {
        if (Database.instance) {
            return Database.instance;
        }
        
        this.connection = null;
        this.connected = false;
        Database.instance = this;
    }
    
    connect(connectionString) {
        if (!this.connected) {
            console.log('Connecting to database...');
            this.connection = { connectionString };
            this.connected = true;
        }
        return this.connection;
    }
    
    query(sql) {
        if (!this.connected) {
            throw new Error('Database not connected');
        }
        console.log(`Executing query: ${sql}`);
        return Promise.resolve([]);
    }
}

// 导出单例实例
module.exports = new Database();

// 在任何地方使用都是同一个实例
const db1 = require('./database');
const db2 = require('./database');
console.log(db1 === db2); // true
```

## Node.js特有功能

### 1. 全局变量

```javascript
// globals-demo.js

console.log('Module information:');
console.log('__filename:', __filename);    // 当前文件的绝对路径
console.log('__dirname:', __dirname);      // 当前目录的绝对路径
console.log('module.id:', module.id);      // 模块标识符
console.log('module.filename:', module.filename); // 同__filename
console.log('module.loaded:', module.loaded);     // 模块是否已加载完成
console.log('module.parent:', module.parent);     // 父模块
console.log('module.children:', module.children); // 子模块列表

// require对象的属性
console.log('require.main:', require.main);       // 主模块
console.log('require.cache keys:', Object.keys(require.cache)); // 模块缓存
console.log('require.resolve("./utils"):', require.resolve('./utils')); // 解析模块路径

// 实用的路径操作
const path = require('path');

// 获取当前模块所在目录的其他文件
const configPath = path.join(__dirname, 'config.json');
const utilsPath = path.join(__dirname, 'utils', 'helpers.js');

console.log('Config path:', configPath);
console.log('Utils path:', utilsPath);
```

### 2. require.resolve

```javascript
// resolve-demo.js

// require.resolve返回模块的绝对路径，但不加载模块
const modulePath = require.resolve('./my-module');
console.log('Module path:', modulePath);

// 检查模块是否存在
function moduleExists(moduleName) {
    try {
        require.resolve(moduleName);
        return true;
    } catch (error) {
        return false;
    }
}

console.log('lodash exists:', moduleExists('lodash'));
console.log('non-existent exists:', moduleExists('non-existent-module'));

// 解析npm包的子模块
const lodashMapPath = require.resolve('lodash/map');
console.log('Lodash map path:', lodashMapPath);

// 动态加载可选依赖
function loadOptionalModule(moduleName) {
    try {
        const modulePath = require.resolve(moduleName);
        return require(modulePath);
    } catch (error) {
        console.warn(`Optional module ${moduleName} not found`);
        return null;
    }
}

const optionalModule = loadOptionalModule('optional-dependency');
```

### 3. 模块热重载

```javascript
// hot-reload.js

const fs = require('fs');
const path = require('path');

class ModuleHotReloader {
    constructor() {
        this.watchers = new Map();
    }
    
    watch(modulePath, callback) {
        const absolutePath = require.resolve(modulePath);
        
        if (this.watchers.has(absolutePath)) {
            return;
        }
        
        const watcher = fs.watchFile(absolutePath, (curr, prev) => {
            if (curr.mtime > prev.mtime) {
                console.log(`Module ${modulePath} changed, reloading...`);
                
                // 清除缓存
                delete require.cache[absolutePath];
                
                // 重新加载
                try {
                    const reloadedModule = require(modulePath);
                    callback(reloadedModule);
                } catch (error) {
                    console.error('Failed to reload module:', error);
                }
            }
        });
        
        this.watchers.set(absolutePath, watcher);
    }
    
    unwatch(modulePath) {
        const absolutePath = require.resolve(modulePath);
        const watcher = this.watchers.get(absolutePath);
        
        if (watcher) {
            fs.unwatchFile(absolutePath);
            this.watchers.delete(absolutePath);
        }
    }
    
    unwatchAll() {
        this.watchers.forEach((watcher, path) => {
            fs.unwatchFile(path);
        });
        this.watchers.clear();
    }
}

// 使用示例
const reloader = new ModuleHotReloader();

reloader.watch('./config', (newConfig) => {
    console.log('Config reloaded:', newConfig);
    // 更新应用配置
});
```

## 性能优化

### 1. 延迟加载

```javascript
// lazy-loading.js

// 延迟加载大型模块
class LazyLoader {
    constructor() {
        this._cache = new Map();
    }
    
    lazy(modulePath) {
        return new Proxy({}, {
            get: (target, prop) => {
                if (!this._cache.has(modulePath)) {
                    console.log(`Lazy loading: ${modulePath}`);
                    this._cache.set(modulePath, require(modulePath));
                }
                
                const module = this._cache.get(modulePath);
                return module[prop];
            }
        });
    }
}

const loader = new LazyLoader();

// 只有在第一次访问时才会加载模块
const heavyModule = loader.lazy('./heavy-computation');

// 这行代码才会触发模块加载
console.log(heavyModule.calculate(100));
```

### 2. 模块预加载

```javascript
// preload.js

class ModulePreloader {
    constructor() {
        this.preloaded = new Map();
    }
    
    preload(modules) {
        modules.forEach(modulePath => {
            setImmediate(() => {
                try {
                    this.preloaded.set(modulePath, require(modulePath));
                    console.log(`Preloaded: ${modulePath}`);
                } catch (error) {
                    console.warn(`Failed to preload: ${modulePath}`, error);
                }
            });
        });
    }
    
    get(modulePath) {
        if (this.preloaded.has(modulePath)) {
            return this.preloaded.get(modulePath);
        }
        return require(modulePath);
    }
}

// 应用启动时预加载常用模块
const preloader = new ModulePreloader();
preloader.preload([
    'lodash',
    'moment',
    './utils/helpers',
    './config/database'
]);

// 后续使用预加载的模块
setTimeout(() => {
    const lodash = preloader.get('lodash');
    const helpers = preloader.get('./utils/helpers');
}, 1000);
```

## 最佳实践

### 1. 模块导出策略

```javascript
// export-strategies.js

// ✅ 推荐：明确的导出
module.exports = {
    // 明确列出所有导出
    createUser,
    updateUser,
    deleteUser,
    UserValidationError
};

// ✅ 推荐：单一职责导出
module.exports = class UserService {
    constructor(database) {
        this.db = database;
    }
    
    async createUser(userData) {
        // 实现
    }
};

// ❌ 避免：混用exports和module.exports
exports.method1 = () => {};
module.exports.method2 = () => {}; // 不一致

// ❌ 避免：导出过多内容
module.exports = {
    // 导出了太多不相关的内容
    UserService,
    ProductService,
    OrderService,
    DatabaseConnection,
    Logger,
    Config,
    Utils
};
```

### 2. 错误处理

```javascript
// error-handling.js

// ✅ 推荐：优雅的错误处理
function safeRequire(modulePath, fallback = null) {
    try {
        return require(modulePath);
    } catch (error) {
        if (error.code === 'MODULE_NOT_FOUND') {
            console.warn(`Module not found: ${modulePath}`);
            return fallback;
        }
        throw error; // 重新抛出其他类型的错误
    }
}

// 使用示例
const optionalConfig = safeRequire('./optional-config', {});
const logger = safeRequire('./logger', console);

// ✅ 推荐：模块初始化错误处理
function createDatabaseModule() {
    let connection = null;
    
    function connect() {
        if (!connection) {
            try {
                connection = require('./database-connection')();
            } catch (error) {
                console.error('Failed to initialize database:', error);
                throw new Error('Database module initialization failed');
            }
        }
        return connection;
    }
    
    return {
        connect,
        query: (sql) => connect().query(sql)
    };
}

module.exports = createDatabaseModule();
```

### 3. 模块测试

```javascript
// testable-module.js

// ✅ 可测试的模块设计
class UserService {
    constructor(dependencies = {}) {
        // 依赖注入，便于测试
        this.database = dependencies.database || require('./database');
        this.logger = dependencies.logger || require('./logger');
        this.emailService = dependencies.emailService || require('./email-service');
    }
    
    async createUser(userData) {
        try {
            // 验证数据
            this.validateUserData(userData);
            
            // 创建用户
            const user = await this.database.users.create(userData);
            
            // 发送欢迎邮件
            await this.emailService.sendWelcomeEmail(user.email);
            
            this.logger.info(`User created: ${user.id}`);
            return user;
        } catch (error) {
            this.logger.error('Failed to create user:', error);
            throw error;
        }
    }
    
    validateUserData(userData) {
        if (!userData.email) {
            throw new Error('Email is required');
        }
        // 更多验证逻辑
    }
}

module.exports = UserService;

// user-service.test.js
const UserService = require('./user-service');

describe('UserService', () => {
    it('should create user successfully', async () => {
        // 模拟依赖
        const mockDatabase = {
            users: {
                create: jest.fn().mockResolvedValue({ id: 1, email: 'test@example.com' })
            }
        };
        
        const mockLogger = {
            info: jest.fn(),
            error: jest.fn()
        };
        
        const mockEmailService = {
            sendWelcomeEmail: jest.fn().mockResolvedValue(true)
        };
        
        // 注入模拟依赖
        const userService = new UserService({
            database: mockDatabase,
            logger: mockLogger,
            emailService: mockEmailService
        });
        
        const userData = { email: 'test@example.com', name: 'Test User' };
        const result = await userService.createUser(userData);
        
        expect(result.id).toBe(1);
        expect(mockDatabase.users.create).toHaveBeenCalledWith(userData);
        expect(mockEmailService.sendWelcomeEmail).toHaveBeenCalledWith('test@example.com');
    });
});
```

## 总结

CommonJS作为Node.js的原生模块系统，具有以下特点：

- ✅ **简单易用**: 语法简洁，学习成本低
- ✅ **同步加载**: 适合服务器端开发，模块加载可预期
- ✅ **动态特性**: 支持运行时动态加载模块
- ✅ **成熟生态**: 大量npm包基于CommonJS构建
- ✅ **工具支持**: 丰富的开发工具和调试支持

理解CommonJS对于Node.js开发和理解JavaScript模块化演进历程都非常重要。虽然ES模块是未来趋势，但CommonJS仍在现代开发中扮演重要角色。

---

**下一章**: [require与module.exports](./require-exports.md) →