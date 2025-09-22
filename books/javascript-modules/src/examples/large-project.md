# 大型项目模块组织

本章将探讨如何在大型前端项目中进行模块化架构设计，包括项目结构规划、模块间通信、依赖管理和团队协作等方面的最佳实践。

## 项目背景

我们以一个大型电商平台前端项目为例，该项目具有以下特点：
- 多个团队并行开发
- 功能模块复杂且相互关联
- 需要支持多种业务场景
- 代码库规模超过100万行
- 需要支持微前端架构

## 整体架构设计

### 1. 分层架构

```
大型电商平台
├── 应用层 (Applications)
│   ├── 用户端应用 (Customer App)
│   ├── 商家端应用 (Merchant App)
│   └── 管理端应用 (Admin App)
├── 业务层 (Business)
│   ├── 用户模块 (User)
│   ├── 商品模块 (Product)
│   ├── 订单模块 (Order)
│   ├── 支付模块 (Payment)
│   └── 营销模块 (Marketing)
├── 基础设施层 (Infrastructure)
│   ├── 网络服务 (Network)
│   ├── 状态管理 (State)
│   ├── 路由管理 (Router)
│   └── 缓存服务 (Cache)
└── 通用层 (Common)
    ├── UI组件库 (Components)
    ├── 工具函数 (Utils)
    ├── 常量定义 (Constants)
    └── 类型定义 (Types)
```

### 2. 目录结构设计

```
ecommerce-platform/
├── apps/                          # 应用层
│   ├── customer/                  # 用户端应用
│   ├── merchant/                  # 商家端应用
│   └── admin/                     # 管理端应用
├── packages/                      # 共享包
│   ├── business/                  # 业务模块
│   │   ├── user/
│   │   ├── product/
│   │   ├── order/
│   │   ├── payment/
│   │   └── marketing/
│   ├── infrastructure/            # 基础设施
│   │   ├── network/
│   │   ├── state/
│   │   ├── router/
│   │   └── cache/
│   ├── ui/                        # UI组件库
│   │   ├── components/
│   │   ├── themes/
│   │   └── styles/
│   └── common/                    # 通用工具
│       ├── utils/
│       ├── constants/
│       ├── types/
│       └── hooks/
├── tools/                         # 开发工具
│   ├── build/
│   ├── eslint-config/
│   └── test-utils/
├── docs/                          # 文档
└── scripts/                       # 脚本
```

## 模块设计原则

### 1. 领域驱动设计

每个业务模块按照领域驱动设计(DDD)原则组织：

```typescript
// packages/business/user/
├── src/
│   ├── domain/                    # 领域层
│   │   ├── entities/              # 实体
│   │   │   ├── User.ts
│   │   │   └── Profile.ts
│   │   ├── valueObjects/          # 值对象
│   │   │   ├── Email.ts
│   │   │   └── Phone.ts
│   │   ├── services/              # 领域服务
│   │   │   └── UserService.ts
│   │   └── repositories/          # 仓储接口
│   │       └── UserRepository.ts
│   ├── infrastructure/            # 基础设施层
│   │   ├── repositories/          # 仓储实现
│   │   │   └── ApiUserRepository.ts
│   │   └── services/              # 外部服务
│   │       └── AuthService.ts
│   ├── application/               # 应用层
│   │   ├── useCases/              # 用例
│   │   │   ├── CreateUser.ts
│   │   │   ├── UpdateProfile.ts
│   │   │   └── GetUserById.ts
│   │   └── dtos/                  # 数据传输对象
│   │       ├── CreateUserDto.ts
│   │       └── UpdateProfileDto.ts
│   └── presentation/              # 表示层
│       ├── components/            # 组件
│       ├── hooks/                 # 钩子
│       └── stores/                # 状态管理
└── package.json
```

### 2. 模块边界定义

```typescript
// packages/business/user/src/index.ts
// 只导出应用层和表示层的公共接口
export { CreateUser, UpdateProfile, GetUserById } from './application/useCases';
export { CreateUserDto, UpdateProfileDto } from './application/dtos';
export { UserProfile, UserSettings } from './presentation/components';
export { useUser, useUserProfile } from './presentation/hooks';
export { userStore } from './presentation/stores';

// 类型定义
export type { User, Profile } from './domain/entities';
export type { Email, Phone } from './domain/valueObjects';
```

### 3. 依赖注入配置

