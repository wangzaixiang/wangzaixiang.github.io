# 动态导入

动态导入（Dynamic Import）是ES2020引入的功能，允许在运行时按需加载模块。与静态导入不同，动态导入提供了更大的灵活性，支持条件加载、懒加载和代码分割等高级用法。

## 基本语法

### import() 函数

```javascript
// 基本动态导入语法
import('./module.js')
    .then(module => {
        // 使用导入的模块
        console.log(module.default);
        console.log(module.namedExport);
    })
    .catch(err => {
        console.error('Failed to load module:', err);
    });

// 使用 async/await
async function loadModule() {
    try {
        const module = await import('./module.js');
        return module;
    } catch (error) {
        console.error('Module loading failed:', error);
        throw error;
    }
}
```

### 顶层 await 与动态导入

```javascript
// top-level-await.js
// 在支持顶层await的环境中使用

// 条件性加载模块
const isDevelopment = process.env.NODE_ENV === 'development';

if (isDevelopment) {
    const devTools = await import('./dev-tools.js');
    devTools.setup();
    console.log('Development tools loaded');
}

// 动态选择实现
const preferredImplementation = getPreferredImplementation();
const implementation = await import(`./implementations/${preferredImplementation}.js`);

export default implementation.default;
```

## 动态导入的应用场景

### 1. 条件加载

```javascript
// conditional-loading.js

// 根据用户权限加载不同的模块
async function loadUserInterface(userRole) {
    switch (userRole) {
        case 'admin':
            const adminUI = await import('./admin-ui.js');
            return adminUI.default;
            
        case 'moderator':
            const modUI = await import('./moderator-ui.js');
            return modUI.default;
            
        case 'user':
        default:
            const userUI = await import('./user-ui.js');
            return userUI.default;
    }
}

// 根据功能特性加载polyfill
async function loadPolyfills() {
    const promises = [];
    
    if (!('IntersectionObserver' in window)) {
        promises.push(import('./polyfills/intersection-observer.js'));
    }
    
    if (!('fetch' in window)) {
        promises.push(import('./polyfills/fetch.js'));
    }
    
    if (!('Promise' in window)) {
        promises.push(import('./polyfills/promise.js'));
    }
    
    return Promise.all(promises);
}

// 使用示例
const user = getCurrentUser();
const ui = await loadUserInterface(user.role);
ui.render(document.body);
```

### 2. 懒加载和代码分割

```javascript
// lazy-loading.js

// 路由懒加载
const routes = {
    '/': () => import('./pages/Home.js'),
    '/about': () => import('./pages/About.js'),
    '/contact': () => import('./pages/Contact.js'),
    '/dashboard': () => import('./pages/Dashboard.js'),
    '/settings': () => import('./pages/Settings.js')
};

async function navigateTo(path) {
    const loadPage = routes[path];
    
    if (!loadPage) {
        throw new Error(`No route found for ${path}`);
    }
    
    try {
        const pageModule = await loadPage();
        const page = new pageModule.default();
        
        // 清理当前页面
        clearCurrentPage();
        
        // 渲染新页面
        page.render(document.getElementById('app'));
        
        // 更新浏览器历史
        history.pushState({}, '', path);
    } catch (error) {
        console.error(`Failed to load page ${path}:`, error);
        showErrorPage();
    }
}

// 组件懒加载
class ComponentLoader {
    constructor() {
        this.cache = new Map();
    }
    
    async loadComponent(name) {
        // 检查缓存
        if (this.cache.has(name)) {
            return this.cache.get(name);
        }
        
        try {
            const module = await import(`./components/${name}.js`);
            const component = module.default;
            
            // 缓存组件
            this.cache.set(name, component);
            
            return component;
        } catch (error) {
            console.error(`Failed to load component ${name}:`, error);
            return null;
        }
    }
}

const loader = new ComponentLoader();

// 使用示例
document.addEventListener('click', async (event) => {
    if (event.target.dataset.component) {
        const componentName = event.target.dataset.component;
        const Component = await loader.loadComponent(componentName);
        
        if (Component) {
            const instance = new Component();
            instance.mount(event.target.parentElement);
        }
    }
});
```

### 3. 插件系统

