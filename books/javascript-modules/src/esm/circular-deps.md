# 循环依赖处理

循环依赖是模块化开发中常见但复杂的问题。当两个或多个模块相互依赖时，就形成了循环依赖。ES模块系统具有处理循环依赖的能力，但理解其机制并采用正确的设计模式对于构建健壮的应用至关重要。

## 什么是循环依赖

### 基本概念

```javascript
// 简单的循环依赖示例

// moduleA.js
import { functionB } from './moduleB.js';

export function functionA() {
    console.log('Function A called');
    return functionB();
}

// moduleB.js  
import { functionA } from './moduleA.js';  // 循环依赖！

export function functionB() {
    console.log('Function B called');
    return 'Result from B';
}

// main.js
import { functionA } from './moduleA.js';
functionA(); // 这可能会导致问题
```

### 循环依赖的类型

```javascript
// 1. 直接循环依赖（A → B → A）
// a.js
import { b } from './b.js';
export const a = 'a';

// b.js
import { a } from './a.js';
export const b = 'b';

// 2. 间接循环依赖（A → B → C → A）
// a.js
import { b } from './b.js';
export const a = 'a';

// b.js
import { c } from './c.js';
export const b = 'b';

// c.js
import { a } from './a.js';
export const c = 'c';

// 3. 复杂循环依赖（多个模块形成环）
// user.js
import { Order } from './order.js';
import { Product } from './product.js';

// order.js
import { User } from './user.js';
import { Product } from './product.js';

// product.js
import { User } from './user.js';
import { Order } from './order.js';
```

## ES模块中循环依赖的行为

### 1. 模块加载顺序

```javascript
// 演示ES模块如何处理循环依赖

// a.js
console.log('a.js start');
import { b } from './b.js';
console.log('a.js - b imported:', b);
export const a = 'value-a';
console.log('a.js end');

// b.js
console.log('b.js start');
import { a } from './a.js';
// 注意：在现代Node.js中，直接访问a会抛出ReferenceError
// console.log('b.js - a imported:', a); // ReferenceError: Cannot access 'a' before initialization

// 使用函数延迟访问来避免TDZ错误
export function getA() {
    return a; // 这里可以安全访问，因为调用时a已经初始化
}

export const b = 'value-b';
console.log('b.js end');

// main.js
import { a } from './a.js';
import { getA } from './b.js';
console.log('main.js - a:', a);
console.log('main.js - getA():', getA());

// 执行结果：
// b.js start
// b.js end
// a.js start
// a.js - b imported: value-b
// a.js end
// main.js - a: value-a
// main.js - getA(): value-a
```

> **重要提示**: 
> - 在现代JavaScript引擎（如Node.js v14+）中，直接访问未初始化的绑定会抛出`ReferenceError`，这是由于Temporal Dead Zone (TDZ)的保护机制
> - 早期的ES模块实现可能返回`undefined`，但现代实现更加严格
> - 推荐使用函数延迟访问或重构代码来避免循环依赖问题

#### 执行顺序解析

你可能会好奇：**为什么执行顺序是 `b.js start` → `a.js start` → `main.js`？**

这是ES模块**深度优先加载策略**的结果：

1. **main.js** 开始执行，遇到 `import { a } from './a.js'`
2. 引擎暂停 main.js，开始加载 **a.js**
3. **a.js** 执行，遇到 `import { b } from './b.js'`
4. 引擎暂停 a.js，开始加载 **b.js**
5. **b.js** 执行，遇到 `import { a } from './a.js'`
6. 引擎检测到循环依赖（a.js 已在加载中），创建未初始化绑定
7. **b.js** 继续执行完成 → 输出 "b.js start" 和 "b.js end"
8. 返回 **a.js** 继续执行 → 输出 "a.js start" 和 "a.js end"
9. 返回 **main.js** 继续执行 → 输出最终结果

**调用栈演示：**
```
时间线    调用栈状态
T1:      [main.js]
T2:      [main.js, a.js]
T3:      [main.js, a.js, b.js]
T4:      [main.js, a.js, b.js] ← 检测循环依赖
T5:      [main.js, a.js] ← b.js 完成
T6:      [main.js] ← a.js 完成
T7:      [] ← main.js 完成
```

