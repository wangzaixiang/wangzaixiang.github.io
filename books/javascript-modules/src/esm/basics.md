# ES模块基础

ES模块（ECMAScript Modules，简称ESM）是JavaScript的官方模块系统，在ES6（ES2015）中正式引入。它提供了一种标准化的方式来组织和重用JavaScript代码。

## 什么是ES模块

ES模块是JavaScript语言层面的模块系统，具有以下特点：

- **静态结构**: 模块的导入导出关系在编译时确定
- **严格模式**: 模块代码自动运行在严格模式下
- **顶层作用域**: 每个模块都有自己的顶层作用域
- **异步加载**: 支持异步模块加载
- **树摇友好**: 支持静态分析和死代码消除

## 基本语法

### 导出（Export）

ES模块提供了多种导出方式：

#### 1. 命名导出（Named Exports）

```javascript
// math.js

// 导出变量
export const PI = 3.14159;
export let counter = 0;

// 导出函数
export function add(a, b) {
    return a + b;
}

export function multiply(a, b) {
    return a * b;
}

// 导出类
export class Calculator {
    add(a, b) {
        return a + b;
    }
}

// 批量导出
const subtract = (a, b) => a - b;
const divide = (a, b) => a / b;

export { subtract, divide };
```

#### 2. 默认导出（Default Export）

```javascript
// user.js

// 默认导出类
export default class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
    }
    
    getInfo() {
        return `${this.name} <${this.email}>`;
    }
}

// 或者导出函数
// export default function createUser(name, email) {
//     return new User(name, email);
// }

// 或者导出值
// export default {
//     apiUrl: 'https://api.example.com',
//     timeout: 5000
// };
```

#### 3. 混合导出

```javascript
// api.js

// 默认导出
class ApiClient {
    constructor(baseUrl) {
        this.baseUrl = baseUrl;
    }
    
    async get(endpoint) {
        const response = await fetch(`${this.baseUrl}${endpoint}`);
        return response.json();
    }
}

export default ApiClient;

// 同时提供命名导出
export const DEFAULT_TIMEOUT = 5000;
export const HTTP_METHODS = ['GET', 'POST', 'PUT', 'DELETE'];

export function createApiClient(baseUrl) {
    return new ApiClient(baseUrl);
}
```

### 导入（Import）

#### 1. 导入命名导出

```javascript
// main.js

// 导入特定的命名导出
import { add, multiply, PI } from './math.js';

console.log(add(2, 3)); // 5
console.log(multiply(4, 5)); // 20
console.log(PI); // 3.14159

// 导入时重命名
import { add as sum, multiply as product } from './math.js';

console.log(sum(2, 3)); // 5
console.log(product(4, 5)); // 20

// 导入所有命名导出
import * as MathUtils from './math.js';

console.log(MathUtils.add(2, 3)); // 5
console.log(MathUtils.PI); // 3.14159
```

#### 2. 导入默认导出

```javascript
// app.js

// 导入默认导出
import User from './user.js';

const user = new User('John', 'john@example.com');
console.log(user.getInfo());

// 默认导出可以用任意名称
import MyUser from './user.js'; // 同样有效
```

#### 3. 混合导入

```javascript
// client.js

// 同时导入默认导出和命名导出
import ApiClient, { DEFAULT_TIMEOUT, createApiClient } from './api.js';

const client = new ApiClient('https://api.example.com');
const anotherClient = createApiClient('https://another-api.com');

console.log(`Default timeout: ${DEFAULT_TIMEOUT}ms`);
```

#### 4. 仅导入模块（无绑定）

```javascript
// 仅执行模块代码，不导入任何绑定
import './polyfills.js';
import './init-global-config.js';
```

## 模块的执行特性

### 1. 严格模式

ES模块代码自动运行在严格模式下，这带来了更严格的语法检查和更安全的执行环境：

