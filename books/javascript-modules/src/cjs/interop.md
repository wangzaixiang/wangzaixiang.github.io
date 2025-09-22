# 模块互操作性

在现代JavaScript开发中，经常需要在同一个项目中使用不同的模块系统。本章将深入探讨CommonJS与ES模块的互操作性，以及如何在实际项目中处理模块系统的混合使用。

## CommonJS与ES模块的差异

### 1. 语法差异

```javascript
// CommonJS
const fs = require('fs');
const { readFile } = require('fs');
module.exports = { utility };
exports.helper = function() {};

// ES模块
import fs from 'fs';
import { readFile } from 'fs';
export { utility };
export const helper = function() {};
export default utility;
```

### 2. 执行时机差异

```javascript
// CommonJS - 同步加载
console.log('开始');
const utils = require('./utils'); // 同步执行
console.log('结束');

// ES模块 - 静态分析
console.log('开始');
import utils from './utils.js'; // 编译时确定
console.log('结束');
```

### 3. 值复制 vs 实时绑定

```javascript
// CommonJS - 值复制
// counter.js (CommonJS)
let count = 0;
function increment() {
    count++;
}
function getCount() {
    return count;
}
module.exports = { count, increment, getCount };

// main.js
const { count, increment, getCount } = require('./counter');
console.log(count); // 0
increment();
console.log(count); // 0 (值复制，不会更新)
console.log(getCount()); // 1 (通过函数获取最新值)
```

```javascript
// ES模块 - 实时绑定
// counter.mjs (ES模块)
let count = 0;
export function increment() {
    count++;
}
export { count };

// main.mjs
import { count, increment } from './counter.mjs';
console.log(count); // 0
increment();
console.log(count); // 1 (实时绑定，会更新)
```

## Node.js中的互操作性

### 1. ES模块导入CommonJS

```javascript
// utils.js (CommonJS)
function add(a, b) {
    return a + b;
}

function multiply(a, b) {
    return a * b;
}

module.exports = {
    add,
    multiply,
    default: { add, multiply } // 可选：显式默认导出
};
```

```javascript
// main.mjs (ES模块)

// 方式1：默认导入
import utils from './utils.js';
console.log(utils.add(2, 3)); // 5

// 方式2：命名导入（如果CommonJS模块支持）
import { add, multiply } from './utils.js';
console.log(add(2, 3)); // 5

// 方式3：命名空间导入
import * as utils from './utils.js';
console.log(utils.add(2, 3)); // 5
```

### 2. CommonJS导入ES模块

CommonJS不能直接使用`import`，需要使用动态导入：

```javascript
// math.mjs (ES模块)
export function add(a, b) {
    return a + b;
}

export function subtract(a, b) {
    return a - b;
}

export default {
    version: '1.0.0'
};
```

```javascript
// app.js (CommonJS)

// 方式1：使用动态import
async function main() {
    const math = await import('./math.mjs');
    console.log(math.add(2, 3)); // 5
    console.log(math.default.version); // '1.0.0'
    
    // 解构导入
    const { add, subtract } = await import('./math.mjs');
    console.log(add(5, 3)); // 8
}

main();

// 方式2：使用import()表达式
import('./math.mjs').then(math => {
    console.log(math.add(2, 3));
});

// 方式3：在函数中使用
function loadMath() {
    return import('./math.mjs');
}
```

### 3. 混合使用示例

```javascript
// config.js (CommonJS)
const defaults = {
    port: 3000,
    host: 'localhost'
};

function createConfig(overrides = {}) {
    return { ...defaults, ...overrides };
}

module.exports = { defaults, createConfig };
```

```javascript
// server.mjs (ES模块)
import express from 'express';
import configModule from './config.js';

const { createConfig } = configModule;

const config = createConfig({
    port: process.env.PORT || 8080
});

const app = express();

app.listen(config.port, () => {
    console.log(`Server running on port ${config.port}`);
});
```

## 打包工具中的互操作性

### 1. Webpack配置

