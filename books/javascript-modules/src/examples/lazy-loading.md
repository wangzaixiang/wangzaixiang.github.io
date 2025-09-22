# 模块懒加载实现

本章将深入探讨JavaScript模块懒加载的实现方案，包括代码分割、动态导入、预加载策略和性能优化技巧。

## 懒加载概述

### 什么是懒加载

懒加载（Lazy Loading）是一种优化策略，将资源的加载推迟到实际需要时才进行。在模块化开发中，懒加载可以：

- 减少初始加载时间
- 降低内存占用
- 按需加载功能模块
- 改善用户体验

### 懒加载的应用场景

1. **路由级别**：页面路由按需加载
2. **组件级别**：复杂组件延迟加载
3. **功能模块**：可选功能的动态加载
4. **第三方库**：大型依赖的延迟引入

## 基础实现方案

### 1. 动态导入基础

```javascript
// 传统的静态导入
import { heavyFunction } from './heavy-module.js';

// 动态导入 - 返回Promise
const loadHeavyModule = async () => {
  const { heavyFunction } = await import('./heavy-module.js');
  return heavyFunction;
};

// 使用示例
async function handleClick() {
  const heavyFunction = await loadHeavyModule();
  const result = heavyFunction(data);
  console.log(result);
}
```

### 2. 条件懒加载

```javascript
// 基于条件的懒加载
class FeatureManager {
  private loadedModules = new Map();

  async loadFeature(featureName: string, condition: boolean) {
    if (!condition) return null;
    
    if (this.loadedModules.has(featureName)) {
      return this.loadedModules.get(featureName);
    }

    let module;
    switch (featureName) {
      case 'charts':
        module = await import('./features/charts.js');
        break;
      case 'editor':
        module = await import('./features/editor.js');
        break;
      case 'analytics':
        module = await import('./features/analytics.js');
        break;
      default:
        throw new Error(`Unknown feature: ${featureName}`);
    }

    this.loadedModules.set(featureName, module);
    return module;
  }
}

// 使用示例
const featureManager = new FeatureManager();

// 基于用户权限加载功能
if (user.hasPermission('analytics')) {
  const analytics = await featureManager.loadFeature('analytics', true);
  analytics.initialize();
}
```

## React中的懒加载

### 1. 组件懒加载

```jsx
// React.lazy 基础用法
import React, { Suspense, lazy } from 'react';

// 懒加载组件
const LazyComponent = lazy(() => import('./HeavyComponent'));
const LazyModal = lazy(() => import('./Modal'));

function App() {
  const [showModal, setShowModal] = useState(false);

  return (
    <div>
      <Suspense fallback={<div>Loading component...</div>}>
        <LazyComponent />
      </Suspense>
      
      {showModal && (
        <Suspense fallback={<div>Loading modal...</div>}>
          <LazyModal onClose={() => setShowModal(false)} />
        </Suspense>
      )}
    </div>
  );
}
```

### 2. 高级懒加载Hook

```typescript
// useLazyComponent Hook
import { useState, useCallback, ComponentType } from 'react';

type LazyComponentLoader<T = {}> = () => Promise<{ default: ComponentType<T> }>;

interface UseLazyComponentReturn<T> {
  LazyComponent: ComponentType<T> | null;
  loading: boolean;
  error: Error | null;
  loadComponent: () => Promise<void>;
}

function useLazyComponent<T = {}>(
  loader: LazyComponentLoader<T>
): UseLazyComponentReturn<T> {
  const [LazyComponent, setLazyComponent] = useState<ComponentType<T> | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const loadComponent = useCallback(async () => {
    if (LazyComponent) return; // 已加载

    setLoading(true);
    setError(null);

    try {
      const module = await loader();
      setLazyComponent(() => module.default);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [loader, LazyComponent]);

  return { LazyComponent, loading, error, loadComponent };
}

// 使用示例
function FeaturePage() {
  const { 
    LazyComponent: ChartComponent, 
    loading, 
    error, 
    loadComponent 
  } = useLazyComponent(() => import('./ChartComponent'));

  return (
    <div>
      <button onClick={loadComponent}>
        Load Chart Component
      </button>
      
      {loading && <div>Loading chart...</div>}
      {error && <div>Error loading chart: {error.message}</div>}
      {ChartComponent && <ChartComponent data={data} />}
    </div>
  );
}
```

