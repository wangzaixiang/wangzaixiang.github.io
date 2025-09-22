# Deno的模块系统

Deno 是一个现代的 JavaScript/TypeScript 运行时，由 Node.js 的创始人 Ryan Dahl 开发。它从设计之初就原生支持 ES 模块，并采用了基于 URL 的模块导入系统，为模块化开发带来了全新的体验。

## Deno 模块系统特点

### 核心设计理念

#### 1. **原生 ES 模块支持**
- **ES 模块优先**：Deno 从一开始就完全支持 ES 模块
- **无需 package.json**：不依赖 npm 的包管理模式
- **标准化导入**：使用标准的 import/export 语法

#### 2. **基于 URL 的模块导入**
- **去中心化**：模块可以从任何 URL 导入
- **明确的依赖**：依赖关系通过 URL 明确表达
- **版本控制**：URL 中包含版本信息

#### 3. **安全优先**
- **权限系统**：模块需要明确的权限才能访问系统资源
- **沙盒执行**：默认情况下模块运行在受限环境中

## 基础模块语法

### **标准导入导出**

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

### **URL 导入**

```typescript
// 从远程 URL 导入
import { serve } from 'https://deno.land/std@0.200.0/http/server.ts';
import { parse } from 'https://deno.land/std@0.200.0/flags/mod.ts';

// 从 CDN 导入第三方库
import lodash from 'https://cdn.skypack.dev/lodash@4.17.21';
import React from 'https://esm.sh/react@18.2.0';

// 使用具体版本确保稳定性
import { assertEquals } from 'https://deno.land/std@0.200.0/testing/asserts.ts';
```

### **相对路径导入**

```typescript
// 项目结构
// src/
//   ├── utils/
//   │   ├── helpers.ts
//   │   └── constants.ts
//   ├── components/
//   │   └── Button.ts
//   └── app.ts

// app.ts
import { formatDate } from './utils/helpers.ts';
import { API_URL } from './utils/constants.ts';
import Button from './components/Button.ts';

// helpers.ts
export function formatDate(date: Date): string {
  return date.toISOString().split('T')[0];
}

// 注意：Deno 要求完整的文件扩展名
import { config } from './config.ts'; // ✅ 正确
import { config } from './config';     // ❌ 错误
```

## 依赖管理

### **deps.ts 模式**

```typescript
// deps.ts - 集中管理外部依赖
export { serve } from 'https://deno.land/std@0.200.0/http/server.ts';
export { parse } from 'https://deno.land/std@0.200.0/flags/mod.ts';
export { join, dirname } from 'https://deno.land/std@0.200.0/path/mod.ts';
export { assert, assertEquals } from 'https://deno.land/std@0.200.0/testing/asserts.ts';

// 重新导出并重命名
export { default as lodash } from 'https://cdn.skypack.dev/lodash@4.17.21';
export { default as React } from 'https://esm.sh/react@18.2.0';

// 应用代码中使用
// app.ts
import { serve, parse, React } from './deps.ts';

const handler = (req: Request) => {
  const url = new URL(req.url);
  const params = parse(url.searchParams.toString());
  
  return new Response(`Hello ${params.name || 'World'}!`);
};

serve(handler, { port: 8000 });
```

### **Import Maps**

```json
// import_map.json
{
  "imports": {
    "std/": "https://deno.land/std@0.200.0/",
    "fmt/": "https://deno.land/std@0.200.0/fmt/",
    "testing/": "https://deno.land/std@0.200.0/testing/",
    "react": "https://esm.sh/react@18.2.0",
    "react-dom": "https://esm.sh/react-dom@18.2.0",
    "@/": "./src/"
  }
}
```

```typescript
// 使用 Import Maps 后的代码
import { serve } from 'std/http/server.ts';
import { red, bold } from 'fmt/colors.ts';
import { assertEquals } from 'testing/asserts.ts';
import React from 'react';
import { config } from '@/config.ts';

// 启动时指定 import map
// deno run --import-map=import_map.json app.ts
```

### **deno.json 配置**

```json
{
  "compilerOptions": {
    "allowJs": true,
    "lib": ["deno.window"],
    "strict": true
  },
  "importMap": "./import_map.json",
  "tasks": {
    "dev": "deno run --watch --allow-net --allow-read app.ts",
    "test": "deno test --allow-all",
    "fmt": "deno fmt",
    "lint": "deno lint"
  },
  "fmt": {
    "files": {
      "include": ["src/", "tests/"],
      "exclude": ["dist/"]
    }
  },
  "lint": {
    "files": {
      "include": ["src/"],
      "exclude": ["dist/"]
    }
  }
}
```

## 内置模块和标准库