```typescript
// packages/business/user/src/container.ts
import { Container } from 'inversify';
import { UserRepository } from './domain/repositories/UserRepository';
import { ApiUserRepository } from './infrastructure/repositories/ApiUserRepository';
import { CreateUser } from './application/useCases/CreateUser';

const userContainer = new Container();

// 绑定依赖
userContainer.bind<UserRepository>('UserRepository').to(ApiUserRepository);
userContainer.bind<CreateUser>('CreateUser').to(CreateUser);

export { userContainer };
```

## 模块间通信策略

### 1. 事件驱动架构

```typescript
// packages/infrastructure/events/src/EventBus.ts
export interface Event {
  type: string;
  payload: any;
  timestamp: Date;
  source: string;
}

export class EventBus {
  private listeners: Map<string, Set<(event: Event) => void>> = new Map();

  subscribe(eventType: string, listener: (event: Event) => void): void {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, new Set());
    }
    this.listeners.get(eventType)!.add(listener);
  }

  unsubscribe(eventType: string, listener: (event: Event) => void): void {
    this.listeners.get(eventType)?.delete(listener);
  }

  publish(event: Event): void {
    const listeners = this.listeners.get(event.type);
    if (listeners) {
      listeners.forEach(listener => listener(event));
    }
  }
}

// 单例实例
export const eventBus = new EventBus();
```

### 2. 跨模块通信示例

```typescript
// packages/business/order/src/application/useCases/CreateOrder.ts
import { eventBus } from '@platform/infrastructure/events';
import { Order } from '../domain/entities/Order';

export class CreateOrder {
  async execute(orderData: CreateOrderDto): Promise<Order> {
    const order = await this.orderRepository.create(orderData);
    
    // 发布订单创建事件
    eventBus.publish({
      type: 'ORDER_CREATED',
      payload: { orderId: order.id, userId: order.userId },
      timestamp: new Date(),
      source: 'order-module'
    });
    
    return order;
  }
}

// packages/business/user/src/application/useCases/UpdateUserPoints.ts
import { eventBus } from '@platform/infrastructure/events';

export class UpdateUserPoints {
  constructor() {
    // 监听订单创建事件
    eventBus.subscribe('ORDER_CREATED', this.handleOrderCreated.bind(this));
  }

  private async handleOrderCreated(event: Event): Promise<void> {
    const { userId } = event.payload;
    await this.userRepository.addPoints(userId, 100);
  }
}
```

## 状态管理架构

### 1. 分层状态管理

```typescript
// packages/infrastructure/state/src/Store.ts
import { configureStore } from '@reduxjs/toolkit';
import { userSlice } from '@platform/business/user';
import { productSlice } from '@platform/business/product';
import { orderSlice } from '@platform/business/order';

export const store = configureStore({
  reducer: {
    // 业务状态
    user: userSlice.reducer,
    product: productSlice.reducer,
    order: orderSlice.reducer,
    
    // 应用状态
    ui: uiSlice.reducer,
    router: routerSlice.reducer,
    
    // 基础设施状态
    network: networkSlice.reducer,
    cache: cacheSlice.reducer
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: ['persist/PERSIST']
      }
    }).concat(
      // 自定义中间件
      eventMiddleware,
      cacheMiddleware,
      loggerMiddleware
    )
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
```

### 2. 模块状态隔离

```typescript
// packages/business/user/src/presentation/stores/userSlice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface UserState {
  currentUser: User | null;
  profile: Profile | null;
  loading: boolean;
  error: string | null;
}

const initialState: UserState = {
  currentUser: null,
  profile: null,
  loading: false,
  error: null
};

export const userSlice = createSlice({
  name: 'user',
  initialState,
  reducers: {
    setUser: (state, action: PayloadAction<User>) => {
      state.currentUser = action.payload;
    },
    setProfile: (state, action: PayloadAction<Profile>) => {
      state.profile = action.payload;
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    setError: (state, action: PayloadAction<string>) => {
      state.error = action.payload;
    }
  }
});

export const { setUser, setProfile, setLoading, setError } = userSlice.actions;
```

## 组件库设计

### 1. 设计系统组件

