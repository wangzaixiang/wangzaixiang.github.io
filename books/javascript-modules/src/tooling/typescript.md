# TypeScript模块

TypeScript作为JavaScript的超集，为模块化开发带来了强大的类型系统支持。它不仅提供了编译时类型检查，还增强了模块的导入导出机制，为大型项目的模块化架构提供了坚实的基础。

## TypeScript模块系统

### 模块语法增强
TypeScript在ES模块基础上增加了类型信息：

```typescript
// 类型导出
export type UserType = {
  id: number;
  name: string;
  email: string;
};

export interface ApiResponse<T> {
  data: T;
  status: number;
  message: string;
}

// 值和类型同时导出
export class UserService {
  async getUser(id: number): Promise<UserType> {
    // 实现...
  }
}

// 命名空间导出
export namespace Utils {
  export function formatDate(date: Date): string {
    return date.toISOString();
  }
  
  export type DateFormat = 'ISO' | 'Local' | 'UTC';
}

// 条件类型导出
export type ApiEndpoint<T extends string> = T extends 'users' 
  ? UserType[] 
  : T extends 'posts' 
  ? PostType[] 
  : unknown;
```

### 模块导入的类型支持
```typescript
// 类型导入
import type { UserType, ApiResponse } from './types';
import type * as Types from './types';

// 值导入
import { UserService } from './services';
import { Utils } from './utils';

// 混合导入
import { CONFIG, type ConfigType } from './config';

// 动态导入与类型
const loadModule = async (): Promise<typeof import('./heavy-module')> => {
  return await import('./heavy-module');
};

// 条件导入
type ModuleType = typeof import('./module');
type AsyncModuleType = Awaited<typeof import('./async-module')>;
```

### 模块声明和环境声明
```typescript
// 全局模块声明
declare global {
  interface Window {
    __APP_CONFIG__: AppConfig;
  }
  
  namespace NodeJS {
    interface ProcessEnv {
      NODE_ENV: 'development' | 'production' | 'test';
      API_URL: string;
    }
  }
}

// 模块声明
declare module '*.vue' {
  import { DefineComponent } from 'vue';
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module '*.module.css' {
  const classes: { readonly [key: string]: string };
  export default classes;
}

// 扩展已有模块
declare module 'express' {
  interface Request {
    user?: User;
  }
}

// 第三方库类型声明
declare module 'some-untyped-library' {
  export function someFunction(arg: string): number;
  export const CONSTANT: string;
}
```

## 编译配置

### tsconfig.json详解
```json
{
  "compilerOptions": {
    // 模块系统配置
    "module": "ES2022",
    "moduleResolution": "node",
    "target": "ES2020",
    
    // 模块检测
    "moduleDetection": "auto",
    
    // 输出控制
    "outDir": "./dist",
    "rootDir": "./src",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    
    // 模块解析
    "baseUrl": "./",
    "paths": {
      "@/*": ["src/*"],
      "@components/*": ["src/components/*"],
      "@utils/*": ["src/utils/*"]
    },
    
    // 类型检查
    "strict": true,
    "noImplicitReturns": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    
    // ES模块互操作
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "forceConsistentCasingInFileNames": true,
    
    // 实验性特性
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    
    // 增量编译
    "incremental": true,
    "tsBuildInfoFile": ".tsbuildinfo",
    
    // 类型导入
    "verbatimModuleSyntax": false,
    "allowImportingTsExtensions": false
  },
  
  // 项目引用
  "references": [
    { "path": "./packages/core" },
    { "path": "./packages/utils" }
  ],
  
  // 包含和排除
  "include": [
    "src/**/*",
    "types/**/*"
  ],
  "exclude": [
    "node_modules",
    "dist",
    "**/*.test.ts"
  ]
}
```