### **Deno 标准库**

```typescript
// 文件系统操作
import { copy, ensureDir, exists } from 'std/fs/mod.ts';
import { readLines } from 'std/io/mod.ts';

// 网络请求
import { serve } from 'std/http/server.ts';
import { Status } from 'std/http/http_status.ts';

// 路径处理
import { join, dirname, basename, extname } from 'std/path/mod.ts';

// 日期时间
import { format, parse } from 'std/datetime/mod.ts';

// 编码解码
import { encode, decode } from 'std/encoding/base64.ts';
import { stringify, parse as parseCSV } from 'std/encoding/csv.ts';

// 测试工具
import { assertEquals, assertThrows } from 'std/testing/asserts.ts';
import { FakeTime } from 'std/testing/time.ts';

// 示例：文件服务器
async function fileServer() {
  const handler = async (req: Request): Promise<Response> => {
    const url = new URL(req.url);
    const filePath = join('.', url.pathname);
    
    if (await exists(filePath)) {
      const file = await Deno.readFile(filePath);
      return new Response(file);
    }
    
    return new Response('Not Found', { status: Status.NotFound });
  };
  
  console.log('Server running on http://localhost:8000');
  await serve(handler, { port: 8000 });
}
```

### **Web APIs**

```typescript
// Deno 支持现代 Web APIs
// Fetch API
const response = await fetch('https://api.github.com/users/denoland');
const data = await response.json();

// Web Streams
const readable = new ReadableStream({
  start(controller) {
    controller.enqueue('Hello ');
    controller.enqueue('World!');
    controller.close();
  }
});

// URL 构造器
const url = new URL('/api/users', 'https://example.com');
url.searchParams.set('page', '1');

// 加密 API
const data = new TextEncoder().encode('hello world');
const hashBuffer = await crypto.subtle.digest('SHA-256', data);
const hashArray = Array.from(new Uint8Array(hashBuffer));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
```

## 动态导入和代码分割

### **动态导入**

```typescript
// 条件导入
async function loadEnvironmentConfig(env: string) {
  switch (env) {
    case 'development':
      const devConfig = await import('./config/development.ts');
      return devConfig.default;
    case 'production':
      const prodConfig = await import('./config/production.ts');
      return prodConfig.default;
    default:
      throw new Error(`Unknown environment: ${env}`);
  }
}

// 懒加载模块
async function processData(data: unknown[]) {
  // 只在需要时加载数据处理模块
  const { DataProcessor } = await import('./utils/data-processor.ts');
  const processor = new DataProcessor();
  return processor.process(data);
}

// 插件系统
class PluginManager {
  private plugins = new Map<string, unknown>();
  
  async loadPlugin(name: string, url: string) {
    try {
      const plugin = await import(url);
      this.plugins.set(name, plugin.default);
      console.log(`Plugin ${name} loaded successfully`);
    } catch (error) {
      console.error(`Failed to load plugin ${name}:`, error);
    }
  }
  
  getPlugin(name: string) {
    return this.plugins.get(name);
  }
}
```

### **代码分割策略**

```typescript
// 路由级代码分割
class Router {
  private routes = new Map<string, () => Promise<{ default: Function }>>();
  
  register(path: string, moduleUrl: string) {
    this.routes.set(path, () => import(moduleUrl));
  }
  
  async handle(request: Request): Promise<Response> {
    const url = new URL(request.url);
    const routeLoader = this.routes.get(url.pathname);
    
    if (!routeLoader) {
      return new Response('Not Found', { status: 404 });
    }
    
    try {
      const module = await routeLoader();
      return await module.default(request);
    } catch (error) {
      console.error('Route handler error:', error);
      return new Response('Internal Server Error', { status: 500 });
    }
  }
}

// 使用示例
const router = new Router();
router.register('/api/users', './routes/users.ts');
router.register('/api/posts', './routes/posts.ts');
router.register('/health', './routes/health.ts');
```

## 权限系统

### **权限模型**

```typescript
// Deno 的权限系统确保安全性
// 需要明确指定权限才能访问系统资源

// 文件系统权限
// deno run --allow-read --allow-write app.ts

async function readConfig() {
  try {
    const config = await Deno.readTextFile('./config.json');
    return JSON.parse(config);
  } catch (error) {
    if (error instanceof Deno.errors.PermissionDenied) {
      console.error('Permission denied: Cannot read config file');
      console.error('Run with --allow-read flag');
    }
    throw error;
  }
}

// 网络权限
// deno run --allow-net app.ts

async function fetchData(url: string) {
  try {
    const response = await fetch(url);
    return await response.json();
  } catch (error) {
    if (error instanceof Deno.errors.PermissionDenied) {
      console.error('Permission denied: Cannot make network requests');
      console.error('Run with --allow-net flag');
    }
    throw error;
  }
}

// 环境变量权限
// deno run --allow-env app.ts

function getPort(): number {
  try {
    return parseInt(Deno.env.get('PORT') || '8000', 10);
  } catch (error) {
    if (error instanceof Deno.errors.PermissionDenied) {
      console.error('Permission denied: Cannot access environment variables');
      console.error('Run with --allow-env flag');
    }
    return 8000;
  }
}
```