### 3. 路由懒加载

```jsx
// React Router懒加载
import { Routes, Route } from 'react-router-dom';
import { lazy, Suspense } from 'react';

// 懒加载页面组件
const HomePage = lazy(() => import('./pages/HomePage'));
const AboutPage = lazy(() => import('./pages/AboutPage'));
const ProfilePage = lazy(() => import('./pages/ProfilePage'));
const AdminPage = lazy(() => import('./pages/AdminPage'));

// 加载提示组件
function PageLoader() {
  return (
    <div className="page-loader">
      <div className="spinner" />
      <p>Loading page...</p>
    </div>
  );
}

function AppRouter() {
  return (
    <Suspense fallback={<PageLoader />}>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/about" element={<AboutPage />} />
        <Route path="/profile" element={<ProfilePage />} />
        <Route path="/admin" element={<AdminPage />} />
      </Routes>
    </Suspense>
  );
}
```

## 高级懒加载策略

### 1. 预加载策略

```typescript
// 智能预加载管理器
class PreloadManager {
  private preloadCache = new Map<string, Promise<any>>();
  private priorityQueue: string[] = [];

  // 预加载资源
  preload(modulePath: string, priority: 'high' | 'low' = 'low'): Promise<any> {
    if (this.preloadCache.has(modulePath)) {
      return this.preloadCache.get(modulePath)!;
    }

    const promise = import(modulePath);
    this.preloadCache.set(modulePath, promise);

    if (priority === 'high') {
      this.priorityQueue.unshift(modulePath);
    } else {
      this.priorityQueue.push(modulePath);
    }

    return promise;
  }

  // 在空闲时间预加载
  preloadOnIdle(modules: string[]): void {
    if ('requestIdleCallback' in window) {
      window.requestIdleCallback(() => {
        modules.forEach(module => this.preload(module, 'low'));
      });
    } else {
      // 降级方案
      setTimeout(() => {
        modules.forEach(module => this.preload(module, 'low'));
      }, 100);
    }
  }

  // 基于用户交互预加载
  preloadOnHover(element: HTMLElement, modulePath: string): void {
    element.addEventListener('mouseenter', () => {
      this.preload(modulePath, 'high');
    }, { once: true });
  }

  // 基于网络状态的智能预加载
  smartPreload(modules: string[]): void {
    if ('connection' in navigator) {
      const connection = (navigator as any).connection;
      
      // 快速网络连接时预加载更多
      if (connection.effectiveType === '4g') {
        modules.forEach(module => this.preload(module));
      } else if (connection.effectiveType === '3g') {
        // 3G网络只预加载高优先级模块
        modules.slice(0, 2).forEach(module => this.preload(module));
      }
    }
  }
}

// 使用示例
const preloadManager = new PreloadManager();

// 页面加载后预加载下一可能访问的页面
useEffect(() => {
  preloadManager.preloadOnIdle([
    './pages/ProfilePage',
    './pages/SettingsPage'
  ]);
}, []);

// 悬停时预加载
useEffect(() => {
  const profileLink = document.querySelector('[data-route="/profile"]');
  if (profileLink) {
    preloadManager.preloadOnHover(profileLink, './pages/ProfilePage');
  }
}, []);
```

### 2. 渐进式加载