这种**后进先出**的执行顺序是ES模块系统的核心特征之一。

### 2. 绑定的活性（Live Bindings）

```javascript
// 展示ES模块活绑定如何帮助处理循环依赖

// counter.js
console.log('counter.js loading');
import { increment } from './utils.js';

export let count = 0;

export function getCount() {
    return count;
}

export function setCount(value) {
    count = value;
}

// 初始化时调用increment
increment();

// utils.js
console.log('utils.js loading');
import { count, setCount } from './counter.js';

export function increment() {
    console.log('increment called, current count:', count); // 初始时可能是0
    setCount(count + 1);
}

// main.js
import { getCount } from './counter.js';
import { increment } from './utils.js';

console.log('Initial count:', getCount()); // 1
increment();
console.log('After increment:', getCount()); // 2
```

## 检测循环依赖

### 1. 静态分析工具

```javascript
// dependency-analyzer.js

class DependencyAnalyzer {
    constructor() {
        this.dependencies = new Map();
        this.visited = new Set();
        this.visiting = new Set();
    }
    
    addDependency(from, to) {
        if (!this.dependencies.has(from)) {
            this.dependencies.set(from, new Set());
        }
        this.dependencies.get(from).add(to);
    }
    
    findCircularDependencies() {
        const cycles = [];
        
        for (const module of this.dependencies.keys()) {
            if (!this.visited.has(module)) {
                const path = [];
                const cycle = this.dfs(module, path);
                if (cycle) {
                    cycles.push(cycle);
                }
            }
        }
        
        return cycles;
    }
    
    dfs(module, path) {
        if (this.visiting.has(module)) {
            // 找到循环依赖
            const cycleStart = path.indexOf(module);
            return path.slice(cycleStart).concat([module]);
        }
        
        if (this.visited.has(module)) {
            return null;
        }
        
        this.visiting.add(module);
        path.push(module);
        
        const dependencies = this.dependencies.get(module) || new Set();
        for (const dep of dependencies) {
            const cycle = this.dfs(dep, [...path]);
            if (cycle) {
                return cycle;
            }
        }
        
        this.visiting.delete(module);
        this.visited.add(module);
        path.pop();
        
        return null;
    }
    
    generateReport() {
        const cycles = this.findCircularDependencies();
        
        if (cycles.length === 0) {
            return 'No circular dependencies found.';
        }
        
        let report = `Found ${cycles.length} circular dependency(ies):\n\n`;
        
        cycles.forEach((cycle, index) => {
            report += `${index + 1}. ${cycle.join(' → ')}\n`;
        });
        
        return report;
    }
}

// 使用示例
const analyzer = new DependencyAnalyzer();

// 添加依赖关系
analyzer.addDependency('user.js', 'order.js');
analyzer.addDependency('order.js', 'product.js');
analyzer.addDependency('product.js', 'user.js');

console.log(analyzer.generateReport());
// 输出: Found 1 circular dependency(ies):
//       1. user.js → order.js → product.js → user.js
```

### 2. 运行时检测