**严格模式的主要特征：**
- 禁止使用未声明的变量
- 禁止删除不可删除的属性
- 函数参数名必须唯一
- 禁止八进制字面量
- `this`在函数中不会自动指向全局对象

```javascript
// module.js
// 模块代码自动运行在严格模式下

// 以下代码在模块中会报错
// undeclaredVariable = 'value'; // ReferenceError
// delete Object.prototype; // TypeError

console.log(this); // undefined（非浏览器环境）
```

### 2. 顶层作用域

```javascript
// module1.js
var globalVar = 'module1';
let moduleVar = 'private to module1';

export { moduleVar };

// module2.js
var globalVar = 'module2'; // 不会与module1冲突
let moduleVar = 'private to module2';

export { moduleVar };

// main.js
import { moduleVar as var1 } from './module1.js';
import { moduleVar as var2 } from './module2.js';

console.log(var1); // 'private to module1'
console.log(var2); // 'private to module2'
```

### 3. 模块单例

```javascript
// counter.js
let count = 0;

export function increment() {
    return ++count;
}

export function getCount() {
    return count;
}

// module1.js
import { increment } from './counter.js';
console.log(increment()); // 1

// module2.js
import { increment, getCount } from './counter.js';
console.log(increment()); // 2
console.log(getCount()); // 2
```

## 在HTML中使用ES模块

### 1. 基本用法

```html
<!DOCTYPE html>
<html>
<head>
    <title>ES Modules Demo</title>
</head>
<body>
    <!-- 使用type="module"标识ES模块 -->
    <script type="module" src="./main.js"></script>
    
    <!-- 内联模块脚本 -->
    <script type="module">
        import { greet } from './utils.js';
        greet('World');
    </script>
</body>
</html>
```

### 2. 模块脚本的特性

```html
<!-- ES模块脚本的特点 -->
<script type="module">
    // 1. 自动延迟执行（相当于defer）
    console.log('Module script executed');
    
    // 2. 严格模式
    console.log(this); // undefined
    
    // 3. 支持顶层await（现代浏览器）
    const data = await fetch('/api/data').then(r => r.json());
    console.log(data);
</script>

<!-- 传统脚本（用于对比） -->
<script>
    console.log('Regular script executed first');
</script>
```

### 3. 浏览器兼容性处理

```html
<!-- 现代浏览器使用ES模块 -->
<script type="module" src="./modern-app.js"></script>

<!-- 旧浏览器降级方案 -->
<script nomodule src="./legacy-app.js"></script>
```

## Node.js中的ES模块

### 1. 启用ES模块

方式1：使用`.mjs`扩展名
```javascript
// app.mjs
import { readFile } from 'fs/promises';

const content = await readFile('./package.json', 'utf-8');
console.log(content);
```

方式2：在`package.json`中设置`"type": "module"`
```json
{
  "type": "module",
  "main": "app.js"
}
```

```javascript
// app.js (现在作为ES模块运行)
import { readFile } from 'fs/promises';

const content = await readFile('./package.json', 'utf-8');
console.log(content);
```

### 2. 内置模块的导入

```javascript
// Node.js内置模块
import { readFile, writeFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

// 获取当前文件路径（在ES模块中__dirname不可用）
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const filePath = join(__dirname, 'data.txt');
const data = await readFile(filePath, 'utf-8');
```

## ES模块的优势

### 1. 静态分析

```javascript
// 静态结构使工具能够分析依赖关系
import { usedFunction } from './utils.js';
// import { unusedFunction } from './utils.js'; // 可以被工具检测并移除

// 动态导入无法进行静态分析
// const moduleName = getModuleName();
// import(moduleName); // 运行时确定
```

### 2. 树摇（Tree Shaking）

```javascript
// utils.js
export function add(a, b) { return a + b; }
export function subtract(a, b) { return a - b; }
export function multiply(a, b) { return a * b; }
export function divide(a, b) { return a / b; }

// main.js
import { add, multiply } from './utils.js';
// 打包工具可以自动移除unused的subtract和divide函数

console.log(add(2, 3));
console.log(multiply(4, 5));
```

