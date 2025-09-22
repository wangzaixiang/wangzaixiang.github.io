# 导入与导出

ES模块的导入和导出是模块系统的核心功能。本章将深入探讨各种导入导出模式、高级用法以及最佳实践。

## 导出的详细语法

### 1. 声明时导出

```javascript
// declarations.js

// 导出变量声明
export const API_VERSION = 'v1';
export let currentUser = null;
export var debugMode = false;

// 导出函数声明
export function getUserById(id) {
    return fetch(`/api/users/${id}`);
}

// 导出异步函数
export async function fetchUserData(id) {
    const response = await getUserById(id);
    return response.json();
}

// 导出生成器函数
export function* numberGenerator() {
    let i = 0;
    while (true) {
        yield i++;
    }
}

// 导出类声明
export class UserManager {
    constructor() {
        this.users = new Map();
    }
    
    addUser(user) {
        this.users.set(user.id, user);
    }
    
    getUser(id) {
        return this.users.get(id);
    }
}
```

### 2. 先声明后导出

```javascript
// deferred-exports.js

// 先声明
const CONFIG = {
    apiUrl: 'https://api.example.com',
    timeout: 5000,
    retryAttempts: 3
};

function validateConfig(config) {
    return config.apiUrl && config.timeout > 0;
}

class Logger {
    constructor(level = 'info') {
        this.level = level;
    }
    
    log(message) {
        console.log(`[${this.level.toUpperCase()}] ${message}`);
    }
}

// 后导出
export { CONFIG, validateConfig, Logger };

// 导出时重命名
export { CONFIG as defaultConfig };
export { Logger as EventLogger };
```

### 3. 默认导出的多种形式

```javascript
// default-exports.js

// 方式1：导出类
export default class Calculator {
    add(a, b) { return a + b; }
    subtract(a, b) { return a - b; }
}

// 方式2：导出函数
// export default function calculate(operation, a, b) {
//     switch (operation) {
//         case 'add': return a + b;
//         case 'subtract': return a - b;
//         default: throw new Error('Unknown operation');
//     }
// }

// 方式3：导出箭头函数
// export default (x, y) => x + y;

// 方式4：导出对象
// export default {
//     version: '1.0.0',
//     author: 'John Doe',
//     calculate: (a, b) => a + b
// };

// 方式5：导出表达式
// const multiplier = 2;
// export default multiplier * 10;

// 方式6：先声明后导出
// class AdvancedCalculator {
//     // ... implementation
// }
// export default AdvancedCalculator;
```

### 4. 聚合导出（Re-exports）

```javascript
// utils/index.js - 模块聚合文件

// 重新导出所有命名导出
export * from './string-utils.js';
export * from './number-utils.js';
export * from './date-utils.js';

// 重新导出特定的命名导出
export { debounce, throttle } from './function-utils.js';
export { formatCurrency } from './number-utils.js';

// 重新导出并重命名
export { 
    parseDate as parseDateString,
    formatDate as formatDateString 
} from './date-utils.js';

// 重新导出默认导出
export { default as StringValidator } from './string-validator.js';
export { default as NumberValidator } from './number-validator.js';

// 混合导出：本地定义 + 重新导出
export const UTILS_VERSION = '2.1.0';

export function createUtilsBundle() {
    return {
        strings: './string-utils.js',
        numbers: './number-utils.js',
        dates: './date-utils.js'
    };
}
```

## 导入的详细语法

### 1. 命名导入的各种形式

```javascript
// named-imports.js

// 基本命名导入
import { API_VERSION, currentUser } from './declarations.js';

// 导入时重命名
import { 
    getUserById as fetchUser,
    UserManager as UserService 
} from './declarations.js';

// 批量导入多个命名导出
import { 
    API_VERSION,
    currentUser,
    getUserById,
    UserManager 
} from './declarations.js';

// 导入所有命名导出到命名空间对象
import * as UserAPI from './declarations.js';
console.log(UserAPI.API_VERSION);
const manager = new UserAPI.UserManager();

// 混合导入：默认导出 + 命名导出
import Calculator, { 
    API_VERSION, 
    getUserById 
} from './mixed-exports.js';
```

### 2. 默认导入

```javascript
// default-imports.js

// 导入默认导出（可以使用任意名称）
import Calculator from './default-exports.js';
import MyCalculator from './default-exports.js'; // 同样有效
import Calc from './default-exports.js'; // 也可以

// 使用导入的默认导出
const calc = new Calculator();
console.log(calc.add(2, 3));

// 如果默认导出是函数
import calculate from './function-export.js';
const result = calculate('add', 5, 3);

// 如果默认导出是对象
import config from './config-export.js';
console.log(config.version);
```

### 3. 副作用导入

```javascript
// side-effect-imports.js

// 仅执行模块，不导入任何绑定
import './polyfills.js';          // 执行polyfill代码
import './global-styles.css';     // 加载CSS（通过构建工具处理）
import './init-app.js';           // 执行初始化代码

// 这些导入会执行目标模块的代码，但不创建任何绑定
console.log('All side effects have been applied');
```

