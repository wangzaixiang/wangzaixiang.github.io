# 微前端模块化

微前端架构将前端应用拆分为多个独立的、可部署的前端应用，每个应用负责特定的业务域。本章将探讨如何使用JavaScript模块系统实现微前端架构。

## 微前端基础概念

### 微前端架构特点

```javascript
// 微前端架构的核心特征
const MicrofrontendFeatures = {
  // 技术栈无关性
  technologyAgnostic: {
    react: 'React应用',
    vue: 'Vue应用',
    angular: 'Angular应用',
    vanilla: '原生JavaScript应用'
  },
  
  // 独立部署
  independentDeployment: {
    cicd: '每个微前端有自己的CI/CD流水线',
    versioning: '独立版本管理',
    rollback: '独立回滚能力'
  },
  
  // 团队自治
  teamAutonomy: {
    ownership: '团队完全拥有微前端的开发和维护',
    decisions: '技术决策独立',
    timeline: '开发周期独立'
  }
};

// 微前端应用注册表
class MicrofrontendRegistry {
  constructor() {
    this.applications = new Map();
    this.routes = new Map();
    this.sharedDependencies = new Map();
  }

  // 注册微前端应用
  register(name, config) {
    this.applications.set(name, {
      name,
      url: config.url,
      mount: config.mount,
      unmount: config.unmount,
      activeWhen: config.activeWhen,
      dependencies: config.dependencies || [],
      sharedLibraries: config.sharedLibraries || []
    });
  }

  // 获取激活的应用
  getActiveApplications(location) {
    return Array.from(this.applications.values())
      .filter(app => app.activeWhen(location));
  }

  // 预加载应用
  async preloadApplication(name) {
    const app = this.applications.get(name);
    if (!app || app.loaded) return;

    try {
      const module = await import(app.url);
      app.loaded = true;
      app.module = module;
      return module;
    } catch (error) {
      console.error(`Failed to preload application ${name}:`, error);
      throw error;
    }
  }
}
```

## 模块联邦 (Module Federation)

### Webpack Module Federation

```javascript
// webpack.config.js - 主应用 (Shell/Host)
const ModuleFederationPlugin = require('@module-federation/webpack');

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  devServer: {
    port: 3000,
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'shell',
      remotes: {
        'user-app': 'userApp@http://localhost:3001/remoteEntry.js',
        'product-app': 'productApp@http://localhost:3002/remoteEntry.js',
        'cart-app': 'cartApp@http://localhost:3003/remoteEntry.js'
      },
      shared: {
        react: {
          singleton: true,
          requiredVersion: '^18.0.0'
        },
        'react-dom': {
          singleton: true,
          requiredVersion: '^18.0.0'
        }
      }
    })
  ]
};

// 微前端应用配置
// webpack.config.js - 用户管理应用
module.exports = {
  mode: 'development',
  entry: './src/index.js',
  devServer: {
    port: 3001,
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'userApp',
      filename: 'remoteEntry.js',
      exposes: {
        './UserList': './src/components/UserList',
        './UserProfile': './src/components/UserProfile',
        './UserManagement': './src/UserManagement'
      },
      shared: {
        react: {
          singleton: true,
          requiredVersion: '^18.0.0'
        },
        'react-dom': {
          singleton: true,
          requiredVersion: '^18.0.0'
        }
      }
    })
  ]
};
```

### 动态模块加载