### 多包项目配置
```typescript
// packages/core/tsconfig.json
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "outDir": "./dist",
    "rootDir": "./src",
    "composite": true
  },
  "include": ["src/**/*"],
  "references": []
}

// packages/utils/tsconfig.json  
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "outDir": "./dist",
    "rootDir": "./src",
    "composite": true
  },
  "include": ["src/**/*"],
  "references": [
    { "path": "../core" }
  ]
}

// tsconfig.base.json
{
  "compilerOptions": {
    "module": "ES2022",
    "target": "ES2020",
    "moduleResolution": "node",
    "strict": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "incremental": true
  }
}
```

## 高级模块模式

### 模块扩展模式
```typescript
// 基础模块
// base-module.ts
export interface BaseConfig {
  name: string;
  version: string;
}

export class BaseService {
  constructor(protected config: BaseConfig) {}
  
  getName(): string {
    return this.config.name;
  }
}

// 扩展模块
// extended-module.ts
import { BaseConfig, BaseService } from './base-module';

export interface ExtendedConfig extends BaseConfig {
  features: string[];
  debug: boolean;
}

export class ExtendedService extends BaseService {
  constructor(protected config: ExtendedConfig) {
    super(config);
  }
  
  getFeatures(): string[] {
    return this.config.features;
  }
  
  isDebugEnabled(): boolean {
    return this.config.debug;
  }
}

// 模块聚合
export * from './base-module';
export { ExtendedService, type ExtendedConfig } from './extended-module';
```

### 插件架构模式
```typescript
// 插件系统类型定义
export interface Plugin<T = any> {
  name: string;
  version: string;
  install(app: App, options?: T): void;
  uninstall?(app: App): void;
}

export interface App {
  use<T>(plugin: Plugin<T>, options?: T): this;
  unuse(pluginName: string): this;
  getPlugin<T extends Plugin>(name: string): T | undefined;
}

// 插件实现
export class ValidationPlugin implements Plugin<ValidationOptions> {
  name = 'validation';
  version = '1.0.0';
  
  install(app: App, options: ValidationOptions = {}) {
    // 安装验证插件
    app.addValidator(new Validator(options));
  }
  
  uninstall(app: App) {
    app.removeValidator(this.name);
  }
}

// 应用实现
export class Application implements App {
  private plugins = new Map<string, Plugin>();
  private validators = new Map<string, Validator>();
  
  use<T>(plugin: Plugin<T>, options?: T): this {
    if (this.plugins.has(plugin.name)) {
      throw new Error(`Plugin ${plugin.name} already installed`);
    }
    
    plugin.install(this, options);
    this.plugins.set(plugin.name, plugin);
    return this;
  }
  
  unuse(pluginName: string): this {
    const plugin = this.plugins.get(pluginName);
    if (plugin?.uninstall) {
      plugin.uninstall(this);
    }
    this.plugins.delete(pluginName);
    return this;
  }
  
  getPlugin<T extends Plugin>(name: string): T | undefined {
    return this.plugins.get(name) as T;
  }
  
  addValidator(validator: Validator): void {
    this.validators.set(validator.name, validator);
  }
  
  removeValidator(name: string): void {
    this.validators.delete(name);
  }
}
```

### 工厂模式模块
```typescript
// 工厂接口
export interface Factory<T> {
  create(...args: any[]): T;
  canHandle(type: string): boolean;
}

// 具体工厂
export class HttpClientFactory implements Factory<HttpClient> {
  canHandle(type: string): boolean {
    return ['axios', 'fetch', 'xhr'].includes(type);
  }
  
  create(type: 'axios' | 'fetch' | 'xhr', config?: any): HttpClient {
    switch (type) {
      case 'axios':
        return new AxiosClient(config);
      case 'fetch':
        return new FetchClient(config);
      case 'xhr':
        return new XhrClient(config);
      default:
        throw new Error(`Unsupported client type: ${type}`);
    }
  }
}

// 工厂注册器
export class FactoryRegistry {
  private factories = new Map<string, Factory<any>>();
  
  register<T>(name: string, factory: Factory<T>): void {
    this.factories.set(name, factory);
  }
  
  create<T>(name: string, type: string, ...args: any[]): T {
    const factory = this.factories.get(name);
    if (!factory) {
      throw new Error(`Factory ${name} not found`);
    }
    
    if (!factory.canHandle(type)) {
      throw new Error(`Factory ${name} cannot handle type ${type}`);
    }
    
    return factory.create(type, ...args);
  }
}

// 使用示例
const registry = new FactoryRegistry();
registry.register('http', new HttpClientFactory());

const client = registry.create<HttpClient>('http', 'axios', {
  baseURL: 'https://api.example.com'
});
```