```javascript
// runtime-cycle-detector.js

class RuntimeCycleDetector {
    constructor() {
        this.importStack = [];
        this.importGraph = new Map();
    }
    
    beforeImport(modulePath, currentModule) {
        // 检查是否形成循环
        if (this.importStack.includes(modulePath)) {
            const cycleStart = this.importStack.indexOf(modulePath);
            const cycle = this.importStack.slice(cycleStart).concat([modulePath]);
            
            console.warn('🔄 Circular dependency detected:', cycle.join(' → '));
            
            // 记录到图中
            this.recordCycle(cycle);
            
            return true; // 检测到循环
        }
        
        this.importStack.push(modulePath);
        return false; // 未检测到循环
    }
    
    afterImport(modulePath) {
        const index = this.importStack.indexOf(modulePath);
        if (index !== -1) {
            this.importStack.splice(index, 1);
        }
    }
    
    recordCycle(cycle) {
        const cycleKey = cycle.slice().sort().join('|');
        if (!this.importGraph.has(cycleKey)) {
            this.importGraph.set(cycleKey, {
                cycle: cycle,
                count: 1,
                firstDetected: new Date()
            });
        } else {
            this.importGraph.get(cycleKey).count++;
        }
    }
    
    getReport() {
        const cycles = Array.from(this.importGraph.values());
        
        if (cycles.length === 0) {
            return 'No circular dependencies detected during runtime.';
        }
        
        let report = 'Runtime Circular Dependencies Report:\n\n';
        
        cycles.forEach((info, index) => {
            report += `${index + 1}. ${info.cycle.join(' → ')}\n`;
            report += `   Detected ${info.count} time(s)\n`;
            report += `   First detected: ${info.firstDetected.toISOString()}\n\n`;
        });
        
        return report;
    }
}

// 创建全局检测器实例
const cycleDetector = new RuntimeCycleDetector();

// 模拟导入钩子（实际实现可能需要构建工具支持）
function importWithDetection(modulePath, currentModule) {
    const hasCycle = cycleDetector.beforeImport(modulePath, currentModule);
    
    if (hasCycle) {
        console.log(`⚠️  Proceeding with import despite circular dependency: ${modulePath}`);
    }
    
    // 执行实际导入
    const result = import(modulePath);
    
    cycleDetector.afterImport(modulePath);
    
    return result;
}
```

## 循环依赖的解决方案

### 1. 重构消除循环依赖

```javascript
// 问题：用户和订单模块相互依赖

// 原始设计（有循环依赖）
// user.js
import { Order } from './order.js';

export class User {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
    
    getOrders() {
        return Order.findByUserId(this.id);
    }
}

// order.js
import { User } from './user.js';

export class Order {
    constructor(id, userId, amount) {
        this.id = id;
        this.userId = userId;
        this.amount = amount;
    }
    
    getUser() {
        return User.findById(this.userId);
    }
    
    static findByUserId(userId) {
        // 查找逻辑
        return [];
    }
}

// 解决方案1：提取到服务层
// user.js
export class User {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
}

// order.js
export class Order {
    constructor(id, userId, amount) {
        this.id = id;
        this.userId = userId;
        this.amount = amount;
    }
}

// user-service.js
import { User } from './user.js';
import { Order } from './order.js';

export class UserService {
    static async getUserWithOrders(userId) {
        const user = await User.findById(userId);
        const orders = await Order.findByUserId(userId);
        
        return {
            user,
            orders
        };
    }
    
    static async getOrderWithUser(orderId) {
        const order = await Order.findById(orderId);
        const user = await User.findById(order.userId);
        
        return {
            order,
            user
        };
    }
}
```

### 2. 依赖注入模式

```javascript
// dependency-injection.js

// 用户仓库
export class UserRepository {
    async findById(id) {
        // 数据库查询逻辑
        return { id, name: `User ${id}` };
    }
    
    async findByIds(ids) {
        return ids.map(id => ({ id, name: `User ${id}` }));
    }
}

// 订单仓库
export class OrderRepository {
    async findById(id) {
        return { id, userId: 1, amount: 100 };
    }
    
    async findByUserId(userId) {
        return [
            { id: 1, userId, amount: 100 },
            { id: 2, userId, amount: 200 }
        ];
    }
}

// 用户服务（注入订单仓库）
export class UserService {
    constructor(orderRepository) {
        this.orderRepository = orderRepository;
    }
    
    async getUserOrders(userId) {
        return this.orderRepository.findByUserId(userId);
    }
}

// 订单服务（注入用户仓库）
export class OrderService {
    constructor(userRepository) {
        this.userRepository = userRepository;
    }
    
    async getOrderUser(orderId) {
        const order = await this.findById(orderId);
        return this.userRepository.findById(order.userId);
    }
}

// 应用组装器（无循环依赖）
// app.js
import { UserRepository } from './user-repository.js';
import { OrderRepository } from './order-repository.js';
import { UserService } from './user-service.js';
import { OrderService } from './order-service.js';

export function createServices() {
    const userRepository = new UserRepository();
    const orderRepository = new OrderRepository();
    
    const userService = new UserService(orderRepository);
    const orderService = new OrderService(userRepository);
    
    return {
        userService,
        orderService,
        userRepository,
        orderRepository
    };
}
```