```typescript
// packages/ui/components/src/Button/Button.tsx
import React from 'react';
import { styled } from '@platform/ui/themes';

export interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'small' | 'medium' | 'large';
  disabled?: boolean;
  loading?: boolean;
  children: React.ReactNode;
  onClick?: () => void;
}

const StyledButton = styled.button<ButtonProps>`
  /* 使用设计系统的token */
  font-family: ${({ theme }) => theme.typography.fontFamily};
  font-size: ${({ theme, size }) => theme.typography.fontSize[size || 'medium']};
  padding: ${({ theme, size }) => theme.spacing.button[size || 'medium']};
  border-radius: ${({ theme }) => theme.borderRadius.medium};
  
  /* 变体样式 */
  ${({ theme, variant }) => {
    switch (variant) {
      case 'primary':
        return `
          background-color: ${theme.colors.primary[500]};
          color: ${theme.colors.white};
          border: none;
        `;
      case 'secondary':
        return `
          background-color: transparent;
          color: ${theme.colors.primary[500]};
          border: 1px solid ${theme.colors.primary[500]};
        `;
      case 'danger':
        return `
          background-color: ${theme.colors.danger[500]};
          color: ${theme.colors.white};
          border: none;
        `;
      default:
        return '';
    }
  }}
`;

export const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  size = 'medium',
  disabled = false,
  loading = false,
  children,
  onClick
}) => {
  return (
    <StyledButton
      variant={variant}
      size={size}
      disabled={disabled || loading}
      onClick={onClick}
    >
      {loading ? 'Loading...' : children}
    </StyledButton>
  );
};
```

### 2. 复合组件模式

```typescript
// packages/ui/components/src/DataTable/DataTable.tsx
import React from 'react';

interface DataTableProps<T> {
  data: T[];
  children: React.ReactNode;
}

interface DataTableHeaderProps {
  children: React.ReactNode;
}

interface DataTableBodyProps {
  children: React.ReactNode;
}

interface DataTableRowProps<T> {
  item: T;
  children: (item: T) => React.ReactNode;
}

function DataTable<T>({ data, children }: DataTableProps<T>) {
  return (
    <table className="data-table">
      {children}
    </table>
  );
}

function DataTableHeader({ children }: DataTableHeaderProps) {
  return (
    <thead>
      <tr>{children}</tr>
    </thead>
  );
}

function DataTableBody<T>({ children }: DataTableBodyProps) {
  return <tbody>{children}</tbody>;
}

function DataTableRow<T>({ item, children }: DataTableRowProps<T>) {
  return <tr>{children(item)}</tr>;
}

// 复合组件导出
DataTable.Header = DataTableHeader;
DataTable.Body = DataTableBody;
DataTable.Row = DataTableRow;

export { DataTable };
```

## 性能优化策略

### 1. 代码分割策略

```typescript
// apps/customer/src/routes/index.tsx
import { lazy, Suspense } from 'react';
import { Routes, Route } from 'react-router-dom';
import { LoadingSpinner } from '@platform/ui/components';

// 路由级别的代码分割
const HomePage = lazy(() => import('../pages/HomePage'));
const ProductPage = lazy(() => import('../pages/ProductPage'));
const OrderPage = lazy(() => import('../pages/OrderPage'));
const UserPage = lazy(() => import('../pages/UserPage'));

export function AppRoutes() {
  return (
    <Suspense fallback={<LoadingSpinner />}>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/products/*" element={<ProductPage />} />
        <Route path="/orders/*" element={<OrderPage />} />
        <Route path="/user/*" element={<UserPage />} />
      </Routes>
    </Suspense>
  );
}
```

### 2. 模块级别的懒加载

```typescript
// packages/business/product/src/index.ts
import { lazy } from 'react';

// 懒加载复杂组件
export const ProductCatalog = lazy(() => import('./presentation/components/ProductCatalog'));
export const ProductDetail = lazy(() => import('./presentation/components/ProductDetail'));

// 立即导出轻量级模块
export { useProduct } from './presentation/hooks/useProduct';
export { productStore } from './presentation/stores/productSlice';
export type { Product, ProductCategory } from './domain/entities';
```

### 3. 缓存策略

```typescript
// packages/infrastructure/cache/src/CacheManager.ts
export class CacheManager {
  private cache = new Map<string, { data: any; expiry: number }>();

  set<T>(key: string, data: T, ttl: number = 5 * 60 * 1000): void {
    this.cache.set(key, {
      data,
      expiry: Date.now() + ttl
    });
  }

  get<T>(key: string): T | null {
    const cached = this.cache.get(key);
    if (!cached) return null;

    if (Date.now() > cached.expiry) {
      this.cache.delete(key);
      return null;
    }

    return cached.data as T;
  }

  invalidate(pattern: string): void {
    const regex = new RegExp(pattern);
    for (const key of this.cache.keys()) {
      if (regex.test(key)) {
        this.cache.delete(key);
      }
    }
  }
}

export const cacheManager = new CacheManager();
```

## 构建和部署

