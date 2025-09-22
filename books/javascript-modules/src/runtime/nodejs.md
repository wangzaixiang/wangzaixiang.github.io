# Node.js中的模块

Node.js 作为服务端 JavaScript 运行环境，从一开始就支持模块化开发。它经历了从 CommonJS 到 ES 模块的演进，为服务端应用提供了强大的模块管理能力。

## Node.js 模块系统演进

### 发展历程

#### 1. **CommonJS 时代** (2009-2017)
- **原生 CommonJS 支持**：Node.js 从创建之初就支持 require/exports
- **同步加载**：适合服务端环境的同步模块加载
- **npm 生态**：围绕 CommonJS 建立的庞大包管理生态系统

#### 2. **ES 模块支持** (2017-至今)
- **实验性支持** (Node.js 8.5+)：通过 flag 启用 ES 模块
- **稳定支持** (Node.js 12+)：正式支持 ES 模块
- **双模块系统**：CommonJS 和 ES 模块并存

### 当前状态 (Node.js 18+)

```javascript
// Node.js 现在同时支持两种模块系统
// CommonJS (默认)
const fs = require('fs');
const { readFile } = require('fs/promises');

// ES 模块 (需要配置)
import fs from 'fs';
import { readFile } from 'fs/promises';
```

## CommonJS 模块系统

### 基础语法

#### **module.exports 和 exports**

```javascript
// math.js - 多种导出方式
// 1. 单个函数导出
module.exports = function add(a, b) {
  return a + b;
};

// 2. 多个导出
exports.add = (a, b) => a + b;
exports.subtract = (a, b) => a - b;
exports.PI = 3.14159;

// 3. 对象导出
module.exports = {
  add: (a, b) => a + b,
  subtract: (a, b) => a - b,
  constants: {
    PI: 3.14159,
    E: 2.71828
  }
};

// 4. 类导出
class Calculator {
  add(a, b) { return a + b; }
  subtract(a, b) { return a - b; }
}

module.exports = Calculator;
```

#### **require 导入**

```javascript
// 导入方式
// 1. 导入整个模块
const math = require('./math');
console.log(math.add(2, 3)); // 5

// 2. 解构导入
const { add, subtract } = require('./math');
console.log(add(2, 3)); // 5

// 3. 导入类
const Calculator = require('./calculator');
const calc = new Calculator();

// 4. 导入 Node.js 内置模块
const fs = require('fs');
const path = require('path');
const { createServer } = require('http');
```

### 模块解析机制

#### **模块查找顺序**

```javascript
// require('./user') 的查找顺序：
// 1. ./user
// 2. ./user.js
// 3. ./user.json
// 4. ./user.node
// 5. ./user/package.json (main字段)
// 6. ./user/index.js
// 7. ./user/index.json
// 8. ./user/index.node

// require('lodash') 的查找顺序：
// 1. 当前目录 node_modules/lodash
// 2. 父目录 ../node_modules/lodash
// 3. 继续向上查找直到根目录
// 4. 全局 node_modules 目录
// 5. Node.js 内置模块
```

#### **package.json 配置**

```json
{
  "name": "my-package",
  "version": "1.0.0",
  "main": "lib/index.js",          // CommonJS 入口
  "exports": {                     // 现代模块解析
    ".": {
      "import": "./esm/index.js",  // ES 模块入口
      "require": "./cjs/index.js"  // CommonJS 入口
    },
    "./utils": {
      "import": "./esm/utils.js",
      "require": "./cjs/utils.js"
    }
  },
  "type": "commonjs"               // 默认模块类型
}
```

### 模块缓存机制

```javascript
// 模块缓存示例
// counter.js
let count = 0;
exports.increment = () => ++count;
exports.getCount = () => count;

// app.js
const counter1 = require('./counter');
const counter2 = require('./counter'); // 从缓存加载

counter1.increment();
console.log(counter2.getCount()); // 1 - 共享状态

// 手动清除缓存
delete require.cache[require.resolve('./counter')];
const counter3 = require('./counter');
console.log(counter3.getCount()); // 0 - 重新初始化
```

