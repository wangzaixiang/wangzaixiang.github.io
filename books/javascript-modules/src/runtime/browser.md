# 浏览器中的模块

浏览器作为 JavaScript 模块化的重要运行环境，经历了从无原生支持到全面支持 ES 模块的发展历程。现代浏览器对模块的支持为前端开发带来了革命性的变化。

## 浏览器模块支持演进

### 历史发展阶段

#### 1. **早期阶段** (2009-2014)
- **无原生模块支持**：浏览器只能执行单个 JS 文件
- **全局变量依赖**：通过全局对象共享代码
- **手动依赖管理**：开发者需要手动控制脚本加载顺序

```html
<!-- 早期的依赖管理方式 -->
<script src="jquery.js"></script>
<script src="bootstrap.js"></script> <!-- 依赖 jQuery -->
<script src="app.js"></script>      <!-- 依赖前两者 -->
```

#### 2. **AMD/RequireJS 时代** (2011-2015)
- **异步模块定义**：RequireJS 提供浏览器端模块化
- **动态加载**：支持按需加载模块
- **依赖声明**：明确的依赖关系管理

```javascript
// AMD 模块定义
define(['jquery', 'underscore'], function($, _) {
  return {
    init: function() {
      // 模块逻辑
    }
  };
});
```

#### 3. **打包工具时代** (2012-2020)
- **Webpack/Browserify**：将 Node.js 模块系统带到浏览器
- **构建时解析**：编译时解决模块依赖
- **单文件输出**：打包成一个或几个文件

#### 4. **原生 ES 模块时代** (2017-至今)
- **ES6 模块原生支持**：所有现代浏览器支持
- **静态导入**：编译时确定依赖关系
- **动态导入**：运行时按需加载

## ES 模块在浏览器中的使用

### 基础语法支持

#### **静态导入/导出**

```html
<!-- 启用模块模式 -->
<script type="module">
  import { utils } from './utils.js';
  import React from './react.js';
  
  console.log(utils.formatDate(new Date()));
</script>
```

```javascript
// utils.js
export function formatDate(date) {
  return date.toISOString().split('T')[0];
}

export const API_URL = 'https://api.example.com';

// 默认导出
export default class Logger {
  log(message) {
    console.log(`[${new Date().toISOString()}] ${message}`);
  }
}
```

#### **动态导入**

```javascript
// 条件加载
async function loadFeature() {
  if (window.innerWidth > 768) {
    const { default: DesktopUI } = await import('./desktop-ui.js');
    return new DesktopUI();
  } else {
    const { default: MobileUI } = await import('./mobile-ui.js');
    return new MobileUI();
  }
}

// 懒加载路由
async function handleRoute(path) {
  switch (path) {
    case '/dashboard':
      const dashboard = await import('./pages/dashboard.js');
      dashboard.render();
      break;
    case '/profile':
      const profile = await import('./pages/profile.js');
      profile.render();
      break;
  }
}
```

### 模块解析规则

#### **相对路径导入**

```javascript
// 当前目录
import { config } from './config.js';

// 父目录
import { utils } from '../utils/helpers.js';

// 深层路径
import { Component } from '../../shared/components/Button.js';
```

#### **绝对路径导入**

```javascript
// 完整 URL
import { lodash } from 'https://cdn.skypack.dev/lodash';

// 从根路径
import { constants } from '/js/constants.js';
```

#### **Import Maps** (现代方案)

```html
<script type="importmap">
{
  "imports": {
    "react": "https://esm.sh/react@18",
    "react-dom": "https://esm.sh/react-dom@18",
    "lodash": "https://cdn.skypack.dev/lodash",
    "@/": "/src/",
    "@utils/": "/src/utils/"
  }
}
</script>

<script type="module">
  // 使用映射的模块名
  import React from 'react';
  import { render } from 'react-dom';
  import { debounce } from 'lodash';
  import { API } from '@/api.js';
  import { helpers } from '@utils/helpers.js';
</script>
```

## 浏览器环境特性

### **网络加载特性**

#### **HTTP/2 多路复用**

