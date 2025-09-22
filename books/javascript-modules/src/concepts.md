# 模块化的核心概念

本章将介绍 JavaScript 模块化的基础概念，帮助你建立对模块系统的整体认知。

## 什么是模块

**模块（Module）** 是一个独立的、可复用的代码单元，它封装了特定的功能，并可以与其他模块进行交互。

### 模块的基本特征

1. **封装性**: 模块内部实现细节对外部不可见
2. **独立性**: 模块具有独立的作用域
3. **可复用性**: 可以在多个地方使用同一个模块
4. **接口明确**: 通过明确的接口进行交互

```javascript
// 一个简单的模块示例
// math.js
const PI = 3.14159;

function add(a, b) {
    return a + b;
}

function multiply(a, b) {
    return a * b;
}

// 对外提供的接口
export { add, multiply, PI };
```

## 模块系统的核心组成

### 1. 模块定义（Module Definition）

模块定义规定了如何创建和组织模块：

```javascript
// ES6 模块定义
// user.js
class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
    }
    
    getInfo() {
        return `${this.name} <${this.email}>`;
    }
}

// 导出模块
export default User;
export { User as UserClass };
```

### 2. 模块导入（Module Import）

定义如何使用其他模块的功能：

```javascript
// app.js
import User from './user.js';              // 导入默认导出
import { UserClass } from './user.js';     // 导入命名导出
import * as UserModule from './user.js';   // 导入所有导出

const user = new User('John', 'john@example.com');
```

### 3. 模块解析（Module Resolution）

规定如何查找和加载模块：

```javascript
// 不同的模块路径解析
import utils from './utils.js';           // 相对路径
import lodash from 'lodash';              // npm 包
import config from '../config/app.js';   // 相对路径（上级目录）
import api from '/src/api/index.js';     // 绝对路径
```

### 4. 模块加载（Module Loading）

模块系统决定何时以及如何加载模块：

```javascript
// 静态导入（编译时确定）
import { calculateTax } from './tax-utils.js';

// 动态导入（运行时确定）
const loadTaxUtils = async () => {
    const module = await import('./tax-utils.js');
    return module.calculateTax;
};
```

## 模块的作用域（Module Scope）

### 作用域隔离

每个模块都有自己的作用域，不会污染全局作用域：

```javascript
// module1.js
const message = 'Hello from module1';
let counter = 0;

export function increment() {
    counter++;
    console.log(`Module1 counter: ${counter}`);
}

// module2.js  
const message = 'Hello from module2'; // 不会与 module1 冲突
let counter = 100;

export function increment() {
    counter++;
    console.log(`Module2 counter: ${counter}`);
}
```

### 私有和公开成员

```javascript
// user-service.js
// 私有成员（不导出）
const API_KEY = 'secret-key';
const cache = new Map();

function validateUser(user) {
    return user && user.name && user.email;
}

// 公开成员（导出）
export class UserService {
    async getUser(id) {
        if (cache.has(id)) {
            return cache.get(id);
        }
        
        const user = await fetch(`/api/users/${id}`, {
            headers: { 'Authorization': API_KEY }
        }).then(r => r.json());
        
        if (validateUser(user)) {
            cache.set(id, user);
            return user;
        }
        
        throw new Error('Invalid user data');
    }
}
```

## 模块依赖关系

### 依赖图（Dependency Graph）

模块之间的依赖关系构成了一个有向图：

```javascript
// 依赖关系示例
// app.js → user-controller.js → user-service.js → api-client.js

// api-client.js
export class ApiClient {
    async get(url) { /* ... */ }
    async post(url, data) { /* ... */ }
}

// user-service.js
import { ApiClient } from './api-client.js';

export class UserService {
    constructor() {
        this.apiClient = new ApiClient();
    }
}

// user-controller.js
import { UserService } from './user-service.js';

export class UserController {
    constructor() {
        this.userService = new UserService();
    }
}

// app.js
import { UserController } from './user-controller.js';

const userController = new UserController();
```

