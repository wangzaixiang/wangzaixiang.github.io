# Bun的模块系统

Bun 是一个现代的 JavaScript/TypeScript 运行时和包管理器，由 Jarred Sumner 开发。它以极速性能为目标，原生支持 TypeScript、JSX，并提供了一套完整的工具链，包括打包器、测试运行器和包管理器。

## Bun 模块系统特点

### 核心设计理念

#### 1. **速度优先**
- **Zig 编写**：使用 Zig 语言编写，性能极佳
- **JavaScriptCore 引擎**：使用 Safari 的 JavaScript 引擎
- **原生优化**：针对模块加载和执行进行了大量优化

#### 2. **现代标准支持**
- **原生 ES 模块**：完全支持 ES 模块语法
- **TypeScript 内置**：无需额外配置即可运行 TypeScript
- **JSX 原生支持**：直接运行 JSX 代码

#### 3. **Node.js 兼容性**
- **API 兼容**：兼容大部分 Node.js API
- **npm 生态**：支持现有的 npm 包
- **无缝迁移**：现有 Node.js 项目可以轻松迁移

## 基础模块语法

### **ES 模块支持**

```typescript
// math.ts
export function add(a: number, b: number): number {
  return a + b;
}

export const PI = 3.14159;

export default class Calculator {
  multiply(a: number, b: number): number {
    return a * b;
  }
}

// app.ts
import Calculator, { add, PI } from './math.ts';
import * as math from './math.ts';

const calc = new Calculator();
console.log(add(2, 3));
console.log(calc.multiply(4, 5));
```

### **TypeScript 零配置支持**

```typescript
// 无需 tsconfig.json 即可运行
// types.ts
interface User {
  id: number;
  name: string;
  email: string;
}

export type UserResponse = {
  user: User;
  token: string;
};

// api.ts
import type { UserResponse } from './types.ts';

export async function fetchUser(id: number): Promise<UserResponse> {
  const response = await fetch(`/api/users/${id}`);
  return response.json();
}

// 直接运行：bun run api.ts
```

### **JSX 原生支持**

```tsx
// component.tsx
interface Props {
  name: string;
  count: number;
}

export function Counter({ name, count }: Props) {
  return (
    <div>
      <h1>Hello {name}!</h1>
      <p>Count: {count}</p>
    </div>
  );
}

// app.tsx
import { Counter } from './component.tsx';

function App() {
  return (
    <div>
      <Counter name="World" count={42} />
    </div>
  );
}

// 直接运行：bun run app.tsx
```

## 内置模块和 API

### **Bun 专有 API**

```typescript
// 文件操作 - Bun.file API
const file = Bun.file('./data.json');
const contents = await file.text();
const data = await file.json();

// 写入文件
await Bun.write('./output.txt', 'Hello Bun!');
await Bun.write('./data.json', { message: 'Hello' });

// 密码哈希
const password = 'secret123';
const hash = await Bun.password.hash(password);
const isValid = await Bun.password.verify(password, hash);

// 环境变量
const port = Bun.env.PORT || '3000';
const isDev = Bun.env.NODE_ENV === 'development';
```

### **高性能 HTTP 服务器**

```typescript
// server.ts
const server = Bun.serve({
  port: 3000,
  async fetch(req) {
    const url = new URL(req.url);
    
    if (url.pathname === '/api/hello') {
      return new Response(JSON.stringify({ message: 'Hello Bun!' }), {
        headers: { 'Content-Type': 'application/json' }
      });
    }
    
    if (url.pathname === '/') {
      return new Response('Hello World!');
    }
    
    return new Response('Not Found', { status: 404 });
  },
});

console.log(`Server running on http://localhost:${server.port}`);
```

### **WebSocket 支持**

```typescript
// websocket-server.ts
const server = Bun.serve({
  port: 3001,
  async fetch(req, server) {
    const success = server.upgrade(req);
    if (success) {
      return undefined;
    }
    return new Response('Upgrade failed', { status: 500 });
  },
  websocket: {
    message(ws, message) {
      console.log('Received:', message);
      ws.send(`Echo: ${message}`);
    },
    open(ws) {
      console.log('WebSocket opened');
      ws.subscribe('chat');
    },
    close(ws, code, message) {
      console.log('WebSocket closed');
    }
  }
});