```javascript
// shell应用 - 动态加载微前端
class MicrofrontendLoader {
  constructor() {
    this.loadedApps = new Map();
    this.loadingPromises = new Map();
  }

  async loadMicrofrontend(appName, remoteName) {
    if (this.loadedApps.has(appName)) {
      return this.loadedApps.get(appName);
    }

    if (this.loadingPromises.has(appName)) {
      return this.loadingPromises.get(appName);
    }

    const loadingPromise = this._loadRemoteModule(appName, remoteName);
    this.loadingPromises.set(appName, loadingPromise);

    try {
      const app = await loadingPromise;
      this.loadedApps.set(appName, app);
      this.loadingPromises.delete(appName);
      return app;
    } catch (error) {
      this.loadingPromises.delete(appName);
      throw error;
    }
  }

  async _loadRemoteModule(appName, remoteName) {
    try {
      // 动态导入远程模块
      const module = await import(remoteName);
      
      return {
        name: appName,
        module,
        mount: module.mount || (() => {}),
        unmount: module.unmount || (() => {}),
        update: module.update || (() => {})
      };
    } catch (error) {
      console.error(`Failed to load microfrontend ${appName}:`, error);
      
      // 返回错误边界组件
      return this._createErrorBoundary(appName, error);
    }
  }

  _createErrorBoundary(appName, error) {
    return {
      name: appName,
      module: {
        default: () => ({
          render: (container) => {
            container.innerHTML = `
              <div class="microfrontend-error">
                <h3>应用 ${appName} 加载失败</h3>
                <p>错误信息: ${error.message}</p>
                <button onclick="location.reload()">重试</button>
              </div>
            `;
          }
        })
      },
      mount: () => {},
      unmount: () => {}
    };
  }
}

// 使用示例
const loader = new MicrofrontendLoader();

// React组件中使用
function App() {
  const [userApp, setUserApp] = useState(null);
  const [loading, setLoading] = useState(false);

  const loadUserApp = async () => {
    setLoading(true);
    try {
      const app = await loader.loadMicrofrontend('user-app', 'user-app/UserManagement');
      setUserApp(app);
    } catch (error) {
      console.error('Failed to load user app:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <nav>
        <button onClick={loadUserApp}>加载用户管理</button>
      </nav>
      {loading && <div>加载中...</div>}
      {userApp && <RemoteComponent app={userApp} />}
    </div>
  );
}
```

## 应用间通信

### 事件总线通信

