# 构建一个模块化库

本章将通过实际案例，演示如何从零开始构建一个现代化的JavaScript模块库，涵盖项目架构、模块设计、构建配置和发布流程。

## 项目需求分析

我们将构建一个名为`@utils/toolkit`的工具库，提供以下功能：
- 字符串处理工具
- 数组操作工具  
- 日期格式化工具
- 异步工具函数
- 数据验证工具

### 设计目标

1. **模块化设计**：每个功能模块独立，支持按需导入
2. **多格式支持**：同时支持ES模块和CommonJS
3. **TypeScript支持**：提供完整的类型定义
4. **Tree Shaking友好**：支持构建工具的死代码消除
5. **体积优化**：最小化打包体积

## 项目结构设计

```
toolkit/
├── src/                    # 源代码目录
│   ├── string/            # 字符串工具模块
│   │   ├── index.ts
│   │   ├── capitalize.ts
│   │   ├── kebabCase.ts
│   │   └── truncate.ts
│   ├── array/             # 数组工具模块
│   │   ├── index.ts
│   │   ├── chunk.ts
│   │   ├── unique.ts
│   │   └── groupBy.ts
│   ├── date/              # 日期工具模块
│   │   ├── index.ts
│   │   ├── format.ts
│   │   └── relative.ts
│   ├── async/             # 异步工具模块
│   │   ├── index.ts
│   │   ├── delay.ts
│   │   ├── timeout.ts
│   │   └── retry.ts
│   ├── validation/        # 验证工具模块
│   │   ├── index.ts
│   │   ├── email.ts
│   │   ├── url.ts
│   │   └── phone.ts
│   └── index.ts           # 主入口文件
├── dist/                  # 构建输出目录
├── tests/                 # 测试文件
├── examples/              # 使用示例
├── package.json
├── tsconfig.json
├── rollup.config.js
└── README.md
```

## 模块实现

### 1. 字符串工具模块

```typescript
// src/string/capitalize.ts
export function capitalize(str: string): string {
  if (!str) return str;
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
}

// src/string/kebabCase.ts
export function kebabCase(str: string): string {
  return str
    .replace(/([a-z])([A-Z])/g, '$1-$2')
    .replace(/[\s_]+/g, '-')
    .toLowerCase();
}

// src/string/truncate.ts
export interface TruncateOptions {
  length: number;
  suffix?: string;
  separator?: string;
}

export function truncate(str: string, options: TruncateOptions): string {
  const { length, suffix = '...', separator } = options;
  
  if (str.length <= length) return str;
  
  let truncated = str.slice(0, length - suffix.length);
  
  if (separator) {
    const lastIndex = truncated.lastIndexOf(separator);
    if (lastIndex > 0) {
      truncated = truncated.slice(0, lastIndex);
    }
  }
  
  return truncated + suffix;
}

// src/string/index.ts
export { capitalize } from './capitalize';
export { kebabCase } from './kebabCase';
export { truncate, type TruncateOptions } from './truncate';
```

### 2. 数组工具模块

```typescript
// src/array/chunk.ts
export function chunk<T>(array: T[], size: number): T[][] {
  if (size <= 0) throw new Error('Chunk size must be positive');
  
  const chunks: T[][] = [];
  for (let i = 0; i < array.length; i += size) {
    chunks.push(array.slice(i, i + size));
  }
  return chunks;
}

// src/array/unique.ts
export function unique<T>(array: T[]): T[] {
  return Array.from(new Set(array));
}

export function uniqueBy<T, K>(array: T[], keyFn: (item: T) => K): T[] {
  const seen = new Set<K>();
  return array.filter(item => {
    const key = keyFn(item);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

// src/array/groupBy.ts
export function groupBy<T, K extends string | number | symbol>(
  array: T[], 
  keyFn: (item: T) => K
): Record<K, T[]> {
  return array.reduce((groups, item) => {
    const key = keyFn(item);
    if (!groups[key]) groups[key] = [];
    groups[key].push(item);
    return groups;
  }, {} as Record<K, T[]>);
}

// src/array/index.ts
export { chunk } from './chunk';
export { unique, uniqueBy } from './unique';
export { groupBy } from './groupBy';
```

### 3. 异步工具模块