// 客户端代码
const ws = new WebSocket('ws://localhost:3001');
ws.onmessage = (event) => {
  console.log('Received:', event.data);
};
ws.send('Hello WebSocket!');
```

## 包管理和依赖

### **bun install - 极速包管理**

```bash
# 安装依赖（比 npm 快 20-100 倍）
bun install

# 安装特定包
bun add react react-dom
bun add -d typescript @types/react

# 移除包
bun remove lodash

# 全局安装
bun add -g typescript
```

### **兼容 package.json**

```json
{
  "name": "my-bun-app",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "bun run --watch src/index.ts",
    "build": "bun build src/index.ts --outdir dist",
    "test": "bun test",
    "start": "bun run src/index.ts"
  },
  "dependencies": {
    "react": "^18.2.0",
    "express": "^4.18.2"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/express": "^4.17.17"
  },
  "trustedDependencies": ["esbuild"]
}
```

### **workspaces 支持**

```json
// 根目录 package.json
{
  "name": "monorepo",
  "workspaces": ["packages/*"],
  "devDependencies": {
    "typescript": "^5.0.0"
  }
}

// packages/app/package.json
{
  "name": "@monorepo/app",
  "dependencies": {
    "@monorepo/shared": "workspace:*"
  }
}

// packages/shared/package.json
{
  "name": "@monorepo/shared",
  "exports": {
    ".": "./index.ts"
  }
}
```

## 模块解析和导入

### **Node.js 兼容导入**

```typescript
// 兼容 Node.js 模块
import fs from 'fs';
import path from 'path';
import { readFile } from 'fs/promises';

// 兼容 CommonJS
const express = require('express');
const lodash = require('lodash');

// Bun 特有的快速导入
import { Database } from 'bun:sqlite';
import { spawn } from 'bun:subprocess';
```

### **路径解析增强**

```typescript
// bun.config.js 或 bunfig.toml
export default {
  preload: ['./setup.ts'],
  external: ['sharp', 'canvas'],
  define: {
    __VERSION__: JSON.stringify('1.0.0'),
    __DEV__: 'true'
  }
};

// 使用路径别名
// tsconfig.json 中的 paths 会被自动识别
import { utils } from '@/utils';
import { components } from '~/components';
```

### **动态导入和懒加载**

```typescript
// 条件导入
async function loadRenderer(type: 'server' | 'client') {
  if (type === 'server') {
    const { ServerRenderer } = await import('./server-renderer.ts');
    return new ServerRenderer();
  } else {
    const { ClientRenderer } = await import('./client-renderer.ts');
    return new ClientRenderer();
  }
}

// 插件系统
class PluginLoader {
  private plugins = new Map<string, any>();
  
  async loadPlugin(name: string, path: string) {
    try {
      const plugin = await import(path);
      this.plugins.set(name, plugin.default || plugin);
      console.log(`Plugin ${name} loaded`);
    } catch (error) {
      console.error(`Failed to load plugin ${name}:`, error);
    }
  }
  