```javascript
// webpack.config.js
module.exports = {
    entry: './src/index.js',
    module: {
        rules: [
            {
                test: /\.m?js$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: [
                            ['@babel/preset-env', {
                                modules: false // 保持ES模块用于tree shaking
                            }]
                        ]
                    }
                }
            }
        ]
    },
    resolve: {
        extensions: ['.js', '.mjs', '.cjs']
    }
};
```

### 2. Rollup配置

```javascript
// rollup.config.js
import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

export default {
    input: 'src/index.js',
    output: {
        file: 'dist/bundle.js',
        format: 'esm'
    },
    plugins: [
        nodeResolve({
            preferBuiltins: false
        }),
        commonjs({
            // 将CommonJS模块转换为ES模块
            transformMixedEsModules: true
        })
    ]
};
```

### 3. Vite配置

```javascript
// vite.config.js
import { defineConfig } from 'vite';

export default defineConfig({
    build: {
        rollupOptions: {
            output: {
                format: 'esm'
            }
        }
    },
    optimizeDeps: {
        include: ['legacy-commonjs-package']
    }
});
```

## 实际互操作模式

### 1. 渐进式迁移

```javascript
// 步骤1：创建兼容层
// compat.js
const legacyUtils = require('./legacy-utils');

// 包装CommonJS模块为ES模块友好的格式
export const { helper1, helper2 } = legacyUtils;
export default legacyUtils;
```

```javascript
// 步骤2：新代码使用ES模块
// new-feature.mjs
import { helper1 } from './compat.js';
import modernUtil from './modern-util.mjs';

export function newFeature() {
    return helper1() + modernUtil();
}
```

### 2. 双模式包

创建同时支持CommonJS和ES模块的包：

```json
// package.json
{
  "name": "my-dual-package",
  "version": "1.0.0",
  "type": "module",
  "main": "./dist/index.cjs",
  "module": "./dist/index.mjs",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.cjs"
    }
  }
}
```

```javascript
// src/index.js (源码)
export function utility() {
    return 'Hello from utility';
}

export default {
    utility
};
```

```javascript
// build/build-cjs.js (构建CommonJS版本)
const fs = require('fs');

const esmCode = `
export function utility() {
    return 'Hello from utility';
}

export default {
    utility
};
`;

const cjsCode = `
function utility() {
    return 'Hello from utility';
}

module.exports = {
    utility,
    default: { utility }
};
`;

fs.writeFileSync('./dist/index.cjs', cjsCode);
fs.writeFileSync('./dist/index.mjs', esmCode);
```

### 3. 条件导入

```javascript
// dynamic-loader.js
async function loadModule(modulePath) {
    try {
        // 尝试ES模块导入
        return await import(modulePath);
    } catch (error) {
        // 降级到CommonJS
        if (error.code === 'ERR_REQUIRE_ESM') {
            return require(modulePath);
        }
        throw error;
    }
}

// 使用示例
async function main() {
    const module = await loadModule('./some-module');
    console.log(module.default || module);
}
```

## 常见问题和解决方案

### 1. __dirname和__filename在ES模块中的替代

```javascript
// utils.mjs
import { fileURLToPath } from 'url';
import { dirname } from 'path';

// ES模块中获取当前文件路径
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export { __filename, __dirname };
```

```javascript
// 兼容函数
// path-utils.mjs
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

export function getCurrentDir(importMetaUrl) {
    return dirname(fileURLToPath(importMetaUrl));
}

export function resolvePath(importMetaUrl, ...paths) {
    const currentDir = getCurrentDir(importMetaUrl);
    return join(currentDir, ...paths);
}

// 使用
import { resolvePath } from './path-utils.mjs';
const configPath = resolvePath(import.meta.url, 'config.json');
```

### 2. require.resolve在ES模块中的替代

```javascript
// resolve-utils.mjs
import { createRequire } from 'module';

// 创建require函数
const require = createRequire(import.meta.url);

export function resolveModule(modulePath) {
    return require.resolve(modulePath);
}

export function importModule(modulePath) {
    return require(modulePath);
}

// 使用示例
const modulePath = resolveModule('lodash');
console.log(modulePath); // 绝对路径
```

### 3. 动态require在ES模块中的实现