```javascript
// plugin-system.js

class PluginManager {
    constructor() {
        this.plugins = new Map();
        this.hooks = new Map();
    }
    
    // 动态加载插件
    async loadPlugin(pluginName, config = {}) {
        try {
            const pluginModule = await import(`./plugins/${pluginName}/index.js`);
            const PluginClass = pluginModule.default;
            
            const plugin = new PluginClass(config);
            
            // 初始化插件
            if (typeof plugin.init === 'function') {
                await plugin.init();
            }
            
            // 注册插件
            this.plugins.set(pluginName, plugin);
            
            // 注册插件的钩子
            if (plugin.hooks) {
                Object.entries(plugin.hooks).forEach(([hookName, handler]) => {
                    this.registerHook(hookName, handler);
                });
            }
            
            console.log(`Plugin ${pluginName} loaded successfully`);
            return plugin;
        } catch (error) {
            console.error(`Failed to load plugin ${pluginName}:`, error);
            throw error;
        }
    }
    
    // 卸载插件
    async unloadPlugin(pluginName) {
        const plugin = this.plugins.get(pluginName);
        
        if (!plugin) {
            return false;
        }
        
        // 执行清理
        if (typeof plugin.destroy === 'function') {
            await plugin.destroy();
        }
        
        // 移除钩子
        if (plugin.hooks) {
            Object.keys(plugin.hooks).forEach(hookName => {
                this.unregisterHook(hookName, plugin.hooks[hookName]);
            });
        }
        
        this.plugins.delete(pluginName);
        console.log(`Plugin ${pluginName} unloaded`);
        return true;
    }
    
    registerHook(hookName, handler) {
        if (!this.hooks.has(hookName)) {
            this.hooks.set(hookName, []);
        }
        this.hooks.get(hookName).push(handler);
    }
    
    unregisterHook(hookName, handler) {
        const handlers = this.hooks.get(hookName);
        if (handlers) {
            const index = handlers.indexOf(handler);
            if (index > -1) {
                handlers.splice(index, 1);
            }
        }
    }
    
    // 触发钩子
    async triggerHook(hookName, ...args) {
        const handlers = this.hooks.get(hookName) || [];
        const results = await Promise.all(
            handlers.map(handler => handler(...args))
        );
        return results;
    }
}

// 使用示例
const pluginManager = new PluginManager();

// 加载插件
await pluginManager.loadPlugin('analytics', {
    trackingId: 'GA-XXXXXX-X'
});

await pluginManager.loadPlugin('chat-widget', {
    apiKey: 'chat-api-key',
    position: 'bottom-right'
});

// 触发钩子
await pluginManager.triggerHook('user-login', { userId: 123 });
```

### 4. 国际化和本地化

```javascript
// i18n.js

class I18nManager {
    constructor(defaultLocale = 'en') {
        this.currentLocale = defaultLocale;
        this.translations = new Map();
        this.fallbacks = new Map();
    }
    
    // 动态加载语言包
    async loadLocale(locale) {
        if (this.translations.has(locale)) {
            return this.translations.get(locale);
        }
        
        try {
            // 尝试加载完整语言包
            const fullModule = await import(`./locales/${locale}.js`);
            this.translations.set(locale, fullModule.default);
            return fullModule.default;
        } catch (error) {
            // 如果失败，尝试加载语言的基础版本
            const baseLang = locale.split('-')[0];
            if (baseLang !== locale) {
                try {
                    const baseModule = await import(`./locales/${baseLang}.js`);
                    this.fallbacks.set(locale, baseModule.default);
                    return baseModule.default;
                } catch (baseError) {
                    console.error(`Failed to load locale ${locale} and ${baseLang}:`, error, baseError);
                }
            }
            throw error;
        }
    }
    
    // 设置当前语言
    async setLocale(locale) {
        await this.loadLocale(locale);
        this.currentLocale = locale;
        
        // 触发语言变更事件
        document.dispatchEvent(new CustomEvent('locale-changed', {
            detail: { locale, translations: this.translations.get(locale) }
        }));
    }
    
    // 获取翻译
    t(key, params = {}) {
        const translations = this.translations.get(this.currentLocale) ||
                           this.fallbacks.get(this.currentLocale) ||
                           this.translations.get('en');
        
        if (!translations) {
            return key;
        }
        
        let text = this.getNestedValue(translations, key) || key;
        
        // 替换参数
        Object.entries(params).forEach(([param, value]) => {
            text = text.replace(new RegExp(`\\{\\{${param}\\}\\}`, 'g'), value);
        });
        
        return text;
    }
    
    getNestedValue(obj, path) {
        return path.split('.').reduce((current, key) => {
            return current && current[key] !== undefined ? current[key] : null;
        }, obj);
    }
}

// 使用示例
const i18n = new I18nManager('en');

// 检测用户语言并加载
const userLocale = navigator.language || navigator.userLanguage || 'en';
await i18n.setLocale(userLocale);

// 在应用中使用
console.log(i18n.t('welcome.message', { name: 'John' }));
console.log(i18n.t('navigation.home'));

// 语言切换器
async function switchLanguage(locale) {
    try {
        await i18n.setLocale(locale);
        updateUI();
    } catch (error) {
        console.error('Failed to switch language:', error);
    }
}
```

## 动态导入的高级用法

### 1. 并行加载多个模块