  getPlugin(name: string) {
    return this.plugins.get(name);
  }
}
```

## 内置工具链

### **Bun.build - 内置打包器**

```typescript
// build.ts
await Bun.build({
  entrypoints: ['./src/index.ts'],
  outdir: './dist',
  target: 'browser', // 'browser' | 'bun' | 'node'
  format: 'esm',     // 'esm' | 'cjs' | 'iife'
  minify: true,
  sourcemap: 'external',
  splitting: true,   // 代码分割
  
  // 环境变量处理
  env: 'inline',     // 'inline' | 'PUBLIC_*' | 'disable'
  
  // 插件系统
  plugins: [
    {
      name: 'custom-plugin',
      setup(build) {
        build.onLoad({ filter: /\.custom$/ }, async (args) => {
          const contents = await Bun.file(args.path).text();
          return {
            contents: `export default ${JSON.stringify(contents)}`,
            loader: 'js'
          };
        });
      }
    }
  ],
  
  // 外部依赖
  external: ['react', 'react-dom']
});
```

### **Macros - 构建时代码生成**

```typescript
// database.ts (macro 文件)
export function sql(strings: TemplateStringsArray, ...values: any[]) {
  // 构建时执行，生成优化的 SQL 查询代码
  const query = strings.reduce((acc, str, i) => {
    return acc + str + (values[i] ? `$${i + 1}` : '');
  }, '');
  
  return {
    query,
    params: values,
    execute: (db: any) => db.prepare(query).all(...values)
  };
}

// app.ts
import { sql } from './database.ts' with { type: 'macro' };

// 这会在构建时展开为优化的代码
const users = await sql`SELECT * FROM users WHERE id = ${userId}`;

// 构建后生成的代码类似：
// const users = await db.prepare("SELECT * FROM users WHERE id = $1").all(userId);
```

### **内置测试运行器**

```typescript
// math.test.ts
import { expect, test, describe, beforeAll, afterAll } from 'bun:test';
import { add, subtract } from './math.ts';

describe('Math functions', () => {
  test('addition', () => {
    expect(add(2, 3)).toBe(5);
    expect(add(-1, 1)).toBe(0);
  });
  
  test('subtraction', () => {
    expect(subtract(5, 3)).toBe(2);
    expect(subtract(1, 1)).toBe(0);
  });
  
  test('async operation', async () => {
    const result = await fetch('https://httpbin.org/json');
    const data = await result.json();
    expect(data).toHaveProperty('slideshow');
  });
});

// 运行测试：bun test
// 并行运行，速度极快
```

### **性能基准测试**

```typescript
// benchmark.ts
import { bench, run } from 'bun:test';

bench('string concatenation', () => {
  let result = '';
  for (let i = 0; i < 1000; i++) {
    result += 'a';
  }
});

bench('array join', () => {
  const arr = [];
  for (let i = 0; i < 1000; i++) {
    arr.push('a');
  }
  arr.join('');
});

bench('template literal', () => {
  let result = '';
  for (let i = 0; i < 1000; i++) {
    result = `${result}a`;
  }
});

await run();
```

## 数据库集成

### **内置 SQLite**

```typescript
// database.ts
import { Database } from 'bun:sqlite';

const db = new Database('mydb.sqlite');

// 创建表
db.exec(`
  CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
  )
`);

// 预编译语句（性能最佳）
const insertUser = db.prepare('INSERT INTO users (name, email) VALUES (?, ?)');
const getUserById = db.prepare('SELECT * FROM users WHERE id = ?');
const getAllUsers = db.prepare('SELECT * FROM users');

// 插入数据
insertUser.run('John Doe', 'john@example.com');

// 查询数据
const user = getUserById.get(1);
const users = getAllUsers.all();

// 事务支持
const insertManyUsers = db.transaction((users) => {
  for (const user of users) {
    insertUser.run(user.name, user.email);
  }
});

insertManyUsers([
  { name: 'Alice', email: 'alice@example.com' },
  { name: 'Bob', email: 'bob@example.com' }
]);

db.close();
```

### **ORM 集成示例**

```typescript
// user.model.ts
interface User {
  id?: number;
  name: string;
  email: string;
  createdAt?: Date;
}

class UserModel {
  private db: Database;
  
  constructor(db: Database) {
    this.db = db;
    this.init();
  }
  
  private init() {
    this.db.exec(`
      CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `);
  }
  
  create(user: Omit<User, 'id' | 'createdAt'>): User {
    const stmt = this.db.prepare('INSERT INTO users (name, email) VALUES (?, ?) RETURNING *');
    return stmt.get(user.name, user.email) as User;
  }
  