### **权限检查和处理**

```typescript
// 运行时权限检查
async function safeFileOperation(path: string) {
  // 检查读取权限
  const readStatus = await Deno.permissions.query({ name: 'read', path });
  if (readStatus.state !== 'granted') {
    // 请求权限
    const readRequest = await Deno.permissions.request({ name: 'read', path });
    if (readRequest.state !== 'granted') {
      throw new Error('Read permission denied');
    }
  }
  
  // 检查写入权限
  const writeStatus = await Deno.permissions.query({ name: 'write', path });
  if (writeStatus.state !== 'granted') {
    const writeRequest = await Deno.permissions.request({ name: 'write', path });
    if (writeRequest.state !== 'granted') {
      throw new Error('Write permission denied');
    }
  }
  
  // 安全地执行文件操作
  const content = await Deno.readTextFile(path);
  const modified = content.toUpperCase();
  await Deno.writeTextFile(path, modified);
}

// 权限包装器
class PermissionWrapper {
  async withPermission<T>(
    permission: Deno.PermissionDescriptor,
    operation: () => Promise<T>
  ): Promise<T> {
    const status = await Deno.permissions.query(permission);
    
    if (status.state === 'denied') {
      throw new Error(`Permission denied: ${permission.name}`);
    }
    
    if (status.state === 'prompt') {
      const request = await Deno.permissions.request(permission);
      if (request.state !== 'granted') {
        throw new Error(`Permission rejected: ${permission.name}`);
      }
    }
    
    return await operation();
  }
}

// 使用示例
const permissionWrapper = new PermissionWrapper();

await permissionWrapper.withPermission(
  { name: 'net', host: 'api.example.com' },
  async () => {
    const response = await fetch('https://api.example.com/data');
    return await response.json();
  }
);
```

## 测试和开发工具

### **内置测试框架**

```typescript
// math_test.ts
import { assertEquals, assertThrows } from 'std/testing/asserts.ts';
import { add, divide } from './math.ts';

Deno.test('addition works correctly', () => {
  assertEquals(add(2, 3), 5);
  assertEquals(add(-1, 1), 0);
  assertEquals(add(0, 0), 0);
});

Deno.test('division works correctly', () => {
  assertEquals(divide(10, 2), 5);
  assertEquals(divide(7, 2), 3.5);
});

Deno.test('division by zero throws error', () => {
  assertThrows(
    () => divide(5, 0),
    Error,
    'Division by zero'
  );
});

// 异步测试
Deno.test('async operation', async () => {
  const result = await fetch('https://httpbin.org/json');
  const data = await result.json();
  assertEquals(typeof data, 'object');
});

// 测试权限
Deno.test({
  name: 'file operation test',
  permissions: { read: true, write: true },
  async fn() {
    await Deno.writeTextFile('./test.txt', 'hello');
    const content = await Deno.readTextFile('./test.txt');
    assertEquals(content, 'hello');
    await Deno.remove('./test.txt');
  }
});
```

### **基准测试**

```typescript
// benchmark.ts
import { bench, runBenchmarks } from 'std/testing/bench.ts';

// 简单基准测试
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

// 异步基准测试
bench({
  name: 'async fetch',
  async fn() {
    await fetch('https://httpbin.org/json');
  }
});

// 运行基准测试
if (import.meta.main) {
  runBenchmarks();
}
```

### **开发工具集成**