### 4. 条件导入

```javascript
// conditional-imports.js

// 在条件块中的静态导入（会被提升）
if (process.env.NODE_ENV === 'development') {
    // 这个导入会被提升到模块顶部执行
    import './dev-tools.js';
}

// 正确的条件导入方式：使用动态导入
async function loadDevTools() {
    if (process.env.NODE_ENV === 'development') {
        const devTools = await import('./dev-tools.js');
        devTools.setup();
    }
}

// 或者使用顶层await（现代环境）
if (process.env.NODE_ENV === 'development') {
    const { setup } = await import('./dev-tools.js');
    setup();
}
```

## 模块导入解析规则

### 1. 相对路径导入

```javascript
// 相对路径导入示例
// 文件结构：
// src/
//   ├── components/
//   │   ├── Button.js
//   │   └── Input.js
//   ├── utils/
//   │   └── helpers.js
//   └── app.js

// 在 src/app.js 中
import { Button } from './components/Button.js';    // 相对路径
import { helpers } from './utils/helpers.js';       // 相对路径

// 在 src/components/Button.js 中
import { helpers } from '../utils/helpers.js';      // 上级目录
import { Input } from './Input.js';                 // 同级文件
```

### 2. 绝对路径和包导入

```javascript
// absolute-imports.js

// Node.js 内置模块
import { readFile } from 'fs/promises';
import { join } from 'path';

// npm 包导入
import lodash from 'lodash';
import { debounce } from 'lodash';
import React from 'react';
import { useState } from 'react';

// 作用域包
import { parse } from '@babel/parser';
import { transform } from '@babel/core';

// 子路径导入
import { format } from 'date-fns/format';
import { isValid } from 'date-fns/isValid';
```

### 3. 带扩展名和不带扩展名

```javascript
// extensions.js

// 明确指定扩展名（推荐）
import { utils } from './utils.js';
import config from './config.json';    // JSON文件
import styles from './styles.css';     // CSS文件（需构建工具支持）

// 不指定扩展名（依赖解析器配置）
// import { utils } from './utils';     // 可能解析为utils.js或utils/index.js
// import config from './config';       // 可能解析为config.js或config.json
```

## 导入导出的高级模式

### 1. 条件导出（package.json）

```json
{
  "name": "my-package",
  "exports": {
    ".": {
      "import": "./dist/esm/index.js",
      "require": "./dist/cjs/index.js",
      "types": "./dist/types/index.d.ts"
    },
    "./utils": {
      "import": "./dist/esm/utils.js",
      "require": "./dist/cjs/utils.js"
    },
    "./package.json": "./package.json"
  }
}
```

### 2. 桶文件模式（Barrel Exports）

```javascript
// components/index.js - 桶文件
export { default as Button } from './Button.js';
export { default as Input } from './Input.js';
export { default as Modal } from './Modal.js';
export { default as Form } from './Form.js';

// 也可以重新导出类型（TypeScript）
export type { ButtonProps } from './Button.js';
export type { InputProps } from './Input.js';

// 使用桶文件
// 在其他文件中可以从单一入口导入多个组件
import { Button, Input, Modal } from './components';
```

### 3. 命名空间模式

```javascript
// math/index.js - 命名空间模式
import * as BasicMath from './basic.js';
import * as AdvancedMath from './advanced.js';
import * as Statistics from './statistics.js';

export const Math = {
    Basic: BasicMath,
    Advanced: AdvancedMath,
    Statistics
};

// 也导出具体函数以便直接使用
export { add, subtract, multiply, divide } from './basic.js';
export { sin, cos, tan, log } from './advanced.js';
export { mean, median, mode } from './statistics.js';

// 使用方式
// import { Math } from './math';
// Math.Basic.add(1, 2);
// 
// 或者
// import { add, sin, mean } from './math';
// add(1, 2);
```

### 4. 插件系统模式

```javascript
// plugin-system.js

// 插件注册表
const plugins = new Map();

export function registerPlugin(name, plugin) {
    plugins.set(name, plugin);
}

export function getPlugin(name) {
    return plugins.get(name);
}

export function loadPlugin(name) {
    return import(`./plugins/${name}.js`).then(module => {
        registerPlugin(name, module.default);
        return module.default;
    });
}

// 插件接口
export class PluginBase {
    constructor(options = {}) {
        this.options = options;
    }
    
    init() {
        throw new Error('Plugin must implement init method');
    }
    
    destroy() {
        // 默认清理逻辑
    }
}

// 使用示例
// const authPlugin = await loadPlugin('auth');
// authPlugin.init({ apiKey: 'xxx' });
```

## 导入导出的最佳实践

### 1. 导出优先级