```typescript
// 渐进式模块加载器
class ProgressiveLoader {
  async loadModuleProgressively<T>(
    modulePath: string,
    onProgress?: (progress: number) => void
  ): Promise<T> {
    // 模拟加载进度（实际中可能需要自定义加载器）
    const steps = [
      { progress: 0.2, message: 'Downloading module...' },
      { progress: 0.5, message: 'Parsing module...' },
      { progress: 0.8, message: 'Initializing...' },
      { progress: 1.0, message: 'Complete!' }
    ];

    for (const step of steps) {
      await this.delay(100); // 模拟加载时间
      onProgress?.(step.progress);
    }

    return import(modulePath);
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// 使用示例
function ProgressiveLoadingComponent() {
  const [progress, setProgress] = useState(0);
  const [module, setModule] = useState(null);

  const loadModule = async () => {
    const loader = new ProgressiveLoader();
    const loadedModule = await loader.loadModuleProgressively(
      './heavy-module.js',
      setProgress
    );
    setModule(loadedModule);
  };

  return (
    <div>
      {progress < 1 && (
        <div className="progress-bar">
          <div 
            className="progress-fill" 
            style={{ width: `${progress * 100}%` }}
          />
          <span>{Math.round(progress * 100)}%</span>
        </div>
      )}
      {module && <div>Module loaded successfully!</div>}
    </div>
  );
}
```

### 3. 缓存优化

```typescript
// 模块缓存管理器
class ModuleCacheManager {
  private cache = new Map<string, any>();
  private metadata = new Map<string, { 
    lastAccessed: number; 
    accessCount: number; 
    size: number 
  }>();

  async getModule<T>(modulePath: string): Promise<T> {
    // 检查缓存
    if (this.cache.has(modulePath)) {
      this.updateMetadata(modulePath);
      return this.cache.get(modulePath);
    }

    // 加载模块
    const module = await import(modulePath);
    
    // 缓存模块
    this.cache.set(modulePath, module);
    this.metadata.set(modulePath, {
      lastAccessed: Date.now(),
      accessCount: 1,
      size: this.estimateSize(module)
    });

    // 检查缓存大小限制
    this.evictIfNecessary();

    return module;
  }

  private updateMetadata(modulePath: string): void {
    const meta = this.metadata.get(modulePath);
    if (meta) {
      meta.lastAccessed = Date.now();
      meta.accessCount++;
    }
  }

  private estimateSize(module: any): number {
    // 简单的大小估算
    return JSON.stringify(module).length;
  }

  private evictIfNecessary(): void {
    const maxCacheSize = 10 * 1024 * 1024; // 10MB
    let currentSize = 0;

    for (const [, meta] of this.metadata) {
      currentSize += meta.size;
    }

    if (currentSize > maxCacheSize) {
      // LRU淘汰策略
      const entries = Array.from(this.metadata.entries())
        .sort(([, a], [, b]) => a.lastAccessed - b.lastAccessed);

      const toEvict = entries.slice(0, Math.ceil(entries.length * 0.2));
      for (const [modulePath] of toEvict) {
        this.cache.delete(modulePath);
        this.metadata.delete(modulePath);
      }
    }
  }

  // 预热缓存
  async warmUp(modulePaths: string[]): Promise<void> {
    const promises = modulePaths.map(path => this.getModule(path));
    await Promise.allSettled(promises);
  }

  // 清理缓存
  clear(): void {
    this.cache.clear();
    this.metadata.clear();
  }
}

// 全局缓存实例
export const moduleCache = new ModuleCacheManager();
```

## 性能监控与优化

### 1. 加载性能监控