```typescript
// src/async/delay.ts
export function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// src/async/timeout.ts
export class TimeoutError extends Error {
  constructor(message: string = 'Operation timed out') {
    super(message);
    this.name = 'TimeoutError';
  }
}

export function timeout<T>(
  promise: Promise<T>, 
  ms: number,
  errorMessage?: string
): Promise<T> {
  return Promise.race([
    promise,
    new Promise<never>((_, reject) => 
      setTimeout(() => reject(new TimeoutError(errorMessage)), ms)
    )
  ]);
}

// src/async/retry.ts
export interface RetryOptions {
  retries: number;
  delay?: number;
  backoff?: 'linear' | 'exponential';
  factor?: number;
}

export async function retry<T>(
  fn: () => Promise<T>,
  options: RetryOptions
): Promise<T> {
  const { retries, delay: baseDelay = 1000, backoff = 'linear', factor = 2 } = options;
  
  let lastError: Error;
  
  for (let attempt = 0; attempt <= retries; attempt++) {
    try {
      return await fn();
    } catch (error) {
      lastError = error as Error;
      
      if (attempt === retries) break;
      
      const delayMs = backoff === 'exponential' 
        ? baseDelay * Math.pow(factor, attempt)
        : baseDelay * (attempt + 1);
        
      await delay(delayMs);
    }
  }
  
  throw lastError!;
}

// src/async/index.ts
export { delay } from './delay';
export { timeout, TimeoutError } from './timeout';
export { retry, type RetryOptions } from './retry';
```

### 4. 主入口文件

```typescript
// src/index.ts
// 字符串工具
export * as string from './string';
export { capitalize, kebabCase, truncate } from './string';

// 数组工具
export * as array from './array';
export { chunk, unique, uniqueBy, groupBy } from './array';

// 日期工具
export * as date from './date';

// 异步工具
export * as async from './async';
export { delay, timeout, retry, TimeoutError } from './async';

// 验证工具
export * as validation from './validation';

// 类型导出
export type { TruncateOptions } from './string';
export type { RetryOptions } from './async';
```

## 构建配置

### 1. TypeScript配置

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "declaration": true,
    "declarationMap": true,
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["dist", "node_modules", "tests"]
}
```

### 2. Rollup配置

```javascript
// rollup.config.js
import typescript from '@rollup/plugin-typescript';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from 'rollup-plugin-terser';

const isProduction = process.env.NODE_ENV === 'production';

export default [
  // ES模块构建
  {
    input: 'src/index.ts',
    output: {
      file: 'dist/index.esm.js',
      format: 'esm',
      sourcemap: true
    },
    plugins: [
      nodeResolve(),
      typescript({
        tsconfig: './tsconfig.json',
        declaration: true,
        declarationDir: './dist',
        rootDir: './src'
      }),
      ...(isProduction ? [terser()] : [])
    ]
  },
  // CommonJS构建
  {
    input: 'src/index.ts',
    output: {
      file: 'dist/index.cjs.js',
      format: 'cjs',
      sourcemap: true,
      exports: 'named'
    },
    plugins: [
      nodeResolve(),
      typescript({
        tsconfig: './tsconfig.json'
      }),
      ...(isProduction ? [terser()] : [])
    ]
  },
  // UMD构建（浏览器兼容）
  {
    input: 'src/index.ts',
    output: {
      file: 'dist/index.umd.js',
      format: 'umd',
      name: 'UtilsToolkit',
      sourcemap: true
    },
    plugins: [
      nodeResolve(),
      typescript({
        tsconfig: './tsconfig.json'
      }),
      ...(isProduction ? [terser()] : [])
    ]
  }
];
```

### 3. Package.json配置

```json
{
  "name": "@utils/toolkit",
  "version": "1.0.0",
  "description": "A modular utility library for JavaScript",
  "main": "dist/index.cjs.js",
  "module": "dist/index.esm.js",
  "types": "dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.esm.js",
      "require": "./dist/index.cjs.js",
      "types": "./dist/index.d.ts"
    },
    "./string": {
      "import": "./dist/string/index.js",
      "require": "./dist/string/index.js",
      "types": "./dist/string/index.d.ts"
    },
    "./array": {
      "import": "./dist/array/index.js",
      "require": "./dist/array/index.js", 
      "types": "./dist/array/index.d.ts"
    },
    "./async": {
      "import": "./dist/async/index.js",
      "require": "./dist/async/index.js",
      "types": "./dist/async/index.d.ts"
    }
  },
  "files": [
    "dist"
  ],
  "scripts": {
    "build": "rollup -c",
    "build:prod": "NODE_ENV=production rollup -c",
    "dev": "rollup -c -w",
    "test": "jest",
    "test:watch": "jest --watch",
    "type-check": "tsc --noEmit",
    "lint": "eslint src --ext .ts",
    "prepublishOnly": "npm run build:prod"
  },
  "keywords": ["utilities", "toolkit", "javascript", "typescript"],
  "author": "Your Name",
  "license": "MIT",
  "devDependencies": {
    "@rollup/plugin-node-resolve": "^15.0.0",
    "@rollup/plugin-typescript": "^11.0.0",
    "@types/jest": "^29.0.0",
    "jest": "^29.0.0",
    "rollup": "^3.0.0",
    "rollup-plugin-terser": "^7.0.0",
    "typescript": "^5.0.0"
  }
}
```

## 使用示例

### 1. 完整导入

```javascript
import * as toolkit from '@utils/toolkit';