  findById(id: number): User | null {
    const stmt = this.db.prepare('SELECT * FROM users WHERE id = ?');
    return stmt.get(id) as User | null;
  }
  
  findAll(): User[] {
    const stmt = this.db.prepare('SELECT * FROM users ORDER BY created_at DESC');
    return stmt.all() as User[];
  }
  
  update(id: number, updates: Partial<User>): User | null {
    const fields = Object.keys(updates).map(key => `${key} = ?`).join(', ');
    const values = Object.values(updates);
    const stmt = this.db.prepare(`UPDATE users SET ${fields} WHERE id = ? RETURNING *`);
    return stmt.get(...values, id) as User | null;
  }
  
  delete(id: number): boolean {
    const stmt = this.db.prepare('DELETE FROM users WHERE id = ?');
    const result = stmt.run(id);
    return result.changes > 0;
  }
}

// 使用示例
import { Database } from 'bun:sqlite';

const db = new Database('app.db');
const userModel = new UserModel(db);

const newUser = userModel.create({
  name: 'Alice Smith',
  email: 'alice@example.com'
});

console.log('Created user:', newUser);
```

## 开发工具和调试

### **热重载开发**

```typescript
// dev-server.ts
const server = Bun.serve({
  port: 3000,
  development: true, // 启用开发模式
  
  async fetch(req) {
    const url = new URL(req.url);
    
    // API 路由
    if (url.pathname.startsWith('/api/')) {
      // 动态导入，支持热重载
      const handler = await import(`./api${url.pathname.replace('/api', '')}.ts`);
      return handler.default(req);
    }
    
    // 静态文件服务
    return new Response(Bun.file('./public/index.html'));
  }
});

// 文件监听器
const watcher = Bun.watch({
  recursive: true,
  paths: ['./src'],
  onchange(event, path) {
    console.log(`File changed: ${path}`);
    // 可以发送 WebSocket 消息通知客户端刷新
  }
});

console.log(`Dev server running on http://localhost:${server.port}`);
```

### **调试和日志**

```typescript
// logger.ts
class Logger {
  private isDev = Bun.env.NODE_ENV === 'development';
  
  info(message: string, ...args: any[]) {
    if (this.isDev) {
      console.log(`[INFO] ${new Date().toISOString()} - ${message}`, ...args);
    }
  }
  
  error(message: string, error?: Error) {
    console.error(`[ERROR] ${new Date().toISOString()} - ${message}`);
    if (error && this.isDev) {
      console.error(error.stack);
    }
  }
  
  perf<T>(name: string, fn: () => T): T {
    if (!this.isDev) return fn();
    
    const start = performance.now();
    const result = fn();
    const end = performance.now();
    
    console.log(`[PERF] ${name}: ${(end - start).toFixed(2)}ms`);
    return result;
  }
}

export const logger = new Logger();

// 使用示例
import { logger } from './logger.ts';

logger.perf('database-query', () => {
  return db.prepare('SELECT * FROM users').all();
});
```

## 性能优化

### **预编译和缓存**

```typescript
// 利用 Bun 的编译缓存
// bun.config.js
export default {
  // 预加载模块，提高启动速度
  preload: [
    './src/setup.ts',
    './src/config.ts'
  ],
  
  // 编译目标
  target: 'bun',
  
  // 外部依赖（不编译）
  external: ['sharp', 'canvas'],
  
  // 压缩
  minify: {
    whitespace: true,
    identifiers: true,
    syntax: true
  }
};
```

### **内存和性能监控**

```typescript
// monitor.ts
class PerformanceMonitor {
  private metrics = new Map<string, number[]>();
  
  measure<T>(name: string, fn: () => T): T {
    const start = performance.now();
    
    try {
      const result = fn();
      const duration = performance.now() - start;
      this.recordMetric(name, duration);
      return result;
    } catch (error) {
      const duration = performance.now() - start;
      this.recordMetric(`${name}-error`, duration);
      throw error;
    }
  }
  