## 类型生成和导出

### 自动类型生成
```typescript
// type-generator.ts
import * as ts from 'typescript';
import * as fs from 'fs';

export class TypeGenerator {
  private program: ts.Program;
  private checker: ts.TypeChecker;
  
  constructor(configPath: string) {
    const config = ts.readConfigFile(configPath, ts.sys.readFile);
    const parseResult = ts.parseJsonConfigFileContent(
      config.config,
      ts.sys,
      '.'
    );
    
    this.program = ts.createProgram(
      parseResult.fileNames,
      parseResult.options
    );
    this.checker = this.program.getTypeChecker();
  }
  
  generateApiTypes(sourceFile: string): string {
    const source = this.program.getSourceFile(sourceFile);
    if (!source) {
      throw new Error(`Source file ${sourceFile} not found`);
    }
    
    const types: string[] = [];
    
    ts.forEachChild(source, node => {
      if (ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node)) {
        const type = this.checker.getTypeAtLocation(node);
        const typeString = this.checker.typeToString(type);
        types.push(`export type ${node.name.text} = ${typeString};`);
      }
    });
    
    return types.join('\n');
  }
  
  generateSchemaTypes(schema: any): string {
    // 从JSON Schema生成TypeScript类型
    const generateType = (obj: any, name: string): string => {
      if (obj.type === 'object') {
        const properties = Object.entries(obj.properties || {})
          .map(([key, value]: [string, any]) => {
            const optional = !obj.required?.includes(key) ? '?' : '';
            const type = this.mapJsonSchemaType(value);
            return `${key}${optional}: ${type}`;
          })
          .join(';\n  ');
        
        return `export interface ${name} {\n  ${properties}\n}`;
      }
      
      return `export type ${name} = ${this.mapJsonSchemaType(obj)};`;
    };
    
    return generateType(schema, 'GeneratedType');
  }
  
  private mapJsonSchemaType(schema: any): string {
    switch (schema.type) {
      case 'string':
        return schema.enum ? schema.enum.map((v: string) => `'${v}'`).join(' | ') : 'string';
      case 'number':
      case 'integer':
        return 'number';
      case 'boolean':
        return 'boolean';
      case 'array':
        return `${this.mapJsonSchemaType(schema.items)}[]`;
      case 'object':
        return 'object';
      default:
        return 'unknown';
    }
  }
}
```

### 声明文件生成
```typescript
// declaration-bundler.ts
export class DeclarationBundler {
  constructor(private options: {
    input: string;
    output: string;
    external?: string[];
  }) {}
  
  async bundle(): Promise<void> {
    const program = ts.createProgram([this.options.input], {
      declaration: true,
      emitDeclarationOnly: true,
      moduleResolution: ts.ModuleResolutionKind.NodeJs,
      module: ts.ModuleKind.ES2022,
      target: ts.ScriptTarget.ES2020
    });
    
    const declarations = new Map<string, string>();
    
    // 收集所有声明
    program.emit(undefined, (fileName, text) => {
      if (fileName.endsWith('.d.ts')) {
        declarations.set(fileName, text);
      }
    });
    
    // 合并声明
    const bundled = this.mergeDeclarations(declarations);
    
    // 写入输出文件
    fs.writeFileSync(this.options.output, bundled);
  }
  
  private mergeDeclarations(declarations: Map<string, string>): string {
    const imports = new Set<string>();
    const exports = new Set<string>();
    let content = '';
    
    for (const [fileName, text] of declarations) {
      // 解析导入导出
      const lines = text.split('\n');
      
      for (const line of lines) {
        if (line.startsWith('import ')) {
          imports.add(line);
        } else if (line.startsWith('export ')) {
          exports.add(line);
        } else if (line.trim() && !line.startsWith('//')) {
          content += line + '\n';
        }
      }
    }
    
    // 组装最终输出
    const result = [
      ...Array.from(imports),
      '',
      content,
      '',
      ...Array.from(exports)
    ].join('\n');
    
    return result;
  }
}
```