## ES 模块系统

### 配置和启用

#### **package.json 配置**

```json
{
  "type": "module",  // 启用 ES 模块模式
  "exports": {
    ".": {
      "import": "./index.js",
      "require": "./index.cjs"
    }
  }
}
```

#### **文件扩展名规则**

```javascript
// 在 "type": "module" 项目中：
// .js   - ES 模块
// .mjs  - ES 模块 (明确)
// .cjs  - CommonJS 模块

// 在 "type": "commonjs" 项目中 (默认)：
// .js   - CommonJS 模块
// .mjs  - ES 模块
// .cjs  - CommonJS 模块
```

### ES 模块语法

#### **导入导出**

```javascript
// math.mjs
export function add(a, b) {
  return a + b;
}

export const PI = 3.14159;

export default class Calculator {
  multiply(a, b) {
    return a * b;
  }
}

// app.mjs
import Calculator, { add, PI } from './math.mjs';
import * as math from './math.mjs';

const calc = new Calculator();
console.log(add(2, 3));
console.log(calc.multiply(4, 5));
```

#### **动态导入**

```javascript
// 条件导入
async function loadModule(env) {
  if (env === 'production') {
    const prod = await import('./config/production.js');
    return prod.config;
  } else {
    const dev = await import('./config/development.js');
    return dev.config;
  }
}

// 懒加载
async function processLargeData(data) {
  // 只在需要时加载重型模块
  const { heavyProcessor } = await import('./heavy-processor.js');
  return heavyProcessor.process(data);
}
```

### 内置模块导入

```javascript
// ES 模块方式导入内置模块
import { readFile, writeFile } from 'fs/promises';
import { join, dirname } from 'path';
import { createServer } from 'http';
import { fileURLToPath } from 'url';

// 获取当前文件路径 (ES 模块中没有 __dirname)
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 使用 import() 动态导入 CommonJS 模块
const config = await import('./config.json', {
  assert: { type: 'json' }
});
```

## 双模块系统互操作

### CommonJS 导入 ES 模块

```javascript
// CommonJS 文件 (.cjs 或在 type: "commonjs" 项目中)
async function loadESModule() {
  // 只能通过动态导入
  const { default: Calculator, add } = await import('./math.mjs');
  
  const calc = new Calculator();
  return calc.multiply(add(2, 3), 4);
}

// 不能使用 require() 导入 ES 模块
// const math = require('./math.mjs'); // 错误!
```

### ES 模块导入 CommonJS

```javascript
// ES 模块文件 (.mjs 或在 type: "module" 项目中)
// 1. 默认导入
import fs from 'fs';              // 整个 exports 对象
import express from 'express';    // CommonJS 库

// 2. 命名导入 (如果支持)
import { readFile } from 'fs/promises';

// 3. 动态导入
const lodash = await import('lodash');
const _ = lodash.default;

// 4. 创建 require 函数 (兼容方案)
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const oldModule = require('./old-commonjs-module');
```

## Node.js 特有特性

### **文件系统操作**

```javascript
// ES 模块中的文件操作
import { readFile, writeFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function processFile() {
  const configPath = join(__dirname, 'config.json');
  const data = await readFile(configPath, 'utf8');
  const config = JSON.parse(data);
  
  // 处理配置...
  
  await writeFile(configPath, JSON.stringify(config, null, 2));
}
```

### **环境变量和配置**

```javascript
// 环境配置模块
// config.js
const config = {
  port: process.env.PORT || 3000,
  database: {
    host: process.env.DB_HOST || 'localhost',
    port: process.env.DB_PORT || 5432,
    name: process.env.DB_NAME || 'myapp'
  },
  jwt: {
    secret: process.env.JWT_SECRET || 'default-secret',
    expiresIn: process.env.JWT_EXPIRES || '1h'
  }
};

export default config;

// app.js
import config from './config.js';
import { createServer } from 'http';

const server = createServer((req, res) => {
  res.writeHead(200, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify({ message: 'Server running', config }));
});

server.listen(config.port, () => {
  console.log(`Server running on port ${config.port}`);
});
```