```javascript
// good-exports.js

// 1. 优先使用命名导出
export function createUser(data) { /* ... */ }
export function updateUser(id, data) { /* ... */ }
export function deleteUser(id) { /* ... */ }

// 2. 主要功能使用默认导出
export default class UserService {
    constructor() {
        this.cache = new Map();
    }
    
    async getUser(id) {
        if (this.cache.has(id)) {
            return this.cache.get(id);
        }
        // ... fetch logic
    }
}

// 3. 常量和配置使用命名导出
export const DEFAULT_TIMEOUT = 5000;
export const USER_ROLES = ['admin', 'user', 'guest'];
```

### 2. 导入组织

```javascript
// organized-imports.js

// 1. Node.js 内置模块
import { readFile } from 'fs/promises';
import { join, dirname } from 'path';

// 2. 第三方依赖
import React, { useState, useEffect } from 'react';
import { format } from 'date-fns';
import axios from 'axios';

// 3. 内部模块（按层级分组）
import { config } from '../config/app.js';
import { UserService } from '../services/user.js';
import { validateInput } from '../utils/validation.js';

// 4. 相对导入
import { Button } from './Button.js';
import { Modal } from './Modal.js';
import styles from './UserForm.module.css';
```

### 3. 避免循环依赖

```javascript
// 反模式：循环依赖
// user.js
import { Order } from './order.js';
export class User {
    getOrders() {
        return Order.findByUserId(this.id);
    }
}

// order.js
import { User } from './user.js';  // 循环依赖！
export class Order {
    getUser() {
        return User.findById(this.userId);
    }
}

// 解决方案1：依赖注入
// user.js
export class User {
    getOrders(orderService) {
        return orderService.findByUserId(this.id);
    }
}

// order.js
export class Order {
    getUser(userService) {
        return userService.findById(this.userId);
    }
}

// 解决方案2：提取到服务层
// user-order-service.js
import { User } from './user.js';
import { Order } from './order.js';

export class UserOrderService {
    getUserOrders(userId) {
        return Order.findByUserId(userId);
    }
    
    getOrderUser(orderId) {
        const order = Order.findById(orderId);
        return User.findById(order.userId);
    }
}
```

### 4. 模块重新导出策略

```javascript
// api/index.js - API 模块聚合

// 按功能模块重新导出
export * as Users from './users.js';
export * as Orders from './orders.js';
export * as Products from './products.js';

// 导出常用的API函数
export { 
    createUser, 
    getUserById 
} from './users.js';

export { 
    createOrder, 
    getOrdersByUser 
} from './orders.js';

// 导出配置和常量
export { API_BASE_URL, DEFAULT_HEADERS } from './config.js';

// 导出默认客户端
export { default as ApiClient } from './client.js';

// 使用示例
// 方式1：使用命名空间
// import { Users, Orders } from './api';
// const user = await Users.getUserById(1);
// const orders = await Orders.getOrdersByUser(1);

// 方式2：直接导入常用函数
// import { createUser, getOrdersByUser } from './api';
```

## 常见陷阱和注意事项

### 1. 导入提升

```javascript
// import-hoisting.js

console.log('This runs first');

// 这个导入会被提升到模块顶部执行
import { someFunction } from './utils.js';

console.log('This runs second');

// 相当于：
// import { someFunction } from './utils.js';  // <- 实际执行位置
// console.log('This runs first');
// console.log('This runs second');
```

### 2. 默认导出的陷阱

```javascript
// default-export-pitfalls.js

// 陷阱1：默认导出的重命名容易出错
// math.js
export default function add(a, b) { return a + b; }

// main.js
import subtract from './math.js';  // 错误：以为导入的是subtract函数
console.log(subtract(5, 3));       // 实际上是add函数，结果是8而不是2

// 解决方案：使用命名导出
// math.js
export function add(a, b) { return a + b; }
export function subtract(a, b) { return a - b; }

// main.js
import { add, subtract } from './math.js';  // 明确的函数名
```

### 3. 导入绑定的活性

```javascript
// live-bindings.js

// counter.js
export let count = 0;

export function increment() {
    count++;
}

export function getCount() {
    return count;
}

// main.js
import { count, increment } from './counter.js';

console.log(count);    // 0
increment();
console.log(count);    // 1 (绑定是活的，会更新)

// 但是不能直接修改导入的绑定
// count = 10;         // TypeError: Assignment to constant variable
```

## 总结

ES模块的导入导出系统提供了强大而灵活的代码组织方式：

- ✅ **多样的导出方式**: 命名导出、默认导出、重新导出
- ✅ **灵活的导入语法**: 支持重命名、命名空间、条件导入
- ✅ **静态结构**: 便于工具分析和优化
- ✅ **活绑定**: 导入的绑定能反映导出模块的状态变化
- ✅ **良好的重用性**: 支持模块聚合和重新导出模式

掌握这些导入导出的技巧和最佳实践，能够帮助你构建更清晰、更可维护的模块化代码。在下一章中，我们将探讨ES模块的动态导入功能。

---

**下一章**: [动态导入](./dynamic-import.md) →