### 3. 循环依赖处理

```javascript
// a.js
import { b } from './b.js';
export const a = 'a';
console.log('a.js:', b);

// b.js
import { a } from './a.js';
export const b = 'b';
console.log('b.js:', a); // undefined（但不会报错）

// ES模块能够优雅处理循环依赖
```

**ES模块循环依赖处理机制：**

1. **模块记录创建**: 在解析阶段，所有模块都会创建模块记录，但不立即执行
2. **深度优先遍历**: 按照依赖图进行深度优先遍历，确定执行顺序
3. **实时绑定**: 通过live binding机制，即使在循环依赖中也能获取到最终的导出值
4. **延迟访问**: 在模块完全初始化之前访问导出可能得到undefined，但不会抛出错误

这种机制使得ES模块能够安全地处理循环依赖，详细内容请参阅[循环依赖章节](./circular-deps.md)。

## 常见模式和最佳实践

### 1. 重新导出（Re-exports）

```javascript
// components/index.js - 统一导出文件
export { Button } from './Button.js';
export { Input } from './Input.js';
export { Modal } from './Modal.js';

// 或者使用通配符重新导出
export * from './Button.js';
export * from './Input.js';

// 重新导出并重命名
export { default as MyButton } from './Button.js';
```

### 2. 条件导出

```javascript
// config.js
const isDevelopment = process.env.NODE_ENV === 'development';

if (isDevelopment) {
    // 动态导入开发工具
    const devTools = await import('./dev-tools.js');
    devTools.setupDevMode();
}

export const config = {
    apiUrl: isDevelopment ? 'http://localhost:3000' : 'https://api.prod.com'
};
```

### 3. 工厂函数模式

```javascript
// database.js
export function createDatabase(config) {
    return {
        connect() {
            console.log(`Connecting to ${config.host}:${config.port}`);
        },
        
        query(sql) {
            console.log(`Executing: ${sql}`);
        }
    };
}

// main.js
import { createDatabase } from './database.js';

const db = createDatabase({
    host: 'localhost',
    port: 5432
});

db.connect();
```

## Live Binding（实时绑定）

Live Binding是ES模块的一个核心特性，它使得导入的绑定与导出的绑定保持**实时同步**。这与CommonJS的值复制完全不同。

### 什么是Live Binding

Live Binding意味着导入的变量是对导出变量的**实时引用**，而不是值的拷贝。当导出模块中的值发生变化时，导入模块中的对应变量也会自动更新。

### Live Binding vs 值复制

#### CommonJS（值复制）

```javascript
// counter.js (CommonJS)
let count = 0;

function increment() {
    count++;
}

function getCount() {
    return count;
}

// 导出的是值的副本
module.exports = { count, increment, getCount };
```

```javascript
// main.js (CommonJS)
const { count, increment, getCount } = require('./counter');

console.log(count); // 0
increment();
console.log(count); // 0 ← 还是0，因为是值复制
console.log(getCount()); // 1 ← 通过函数才能获取最新值
```

#### ES模块（Live Binding）

```javascript
// counter.mjs (ES模块)
export let count = 0;

export function increment() {
    count++;
}

export function decrement() {
    count--;
}

export function reset() {
    count = 0;
}
```

```javascript
// main.mjs (ES模块)
import { count, increment, decrement, reset } from './counter.mjs';

console.log(count); // 0

increment();
console.log(count); // 1 ← 自动更新！

increment();
increment();
console.log(count); // 3 ← 继续同步！

decrement();
console.log(count); // 2 ← 实时反映变化

reset();
console.log(count); // 0 ← 重置后也同步
```

### Live Binding的工作原理