```typescript
// 模块加载性能监控
class ModulePerformanceMonitor {
  private metrics = new Map<string, {
    loadTime: number;
    loadCount: number;
    errors: number;
    averageLoadTime: number;
  }>();

  async monitorLoad<T>(modulePath: string, loader: () => Promise<T>): Promise<T> {
    const startTime = performance.now();
    
    try {
      const result = await loader();
      const loadTime = performance.now() - startTime;
      
      this.recordSuccess(modulePath, loadTime);
      return result;
    } catch (error) {
      this.recordError(modulePath);
      throw error;
    }
  }

  private recordSuccess(modulePath: string, loadTime: number): void {
    const existing = this.metrics.get(modulePath);
    
    if (existing) {
      existing.loadCount++;
      existing.averageLoadTime = (existing.averageLoadTime * (existing.loadCount - 1) + loadTime) / existing.loadCount;
    } else {
      this.metrics.set(modulePath, {
        loadTime,
        loadCount: 1,
        errors: 0,
        averageLoadTime: loadTime
      });
    }
  }

  private recordError(modulePath: string): void {
    const existing = this.metrics.get(modulePath);
    if (existing) {
      existing.errors++;
    } else {
      this.metrics.set(modulePath, {
        loadTime: 0,
        loadCount: 0,
        errors: 1,
        averageLoadTime: 0
      });
    }
  }

  getReport(): Record<string, any> {
    const report: Record<string, any> = {};
    
    for (const [modulePath, metrics] of this.metrics) {
      report[modulePath] = {
        ...metrics,
        successRate: metrics.loadCount / (metrics.loadCount + metrics.errors)
      };
    }
    
    return report;
  }

  // 找出性能瓶颈
  getSlowModules(threshold: number = 1000): string[] {
    return Array.from(this.metrics.entries())
      .filter(([, metrics]) => metrics.averageLoadTime > threshold)
      .map(([modulePath]) => modulePath);
  }
}

// 使用示例
const monitor = new ModulePerformanceMonitor();

async function loadModuleWithMonitoring(modulePath: string) {
  return monitor.monitorLoad(modulePath, () => import(modulePath));
}
```

### 2. 自适应加载策略

```typescript
// 自适应模块加载器
class AdaptiveModuleLoader {
  private networkSpeed: 'slow' | 'medium' | 'fast' = 'medium';
  private userEngagement: 'low' | 'medium' | 'high' = 'medium';

  constructor() {
    this.detectNetworkSpeed();
    this.trackUserEngagement();
  }

  async loadModule<T>(
    modulePath: string, 
    options: {
      priority?: 'low' | 'medium' | 'high';
      fallback?: () => Promise<T>;
    } = {}
  ): Promise<T> {
    const strategy = this.getLoadingStrategy(options.priority);
    
    try {
      switch (strategy) {
        case 'immediate':
          return await import(modulePath);
          
        case 'deferred':
          await this.delay(500); // 延迟加载
          return await import(modulePath);
          
        case 'conditional':
          if (this.shouldLoadModule()) {
            return await import(modulePath);
          } else if (options.fallback) {
            return await options.fallback();
          }
          throw new Error('Module loading skipped');
          
        default:
          return await import(modulePath);
      }
    } catch (error) {
      if (options.fallback) {
        return await options.fallback();
      }
      throw error;
    }
  }

  private getLoadingStrategy(priority?: 'low' | 'medium' | 'high'): 'immediate' | 'deferred' | 'conditional' {
    // 高优先级模块立即加载
    if (priority === 'high') return 'immediate';
    
    // 慢网络下延迟加载低优先级模块
    if (this.networkSpeed === 'slow' && priority === 'low') {
      return 'conditional';
    }
    
    // 低参与度用户延迟加载
    if (this.userEngagement === 'low') {
      return 'deferred';
    }
    
    return 'immediate';
  }

  private detectNetworkSpeed(): void {
    if ('connection' in navigator) {
      const connection = (navigator as any).connection;
      const effectiveType = connection.effectiveType;
      
      if (effectiveType === '4g') {
        this.networkSpeed = 'fast';
      } else if (effectiveType === '3g') {
        this.networkSpeed = 'medium';
      } else {
        this.networkSpeed = 'slow';
      }
    }
  }

  private trackUserEngagement(): void {
    let interactionCount = 0;
    const startTime = Date.now();

    ['click', 'keydown', 'scroll'].forEach(event => {
      document.addEventListener(event, () => {
        interactionCount++;
      });
    });

    // 5秒后评估用户参与度
    setTimeout(() => {
      const timeSpent = Date.now() - startTime;
      const engagementScore = interactionCount / (timeSpent / 1000);
      
      if (engagementScore > 2) {
        this.userEngagement = 'high';
      } else if (engagementScore > 0.5) {
        this.userEngagement = 'medium';
      } else {
        this.userEngagement = 'low';
      }
    }, 5000);
  }

  private shouldLoadModule(): boolean {
    return this.networkSpeed !== 'slow' && this.userEngagement !== 'low';
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// 使用示例
const adaptiveLoader = new AdaptiveModuleLoader();

// 高优先级模块（立即加载）
const criticalModule = await adaptiveLoader.loadModule('./critical-module.js', {
  priority: 'high'
});

// 低优先级模块（可能延迟或跳过）
const optionalModule = await adaptiveLoader.loadModule('./optional-module.js', {
  priority: 'low',
  fallback: () => Promise.resolve({ default: () => 'Fallback content' })
});
```