### 1. Monorepo构建配置

```javascript
// turbo.json
{
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", ".next/**"]
    },
    "test": {
      "dependsOn": ["^build"],
      "outputs": ["coverage/**"]
    },
    "lint": {
      "outputs": []
    },
    "dev": {
      "cache": false
    }
  }
}
```

### 2. 微前端部署策略

```typescript
// tools/build/webpack.config.js
const ModuleFederationPlugin = require('@module-federation/webpack');

module.exports = {
  plugins: [
    new ModuleFederationPlugin({
      name: 'customer_app',
      filename: 'remoteEntry.js',
      exposes: {
        './App': './src/App',
        './routes': './src/routes'
      },
      shared: {
        react: { singleton: true },
        'react-dom': { singleton: true },
        '@platform/ui': { singleton: true }
      }
    })
  ]
};
```

## 团队协作

### 1. 模块所有权

```yaml
# .github/CODEOWNERS
# 业务模块所有权
/packages/business/user/ @user-team
/packages/business/product/ @product-team
/packages/business/order/ @order-team
/packages/business/payment/ @payment-team

# 基础设施
/packages/infrastructure/ @platform-team
/packages/ui/ @design-system-team

# 应用
/apps/customer/ @frontend-team
/apps/merchant/ @merchant-team
/apps/admin/ @admin-team
```

### 2. 开发工作流

```typescript
// scripts/dev-workflow.ts
export class DevWorkflow {
  async runTests(modules: string[]): Promise<void> {
    // 只运行相关模块的测试
    for (const module of modules) {
      await this.runModuleTests(module);
    }
  }

  async buildAffectedModules(changedFiles: string[]): Promise<void> {
    const affectedModules = this.getAffectedModules(changedFiles);
    await this.buildModules(affectedModules);
  }

  private getAffectedModules(changedFiles: string[]): string[] {
    // 基于依赖图分析影响的模块
    return this.dependencyGraph.getAffected(changedFiles);
  }
}
```

## 监控和维护

### 1. 模块健康度监控

```typescript
// tools/monitoring/src/ModuleHealthChecker.ts
export class ModuleHealthChecker {
  checkDependencyHealth(): HealthReport {
    const report: HealthReport = {
      circularDependencies: this.findCircularDependencies(),
      unusedDependencies: this.findUnusedDependencies(),
      outdatedDependencies: this.findOutdatedDependencies(),
      bundleSize: this.analyzeBundleSize()
    };
    return report;
  }

  generateReport(): void {
    const health = this.checkDependencyHealth();
    console.log('模块健康度报告:', health);
  }
}
```

### 2. 性能监控

```typescript
// packages/infrastructure/monitoring/src/PerformanceMonitor.ts
export class PerformanceMonitor {
  trackModuleLoad(moduleName: string): void {
    const startTime = performance.now();
    
    // 监控模块加载时间
    import(moduleName).then(() => {
      const loadTime = performance.now() - startTime;
      this.reportMetric('module_load_time', loadTime, { module: moduleName });
    });
  }

  trackComponentRender(componentName: string, renderTime: number): void {
    this.reportMetric('component_render_time', renderTime, { component: componentName });
  }

  private reportMetric(metric: string, value: number, tags: Record<string, string>): void {
    // 发送到监控系统
    analytics.track(metric, value, tags);
  }
}
```

## 最佳实践总结

### 1. 架构设计原则

- **单一职责**：每个模块只负责一个业务领域
- **依赖倒置**：依赖抽象而不是具体实现
- **开放封闭**：对扩展开放，对修改封闭
- **接口隔离**：客户端不应该依赖它不需要的接口

### 2. 模块化策略

- **清晰的边界**：明确定义模块的输入和输出
- **松耦合**：减少模块间的直接依赖
- **高内聚**：相关功能应该在同一个模块内
- **可测试性**：模块应该易于单独测试

### 3. 团队协作

- **模块所有权**：每个模块有明确的负责团队
- **API契约**：模块间通过稳定的API进行交互
- **版本管理**：使用语义化版本管理模块更新
- **文档维护**：保持API文档和架构文档的更新

### 4. 性能优化

- **代码分割**：按需加载模块和组件
- **缓存策略**：合理使用各种缓存机制
- **包体积优化**：避免重复依赖和无用代码
- **运行时优化**：使用虚拟列表、memo等技术

通过这套完整的大型项目模块化方案，我们可以构建出可维护、可扩展、高性能的企业级前端应用。

---

**下一章**: [模块懒加载实现](lazy-loading.md) →