```javascript
// 现代浏览器可以并行加载多个模块
import('./module1.js');  // 并行请求
import('./module2.js');  // 并行请求  
import('./module3.js');  // 并行请求

// 依赖关系自动解析
import { useEffect } from 'react';     // 先加载
import { Component } from './comp.js'; // 再加载依赖 react 的模块
```

#### **预加载优化**

```html
<!-- 模块预加载 -->
<link rel="modulepreload" href="./critical-module.js">
<link rel="modulepreload" href="./utils.js">

<!-- 预解析 DNS -->
<link rel="dns-prefetch" href="//cdn.example.com">

<!-- 预连接到外部域 -->
<link rel="preconnect" href="https://fonts.googleapis.com">
```

#### **缓存策略**

```javascript
// 服务器端设置缓存头
// Cache-Control: public, max-age=31536000, immutable

// 版本化文件名
import { utils } from './utils.a1b2c3.js';

// 动态版本控制
const version = '1.2.3';
const module = await import(`./api.${version}.js`);
```

### **安全特性**

#### **CORS 策略**

```javascript
// 跨域模块加载需要 CORS 支持
try {
  const external = await import('https://external-cdn.com/module.js');
} catch (error) {
  console.error('CORS 错误:', error);
  // 降级方案
  const fallback = await import('./local-fallback.js');
}
```

#### **CSP (内容安全策略)**

```html
<!-- 允许模块脚本 -->
<meta http-equiv="Content-Security-Policy" 
      content="script-src 'self' 'unsafe-inline' https://cdn.skypack.dev;">

<script type="module">
  // 符合 CSP 策略的模块导入
  import { lib } from 'https://cdn.skypack.dev/lib';
</script>
```

#### **Subresource Integrity (SRI)**

```html
<!-- 确保外部模块完整性 -->
<script type="module" 
        src="https://cdn.example.com/module.js"
        integrity="sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GqQ8K/uxy9rx7HNQlGYl1kPzQho1wx4JwY8wC"
        crossorigin="anonymous">
</script>
```

## 性能优化策略

### **代码分割和懒加载**

#### **路由级分割**

```javascript
class Router {
  async navigate(path) {
    this.showLoading();
    
    try {
      const route = this.routes[path];
      const module = await import(route.component);
      
      this.hideLoading();
      module.default.render();
    } catch (error) {
      this.showError('页面加载失败');
    }
  }
}

const router = new Router({
  routes: {
    '/': { component: './pages/home.js' },
    '/about': { component: './pages/about.js' },
    '/products': { component: './pages/products.js' }
  }
});
```

#### **组件级懒加载**

```javascript
class LazyComponent {
  constructor(importFn) {
    this.importFn = importFn;
    this.component = null;
  }
  
  async render(container) {
    if (!this.component) {
      const module = await this.importFn();
      this.component = module.default;
    }
    
    return this.component.render(container);
  }
}

// 使用示例
const HeavyChart = new LazyComponent(() => import('./heavy-chart.js'));

document.querySelector('#load-chart').addEventListener('click', async () => {
  await HeavyChart.render(document.querySelector('#chart-container'));
});
```

### **资源优先级管理**

#### **Critical Resource First**

```html
<!-- 关键资源优先加载 -->
<link rel="modulepreload" href="./critical-app.js" as="script">
<link rel="modulepreload" href="./critical-styles.js" as="script">

<script type="module">
  // 立即加载关键模块
  import('./critical-app.js').then(app => app.init());
  
  // 延迟加载非关键模块
  setTimeout(() => {
    import('./analytics.js');
    import('./social-widgets.js');
  }, 1000);
</script>
```

#### **Intersection Observer 懒加载**