## 构建工具集成

### 1. Webpack代码分割

```javascript
// webpack.config.js
module.exports = {
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        // 供应商代码分割
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
        // 异步代码分割
        async: {
          chunks: 'async',
          minSize: 30000,
          maxSize: 244000,
        },
        // 公共代码分割
        common: {
          name: 'common',
          minChunks: 2,
          chunks: 'all',
          enforce: true
        }
      }
    }
  },
  
  // 动态导入支持
  plugins: [
    new webpack.optimize.SplitChunksPlugin({
      chunks: 'async',
      name: (module, chunks, cacheGroupKey) => {
        const moduleFileName = module
          .identifier()
          .split('/')
          .reduceRight(item => item);
        return `${cacheGroupKey}-${moduleFileName}`;
      }
    })
  ]
};
```

### 2. Rollup懒加载配置

```javascript
// rollup.config.js
export default {
  input: 'src/main.js',
  output: {
    dir: 'dist',
    format: 'es',
    // 动态导入作为单独的chunk
    chunkFileNames: 'chunks/[name]-[hash].js'
  },
  plugins: [
    // 代码分割插件
    resolve(),
    commonjs(),
    // 自定义动态导入处理
    {
      name: 'dynamic-import-handler',
      generateBundle(options, bundle) {
        // 处理动态导入的chunk
        Object.keys(bundle).forEach(fileName => {
          const chunk = bundle[fileName];
          if (chunk.type === 'chunk' && chunk.isDynamicEntry) {
            // 添加预加载提示
            chunk.code = `/* webpackPreload: true */ ${chunk.code}`;
          }
        });
      }
    }
  ]
};
```

## 实际应用案例

### 1. 电商网站的懒加载策略

```typescript
// 电商网站模块懒加载方案
class EcommerceModuleLoader {
  private moduleCache = new Map();

  // 商品列表页面的懒加载
  async loadProductCatalog() {
    const modules = await Promise.allSettled([
      import('./components/ProductGrid'),
      import('./components/FilterPanel'),
      import('./components/SortDropdown')
    ]);

    return {
      ProductGrid: modules[0].status === 'fulfilled' ? modules[0].value.default : null,
      FilterPanel: modules[1].status === 'fulfilled' ? modules[1].value.default : null,
      SortDropdown: modules[2].status === 'fulfilled' ? modules[2].value.default : null
    };
  }

  // 购物车相关功能的懒加载
  async loadCartFeatures() {
    return import('./features/cart').then(module => ({
      Cart: module.Cart,
      CartItem: module.CartItem,
      CartSummary: module.CartSummary,
      useCart: module.useCart
    }));
  }

  // 支付流程的懒加载
  async loadPaymentFlow(paymentMethod: string) {
    switch (paymentMethod) {
      case 'stripe':
        return import('./payment/StripePayment');
      case 'paypal':
        return import('./payment/PayPalPayment');
      case 'alipay':
        return import('./payment/AlipayPayment');
      default:
        return import('./payment/DefaultPayment');
    }
  }

  // 管理后台功能的懒加载
  async loadAdminFeatures(userRole: string) {
    if (userRole !== 'admin') return null;

    const [
      { AdminDashboard },
      { ProductManager },
      { OrderManager },
      { UserManager }
    ] = await Promise.all([
      import('./admin/Dashboard'),
      import('./admin/ProductManager'),
      import('./admin/OrderManager'),
      import('./admin/UserManager')
    ]);

    return {
      AdminDashboard,
      ProductManager,
      OrderManager,
      UserManager
    };
  }
}

// 使用示例
const moduleLoader = new EcommerceModuleLoader();

// 页面级别的懒加载
function ProductPage() {
  const [modules, setModules] = useState(null);

  useEffect(() => {
    moduleLoader.loadProductCatalog().then(setModules);
  }, []);

  if (!modules) return <div>Loading...</div>;

  return (
    <div>
      {modules.ProductGrid && <modules.ProductGrid />}
      {modules.FilterPanel && <modules.FilterPanel />}
      {modules.SortDropdown && <modules.SortDropdown />}
    </div>
  );
}
```