### 循环依赖问题

循环依赖是模块化中需要避免的问题：

```javascript
// 问题示例：循环依赖
// moduleA.js
import { functionB } from './moduleB.js';

export function functionA() {
    return functionB() + 1;
}

// moduleB.js
import { functionA } from './moduleA.js'; // 循环依赖！

export function functionB() {
    return functionA() + 1; // 将导致错误
}
```

**解决方案**：

```javascript
// 方案 1：提取公共依赖
// common.js
export function baseFunction() {
    return 42;
}

// moduleA.js
import { baseFunction } from './common.js';

export function functionA() {
    return baseFunction() + 1;
}

// moduleB.js
import { baseFunction } from './common.js';

export function functionB() {
    return baseFunction() + 2;
}
```

## 模块的生命周期

### 1. 模块加载阶段

```javascript
// 模块被首次导入时执行
console.log('Module initializing...');

const config = {
    apiUrl: process.env.API_URL || 'http://localhost:3000'
};

export { config };
```

### 2. 模块缓存

模块只会被加载和执行一次，后续导入会使用缓存：

```javascript
// counter.js
let count = 0;

export function increment() {
    return ++count;
}

export function getCount() {
    return count;
}

// main.js
import { increment, getCount } from './counter.js';
import { increment as inc2 } from './counter.js'; // 同一个模块实例

console.log(increment()); // 1
console.log(inc2());      // 2 (共享状态)
console.log(getCount());  // 2
```

## 模块化设计原则

### 1. 单一职责原则（SRP）

每个模块应该只有一个明确的职责：

```javascript
// 好的示例：单一职责
// email-validator.js - 只负责邮箱验证
export function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

// password-validator.js - 只负责密码验证
export function validatePassword(password) {
    return password.length >= 8 && /[A-Z]/.test(password);
}
```

### 2. 接口隔离原则（ISP）

模块应该提供精细化的接口：

```javascript
// user-operations.js
export { createUser } from './user-create.js';
export { updateUser } from './user-update.js';
export { deleteUser } from './user-delete.js';
export { findUser } from './user-query.js';

// 使用者可以按需导入
import { createUser, findUser } from './user-operations.js';
```

### 3. 依赖倒置原则（DIP）

高层模块不应该依赖于低层模块，都应该依赖于抽象：

```javascript
// 抽象接口
// storage-interface.js
export class StorageInterface {
    async save(key, value) {
        throw new Error('Method must be implemented');
    }
    
    async load(key) {
        throw new Error('Method must be implemented');
    }
}

// 具体实现
// local-storage.js
import { StorageInterface } from './storage-interface.js';

export class LocalStorage extends StorageInterface {
    async save(key, value) {
        localStorage.setItem(key, JSON.stringify(value));
    }
    
    async load(key) {
        const item = localStorage.getItem(key);
        return item ? JSON.parse(item) : null;
    }
}

// 高层模块
// user-service.js
export class UserService {
    constructor(storage) { // 依赖抽象，而非具体实现
        this.storage = storage;
    }
    
    async saveUser(user) {
        await this.storage.save(`user_${user.id}`, user);
    }
}
```

## 主流模块系统对比

在了解了模块化的核心概念后，让我们来对比一下 JavaScript 生态系统中的主要模块系统：

### 基本特性对比

| 特性 | ES Modules (ESM) | CommonJS (CJS) | AMD | UMD | RequireJS |
|------|------------------|----------------|-----|-----|-----------|
| **标准状态** | ✅ ECMA标准 | ❌ Node.js事实标准 | ❌ 社区标准 | ❌ 社区标准 | ❌ 库实现 |
| **首次发布** | 2015 (ES6) | 2009 | 2011 | 2013 | 2010 |
| **加载方式** | 静态 + 动态 | 同步 | 异步 | 通用 | 异步 |
| **运行环境** | 浏览器 + Node.js | Node.js 主要 | 浏览器主要 | 通用 | 浏览器主要 |
| **语法复杂度** | 简单 | 简单 | 中等 | 复杂 | 中等 |