### **进程管理和集群**

```javascript
// cluster.js - 多进程管理
import cluster from 'cluster';
import { cpus } from 'os';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const numCPUs = cpus().length;

if (cluster.isPrimary) {
  console.log(`Primary ${process.pid} is running`);
  
  // Fork workers
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
  }
  
  cluster.on('exit', (worker, code, signal) => {
    console.log(`Worker ${worker.process.pid} died`);
    cluster.fork(); // 重启死掉的worker
  });
  
} else {
  // Worker 进程
  const { startApp } = await import('./app.js');
  startApp();
  
  console.log(`Worker ${process.pid} started`);
}
```

## 性能优化策略

### **模块预加载和缓存**

```javascript
// 模块预加载器
class ModulePreloader {
  constructor() {
    this.cache = new Map();
  }
  
  async preload(moduleUrls) {
    const promises = moduleUrls.map(async url => {
      try {
        const module = await import(url);
        this.cache.set(url, module);
        return { url, success: true };
      } catch (error) {
        console.warn(`Failed to preload ${url}:`, error);
        return { url, success: false, error };
      }
    });
    
    return Promise.all(promises);
  }
  
  get(url) {
    return this.cache.get(url);
  }
}

// 使用示例
const preloader = new ModulePreloader();
await preloader.preload([
  './heavy-computation.js',
  './database-utils.js',
  './email-service.js'
]);
```

### **代码分割和懒加载**

```javascript
// 路由懒加载
class Router {
  constructor() {
    this.routes = new Map();
  }
  
  register(path, moduleUrl) {
    this.routes.set(path, moduleUrl);
  }
  
  async handle(request) {
    const url = new URL(request.url, 'http://localhost');
    const moduleUrl = this.routes.get(url.pathname);
    
    if (!moduleUrl) {
      return { status: 404, body: 'Not Found' };
    }
    
    try {
      const module = await import(moduleUrl);
      return await module.default(request);
    } catch (error) {
      console.error('Route handler error:', error);
      return { status: 500, body: 'Internal Server Error' };
    }
  }
}

// 使用示例
const router = new Router();
router.register('/api/users', './handlers/users.js');
router.register('/api/posts', './handlers/posts.js');
router.register('/api/auth', './handlers/auth.js');
```

### **内存管理**

```javascript
// 内存高效的模块加载
import { Worker, isMainThread, parentPort } from 'worker_threads';

class MemoryEfficientProcessor {
  constructor() {
    this.workers = [];
    this.taskQueue = [];
  }
  
  async processInWorker(data, moduleUrl) {
    return new Promise((resolve, reject) => {
      const worker = new Worker(`
        import { parentPort } from 'worker_threads';
        import processor from '${moduleUrl}';
        
        parentPort.on('message', async (data) => {
          try {
            const result = await processor.process(data);
            parentPort.postMessage({ success: true, result });
          } catch (error) {
            parentPort.postMessage({ success: false, error: error.message });
          }
          process.exit(0); // 处理完后立即退出，释放内存
        });
      `, { eval: true });
      
      worker.postMessage(data);
      
      worker.on('message', ({ success, result, error }) => {
        if (success) {
          resolve(result);
        } else {
          reject(new Error(error));
        }
      });
      
      worker.on('error', reject);
    });
  }
}
```

## 调试和开发工具

### **调试配置**