### 2. 多语言网站的懒加载

```typescript
// 多语言资源懒加载
class I18nLazyLoader {
  private loadedLanguages = new Set<string>();
  private languageCache = new Map<string, any>();

  async loadLanguage(locale: string): Promise<any> {
    // 检查缓存
    if (this.languageCache.has(locale)) {
      return this.languageCache.get(locale);
    }

    // 检查是否已在加载中
    if (this.loadedLanguages.has(locale)) {
      return new Promise(resolve => {
        const checkCache = () => {
          if (this.languageCache.has(locale)) {
            resolve(this.languageCache.get(locale));
          } else {
            setTimeout(checkCache, 50);
          }
        };
        checkCache();
      });
    }

    this.loadedLanguages.add(locale);

    try {
      // 动态加载语言包
      const translations = await import(`./locales/${locale}.json`);
      this.languageCache.set(locale, translations.default);
      return translations.default;
    } catch (error) {
      // 降级到英语
      if (locale !== 'en') {
        return this.loadLanguage('en');
      }
      throw error;
    }
  }

  // 预加载用户可能需要的语言
  async preloadLanguages(preferredLanguages: string[]): Promise<void> {
    const promises = preferredLanguages.map(lang => 
      this.loadLanguage(lang).catch(() => null) // 忽略加载失败
    );
    await Promise.allSettled(promises);
  }

  // 智能语言切换
  async switchLanguage(locale: string): Promise<any> {
    const translations = await this.loadLanguage(locale);
    
    // 更新页面语言
    document.documentElement.lang = locale;
    
    // 触发语言更新事件
    window.dispatchEvent(new CustomEvent('languageChanged', {
      detail: { locale, translations }
    }));

    return translations;
  }
}

// React Hook for i18n lazy loading
function useI18nLazy(locale: string) {
  const [translations, setTranslations] = useState(null);
  const [loading, setLoading] = useState(false);
  const loader = useRef(new I18nLazyLoader());

  useEffect(() => {
    setLoading(true);
    loader.current.loadLanguage(locale)
      .then(setTranslations)
      .finally(() => setLoading(false));
  }, [locale]);

  const switchLanguage = useCallback((newLocale: string) => {
    return loader.current.switchLanguage(newLocale);
  }, []);

  return { translations, loading, switchLanguage };
}
```

## 最佳实践总结

### 1. 懒加载策略选择

- **路由级别**：适合页面较多的应用
- **组件级别**：适合复杂的交互组件
- **功能级别**：适合可选功能模块
- **资源级别**：适合大型依赖库

### 2. 性能优化技巧

- **预加载**：基于用户行为预测需要的模块
- **缓存策略**：合理使用内存和持久化缓存
- **降级方案**：提供加载失败时的备选方案
- **监控指标**：跟踪加载性能和成功率

### 3. 用户体验考虑

- **加载指示器**：提供清晰的加载状态反馈
- **渐进增强**：确保核心功能在模块加载前可用
- **错误处理**：优雅处理加载失败的情况
- **无感知切换**：尽量减少用户等待时间

### 4. 开发工具集成

- **构建优化**：配置合适的代码分割策略
- **开发调试**：使用开发工具监控模块加载
- **测试策略**：模拟不同网络条件进行测试
- **性能监控**：建立完善的性能监控体系

通过合理应用这些懒加载技术，我们可以显著提升应用的性能和用户体验，同时保持代码的可维护性。

---

**下一章**: [模块相关工具对比](../appendix/tools-comparison.md) →