```javascript
// 演示Live Binding的内部机制

// module-a.mjs
export let sharedValue = 'initial';
export let counter = 0;

export function updateValue(newValue) {
    sharedValue = newValue;
    counter++;
    console.log(`模块A: 值更新为 ${sharedValue}, 计数器: ${counter}`);
}

export function getTimestamp() {
    return Date.now();
}
```

```javascript
// module-b.mjs  
import { sharedValue, counter, updateValue } from './module-a.mjs';

export function showValues() {
    console.log(`模块B看到: ${sharedValue}, 计数器: ${counter}`);
}

export function triggerUpdate() {
    updateValue('从模块B更新');
}
```

```javascript
// main.mjs
import { sharedValue, counter, updateValue } from './module-a.mjs';
import { showValues, triggerUpdate } from './module-b.mjs';

console.log('=== Live Binding 演示 ===');

console.log(`主模块初始值: ${sharedValue}, 计数器: ${counter}`);

// 从主模块更新
updateValue('从主模块更新');
console.log(`主模块更新后: ${sharedValue}, 计数器: ${counter}`);

// 从模块B更新
triggerUpdate();
console.log(`B模块更新后: ${sharedValue}, 计数器: ${counter}`);

// 显示所有模块都看到相同的值
showValues();

// 执行结果：
// 主模块初始值: initial, 计数器: 0
// 模块A: 值更新为 从主模块更新, 计数器: 1
// 主模块更新后: 从主模块更新, 计数器: 1
// 模块A: 值更新为 从模块B更新, 计数器: 2
// B模块更新后: 从模块B更新, 计数器: 2
// 模块B看到: 从模块B更新, 计数器: 2
```

### Live Binding的重要特征

#### 1. 只读性

```javascript
// readonly-demo.mjs
export let value = 'original';

export function setValue(newValue) {
    value = newValue;
}
```

```javascript
// main.mjs
import { value, setValue } from './readonly-demo.mjs';

console.log(value); // 'original'

// 以下操作会报错！
// value = 'modified'; // TypeError: Assignment to constant variable.

// 只能通过导出模块的函数来修改
setValue('modified');
console.log(value); // 'modified' ← Live Binding生效
```

#### 2. 时间敏感性

```javascript
// timing-demo.mjs
export let asyncValue = 'loading...';

// 模拟异步操作
setTimeout(() => {
    asyncValue = 'loaded!';
    console.log('异步操作完成，值已更新');
}, 1000);

export function getCurrentValue() {
    return asyncValue;
}
```

```javascript
// main.mjs
import { asyncValue, getCurrentValue } from './timing-demo.mjs';

console.log('立即读取:', asyncValue); // 'loading...'

setTimeout(() => {
    console.log('1秒后读取:', asyncValue); // 'loaded!' ← Live Binding自动更新
}, 1500);

// 也可以通过函数获取
setTimeout(() => {
    console.log('通过函数:', getCurrentValue()); // 'loaded!'
}, 1500);
```

#### 3. 循环依赖中的Live Binding

```javascript
// circular-a.mjs
import { bValue, setBValue } from './circular-b.mjs';

export let aValue = 'from-a';

export function setAValue(newValue) {
    aValue = newValue;
}

export function showBValue() {
    console.log('A模块看到B的值:', bValue);
}

// 初始化时调用B模块的函数
setBValue('a-modified-b');
```

```javascript
// circular-b.mjs  
import { aValue, setAValue } from './circular-a.mjs';

export let bValue = 'from-b';

export function setBValue(newValue) {
    bValue = newValue;
}

export function showAValue() {
    console.log('B模块看到A的值:', aValue);
}

// 初始化时调用A模块的函数
setAValue('b-modified-a');
```

```javascript
// main.mjs
import { aValue, showBValue } from './circular-a.mjs';
import { bValue, showAValue } from './circular-b.mjs';

console.log('主模块看到:');
console.log('A值:', aValue); // 'b-modified-a'
console.log('B值:', bValue); // 'a-modified-b'

showAValue(); // B模块看到A的值: b-modified-a
showBValue(); // A模块看到B的值: a-modified-b
```