```javascript
// parallel-loading.js

// 并行加载多个相关模块
async function loadDashboardModules() {
    const [
        chartsModule,
        tablesModule,
        filtersModule,
        exportModule
    ] = await Promise.all([
        import('./charts.js'),
        import('./tables.js'),
        import('./filters.js'),
        import('./export.js')
    ]);
    
    return {
        Charts: chartsModule.default,
        Tables: tablesModule.default,
        Filters: filtersModule.default,
        Export: exportModule.default
    };
}

// 使用Promise.allSettled处理部分失败
async function loadOptionalModules() {
    const results = await Promise.allSettled([
        import('./analytics.js'),
        import('./chat.js'),
        import('./notifications.js'),
        import('./help.js')
    ]);
    
    const loadedModules = {};
    
    results.forEach((result, index) => {
        const moduleNames = ['analytics', 'chat', 'notifications', 'help'];
        const moduleName = moduleNames[index];
        
        if (result.status === 'fulfilled') {
            loadedModules[moduleName] = result.value.default;
            console.log(`${moduleName} module loaded successfully`);
        } else {
            console.warn(`${moduleName} module failed to load:`, result.reason);
        }
    });
    
    return loadedModules;
}
```

### 2. 模块预加载

```javascript
// preloading.js

class ModulePreloader {
    constructor() {
        this.preloadCache = new Map();
        this.loadingPromises = new Map();
    }
    
    // 预加载模块（但不执行）
    preload(modulePath) {
        if (this.preloadCache.has(modulePath)) {
            return this.preloadCache.get(modulePath);
        }
        
        // 创建link标签进行预加载
        const link = document.createElement('link');
        link.rel = 'modulepreload';
        link.href = modulePath;
        document.head.appendChild(link);
        
        const promise = import(modulePath).then(module => {
            this.preloadCache.set(modulePath, module);
            return module;
        });
        
        this.loadingPromises.set(modulePath, promise);
        return promise;
    }
    
    // 立即获取预加载的模块
    async getPreloaded(modulePath) {
        if (this.preloadCache.has(modulePath)) {
            return this.preloadCache.get(modulePath);
        }
        
        if (this.loadingPromises.has(modulePath)) {
            return this.loadingPromises.get(modulePath);
        }
        
        return import(modulePath);
    }
    
    // 批量预加载
    preloadBatch(modulePaths) {
        return Promise.all(modulePaths.map(path => this.preload(path)));
    }
    
    // 智能预加载：根据用户行为预测
    intelligentPreload(userBehavior) {
        const predictions = this.predictNextModules(userBehavior);
        return this.preloadBatch(predictions);
    }
    
    predictNextModules(behavior) {
        // 简单的预测逻辑
        const moduleMap = {
            'viewing-products': ['./cart.js', './checkout.js'],
            'in-cart': ['./payment.js', './shipping.js'],
            'profile-page': ['./settings.js', './orders.js']
        };
        
        return moduleMap[behavior] || [];
    }
}

const preloader = new ModulePreloader();

// 在应用启动时预加载关键模块
preloader.preloadBatch([
    './router.js',
    './auth.js',
    './api-client.js'
]);

// 根据用户行为智能预加载
document.addEventListener('mouseover', (event) => {
    if (event.target.dataset.preload) {
        preloader.preload(event.target.dataset.preload);
    }
});
```

### 3. 容错和重试机制

```javascript
// error-handling.js

class RobustModuleLoader {
    constructor(options = {}) {
        this.maxRetries = options.maxRetries || 3;
        this.retryDelay = options.retryDelay || 1000;
        this.fallbacks = options.fallbacks || {};
    }
    
    async loadWithRetry(modulePath, retryCount = 0) {
        try {
            return await import(modulePath);
        } catch (error) {
            console.warn(`Failed to load ${modulePath} (attempt ${retryCount + 1}):`, error);
            
            if (retryCount < this.maxRetries) {
                // 指数退避
                const delay = this.retryDelay * Math.pow(2, retryCount);
                await this.sleep(delay);
                return this.loadWithRetry(modulePath, retryCount + 1);
            }
            
            // 尝试使用fallback
            const fallbackPath = this.fallbacks[modulePath];
            if (fallbackPath && fallbackPath !== modulePath) {
                console.log(`Trying fallback for ${modulePath}: ${fallbackPath}`);
                return this.loadWithRetry(fallbackPath);
            }
            
            throw new Error(`Failed to load module ${modulePath} after ${this.maxRetries} retries`);
        }
    }
    
    async loadWithFallback(primaryPath, fallbackPath) {
        try {
            return await import(primaryPath);
        } catch (error) {
            console.warn(`Primary module ${primaryPath} failed, using fallback:`, error);
            return import(fallbackPath);
        }
    }
    
    sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

// 使用示例
const loader = new RobustModuleLoader({
    maxRetries: 3,
    retryDelay: 1000,
    fallbacks: {
        './advanced-charts.js': './basic-charts.js',
        './high-res-images.js': './low-res-images.js'
    }
});

try {
    const chartsModule = await loader.loadWithRetry('./advanced-charts.js');
    console.log('Advanced charts loaded successfully');
} catch (error) {
    console.error('All chart loading attempts failed:', error);
}
```