```javascript
class ModuleLazyLoader {
  constructor() {
    this.observer = new IntersectionObserver(this.handleIntersection.bind(this));
  }
  
  observe(element, moduleUrl) {
    element.dataset.moduleUrl = moduleUrl;
    this.observer.observe(element);
  }
  
  async handleIntersection(entries) {
    for (const entry of entries) {
      if (entry.isIntersecting) {
        const moduleUrl = entry.target.dataset.moduleUrl;
        const module = await import(moduleUrl);
        
        module.default.render(entry.target);
        this.observer.unobserve(entry.target);
      }
    }
  }
}

// 使用示例
const loader = new ModuleLazyLoader();
loader.observe(document.querySelector('#feature-section'), './feature.js');
```

## 错误处理和调试

### **模块加载错误处理**

```javascript
class ModuleLoader {
  async loadModule(url, retries = 3) {
    for (let i = 0; i < retries; i++) {
      try {
        return await import(url);
      } catch (error) {
        console.warn(`模块加载失败 (${i + 1}/${retries}):`, error);
        
        if (i === retries - 1) {
          // 最终失败，尝试降级方案
          return this.loadFallback(url);
        }
        
        // 重试前等待
        await new Promise(resolve => setTimeout(resolve, 1000 * (i + 1)));
      }
    }
  }
  
  async loadFallback(originalUrl) {
    const fallbackMap = {
      './analytics.js': './analytics-lite.js',
      './charts.js': './charts-basic.js'
    };
    
    const fallbackUrl = fallbackMap[originalUrl];
    if (fallbackUrl) {
      console.info('使用降级模块:', fallbackUrl);
      return import(fallbackUrl);
    }
    
    throw new Error(`无法加载模块: ${originalUrl}`);
  }
}
```

### **开发环境调试**

```javascript
// 开发环境模块热重载
if (import.meta.hot) {
  import.meta.hot.accept('./utils.js', (newModule) => {
    console.log('模块已更新:', newModule);
    // 重新初始化使用该模块的组件
    app.updateUtils(newModule);
  });
}

// 模块加载性能监控
const moduleLoadTimes = new Map();

const originalImport = window.import;
window.import = function(url) {
  const startTime = performance.now();
  
  return originalImport(url).then(module => {
    const loadTime = performance.now() - startTime;
    moduleLoadTimes.set(url, loadTime);
    console.log(`模块 ${url} 加载耗时: ${loadTime.toFixed(2)}ms`);
    return module;
  });
};
```

## 兼容性和降级策略

### **浏览器支持检测**

```javascript
// 检测 ES 模块支持
function supportsESModules() {
  try {
    return typeof importFn === 'function' || 'noModule' in document.createElement('script');
  } catch {
    return false;
  }
}

// 渐进式加载策略
if (supportsESModules()) {
  // 现代浏览器：使用 ES 模块
  import('./modern-app.js').then(app => app.init());
} else {
  // 旧浏览器：加载打包后的版本
  const script = document.createElement('script');
  script.src = './legacy-app.bundle.js';
  document.head.appendChild(script);
}
```

### **Polyfill 和 Shim**

```html
<!-- 使用 nomodule 为旧浏览器提供降级 -->
<script type="module" src="./modern-app.js"></script>
<script nomodule src="./legacy-app.js"></script>

<!-- SystemJS 作为 ES 模块 polyfill -->
<script src="https://unpkg.com/systemjs/dist/system.min.js"></script>
<script>
  if (!window.supportsESModules) {
    System.import('./app.js');
  }
</script>
```

## 最佳实践总结

### **性能优化**
1. **使用 Import Maps** 管理模块映射
2. **实施代码分割** 减少初始加载时间
3. **合理使用预加载** 优化关键路径
4. **监控模块加载性能** 识别瓶颈

### **可维护性**
1. **明确的模块边界** 避免循环依赖
2. **一致的导入路径** 使用绝对路径或别名
3. **适当的错误处理** 提供降级方案
4. **文档化依赖关系** 便于团队协作

### **安全性**
1. **验证外部模块** 使用 SRI 确保完整性
2. **配置 CSP 策略** 限制模块来源
3. **审查第三方依赖** 定期更新和检查

浏览器中的模块化为现代 Web 开发提供了强大的能力，合理利用这些特性可以构建出高性能、可维护的 Web 应用。

---

**下一章**: [Node.js中的模块](./nodejs.md) →