```typescript
// dev.ts - 开发环境工具
import { serve } from 'std/http/server.ts';
import { serveDir } from 'std/http/file_server.ts';

class DevServer {
  private watcher?: Deno.FsWatcher;
  
  async start(port = 8000) {
    console.log(`Dev server starting on http://localhost:${port}`);
    
    // 文件监听和热重载
    this.setupFileWatcher();
    
    const handler = async (req: Request): Promise<Response> => {
      const url = new URL(req.url);
      
      // API 路由
      if (url.pathname.startsWith('/api/')) {
        return await this.handleApiRequest(req);
      }
      
      // 静态文件服务
      return serveDir(req, {
        fsRoot: './public',
        showDirListing: true,
      });
    };
    
    await serve(handler, { port });
  }
  
  private setupFileWatcher() {
    this.watcher = Deno.watchFs(['./src'], { recursive: true });
    
    (async () => {
      for await (const event of this.watcher!) {
        if (event.kind === 'modify') {
          console.log(`File changed: ${event.paths.join(', ')}`);
          // 触发重新加载
          await this.broadcastReload();
        }
      }
    })();
  }
  
  private async handleApiRequest(req: Request): Promise<Response> {
    const url = new URL(req.url);
    
    // 动态加载 API 路由
    try {
      const routePath = `./api${url.pathname.replace('/api', '')}.ts`;
      const module = await import(routePath);
      return await module.default(req);
    } catch (error) {
      console.error('API route error:', error);
      return new Response('API Error', { status: 500 });
    }
  }
  
  private async broadcastReload() {
    // 实现热重载逻辑
    console.log('Broadcasting reload...');
  }
  
  stop() {
    this.watcher?.close();
  }
}

// 启动开发服务器
if (import.meta.main) {
  const server = new DevServer();
  await server.start();
}
```

## 性能优化

### **模块缓存和预编译**

```typescript
// 缓存管理
class ModuleCache {
  private cache = new Map<string, unknown>();
  private loading = new Map<string, Promise<unknown>>();
  
  async load<T>(url: string): Promise<T> {
    // 检查缓存
    if (this.cache.has(url)) {
      return this.cache.get(url) as T;
    }
    
    // 检查是否正在加载
    if (this.loading.has(url)) {
      return await this.loading.get(url) as T;
    }
    
    // 开始加载
    const loadPromise = this.loadModule<T>(url);
    this.loading.set(url, loadPromise);
    
    try {
      const module = await loadPromise;
      this.cache.set(url, module);
      return module;
    } finally {
      this.loading.delete(url);
    }
  }
  
  private async loadModule<T>(url: string): Promise<T> {
    const module = await import(url);
    return module.default || module;
  }
  
  invalidate(url: string) {
    this.cache.delete(url);
  }
  
  clear() {
    this.cache.clear();
    this.loading.clear();
  }
}

// 使用示例
const moduleCache = new ModuleCache();

async function loadProcessor() {
  return await moduleCache.load('./processors/heavy-processor.ts');
}
```

### **Bundle 和部署**

```typescript
// build.ts - 生产环境构建脚本
import { bundle } from 'https://deno.land/x/emit@0.26.0/mod.ts';

async function buildForProduction() {
  console.log('Building for production...');
  
  const result = await bundle('./src/app.ts', {
    compilerOptions: {
      sourceMap: false,
      target: 'ES2022',
    },
  });
  
  // 写入构建结果
  await Deno.writeTextFile('./dist/app.js', result.code);
  
  // 生成元数据
  const metadata = {
    buildTime: new Date().toISOString(),
    version: '1.0.0',
    modules: result.files,
  };
  
  await Deno.writeTextFile(
    './dist/metadata.json',
    JSON.stringify(metadata, null, 2)
  );
  
  console.log('Build completed successfully!');
}

// Docker 部署配置
const dockerFile = `
FROM denoland/deno:1.37.0

WORKDIR /app

# 缓存依赖
COPY deps.ts ./
RUN deno cache deps.ts

# 复制源码
COPY . .

# 缓存入口文件
RUN deno cache app.ts

# 运行应用
CMD ["run", "--allow-net", "--allow-read", "app.ts"]
`;

await Deno.writeTextFile('./Dockerfile', dockerFile);
```

## 最佳实践总结

### **模块组织**
1. **使用 deps.ts 集中管理依赖** - 便于版本控制和更新
2. **Import Maps 简化导入路径** - 提高代码可读性
3. **明确的文件扩展名** - Deno 要求完整的文件扩展名

### **安全性**
1. **最小权限原则** - 只授予必要的权限
2. **运行时权限检查** - 优雅处理权限不足的情况
3. **URL 验证** - 验证远程模块的来源和完整性

### **性能优化**
1. **模块缓存** - 避免重复加载相同模块
2. **代码分割** - 按需加载非关键模块
3. **版本锁定** - 使用具体版本确保稳定性

### **开发体验**
1. **内置工具链** - 充分利用 Deno 的内置功能
2. **TypeScript 优先** - 享受类型安全的开发体验
3. **标准化格式** - 使用 `deno fmt` 和 `deno lint`

Deno 的模块系统代表了 JavaScript 生态系统的现代化方向，通过标准化的导入机制、强大的安全模型和优秀的开发体验，为构建安全、高效的应用提供了新的可能性。

---

**下一章**: [最佳实践](../best-practices/design.md) →