### 3. 事件驱动模式

```javascript
// event-driven.js

// 事件总线
class EventBus {
    constructor() {
        this.listeners = new Map();
    }
    
    on(event, callback) {
        if (!this.listeners.has(event)) {
            this.listeners.set(event, []);
        }
        this.listeners.get(event).push(callback);
    }
    
    off(event, callback) {
        const callbacks = this.listeners.get(event);
        if (callbacks) {
            const index = callbacks.indexOf(callback);
            if (index > -1) {
                callbacks.splice(index, 1);
            }
        }
    }
    
    emit(event, data) {
        const callbacks = this.listeners.get(event) || [];
        callbacks.forEach(callback => callback(data));
    }
}

export const eventBus = new EventBus();

// 用户模块（不直接依赖订单）
import { eventBus } from './event-bus.js';

export class User {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
    
    static create(userData) {
        const user = new User(userData.id, userData.name);
        
        // 发布用户创建事件
        eventBus.emit('user:created', user);
        
        return user;
    }
    
    delete() {
        // 发布用户删除事件
        eventBus.emit('user:deleted', { userId: this.id });
    }
}

// 订单模块（通过事件响应用户变化）
import { eventBus } from './event-bus.js';

export class Order {
    constructor(id, userId, amount) {
        this.id = id;
        this.userId = userId;
        this.amount = amount;
    }
    
    static init() {
        // 监听用户删除事件
        eventBus.on('user:deleted', Order.handleUserDeleted);
    }
    
    static handleUserDeleted(data) {
        console.log(`Handling deletion of orders for user ${data.userId}`);
        // 处理用户删除后的订单清理逻辑
    }
}

// 初始化
Order.init();
```

### 4. 延迟导入模式

```javascript
// lazy-import.js

// 用户模块
export class User {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
    
    async getOrders() {
        // 延迟导入订单模块
        const { OrderService } = await import('./order-service.js');
        return OrderService.findByUserId(this.id);
    }
}

// 订单模块
export class Order {
    constructor(id, userId, amount) {
        this.id = id;
        this.userId = userId;
        this.amount = amount;
    }
    
    async getUser() {
        // 延迟导入用户模块
        const { UserService } = await import('./user-service.js');
        return UserService.findById(this.userId);
    }
}

// 订单服务
export class OrderService {
    static findByUserId(userId) {
        // 查找逻辑
        return Promise.resolve([
            new Order(1, userId, 100),
            new Order(2, userId, 200)
        ]);
    }
}

// 用户服务
export class UserService {
    static findById(userId) {
        // 查找逻辑
        return Promise.resolve(new User(userId, `User ${userId}`));
    }
}
```

### 5. 中介者模式

```javascript
// mediator-pattern.js

// 中介者
export class AppMediator {
    constructor() {
        this.userService = null;
        this.orderService = null;
    }
    
    setUserService(userService) {
        this.userService = userService;
    }
    
    setOrderService(orderService) {
        this.orderService = orderService;
    }
    
    async getUserOrders(userId) {
        return this.orderService.findByUserId(userId);
    }
    
    async getOrderUser(orderId) {
        const order = await this.orderService.findById(orderId);
        return this.userService.findById(order.userId);
    }
    
    async getUserWithOrders(userId) {
        const user = await this.userService.findById(userId);
        const orders = await this.orderService.findByUserId(userId);
        
        return { user, orders };
    }
}

// 创建全局中介者实例
export const mediator = new AppMediator();

// 用户服务
import { mediator } from './mediator.js';

export class UserService {
    constructor() {
        mediator.setUserService(this);
    }
    
    async findById(id) {
        return { id, name: `User ${id}` };
    }
    
    async getUserOrders(userId) {
        return mediator.getUserOrders(userId);
    }
}

// 订单服务
import { mediator } from './mediator.js';

export class OrderService {
    constructor() {
        mediator.setOrderService(this);
    }
    
    async findById(id) {
        return { id, userId: 1, amount: 100 };
    }
    
    async findByUserId(userId) {
        return [
            { id: 1, userId, amount: 100 },
            { id: 2, userId, amount: 200 }
        ];
    }
    
    async getOrderUser(orderId) {
        return mediator.getOrderUser(orderId);
    }
}
```