const result = toolkit.string.capitalize('hello world');
const chunks = toolkit.array.chunk([1, 2, 3, 4, 5], 2);
```

### 2. 按需导入

```javascript
import { capitalize, chunk } from '@utils/toolkit';

const title = capitalize('hello world');
const groups = chunk([1, 2, 3, 4, 5], 2);
```

### 3. 模块化导入

```javascript
import { capitalize, kebabCase } from '@utils/toolkit/string';
import { delay, retry } from '@utils/toolkit/async';

// 字符串处理
const title = capitalize('hello world');
const slug = kebabCase('Hello World API');

// 异步操作
await delay(1000);
const result = await retry(() => fetchData(), { retries: 3 });
```

### 4. CommonJS使用

```javascript
const { capitalize, chunk } = require('@utils/toolkit');

const title = capitalize('hello world');
const groups = chunk([1, 2, 3, 4, 5], 2);
```

## 测试策略

### 1. 单元测试

```javascript
// tests/string/capitalize.test.ts
import { capitalize } from '../../src/string/capitalize';

describe('capitalize', () => {
  test('should capitalize first letter', () => {
    expect(capitalize('hello')).toBe('Hello');
  });

  test('should handle empty string', () => {
    expect(capitalize('')).toBe('');
  });

  test('should handle already capitalized string', () => {
    expect(capitalize('Hello')).toBe('Hello');
  });
});
```

### 2. 集成测试

```javascript
// tests/integration/exports.test.ts
import * as toolkit from '../../src';

describe('Module Exports', () => {
  test('should export all string utilities', () => {
    expect(typeof toolkit.capitalize).toBe('function');
    expect(typeof toolkit.kebabCase).toBe('function');
    expect(typeof toolkit.truncate).toBe('function');
  });

  test('should export namespaced modules', () => {
    expect(typeof toolkit.string.capitalize).toBe('function');
    expect(typeof toolkit.array.chunk).toBe('function');
    expect(typeof toolkit.async.delay).toBe('function');
  });
});
```

## 优化技巧

### 1. Tree Shaking优化

```javascript
// 确保每个函数都是独立导出
// ❌ 不好的做法
const utils = {
  capitalize: (str) => str.charAt(0).toUpperCase() + str.slice(1),
  kebabCase: (str) => str.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase()
};
export default utils;

// ✅ 好的做法
export function capitalize(str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

export function kebabCase(str) {
  return str.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase();
}
```

### 2. 包体积优化

```javascript
// 避免引入大型依赖
// ❌ 引入整个lodash
import _ from 'lodash';

// ✅ 只引入需要的函数
import { isEqual } from 'lodash/isEqual';

// ✅ 或者自己实现简单版本
export function isEqual(a, b) {
  return JSON.stringify(a) === JSON.stringify(b);
}
```

### 3. 类型优化

```typescript
// 使用泛型提供更好的类型推断
export function map<T, U>(array: T[], fn: (item: T) => U): U[] {
  return array.map(fn);
}

// 使用条件类型
export type Flatten<T> = T extends (infer U)[] ? U : T;

export function flatten<T>(array: T[]): Flatten<T>[] {
  return array.flat() as Flatten<T>[];
}
```

## 发布流程

### 1. 版本管理

```bash
# 更新版本号
npm version patch  # 修复bug
npm version minor  # 新功能
npm version major  # 破坏性变更

# 构建和发布
npm run build:prod
npm publish
```

### 2. CI/CD配置

```yaml
# .github/workflows/publish.yml
name: Publish to NPM

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      
      - run: npm ci
      - run: npm test
      - run: npm run build:prod
      - run: npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

### 3. 语义化版本

```json
{
  "scripts": {
    "release:patch": "npm version patch && git push --tags",
    "release:minor": "npm version minor && git push --tags", 
    "release:major": "npm version major && git push --tags"
  }
}
```

## 最佳实践总结

1. **模块设计**：保持单一职责，避免模块间强耦合
2. **类型安全**：提供完整的TypeScript类型定义
3. **构建优化**：支持多种模块格式，优化包体积
4. **测试覆盖**：保证高测试覆盖率和质量
5. **文档完善**：提供清晰的API文档和使用示例
6. **版本管理**：遵循语义化版本规范
7. **持续集成**：自动化测试和发布流程

通过这个完整的案例，我们展示了如何构建一个现代化、可维护的JavaScript模块库，从项目架构到发布流程的每个环节都进行了详细说明。

---

**下一章**: [大型项目模块组织](large-project.md) →