  async measureAsync<T>(name: string, fn: () => Promise<T>): Promise<T> {
    const start = performance.now();
    
    try {
      const result = await fn();
      const duration = performance.now() - start;
      this.recordMetric(name, duration);
      return result;
    } catch (error) {
      const duration = performance.now() - start;
      this.recordMetric(`${name}-error`, duration);
      throw error;
    }
  }
  
  private recordMetric(name: string, value: number) {
    if (!this.metrics.has(name)) {
      this.metrics.set(name, []);
    }
    this.metrics.get(name)!.push(value);
  }
  
  getStats(name: string) {
    const values = this.metrics.get(name) || [];
    if (values.length === 0) return null;
    
    const sum = values.reduce((a, b) => a + b, 0);
    const avg = sum / values.length;
    const min = Math.min(...values);
    const max = Math.max(...values);
    
    return { avg, min, max, count: values.length };
  }
  
  getMemoryUsage() {
    return {
      rss: process.memoryUsage.rss(),
      heapUsed: process.memoryUsage().heapUsed,
      heapTotal: process.memoryUsage().heapTotal,
      external: process.memoryUsage().external
    };
  }
}

export const monitor = new PerformanceMonitor();
```

## 生产环境部署

### **Docker 配置**

```dockerfile
# Dockerfile
FROM oven/bun:1.0-alpine

WORKDIR /app

# 复制依赖文件
COPY package.json bun.lockb ./

# 安装依赖
RUN bun install --frozen-lockfile --production

# 复制源码
COPY src/ ./src/

# 构建应用
RUN bun build src/index.ts --outdir dist --target bun

# 设置环境变量
ENV NODE_ENV=production

# 暴露端口
EXPOSE 3000

# 启动应用
CMD ["bun", "run", "dist/index.js"]
```

### **部署脚本**

```typescript
// deploy.ts
import { $ } from 'bun';

async function deploy() {
  console.log('Starting deployment...');
  
  // 运行测试
  await $`bun test`;
  console.log('✅ Tests passed');
  
  // 构建生产版本
  await $`bun run build:prod`;
  console.log('✅ Build completed');
  
  // 构建 Docker 镜像
  await $`docker build -t myapp:latest .`;
  console.log('✅ Docker image built');
  
  // 推送到仓库（如果需要）
  if (Bun.env.PUSH_TO_REGISTRY === 'true') {
    await $`docker push myapp:latest`;
    console.log('✅ Image pushed to registry');
  }
  
  console.log('🚀 Deployment completed!');
}

deploy().catch(console.error);
```

## 最佳实践总结

### **性能优化**
1. **利用 Bun 的速度优势** - 充分使用内置 API
2. **合理使用 Macros** - 在构建时生成优化代码
3. **数据库连接复用** - 使用连接池和预编译语句
4. **避免不必要的依赖** - 利用 Bun 的内置功能

### **开发体验**
1. **零配置 TypeScript** - 直接运行 .ts 文件
2. **热重载开发** - 使用 `--watch` 标志
3. **内置测试工具** - 使用 `bun test` 进行快速测试
4. **一体化工具链** - 减少工具配置复杂度

### **生产环境**
1. **容器化部署** - 使用官方 Docker 镜像
2. **环境变量管理** - 合理配置生产环境变量
3. **监控和日志** - 实施完善的监控体系
4. **渐进式迁移** - 从 Node.js 逐步迁移到 Bun

### **生态兼容性**
1. **Node.js API 兼容** - 大部分现有代码可直接运行
2. **npm 包支持** - 兼容现有的 npm 生态
3. **框架支持** - 支持 React、Vue、Express 等主流框架
4. **工具集成** - 与现有开发工具链良好集成

Bun 代表了 JavaScript 运行时的下一代发展方向，通过极致的性能优化、现代化的工具链和优秀的开发体验，为 JavaScript 开发者提供了一个全新的选择。

---

**下一章**: [最佳实践](../best-practices/design.md) →