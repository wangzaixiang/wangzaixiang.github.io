# 性能优化

在现代Web应用开发中，模块的性能优化对用户体验至关重要。本章将深入探讨JavaScript模块系统的各种性能优化策略。

## 代码分割 (Code Splitting)

代码分割是最重要的性能优化技术之一，它允许我们将应用拆分成多个较小的包，按需加载。

### 动态导入实现代码分割

```javascript
// 路由级别的代码分割
const routes = [
  {
    path: '/dashboard',
    component: () => import('./views/Dashboard.vue')
  },
  {
    path: '/profile',
    component: () => import('./views/Profile.vue')
  }
];

// 功能级别的代码分割
async function loadImageEditor() {
  const { ImageEditor } = await import('./components/ImageEditor.js');
  return new ImageEditor();
}

// 条件加载
if (window.innerWidth > 768) {
  const { DesktopLayout } = await import('./layouts/DesktopLayout.js');
  app.use(DesktopLayout);
} else {
  const { MobileLayout } = await import('./layouts/MobileLayout.js');
  app.use(MobileLayout);
}
```

### Bundle Splitting策略

```javascript
// webpack.config.js
module.exports = {
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        // 第三方库单独打包
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
        // 公共代码提取
        common: {
          name: 'common',
          minChunks: 2,
          chunks: 'all',
          enforce: true
        }
      }
    }
  }
};

// Vite配置
export default {
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          utils: ['lodash', 'moment'],
          ui: ['antd', '@mui/material']
        }
      }
    }
  }
};
```

## 懒加载与预加载

### 模块懒加载模式

```javascript
// 懒加载管理器
class LazyModuleLoader {
  constructor() {
    this.loadedModules = new Map();
    this.loadingPromises = new Map();
  }

  async load(moduleName, importFn) {
    // 如果已加载，直接返回
    if (this.loadedModules.has(moduleName)) {
      return this.loadedModules.get(moduleName);
    }

    // 如果正在加载，返回加载Promise
    if (this.loadingPromises.has(moduleName)) {
      return this.loadingPromises.get(moduleName);
    }

    // 开始加载
    const loadPromise = importFn().then(module => {
      this.loadedModules.set(moduleName, module);
      this.loadingPromises.delete(moduleName);
      return module;
    });

    this.loadingPromises.set(moduleName, loadPromise);
    return loadPromise;
  }
}

// 使用示例
const loader = new LazyModuleLoader();

async function initChart() {
  const module = await loader.load('chart', () => import('./chart/Chart.js'));
  return new module.Chart();
}
```

### 智能预加载

```javascript
// 预加载策略
class PreloadManager {
  constructor() {
    this.preloadedModules = new Set();
    this.observer = new IntersectionObserver(this.handleIntersection.bind(this));
  }

  // 预加载关键模块
  preloadCritical() {
    this.preload(() => import('./core/UserManager.js'));
    this.preload(() => import('./core/Router.js'));
  }

  // 根据用户行为预加载
  preloadOnHover(element, importFn) {
    element.addEventListener('mouseenter', () => {
      this.preload(importFn);
    }, { once: true });
  }

  // 视口预加载
  preloadOnVisible(element, importFn) {
    element.dataset.preload = importFn.toString();
    this.observer.observe(element);
  }

  async preload(importFn) {
    const moduleKey = importFn.toString();
    if (this.preloadedModules.has(moduleKey)) return;

    this.preloadedModules.add(moduleKey);
    
    // 使用link预加载
    const link = document.createElement('link');
    link.rel = 'modulepreload';
    link.href = this.getModuleURL(importFn);
    document.head.appendChild(link);
  }

  handleIntersection(entries) {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const importFn = new Function('return ' + entry.target.dataset.preload)();
        this.preload(importFn);
        this.observer.unobserve(entry.target);
      }
    });
  }
}
```

## Tree Shaking优化

### ES模块Tree Shaking最佳实践

```javascript
// 错误的导入方式 - 会导入整个库
import * as _ from 'lodash';
import { Button } from '@mui/material';

// 正确的导入方式 - 支持Tree Shaking
import { debounce } from 'lodash-es';
import Button from '@mui/material/Button';

// utils.js - 确保可被Tree Shaking
export const formatDate = (date) => {
  return new Intl.DateTimeFormat('zh-CN').format(date);
};

export const formatCurrency = (amount) => {
  return new Intl.NumberFormat('zh-CN', {
    style: 'currency',
    currency: 'CNY'
  }).format(amount);
};

// 避免副作用
export const API_BASE_URL = 'https://api.example.com';

// 而不是
console.log('API module loaded'); // 这会阻止Tree Shaking
export const API_BASE_URL = 'https://api.example.com';
```