### Live Binding的优势

#### 1. 状态同步

```javascript
// state-manager.mjs
export let appState = {
    user: null,
    theme: 'light',
    language: 'en'
};

export function login(user) {
    appState.user = user;
    console.log('用户已登录:', user.name);
}

export function setTheme(theme) {
    appState.theme = theme;
    console.log('主题已切换:', theme);
}

export function setLanguage(language) {
    appState.language = language;
    console.log('语言已切换:', language);
}
```

```javascript
// ui-components.mjs
import { appState } from './state-manager.mjs';

export function renderHeader() {
    const { user, theme } = appState;
    console.log(`渲染头部: 用户=${user?.name || '未登录'}, 主题=${theme}`);
}

export function renderSidebar() {
    const { language, theme } = appState;
    console.log(`渲染侧边栏: 语言=${language}, 主题=${theme}`);
}
```

```javascript
// main.mjs
import { login, setTheme, setLanguage } from './state-manager.mjs';
import { renderHeader, renderSidebar } from './ui-components.mjs';

// 初始渲染
renderHeader(); // 渲染头部: 用户=未登录, 主题=light
renderSidebar(); // 渲染侧边栏: 语言=en, 主题=light

// 状态变化
login({ name: 'Alice', id: 1 });
renderHeader(); // 渲染头部: 用户=Alice, 主题=light ← 自动更新

setTheme('dark');
renderHeader(); // 渲染头部: 用户=Alice, 主题=dark ← 继续同步
renderSidebar(); // 渲染侧边栏: 语言=en, 主题=dark ← 同时更新
```

#### 2. 热重载支持

```javascript
// hot-reload-demo.mjs
export let moduleVersion = '1.0.0';
export let featureFlags = {
    newUI: false,
    experimentalAPI: true
};

// 模拟热重载更新
if (import.meta.hot) {
    import.meta.hot.accept(() => {
        moduleVersion = '1.0.1';
        featureFlags.newUI = true;
        console.log('模块已热重载更新');
    });
}
```

### Live Binding的注意事项

#### 1. 性能考虑

```javascript
// 避免在热点路径中频繁访问Live Binding
import { heavyComputedValue } from './expensive-module.mjs';

// ❌ 不好的做法
for (let i = 0; i < 1000000; i++) {
    if (heavyComputedValue > threshold) {
        // 每次循环都访问Live Binding
    }
}

// ✅ 更好的做法
const cachedValue = heavyComputedValue;
for (let i = 0; i < 1000000; i++) {
    if (cachedValue > threshold) {
        // 使用缓存的值
    }
}
```

#### 2. 调试技巧

```javascript
// debug-utils.mjs
export let debugMode = false;
export let logLevel = 'info';

export function enableDebug() {
    debugMode = true;
    logLevel = 'debug';
}

export function log(message, level = 'info') {
    if (debugMode && shouldLog(level)) {
        console.log(`[${level.toUpperCase()}] ${message}`);
    }
}

function shouldLog(level) {
    const levels = { debug: 0, info: 1, warn: 2, error: 3 };
    return levels[level] >= levels[logLevel];
}
```

## 总结

ES模块是现代JavaScript开发的基础设施，提供了：

- ✅ **标准化语法**: 官方标准，广泛支持
- ✅ **静态结构**: 编译时优化，支持树摇
- ✅ **严格模式**: 更安全的代码执行环境
- ✅ **作用域隔离**: 避免全局变量污染
- ✅ **异步加载**: 更好的性能和用户体验
- ✅ **工具友好**: 丰富的开发工具生态

掌握ES模块的基础语法和特性是现代JavaScript开发的必备技能。在下一章中，我们将深入探讨导入导出的高级用法。

---

**下一章**: [导入与导出](./import-export.md) →