```javascript
// debug.js - 开发环境调试工具
import { inspect } from 'util';

export class Debug {
  constructor(namespace) {
    this.namespace = namespace;
    this.enabled = process.env.DEBUG?.includes(namespace) || false;
  }
  
  log(...args) {
    if (!this.enabled) return;
    
    const timestamp = new Date().toISOString();
    const prefix = `[${timestamp}] ${this.namespace}:`;
    
    console.log(prefix, ...args.map(arg => 
      typeof arg === 'object' ? inspect(arg, { colors: true, depth: 3 }) : arg
    ));
  }
  
  error(...args) {
    const timestamp = new Date().toISOString();
    const prefix = `[${timestamp}] ${this.namespace}:ERROR`;
    console.error(prefix, ...args);
  }
}

// 使用示例
const debug = new Debug('app:server');
debug.log('Server starting...', { port: 3000, env: process.env.NODE_ENV });
```

### **性能监控**

```javascript
// performance.js
import { performance, PerformanceObserver } from 'perf_hooks';

export class PerformanceMonitor {
  constructor() {
    this.metrics = new Map();
    this.setupObserver();
  }
  
  setupObserver() {
    const obs = new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        this.recordMetric(entry.name, entry.duration);
      }
    });
    obs.observe({ entryTypes: ['measure'] });
  }
  
  start(name) {
    performance.mark(`${name}-start`);
  }
  
  end(name) {
    performance.mark(`${name}-end`);
    performance.measure(name, `${name}-start`, `${name}-end`);
  }
  
  recordMetric(name, value) {
    if (!this.metrics.has(name)) {
      this.metrics.set(name, []);
    }
    this.metrics.get(name).push(value);
  }
  
  getAverageTime(name) {
    const times = this.metrics.get(name) || [];
    return times.length > 0 ? times.reduce((a, b) => a + b) / times.length : 0;
  }
}

// 使用示例
const monitor = new PerformanceMonitor();

async function loadHeavyModule() {
  monitor.start('module-load');
  const module = await import('./heavy-module.js');
  monitor.end('module-load');
  return module;
}
```

## 打包和部署

### **生产环境优化**

```javascript
// build.js - 构建脚本
import { build } from 'esbuild';
import { readdir, stat } from 'fs/promises';
import { join } from 'path';

async function findEntryPoints(dir) {
  const entries = [];
  const files = await readdir(dir);
  
  for (const file of files) {
    const filePath = join(dir, file);
    const stats = await stat(filePath);
    
    if (stats.isFile() && file.endsWith('.js')) {
      entries.push(filePath);
    }
  }
  
  return entries;
}

async function buildForProduction() {
  const entryPoints = await findEntryPoints('./src');
  
  await build({
    entryPoints,
    bundle: true,
    minify: true,
    sourcemap: false,
    target: 'node18',
    platform: 'node',
    format: 'esm',
    outdir: './dist',
    external: ['sharp', 'canvas'] // 不打包的原生依赖
  });
  
  console.log('Build completed successfully!');
}

buildForProduction().catch(console.error);
```

### **Docker 配置**

```dockerfile
# Dockerfile
FROM node:18-alpine

WORKDIR /app

# 复制 package 文件
COPY package*.json ./

# 安装依赖
RUN npm ci --only=production

# 复制源码
COPY dist/ ./dist/

# 设置环境变量
ENV NODE_ENV=production

# 启动应用
CMD ["node", "dist/index.js"]
```

## 最佳实践总结

### **模块组织**
1. **清晰的模块边界** - 每个模块职责单一
2. **一致的导入路径** - 使用绝对路径或路径别名
3. **避免循环依赖** - 通过依赖注入或重构解决

### **性能优化**
1. **合理使用缓存** - 利用模块缓存机制
2. **懒加载非关键模块** - 提高启动速度
3. **Worker 隔离** - 防止内存泄漏

### **生产环境**
1. **环境配置分离** - 不同环境使用不同配置
2. **错误处理完善** - 模块加载失败的降级方案
3. **监控和日志** - 跟踪模块性能和错误

Node.js 的模块系统为服务端 JavaScript 开发提供了强大的能力，合理使用这些特性可以构建出高性能、可维护的后端应用。

---

**下一章**: [Deno的模块系统](./deno.md) →