## 性能考虑

### 1. 模块大小优化

```javascript
// module-optimization.js

// 分割大模块为小块
// 代替一个大的utils模块：
// import * as utils from './large-utils.js';

// 使用按需导入：
const stringUtils = () => import('./utils/string.js');
const arrayUtils = () => import('./utils/array.js');
const dateUtils = () => import('./utils/date.js');

// 只在需要时加载具体功能
async function formatUserData(userData) {
    const { format } = await import('./utils/string.js');
    const { sortBy } = await import('./utils/array.js');
    const { formatDate } = await import('./utils/date.js');
    
    return {
        name: format(userData.name),
        dates: userData.dates.map(formatDate),
        sorted: sortBy(userData.items, 'priority')
    };
}
```

### 2. 缓存策略

```javascript
// caching.js

class ModuleCache {
    constructor(options = {}) {
        this.cache = new Map();
        this.maxAge = options.maxAge || 300000; // 5分钟
        this.maxSize = options.maxSize || 50;
    }
    
    async loadModule(modulePath) {
        const cacheKey = modulePath;
        const cached = this.cache.get(cacheKey);
        
        if (cached && this.isValid(cached)) {
            return cached.module;
        }
        
        const module = await import(modulePath);
        
        this.cache.set(cacheKey, {
            module,
            timestamp: Date.now()
        });
        
        this.cleanup();
        return module;
    }
    
    isValid(cached) {
        return Date.now() - cached.timestamp < this.maxAge;
    }
    
    cleanup() {
        if (this.cache.size <= this.maxSize) {
            return;
        }
        
        // 移除最旧的条目
        const entries = Array.from(this.cache.entries());
        entries.sort((a, b) => a[1].timestamp - b[1].timestamp);
        
        const toRemove = entries.slice(0, this.cache.size - this.maxSize);
        toRemove.forEach(([key]) => this.cache.delete(key));
    }
    
    clear() {
        this.cache.clear();
    }
}

const moduleCache = new ModuleCache({ maxAge: 600000, maxSize: 100 });
```

## 动态导入的局限性

### 1. 静态分析限制

```javascript
// limitations.js

// ❌ 这些用法会使打包工具无法进行静态分析
const moduleName = getUserPreference();
import(moduleName); // 完全动态的路径

const modules = ['a', 'b', 'c'];
modules.forEach(name => import(name)); // 循环中的动态导入

// ✅ 更好的做法：使用部分静态路径
const theme = getUserTheme();
import(`./themes/${theme}.js`); // 部分静态路径

// ✅ 或使用显式的模块映射
const moduleMap = {
    theme1: () => import('./themes/theme1.js'),
    theme2: () => import('./themes/theme2.js'),
    theme3: () => import('./themes/theme3.js')
};

const themeLoader = moduleMap[theme];
if (themeLoader) {
    themeLoader().then(module => {
        // 使用主题模块
    });
}
```

### 2. 错误处理的重要性

```javascript
// error-handling-best-practices.js

// ❌ 没有错误处理
import('./module.js').then(module => {
    module.default(); // 如果模块加载失败，这里会出错
});

// ✅ 完善的错误处理
async function loadAndUseModule() {
    try {
        const module = await import('./module.js');
        
        if (!module || !module.default) {
            throw new Error('Module does not have expected exports');
        }
        
        return module.default();
    } catch (error) {
        console.error('Module loading or execution failed:', error);
        
        // 提供fallback逻辑
        return useDefaultBehavior();
    }
}

function useDefaultBehavior() {
    // 默认行为实现
    console.log('Using default behavior due to module loading failure');
}
```

## 总结

动态导入为JavaScript模块系统带来了强大的运行时灵活性：

- ✅ **按需加载**: 减少初始包大小，提升应用启动速度
- ✅ **条件加载**: 根据用户环境、权限等条件加载不同模块
- ✅ **代码分割**: 自动将代码分割为更小的块
- ✅ **插件系统**: 支持运行时扩展应用功能
- ✅ **懒加载**: 延迟加载非关键功能
- ✅ **国际化**: 动态加载语言包和地区化内容

动态导入是现代Web应用性能优化和架构设计的重要工具，合理使用能够显著提升用户体验和应用的可维护性。

---

**下一章**: [模块解析机制](./resolution.md) →