### 语法对比

#### **模块定义语法**

```javascript
// ===== ES Modules (ESM) =====
// 命名导出
export const PI = 3.14159;
export function add(a, b) { return a + b; }

// 默认导出
export default class Calculator {
  multiply(a, b) { return a * b; }
}

// ===== CommonJS (CJS) =====
// 导出
const PI = 3.14159;
function add(a, b) { return a + b; }

module.exports = { PI, add };
// 或者
exports.PI = PI;
exports.add = add;

// 默认导出
module.exports = class Calculator {
  multiply(a, b) { return a * b; }
};

// ===== AMD =====
define(['dependency1', 'dependency2'], function(dep1, dep2) {
  const PI = 3.14159;
  
  function add(a, b) { return a + b; }
  
  class Calculator {
    multiply(a, b) { return a * b; }
  }
  
  return {
    PI,
    add,
    Calculator
  };
});

// ===== UMD =====
(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    // AMD
    define(['dependency'], factory);
  } else if (typeof module === 'object' && module.exports) {
    // CommonJS
    module.exports = factory(require('dependency'));
  } else {
    // Browser globals
    root.MyModule = factory(root.Dependency);
  }
}(typeof self !== 'undefined' ? self : this, function (dependency) {
  const PI = 3.14159;
  
  function add(a, b) { return a + b; }
  
  return { PI, add };
}));

// ===== RequireJS =====
// 定义模块
define('math-utils', ['jquery'], function($) {
  const PI = 3.14159;
  
  function add(a, b) { return a + b; }
  
  return {
    PI: PI,
    add: add
  };
});
```

#### **模块导入语法**

```javascript
// ===== ES Modules (ESM) =====
import Calculator, { PI, add } from './math.js';
import * as MathUtils from './math.js';

// 动态导入
const module = await import('./math.js');

// ===== CommonJS (CJS) =====
const { PI, add } = require('./math');
const Calculator = require('./math');
const MathUtils = require('./math');

// ===== AMD =====
require(['./math'], function(MathUtils) {
  console.log(MathUtils.PI);
});

// ===== RequireJS =====
requirejs.config({
  paths: {
    'math': './math'
  }
});

requirejs(['math'], function(MathUtils) {
  console.log(MathUtils.PI);
});
```

### 技术特性对比

| 特性 | ESM | CommonJS | AMD | UMD | RequireJS |
|------|-----|----------|-----|-----|-----------|
| **Tree Shaking** | ✅ 原生支持 | ❌ 需工具支持 | ❌ 有限支持 | ❌ 不支持 | ❌ 不支持 |
| **静态分析** | ✅ 编译时确定 | ❌ 运行时确定 | ❌ 运行时确定 | ❌ 运行时确定 | ❌ 运行时确定 |
| **循环依赖** | ✅ 原生处理 | ⚠️ 部分支持 | ⚠️ 部分支持 | ⚠️ 部分支持 | ⚠️ 部分支持 |
| **代码分割** | ✅ 动态导入 | ❌ 不支持 | ✅ 异步加载 | ❌ 取决于环境 | ✅ 异步加载 |
| **浏览器支持** | ✅ 现代浏览器 | ❌ 需要打包 | ✅ 直接支持 | ✅ 通用支持 | ✅ 直接支持 |
| **Node.js支持** | ✅ v12+ | ✅ 原生支持 | ❌ 需要库 | ✅ 通过包装 | ❌ 需要库 |
| **打包工具支持** | ✅ 广泛支持 | ✅ 广泛支持 | ⚠️ 部分支持 | ✅ 广泛支持 | ⚠️ 特定工具 |

### 性能对比

| 指标 | ESM | CommonJS | AMD | UMD | RequireJS |
|------|-----|----------|-----|-----|-----------|
| **加载性能** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **解析速度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **运行时开销** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **打包体积** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **启动时间** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |

### 适用场景