### 库的Tree Shaking友好设计

```javascript
// package.json
{
  "name": "my-utils",
  "main": "dist/index.js",
  "module": "dist/index.esm.js",
  "sideEffects": false,
  "exports": {
    ".": {
      "import": "./dist/index.esm.js",
      "require": "./dist/index.js"
    },
    "./string": {
      "import": "./dist/string.esm.js",
      "require": "./dist/string.js"
    }
  }
}

// 模块化导出结构
// src/index.js
export { default as formatDate } from './date/formatDate.js';
export { default as formatCurrency } from './currency/formatCurrency.js';
export { default as debounce } from './function/debounce.js';
export { default as throttle } from './function/throttle.js';

// 分类导出
// src/string/index.js
export { default as capitalize } from './capitalize.js';
export { default as slugify } from './slugify.js';
```

## 缓存策略

### 模块缓存优化

```javascript
// 浏览器缓存策略
class ModuleCache {
  constructor() {
    this.cache = new Map();
    this.version = '1.0.0';
  }

  async loadWithCache(moduleId, importFn) {
    const cacheKey = `${moduleId}_${this.version}`;
    
    // 检查内存缓存
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey);
    }

    // 检查localStorage缓存
    const cached = this.getFromLocalStorage(cacheKey);
    if (cached) {
      this.cache.set(cacheKey, cached);
      return cached;
    }

    // 加载并缓存
    const module = await importFn();
    this.cache.set(cacheKey, module);
    this.saveToLocalStorage(cacheKey, module);
    
    return module;
  }

  getFromLocalStorage(key) {
    try {
      const item = localStorage.getItem(key);
      return item ? JSON.parse(item) : null;
    } catch {
      return null;
    }
  }

  saveToLocalStorage(key, data) {
    try {
      localStorage.setItem(key, JSON.stringify(data));
    } catch {
      // 存储失败时的降级处理
    }
  }
}
```

### HTTP缓存配置

```javascript
// webpack配置 - 文件名哈希
module.exports = {
  output: {
    filename: '[name].[contenthash].js',
    chunkFilename: '[name].[contenthash].chunk.js'
  }
};

// nginx配置示例
// 对于带哈希的文件，设置长期缓存
location ~* \.(js|css)$ {
  if ($uri ~ ".*\.[a-f0-9]{8,}\.(js|css)$") {
    expires 1y;
    add_header Cache-Control "public, immutable";
  }
  
  # 对于不带哈希的文件，设置短期缓存
  expires 1h;
  add_header Cache-Control "public";
}
```

## 打包体积优化

### 依赖分析与优化

```javascript
// webpack-bundle-analyzer使用
const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;

module.exports = {
  plugins: [
    new BundleAnalyzerPlugin({
      analyzerMode: 'static',
      openAnalyzer: false,
      reportFilename: 'bundle-report.html'
    })
  ]
};

// 替换大型依赖
// 使用date-fns替代moment.js
import { format, parseISO } from 'date-fns';

// 使用原生API替代lodash
const unique = array => [...new Set(array)];
const groupBy = (array, key) => 
  array.reduce((groups, item) => {
    const group = item[key];
    groups[group] = groups[group] || [];
    groups[group].push(item);
    return groups;
  }, {});
```

### 压缩与混淆

```javascript
// Terser配置
const TerserPlugin = require('terser-webpack-plugin');

module.exports = {
  optimization: {
    minimizer: [
      new TerserPlugin({
        terserOptions: {
          compress: {
            drop_console: true,
            drop_debugger: true,
            pure_funcs: ['console.log', 'console.info']
          },
          mangle: {
            safari10: true
          }
        }
      })
    ]
  }
};

// 移除开发代码
if (process.env.NODE_ENV === 'production') {
  // 生产环境代码
} else {
  // 开发环境代码，会被打包工具移除
}
```

## 运行时性能优化

### 模块初始化优化

```javascript
// 延迟初始化
class PerformanceModule {
  constructor() {
    this._heavyResource = null;
    this._initialized = false;
  }

  async getHeavyResource() {
    if (!this._initialized) {
      await this.initialize();
    }
    return this._heavyResource;
  }

  async initialize() {
    if (this._initialized) return;
    
    // 只在需要时初始化重资源
    this._heavyResource = await this.createHeavyResource();
    this._initialized = true;
  }

  async createHeavyResource() {
    // 模拟重量级资源创建
    const { HeavyLibrary } = await import('./HeavyLibrary.js');
    return new HeavyLibrary();
  }
}

// 单例模式确保只初始化一次
const performanceModule = new PerformanceModule();
export default performanceModule;
```