## 编译优化

### 增量编译
```typescript
// incremental-compiler.ts
export class IncrementalCompiler {
  private program: ts.SemanticDiagnosticsBuilderProgram;
  private host: ts.CompilerHost;
  
  constructor(private configPath: string) {
    this.setupCompiler();
  }
  
  private setupCompiler(): void {
    const config = ts.readConfigFile(this.configPath, ts.sys.readFile);
    const parseResult = ts.parseJsonConfigFileContent(
      config.config,
      ts.sys,
      '.'
    );
    
    this.host = ts.createIncrementalCompilerHost(parseResult.options);
    
    this.program = ts.createIncrementalProgram({
      rootNames: parseResult.fileNames,
      options: {
        ...parseResult.options,
        incremental: true,
        tsBuildInfoFile: '.tsbuildinfo'
      },
      host: this.host
    });
  }
  
  compile(): ts.Diagnostic[] {
    const emitResult = this.program.emit();
    const diagnostics = [
      ...this.program.getConfigFileParsingDiagnostics(),
      ...this.program.getSyntacticDiagnostics(),
      ...this.program.getSemanticDiagnostics(),
      ...emitResult.diagnostics
    ];
    
    return diagnostics;
  }
  
  getAffectedFiles(): string[] {
    const affectedFiles: string[] = [];
    
    while (true) {
      const result = this.program.getSemanticDiagnosticsOfNextAffectedFile();
      if (!result) break;
      
      if (result.affected.fileName) {
        affectedFiles.push(result.affected.fileName);
      }
    }
    
    return affectedFiles;
  }
  
  watchMode(callback: (diagnostics: ts.Diagnostic[]) => void): void {
    const watchProgram = ts.createWatchProgram(
      ts.createWatchCompilerHost(
        this.configPath,
        {},
        ts.sys,
        ts.createSemanticDiagnosticsBuilderProgram,
        (diagnostic) => callback([diagnostic]),
        (diagnostic) => callback([diagnostic])
      )
    );
  }
}
```

### 并行类型检查
```typescript
// parallel-type-checker.ts
import { Worker, isMainThread, parentPort, workerData } from 'worker_threads';

if (!isMainThread) {
  // Worker线程
  const { files, options } = workerData;
  
  const program = ts.createProgram(files, options);
  const diagnostics = ts.getPreEmitDiagnostics(program);
  
  parentPort?.postMessage({
    diagnostics: diagnostics.map(d => ({
      file: d.file?.fileName,
      start: d.start,
      length: d.length,
      messageText: d.messageText,
      category: d.category,
      code: d.code
    }))
  });
} else {
  // 主线程
  export class ParallelTypeChecker {
    async checkFiles(files: string[], options: ts.CompilerOptions): Promise<ts.Diagnostic[]> {
      const chunkSize = Math.ceil(files.length / 4);
      const chunks = [];
      
      for (let i = 0; i < files.length; i += chunkSize) {
        chunks.push(files.slice(i, i + chunkSize));
      }
      
      const workers = chunks.map(chunk => 
        new Worker(__filename, {
          workerData: { files: chunk, options }
        })
      );
      
      const results = await Promise.all(
        workers.map(worker => 
          new Promise<{ diagnostics: any[] }>((resolve, reject) => {
            worker.on('message', resolve);
            worker.on('error', reject);
          })
        )
      );
      
      // 清理workers
      workers.forEach(worker => worker.terminate());
      
      // 合并结果
      return results.flatMap(result => result.diagnostics);
    }
  }
}
```