| 模块系统 | 最佳适用场景 | 不适用场景 |
|---------|-------------|------------|
| **ES Modules** | • 现代Web应用<br>• Node.js新项目<br>• 需要Tree Shaking<br>• 库开发 | • 旧浏览器支持<br>• 旧版Node.js项目 |
| **CommonJS** | • Node.js服务端<br>• 现有Node.js项目<br>• npm包开发 | • 浏览器直接使用<br>• 需要异步加载 |
| **AMD** | • 浏览器应用<br>• 需要异步加载<br>• 复杂依赖关系 | • Node.js环境<br>• 简单项目 |
| **UMD** | • 跨平台库<br>• 向后兼容<br>• 第三方库发布 | • 现代应用开发<br>• 性能敏感应用 |
| **RequireJS** | • 旧版浏览器应用<br>• 渐进式升级<br>• 教学演示 | • 新项目开发<br>• 现代工具链 |

### 生态系统支持

| 方面 | ESM | CommonJS | AMD | UMD | RequireJS |
|------|-----|----------|-----|-----|-----------|
| **框架支持** | React, Vue, Angular等 | Express, Koa等 | 部分旧框架 | 通用支持 | 旧版库 |
| **工具链** | Webpack, Vite, Rollup | Webpack, Browserify | r.js, Webpack | 所有打包工具 | r.js优化器 |
| **TypeScript** | ✅ 完全支持 | ✅ 完全支持 | ⚠️ 配置复杂 | ⚠️ 类型推导难 | ⚠️ 配置复杂 |
| **测试工具** | Jest, Vitest等 | Jest, Mocha等 | 特定配置 | 需要适配 | 特定配置 |
| **CDN支持** | Skypack, esm.sh | 需要打包 | jsDelivr等 | 广泛支持 | jsDelivr等 |

### 迁移路径

```javascript
// 从 CommonJS 迁移到 ESM
// Before (CommonJS)
const express = require('express');
const { createUser } = require('./user-service');

module.exports = {
  startServer: () => {
    const app = express();
    // ...
  }
};

// After (ESM)
import express from 'express';
import { createUser } from './user-service.js';

export function startServer() {
  const app = express();
  // ...
}

// 从 AMD 迁移到 ESM
// Before (AMD)
define(['lodash', './utils'], function(_, utils) {
  return {
    processData: function(data) {
      return _.map(data, utils.transform);
    }
  };
});

// After (ESM)
import _ from 'lodash';
import { transform } from './utils.js';

export function processData(data) {
  return _.map(data, transform);
}
```

### 未来趋势

| 趋势 | 说明 | 影响 |
|------|------|------|
| **ESM 标准化** | 成为唯一标准 | 其他系统逐步淘汰 |
| **原生支持增强** | 浏览器和Node.js完全支持 | 减少工具依赖 |
| **Import Maps** | 标准化模块映射 | 简化依赖管理 |
| **动态导入普及** | 代码分割标准化 | 提升应用性能 |
| **工具链整合** | 围绕ESM构建 | 开发体验提升 |

## 总结

模块化的核心概念包括：

- ✅ **模块定义**: 如何创建和组织模块
- ✅ **导入导出**: 模块间的接口交互
- ✅ **作用域隔离**: 保证模块独立性
- ✅ **依赖管理**: 构建清晰的依赖关系
- ✅ **设计原则**: 遵循良好的模块化实践

### 选择建议

- **新项目**: 优先选择 **ES Modules**
- **Node.js服务**: **CommonJS** 仍然是主流，但可考虑 **ESM**
- **库开发**: **ESM** + **UMD** 兼容发布
- **旧项目维护**: 渐进式迁移到 **ESM**
- **浏览器兼容**: 使用构建工具将 **ESM** 转译为合适格式

理解这些核心概念和各系统的特点是掌握任何模块系统的基础。在接下来的章节中，我们将深入探讨具体的模块系统实现。

---

**下一章**: [ES模块基础](./esm/basics.md) →