```javascript
// dynamic-require.mjs
import { createRequire } from 'module';

const require = createRequire(import.meta.url);

export function dynamicRequire(modulePath) {
    return require(modulePath);
}

export async function universalImport(modulePath) {
    try {
        // 首先尝试ES模块导入
        return await import(modulePath);
    } catch (esError) {
        try {
            // 降级到CommonJS
            return { default: require(modulePath) };
        } catch (cjsError) {
            throw new Error(`无法加载模块 ${modulePath}: ${esError.message}`);
        }
    }
}
```

### 4. 模块类型检测

```javascript
// module-detector.mjs
import { readFileSync } from 'fs';
import { resolve } from 'path';

export function isESModule(packagePath) {
    try {
        const packageJsonPath = resolve(packagePath, 'package.json');
        const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
        return packageJson.type === 'module';
    } catch {
        return false;
    }
}

export function getModuleType(filePath) {
    if (filePath.endsWith('.mjs')) return 'esm';
    if (filePath.endsWith('.cjs')) return 'commonjs';
    
    // 检查package.json
    const packageDir = findPackageDir(filePath);
    return isESModule(packageDir) ? 'esm' : 'commonjs';
}

function findPackageDir(startPath) {
    let currentPath = resolve(startPath);
    while (currentPath !== resolve(currentPath, '..')) {
        try {
            readFileSync(resolve(currentPath, 'package.json'));
            return currentPath;
        } catch {
            currentPath = resolve(currentPath, '..');
        }
    }
    return null;
}
```

## 最佳实践

### 1. 新项目建议

```javascript
// 推荐的项目结构
project/
├── package.json          // type: "module"
├── src/
│   ├── index.mjs         // ES模块入口
│   ├── utils/
│   │   ├── modern.mjs    // 新代码使用ES模块
│   │   └── legacy.cjs    // 旧代码保持CommonJS
│   └── compat/
│       └── require-wrapper.mjs // CommonJS兼容层
└── build/
    ├── build-dual.js     // 构建双模式包
    └── test-compat.js    // 兼容性测试
```

### 2. 迁移策略

```javascript
// 阶段1：准备阶段
// package.json
{
  "type": "module",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.cjs"
    }
  }
}

// 阶段2：渐进迁移
// 保持CommonJS接口不变，内部逐步迁移
// 阶段3：完全切换
// 移除CommonJS兼容层
```

### 3. 库开发建议

```javascript
// lib/index.js (源码使用ES模块)
export function utility() {
    return 'utility function';
}

export class Helper {
    constructor(options) {
        this.options = options;
    }
    
    process() {
        return 'processed';
    }
}

export default {
    utility,
    Helper
};
```

```javascript
// scripts/build-dual.js
import { rollup } from 'rollup';

async function buildDual() {
    // 构建ES模块版本
    const esmBundle = await rollup({
        input: 'lib/index.js',
        external: ['fs', 'path']
    });
    
    await esmBundle.write({
        file: 'dist/index.mjs',
        format: 'esm'
    });
    
    // 构建CommonJS版本
    const cjsBundle = await rollup({
        input: 'lib/index.js',
        external: ['fs', 'path']
    });
    
    await cjsBundle.write({
        file: 'dist/index.cjs',
        format: 'cjs'
    });
}

buildDual();
```

## 总结

模块互操作性是现代JavaScript开发的重要话题：

- ✅ **渐进迁移**: 可以逐步从CommonJS迁移到ES模块
- ✅ **工具支持**: 现代构建工具提供了良好的互操作支持
- ✅ **双模式包**: 可以创建同时支持两种模块系统的包
- ✅ **动态导入**: 提供了运行时模块加载的灵活性

- ⚠️ **性能考虑**: 频繁的动态导入可能影响性能
- ⚠️ **复杂性**: 混合使用增加了项目的复杂性
- ⚠️ **调试困难**: 互操作问题可能难以调试

理解和掌握模块互操作性对于维护现有项目和开发新项目都至关重要。随着生态系统的发展，ES模块正在成为主流，但CommonJS仍将在相当长的时间内存在。

---

**下一章**: [AMD模块系统](../amd/basics.md) →