## 最佳实践

### 1. 设计原则

```javascript
// design-principles.js

// 原则1：单向依赖
// ✅ 好的设计
// domain/user.js
export class User {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
}

// domain/order.js
import { User } from './user.js';  // 只依赖User，User不依赖Order

export class Order {
    constructor(id, user, amount) {
        this.id = id;
        this.user = user;  // 组合关系
        this.amount = amount;
    }
}

// 原则2：依赖于抽象而非具体实现
// services/user-service.js
export class UserService {
    constructor(userRepository) {  // 依赖抽象接口
        this.userRepository = userRepository;
    }
    
    async createUser(userData) {
        return this.userRepository.save(userData);
    }
}

// 原则3：分层架构
// 表现层 → 应用层 → 领域层 → 基础设施层
// 上层可以依赖下层，下层不应依赖上层
```

### 2. 模块组织策略

```javascript
// module-organization.js

// 策略1：按特性分组（Feature-based）
// features/
//   ├── user/
//   │   ├── user.model.js
//   │   ├── user.service.js
//   │   └── user.controller.js
//   ├── order/
//   │   ├── order.model.js
//   │   ├── order.service.js
//   │   └── order.controller.js
//   └── shared/
//       ├── event-bus.js
//       └── database.js

// 策略2：按层分组（Layer-based）
// src/
//   ├── models/
//   │   ├── user.js
//   │   └── order.js
//   ├── services/
//   │   ├── user-service.js
//   │   └── order-service.js
//   ├── controllers/
//   │   ├── user-controller.js
//   │   └── order-controller.js
//   └── shared/
//       └── interfaces.js

// 策略3：混合方式
// src/
//   ├── core/              # 核心业务逻辑
//   │   ├── user/
//   │   └── order/
//   ├── infrastructure/    # 基础设施
//   │   ├── database/
//   │   └── external-apis/
//   ├── application/       # 应用服务
//   │   └── use-cases/
//   └── presentation/      # 表现层
//       └── controllers/
```

### 3. 工具和配置

```javascript
// tools-config.js

// ESLint配置检测循环导入
// .eslintrc.js
module.exports = {
    plugins: ['import'],
    rules: {
        'import/no-cycle': ['error', { 
            maxDepth: 10,
            ignoreExternal: true 
        }]
    }
};

// Webpack配置显示循环依赖警告
// webpack.config.js
const CircularDependencyPlugin = require('circular-dependency-plugin');

module.exports = {
    plugins: [
        new CircularDependencyPlugin({
            exclude: /node_modules/,
            failOnError: true,
            allowAsyncCycles: false,
            cwd: process.cwd(),
        })
    ]
};

// 自定义检测脚本
// scripts/check-cycles.js
const madge = require('madge');

madge('./src')
    .then((res) => {
        const circular = res.circular();
        if (circular.length > 0) {
            console.error('Circular dependencies found:');
            circular.forEach((cycle) => {
                console.error('  ', cycle.join(' → '));
            });
            process.exit(1);
        } else {
            console.log('✅ No circular dependencies found');
        }
    })
    .catch((err) => {
        console.error('Error analyzing dependencies:', err);
        process.exit(1);
    });
```

## 总结

循环依赖虽然在ES模块中不会直接导致错误，但会增加代码的复杂性和维护难度：

- ✅ **理解ES模块行为**: 掌握模块加载顺序和活绑定机制
- ✅ **及早检测**: 使用工具在开发阶段发现循环依赖
- ✅ **重构设计**: 通过分层、依赖注入等模式消除循环依赖
- ✅ **采用最佳实践**: 遵循单向依赖和分层架构原则
- ✅ **选择合适策略**: 根据项目特点选择适当的解决方案

良好的模块设计应该避免循环依赖，这不仅提高了代码的可测试性和可维护性，也使应用架构更加清晰和稳定。

---

**下一章**: [CommonJS基础](../cjs/basics.md) →