## 构建工具集成

### Webpack集成
```typescript
// webpack.config.ts
import type { Configuration } from 'webpack';
import ForkTsCheckerWebpackPlugin from 'fork-ts-checker-webpack-plugin';

const config: Configuration = {
  entry: './src/index.ts',
  
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: 'ts-loader',
            options: {
              // 只进行转译，类型检查交给ForkTsCheckerWebpackPlugin
              transpileOnly: true,
              
              // 编译选项覆盖
              compilerOptions: {
                module: 'esnext',
                target: 'es2020'
              }
            }
          }
        ],
        exclude: /node_modules/
      }
    ]
  },
  
  plugins: [
    new ForkTsCheckerWebpackPlugin({
      typescript: {
        configFile: 'tsconfig.json',
        diagnosticOptions: {
          semantic: true,
          syntactic: true
        }
      },
      
      // ESLint集成
      eslint: {
        files: './src/**/*.{ts,tsx,js,jsx}'
      }
    })
  ],
  
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    alias: {
      '@': path.resolve(__dirname, 'src')
    }
  }
};

export default config;
```

### Vite集成
```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import typescript from '@rollup/plugin-typescript';

export default defineConfig({
  plugins: [
    typescript({
      tsconfig: './tsconfig.json',
      
      // 类型检查选项
      check: true,
      
      // 声明文件生成
      declaration: true,
      declarationDir: 'dist/types',
      
      // 排除测试文件
      exclude: ['**/*.test.ts', '**/*.spec.ts']
    })
  ],
  
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'MyLib',
      formats: ['es', 'cjs', 'umd']
    },
    
    rollupOptions: {
      external: ['vue', 'react'],
      output: {
        globals: {
          vue: 'Vue',
          react: 'React'
        }
      }
    }
  },
  
  // 类型检查脚本
  define: {
    __VERSION__: JSON.stringify(process.env.npm_package_version)
  }
});
```

### esbuild集成
```typescript
// esbuild.config.ts
import { build } from 'esbuild';
import { promises as fs } from 'fs';

async function buildWithTypes() {
  // JavaScript构建
  await build({
    entryPoints: ['src/index.ts'],
    bundle: true,
    outfile: 'dist/index.js',
    format: 'esm',
    target: 'es2020',
    
    // TypeScript支持
    loader: {
      '.ts': 'ts',
      '.tsx': 'tsx'
    },
    
    // 外部依赖
    external: ['react', 'react-dom']
  });
  
  // 单独的类型生成
  const tsc = spawn('tsc', [
    '--declaration',
    '--emitDeclarationOnly',
    '--outDir',
    'dist/types'
  ]);
  
  await new Promise((resolve, reject) => {
    tsc.on('close', (code) => {
      if (code === 0) resolve(void 0);
      else reject(new Error(`tsc exited with code ${code}`));
    });
  });
}

buildWithTypes().catch(console.error);
```

## 最佳实践

### 模块组织策略
```typescript
// 功能模块结构
// src/modules/user/
//   ├── types.ts          // 类型定义
//   ├── service.ts        // 业务逻辑
//   ├── api.ts           // API调用
//   ├── store.ts         // 状态管理
//   └── index.ts         // 模块导出

// types.ts
export interface User {
  id: number;
  name: string;
  email: string;
  roles: Role[];
}

export interface CreateUserRequest {
  name: string;
  email: string;
  password: string;
}

export type UserRole = 'admin' | 'user' | 'guest';

// service.ts
import type { User, CreateUserRequest } from './types';
import { userApi } from './api';

export class UserService {
  async getUser(id: number): Promise<User> {
    return userApi.get(id);
  }
  
  async createUser(data: CreateUserRequest): Promise<User> {
    return userApi.create(data);
  }
}

export const userService = new UserService();

// index.ts - 统一导出
export type * from './types';
export { UserService, userService } from './service';
export { userApi } from './api';
```