```javascript
// 微前端事件总线
class MicrofrontendEventBus {
  constructor() {
    this.events = new Map();
    this.wildcardEvents = new Map();
    this.middlewares = [];
  }

  // 添加中间件
  use(middleware) {
    this.middlewares.push(middleware);
  }

  // 订阅事件
  on(eventName, handler, options = {}) {
    const { once = false, priority = 0, namespace = 'global' } = options;
    
    const subscription = {
      handler,
      once,
      priority,
      namespace,
      id: Math.random().toString(36).substr(2, 9)
    };

    if (eventName.includes('*')) {
      // 通配符事件
      if (!this.wildcardEvents.has(eventName)) {
        this.wildcardEvents.set(eventName, []);
      }
      this.wildcardEvents.get(eventName).push(subscription);
    } else {
      // 普通事件
      if (!this.events.has(eventName)) {
        this.events.set(eventName, []);
      }
      this.events.get(eventName).push(subscription);
    }

    // 按优先级排序
    const handlers = this.events.get(eventName) || this.wildcardEvents.get(eventName);
    handlers.sort((a, b) => b.priority - a.priority);

    // 返回取消订阅函数
    return () => this.off(eventName, subscription.id);
  }

  // 取消订阅
  off(eventName, handlerOrId) {
    const removeFromArray = (array, predicate) => {
      const index = array.findIndex(predicate);
      if (index > -1) {
        array.splice(index, 1);
      }
    };

    const predicate = typeof handlerOrId === 'string' 
      ? (sub) => sub.id === handlerOrId
      : (sub) => sub.handler === handlerOrId;

    if (this.events.has(eventName)) {
      removeFromArray(this.events.get(eventName), predicate);
    }

    // 检查通配符事件
    this.wildcardEvents.forEach((handlers, pattern) => {
      if (this._matchPattern(pattern, eventName)) {
        removeFromArray(handlers, predicate);
      }
    });
  }

  // 发布事件
  async emit(eventName, data, options = {}) {
    const { timeout = 5000, retries = 0 } = options;
    
    // 应用中间件
    let processedData = data;
    for (const middleware of this.middlewares) {
      processedData = await middleware(eventName, processedData, options);
    }

    const event = {
      name: eventName,
      data: processedData,
      timestamp: Date.now(),
      source: options.source || 'unknown'
    };

    const promises = [];

    // 普通事件处理器
    if (this.events.has(eventName)) {
      const handlers = this.events.get(eventName);
      promises.push(...this._executeHandlers(handlers, event, timeout));
    }

    // 通配符事件处理器
    this.wildcardEvents.forEach((handlers, pattern) => {
      if (this._matchPattern(pattern, eventName)) {
        promises.push(...this._executeHandlers(handlers, event, timeout));
      }
    });

    // 等待所有处理器完成
    try {
      await Promise.allSettled(promises);
    } catch (error) {
      console.error(`Event ${eventName} handling failed:`, error);
      
      if (retries > 0) {
        console.log(`Retrying event ${eventName}, ${retries} attempts left`);
        return this.emit(eventName, data, { ...options, retries: retries - 1 });
      }
    }
  }

  _executeHandlers(handlers, event, timeout) {
    return handlers.map(async (subscription) => {
      try {
        const timeoutPromise = new Promise((_, reject) =>
          setTimeout(() => reject(new Error('Handler timeout')), timeout)
        );

        const handlerPromise = Promise.resolve(subscription.handler(event));
        
        await Promise.race([handlerPromise, timeoutPromise]);

        // 一次性订阅处理
        if (subscription.once) {
          this.off(event.name, subscription.id);
        }
      } catch (error) {
        console.error(`Handler failed for event ${event.name}:`, error);
      }
    });
  }

  _matchPattern(pattern, eventName) {
    const regex = new RegExp(
      pattern.replace(/\*/g, '.*').replace(/\?/g, '.')
    );
    return regex.test(eventName);
  }
}

// 全局事件总线实例
const eventBus = new MicrofrontendEventBus();

// 使用示例
// 用户应用 - 发布用户登录事件
eventBus.emit('user.login', {
  userId: '123',
  username: 'john_doe',
  timestamp: Date.now()
}, { source: 'user-app' });

// 购物车应用 - 监听用户登录
eventBus.on('user.login', (event) => {
  console.log('User logged in, updating cart:', event.data);
  // 更新购物车状态
});

// 导航应用 - 监听所有用户事件
eventBus.on('user.*', (event) => {
  console.log('User event received:', event.name, event.data);
  // 更新导航状态
});
```

### 状态共享