### 内存管理

```javascript
// 模块级别的内存管理
class ModuleMemoryManager {
  constructor() {
    this.modules = new WeakMap();
    this.timers = new Set();
    this.listeners = new Map();
  }

  register(module, cleanup) {
    this.modules.set(module, cleanup);
  }

  addTimer(timerId) {
    this.timers.add(timerId);
  }

  addListener(element, event, handler) {
    const key = { element, event };
    this.listeners.set(key, handler);
    element.addEventListener(event, handler);
  }

  cleanup() {
    // 清理定时器
    this.timers.forEach(timerId => clearTimeout(timerId));
    this.timers.clear();

    // 清理事件监听器
    this.listeners.forEach((handler, { element, event }) => {
      element.removeEventListener(event, handler);
    });
    this.listeners.clear();

    // 调用模块清理函数
    this.modules.forEach(cleanup => cleanup());
  }
}

// 模块使用示例
const memoryManager = new ModuleMemoryManager();

export function initialize() {
  const timer = setTimeout(() => {
    // 定时器逻辑
  }, 1000);
  
  memoryManager.addTimer(timer);
  
  const handler = () => console.log('clicked');
  memoryManager.addListener(document.body, 'click', handler);
}

export function destroy() {
  memoryManager.cleanup();
}
```

## 网络优化

### 模块预加载策略

```html
<!-- HTML中的预加载 -->
<link rel="modulepreload" href="/modules/critical.js">
<link rel="preload" href="/modules/important.js" as="script" crossorigin>

<!-- 动态预加载 -->
<script>
const preloadModule = (href) => {
  const link = document.createElement('link');
  link.rel = 'modulepreload';
  link.href = href;
  document.head.appendChild(link);
};

// 预加载下一个可能需要的模块
preloadModule('/modules/next-page.js');
</script>
```

### CDN优化

```javascript
// 配置CDN加载
const CDN_BASE = 'https://cdn.example.com';

export async function loadFromCDN(moduleName, version = 'latest') {
  const url = `${CDN_BASE}/${moduleName}@${version}/index.js`;
  
  try {
    return await import(url);
  } catch (error) {
    // CDN失败时的降级方案
    console.warn(`CDN load failed for ${moduleName}, falling back to local`);
    return await import(`./local-modules/${moduleName}.js`);
  }
}

// 版本控制
const MODULE_VERSIONS = {
  'chart-lib': '2.1.0',
  'ui-components': '1.5.2'
};

export function getModuleURL(moduleName) {
  const version = MODULE_VERSIONS[moduleName];
  return `${CDN_BASE}/${moduleName}@${version}/index.js`;
}
```

## 性能监控

### 模块加载性能监控

```javascript
class ModulePerformanceMonitor {
  constructor() {
    this.metrics = new Map();
  }

  startLoad(moduleName) {
    this.metrics.set(moduleName, {
      startTime: performance.now(),
      loadTime: null,
      size: null
    });
  }

  endLoad(moduleName, moduleSize) {
    const metric = this.metrics.get(moduleName);
    if (metric) {
      metric.loadTime = performance.now() - metric.startTime;
      metric.size = moduleSize;
    }
  }

  getMetrics() {
    return Array.from(this.metrics.entries()).map(([name, data]) => ({
      name,
      loadTime: data.loadTime,
      size: data.size,
      speed: data.size ? data.size / data.loadTime : null
    }));
  }

  reportMetrics() {
    const metrics = this.getMetrics();
    
    // 发送到分析服务
    if (typeof analytics !== 'undefined') {
      analytics.track('Module Performance', {
        modules: metrics,
        totalModules: metrics.length,
        averageLoadTime: metrics.reduce((sum, m) => sum + m.loadTime, 0) / metrics.length
      });
    }
    
    console.table(metrics);
  }
}

// 使用示例
const monitor = new ModulePerformanceMonitor();

export async function loadModule(moduleName, importFn) {
  monitor.startLoad(moduleName);
  
  try {
    const module = await importFn();
    
    // 估算模块大小（简化版本）
    const moduleSize = JSON.stringify(module).length;
    monitor.endLoad(moduleName, moduleSize);
    
    return module;
  } catch (error) {
    console.error(`Failed to load module ${moduleName}:`, error);
    throw error;
  }
}

// 页面卸载时报告性能数据
window.addEventListener('beforeunload', () => {
  monitor.reportMetrics();
});
```

通过实施这些性能优化策略，可以显著提升JavaScript应用的加载速度和运行性能，为用户提供更好的体验。