### 类型安全的配置
```typescript
// config.ts
interface DatabaseConfig {
  host: string;
  port: number;
  username: string;
  password: string;
  database: string;
}

interface ApiConfig {
  baseURL: string;
  timeout: number;
  retries: number;
}

interface AppConfig {
  database: DatabaseConfig;
  api: ApiConfig;
  features: {
    [K in FeatureFlag]: boolean;
  };
}

type FeatureFlag = 'userManagement' | 'analytics' | 'notifications';

// 配置验证
export function validateConfig(config: unknown): AppConfig {
  // 运行时类型检查
  if (!isObject(config)) {
    throw new Error('Config must be an object');
  }
  
  // 详细验证逻辑...
  return config as AppConfig;
}

// 环境特定配置
export const config: AppConfig = validateConfig({
  database: {
    host: process.env.DB_HOST || 'localhost',
    port: parseInt(process.env.DB_PORT || '5432'),
    username: process.env.DB_USER || 'postgres',
    password: process.env.DB_PASS || '',
    database: process.env.DB_NAME || 'app'
  },
  api: {
    baseURL: process.env.API_URL || 'http://localhost:3000',
    timeout: 5000,
    retries: 3
  },
  features: {
    userManagement: true,
    analytics: process.env.NODE_ENV === 'production',
    notifications: true
  }
});
```

### 依赖注入模式
```typescript
// di-container.ts
type Constructor<T = {}> = new (...args: any[]) => T;
type Token<T> = Constructor<T> | string | symbol;

export class DIContainer {
  private services = new Map<Token<any>, any>();
  private singletons = new Map<Token<any>, any>();
  
  register<T>(token: Token<T>, implementation: Constructor<T>): void {
    this.services.set(token, implementation);
  }
  
  registerSingleton<T>(token: Token<T>, implementation: Constructor<T>): void {
    this.services.set(token, implementation);
    this.singletons.set(token, null);
  }
  
  resolve<T>(token: Token<T>): T {
    if (this.singletons.has(token)) {
      let instance = this.singletons.get(token);
      if (!instance) {
        instance = this.createInstance(token);
        this.singletons.set(token, instance);
      }
      return instance;
    }
    
    return this.createInstance(token);
  }
  
  private createInstance<T>(token: Token<T>): T {
    const implementation = this.services.get(token);
    if (!implementation) {
      throw new Error(`Service ${String(token)} not found`);
    }
    
    // 获取构造函数参数类型
    const dependencies = this.getDependencies(implementation);
    const resolvedDependencies = dependencies.map(dep => this.resolve(dep));
    
    return new implementation(...resolvedDependencies);
  }
  
  private getDependencies(constructor: Constructor): Token<any>[] {
    // 使用reflect-metadata获取依赖
    return Reflect.getMetadata('design:paramtypes', constructor) || [];
  }
}

// 使用装饰器简化注入
export function Injectable<T extends Constructor>(constructor: T) {
  return constructor;
}

export function Inject(token: Token<any>) {
  return function (target: any, propertyKey: string | symbol | undefined, parameterIndex: number) {
    const existingTokens = Reflect.getMetadata('design:paramtypes', target) || [];
    existingTokens[parameterIndex] = token;
    Reflect.defineMetadata('design:paramtypes', existingTokens, target);
  };
}

// 使用示例
@Injectable
export class UserService {
  constructor(
    @Inject('DATABASE') private db: Database,
    @Inject('LOGGER') private logger: Logger
  ) {}
}
```

TypeScript为JavaScript模块化开发带来了强大的类型系统支持，不仅提高了代码质量和开发效率，还为大型项目的模块化架构提供了坚实的基础。掌握TypeScript的模块系统和最佳实践，是现代前端开发的必备技能。

---

**下一章**: [运行环境差异](../runtime/browser.md) →