```javascript
// 微前端状态管理
class MicrofrontendStore {
  constructor() {
    this.state = new Map();
    this.subscribers = new Map();
    this.middlewares = [];
    this.history = [];
    this.maxHistory = 50;
  }

  // 添加中间件
  use(middleware) {
    this.middlewares.push(middleware);
  }

  // 设置状态
  async setState(namespace, updates, options = {}) {
    const { merge = true, notify = true } = options;
    
    const currentState = this.state.get(namespace) || {};
    const newState = merge ? { ...currentState, ...updates } : updates;
    
    // 应用中间件
    let processedState = newState;
    for (const middleware of this.middlewares) {
      processedState = await middleware({
        action: 'setState',
        namespace,
        oldState: currentState,
        newState: processedState,
        updates
      });
    }

    // 记录历史
    this.history.push({
      timestamp: Date.now(),
      namespace,
      oldState: currentState,
      newState: processedState,
      action: 'setState'
    });

    if (this.history.length > this.maxHistory) {
      this.history.shift();
    }

    this.state.set(namespace, processedState);

    // 通知订阅者
    if (notify) {
      this._notifySubscribers(namespace, processedState, currentState);
    }

    return processedState;
  }

  // 获取状态
  getState(namespace, path) {
    const state = this.state.get(namespace);
    
    if (!state) return undefined;
    if (!path) return state;

    // 支持路径访问 'user.profile.name'
    return path.split('.').reduce((obj, key) => obj?.[key], state);
  }

  // 订阅状态变化
  subscribe(namespace, callback, options = {}) {
    const { path, immediate = false } = options;
    
    if (!this.subscribers.has(namespace)) {
      this.subscribers.set(namespace, []);
    }

    const subscription = {
      callback,
      path,
      id: Math.random().toString(36).substr(2, 9)
    };

    this.subscribers.get(namespace).push(subscription);

    // 立即调用回调
    if (immediate) {
      const state = this.getState(namespace, path);
      callback(state, undefined, namespace);
    }

    // 返回取消订阅函数
    return () => {
      const subs = this.subscribers.get(namespace);
      const index = subs.findIndex(sub => sub.id === subscription.id);
      if (index > -1) {
        subs.splice(index, 1);
      }
    };
  }

  // 清理命名空间
  clearNamespace(namespace) {
    this.state.delete(namespace);
    this.subscribers.delete(namespace);
  }

  // 获取所有命名空间
  getNamespaces() {
    return Array.from(this.state.keys());
  }

  _notifySubscribers(namespace, newState, oldState) {
    const subscribers = this.subscribers.get(namespace) || [];
    
    subscribers.forEach(({ callback, path }) => {
      try {
        const newValue = path ? this._getValueByPath(newState, path) : newState;
        const oldValue = path ? this._getValueByPath(oldState, path) : oldState;
        
        // 只在值实际改变时通知
        if (JSON.stringify(newValue) !== JSON.stringify(oldValue)) {
          callback(newValue, oldValue, namespace);
        }
      } catch (error) {
        console.error(`Subscriber callback failed for ${namespace}:`, error);
      }
    });
  }

  _getValueByPath(obj, path) {
    return path.split('.').reduce((current, key) => current?.[key], obj);
  }
}

// 全局状态存储
const microfrontendStore = new MicrofrontendStore();

// 添加日志中间件
microfrontendStore.use(async (context) => {
  console.log('State change:', context);
  return context.newState;
});

// 使用示例
// 用户应用
microfrontendStore.setState('user', {
  isLoggedIn: true,
  profile: {
    id: '123',
    name: 'John Doe',
    email: 'john@example.com'
  }
});

// 购物车应用监听用户状态
const unsubscribe = microfrontendStore.subscribe('user', (userState) => {
  if (userState.isLoggedIn) {
    console.log('User logged in, loading cart for:', userState.profile.name);
    // 加载用户购物车
  }
}, { immediate: true });

// 产品应用只监听用户ID
microfrontendStore.subscribe('user', (userId) => {
  console.log('User ID changed:', userId);
  // 更新推荐产品
}, { path: 'profile.id' });
```

## 路由管理

### 微前端路由系统

```javascript
// 微前端路由管理器
class MicrofrontendRouter {
  constructor() {
    this.routes = new Map();
    this.activeApplications = new Set();
    this.currentLocation = this._getCurrentLocation();
    this.beforeRouteChangeHooks = [];
    this.afterRouteChangeHooks = [];
    
    this._setupEventListeners();
  }

  // 注册路由
  registerRoute(pattern, config) {
    this.routes.set(pattern, {
      ...config,
      pattern,
      regex: this._patternToRegex(pattern),
      active: false
    });
  }

  // 导航到指定路径
  async navigate(path, options = {}) {
    const { replace = false, state = null } = options;
    
    // 执行前置钩子
    for (const hook of this.beforeRouteChangeHooks) {
      const result = await hook(path, this.currentLocation);
      if (result === false) {
        console.log('Navigation cancelled by beforeRouteChange hook');
        return false;
      }
    }

    // 更新浏览器历史
    if (replace) {
      history.replaceState(state, '', path);
    } else {
      history.pushState(state, '', path);
    }

    // 更新当前位置并重新路由
    await this._handleLocationChange(path);
    
    return true;
  }

  // 添加路由变化钩子
  beforeRouteChange(hook) {
    this.beforeRouteChangeHooks.push(hook);
  }

  afterRouteChange(hook) {
    this.afterRouteChangeHooks.push(hook);
  }

  // 获取当前激活的应用
  getActiveApplications() {
    return Array.from(this.activeApplications);
  }

  async _handleLocationChange(newLocation = this._getCurrentLocation()) {
    const oldLocation = this.currentLocation;
    this.currentLocation = newLocation;

    // 计算需要激活和停用的应用
    const { toActivate, toDeactivate } = this._calculateApplicationChanges();

    // 停用应用
    for (const appName of toDeactivate) {
      await this._deactivateApplication(appName);
    }

    // 激活应用
    for (const appName of toActivate) {
      await this._activateApplication(appName);
    }

    // 执行后置钩子
    for (const hook of this.afterRouteChangeHooks) {
      await hook(newLocation, oldLocation);
    }
  }

  _calculateApplicationChanges() {
    const newActiveApps = new Set();
    
    // 检查每个路由是否匹配当前位置
    for (const [pattern, route] of this.routes) {
      if (route.regex.test(this.currentLocation)) {
        newActiveApps.add(route.application);
      }
    }

    const toActivate = new Set([...newActiveApps].filter(app => !this.activeApplications.has(app)));
    const toDeactivate = new Set([...this.activeApplications].filter(app => !newActiveApps.has(app)));

    return { toActivate, toDeactivate };
  }

  async _activateApplication(appName) {
    try {
      const route = this._findRouteByApplication(appName);
      if (!route) return;

      console.log(`Activating application: ${appName}`);
      
      // 加载应用模块
      let appModule;
      if (typeof route.loadApp === 'function') {
        appModule = await route.loadApp();
      } else if (typeof route.loadApp === 'string') {
        appModule = await import(route.loadApp);
      }

      // 挂载应用
      if (appModule && appModule.mount) {
        const mountProps = {
          domElement: route.domElementGetter ? route.domElementGetter() : document.getElementById(route.containerId),
          currentLocation: this.currentLocation,
          routeParams: this._extractRouteParams(route.pattern, this.currentLocation)
        };

        await appModule.mount(mountProps);
        route.mountedModule = appModule;
      }

      this.activeApplications.add(appName);
    } catch (error) {
      console.error(`Failed to activate application ${appName}:`, error);
    }
  }

  async _deactivateApplication(appName) {
    try {
      const route = this._findRouteByApplication(appName);
      if (!route || !route.mountedModule) return;

      console.log(`Deactivating application: ${appName}`);

      // 卸载应用
      if (route.mountedModule.unmount) {
        await route.mountedModule.unmount();
      }

      route.mountedModule = null;
      this.activeApplications.delete(appName);
    } catch (error) {
      console.error(`Failed to deactivate application ${appName}:`, error);
    }
  }

  _findRouteByApplication(appName) {
    for (const route of this.routes.values()) {
      if (route.application === appName) {
        return route;
      }
    }
    return null;
  }

  _extractRouteParams(pattern, path) {
    const paramNames = pattern.match(/:(\w+)/g)?.map(param => param.slice(1)) || [];
    const matches = path.match(this._patternToRegex(pattern));
    
    if (!matches) return {};

    const params = {};
    paramNames.forEach((name, index) => {
      params[name] = matches[index + 1];
    });

    return params;
  }

  _patternToRegex(pattern) {
    const escaped = pattern.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const withParams = escaped.replace(/\\:(\w+)/g, '([^/]+)');
    return new RegExp(`^${withParams}$`);
  }

  _getCurrentLocation() {
    return window.location.pathname + window.location.search + window.location.hash;
  }

  _setupEventListeners() {
    // 监听浏览器历史变化
    window.addEventListener('popstate', () => {
      this._handleLocationChange();
    });

    // 拦截链接点击
    document.addEventListener('click', (event) => {
      if (event.target.tagName === 'A' && event.target.origin === window.location.origin) {
        event.preventDefault();
        this.navigate(event.target.pathname + event.target.search + event.target.hash);
      }
    });
  }
}

// 使用示例
const router = new MicrofrontendRouter();

// 注册微前端路由
router.registerRoute('/users/*', {
  application: 'user-app',
  loadApp: () => import('user-app/UserApp'),
  containerId: 'user-app-container'
});

router.registerRoute('/products/:category', {
  application: 'product-app',
  loadApp: 'product-app/ProductApp',
  domElementGetter: () => document.querySelector('#product-container')
});

router.registerRoute('/cart', {
  application: 'cart-app',
  loadApp: () => import('cart-app/CartApp'),
  containerId: 'cart-app-container'
});

// 添加路由守卫
router.beforeRouteChange(async (to, from) => {
  if (to.startsWith('/admin') && !isUserAdmin()) {
    console.log('Access denied to admin area');
    return false;
  }
  return true;
});

// 启动路由
router._handleLocationChange();
```

## 性能优化

### 资源共享与优化

```javascript
// 共享资源管理器
class SharedResourceManager {
  constructor() {
    this.sharedLibraries = new Map();
    this.loadingPromises = new Map();
    this.dependencyGraph = new Map();
  }

  // 注册共享库
  registerSharedLibrary(name, config) {
    this.sharedLibraries.set(name, {
      name,
      version: config.version,
      url: config.url,
      eager: config.eager || false,
      singleton: config.singleton || false,
      requiredVersion: config.requiredVersion,
      shareScope: config.shareScope || 'default',
      loaded: false,
      module: null
    });

    if (config.eager) {
      this.preloadLibrary(name);
    }
  }

  // 预加载库
  async preloadLibrary(name) {
    if (this.loadingPromises.has(name)) {
      return this.loadingPromises.get(name);
    }

    const library = this.sharedLibraries.get(name);
    if (!library || library.loaded) {
      return library?.module;
    }

    const loadingPromise = this._loadLibrary(library);
    this.loadingPromises.set(name, loadingPromise);

    try {
      const module = await loadingPromise;
      library.loaded = true;
      library.module = module;
      this.loadingPromises.delete(name);
      return module;
    } catch (error) {
      this.loadingPromises.delete(name);
      throw error;
    }
  }

  // 获取共享库
  async getSharedLibrary(name, requiredVersion) {
    const library = this.sharedLibraries.get(name);
    
    if (!library) {
      throw new Error(`Shared library ${name} not found`);
    }

    // 版本兼容性检查
    if (requiredVersion && !this._isVersionCompatible(library.version, requiredVersion)) {
      console.warn(`Version mismatch for ${name}: required ${requiredVersion}, available ${library.version}`);
    }

    // 单例检查
    if (library.singleton && library.loaded) {
      return library.module;
    }

    return this.preloadLibrary(name);
  }

  // 分析依赖关系
  analyzeDependencies(appName, dependencies) {
    this.dependencyGraph.set(appName, dependencies);
    
    // 检测循环依赖
    const visited = new Set();
    const recursionStack = new Set();
    
    const hasCycle = (node) => {
      if (recursionStack.has(node)) return true;
      if (visited.has(node)) return false;

      visited.add(node);
      recursionStack.add(node);

      const deps = this.dependencyGraph.get(node) || [];
      for (const dep of deps) {
        if (hasCycle(dep)) return true;
      }

      recursionStack.delete(node);
      return false;
    };

    if (hasCycle(appName)) {
      console.warn(`Circular dependency detected for application: ${appName}`);
    }
  }

  // 预加载应用依赖
  async preloadDependencies(appName) {
    const dependencies = this.dependencyGraph.get(appName) || [];
    const loadPromises = dependencies.map(dep => this.preloadLibrary(dep));
    
    try {
      await Promise.allSettled(loadPromises);
    } catch (error) {
      console.error(`Failed to preload dependencies for ${appName}:`, error);
    }
  }

  _loadLibrary(library) {
    if (library.url.startsWith('http')) {
      // 远程模块
      return import(library.url);
    } else {
      // 本地模块
      return import(library.url);
    }
  }

  _isVersionCompatible(available, required) {
    // 简化的语义版本检查
    const parseVersion = (version) => version.replace(/[^\d.]/g, '').split('.').map(Number);
    const availableVersion = parseVersion(available);
    const requiredVersion = parseVersion(required);

    // 主版本必须匹配
    return availableVersion[0] === requiredVersion[0] && 
           availableVersion[1] >= requiredVersion[1];
  }
}

// 缓存优化
class MicrofrontendCache {
  constructor() {
    this.moduleCache = new Map();
    this.resourceCache = new Map();
    this.cacheStats = {
      hits: 0,
      misses: 0,
      evictions: 0
    };
    this.maxCacheSize = 50;
  }

  // 缓存模块
  cacheModule(url, module, metadata = {}) {
    if (this.moduleCache.size >= this.maxCacheSize) {
      this._evictOldestModule();
    }

    this.moduleCache.set(url, {
      module,
      timestamp: Date.now(),
      accessCount: 0,
      size: this._estimateSize(module),
      ...metadata
    });
  }

  // 获取缓存的模块
  getCachedModule(url) {
    const cached = this.moduleCache.get(url);
    
    if (cached) {
      cached.accessCount++;
      cached.lastAccessed = Date.now();
      this.cacheStats.hits++;
      return cached.module;
    }

    this.cacheStats.misses++;
    return null;
  }

  // 预取资源
  async prefetchResource(url, priority = 'low') {
    if (this.resourceCache.has(url)) {
      return this.resourceCache.get(url);
    }

    const link = document.createElement('link');
    link.rel = 'prefetch';
    link.href = url;
    link.fetchPriority = priority;
    
    document.head.appendChild(link);

    // 创建预取Promise
    const prefetchPromise = new Promise((resolve, reject) => {
      link.onload = () => resolve(url);
      link.onerror = () => reject(new Error(`Failed to prefetch ${url}`));
    });

    this.resourceCache.set(url, prefetchPromise);
    return prefetchPromise;
  }

  _evictOldestModule() {
    let oldestUrl = null;
    let oldestTime = Date.now();

    for (const [url, cached] of this.moduleCache) {
      if (cached.lastAccessed < oldestTime) {
        oldestTime = cached.lastAccessed;
        oldestUrl = url;
      }
    }

    if (oldestUrl) {
      this.moduleCache.delete(oldestUrl);
      this.cacheStats.evictions++;
    }
  }

  _estimateSize(obj) {
    return JSON.stringify(obj).length;
  }

  getStats() {
    return {
      ...this.cacheStats,
      cacheSize: this.moduleCache.size,
      hitRate: this.cacheStats.hits / (this.cacheStats.hits + this.cacheStats.misses)
    };
  }
}

// 全局实例
const sharedResourceManager = new SharedResourceManager();
const microfrontendCache = new MicrofrontendCache();

// 使用示例
// 注册共享库
sharedResourceManager.registerSharedLibrary('react', {
  version: '18.2.0',
  url: 'https://unpkg.com/react@18/umd/react.production.min.js',
  singleton: true,
  eager: true
});

sharedResourceManager.registerSharedLibrary('lodash', {
  version: '4.17.21',
  url: 'https://unpkg.com/lodash@4.17.21/lodash.min.js',
  singleton: false
});

// 预加载应用依赖
sharedResourceManager.analyzeDependencies('user-app', ['react', 'lodash']);
await sharedResourceManager.preloadDependencies('user-app');
```

通过这些微前端模块化技术，可以构建高度可扩展、可维护的大型前端应用，实现团队自治和技术栈多样化的同时，保持良好的用户体验和系统性能。
