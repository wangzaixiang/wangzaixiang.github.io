# 模块设计原则

良好的模块设计是构建可维护、可扩展应用的基础。本章将介绍模块化开发中的设计原则和最佳实践。

## SOLID 原则在模块化中的应用

### 1. 单一职责原则 (Single Responsibility Principle)

每个模块应该只有一个变化的理由，即只负责一个功能领域。

#### ❌ 违反单一职责的示例

```javascript
// user-manager.js - 职责过多
export class UserManager {
  // 用户数据管理
  async saveUser(user) {
    await this.validateUser(user);
    await this.sendEmail(user);
    return await this.database.save(user);
  }
  
  // 数据验证
  validateUser(user) {
    if (!user.email || !user.name) {
      throw new Error('Invalid user data');
    }
  }
  
  // 邮件发送
  async sendEmail(user) {
    const emailClient = new EmailClient();
    await emailClient.send(user.email, 'Welcome!');
  }
  
  // 数据库操作
  async findUser(id) {
    return await this.database.find(id);
  }
}
```

#### ✅ 遵循单一职责的改进

```javascript
// user-validator.js - 只负责验证
export class UserValidator {
  static validate(user) {
    if (!user.email || !user.name) {
      throw new Error('Invalid user data');
    }
    
    if (!this.isValidEmail(user.email)) {
      throw new Error('Invalid email format');
    }
  }
  
  static isValidEmail(email) {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
  }
}

// email-service.js - 只负责邮件发送
export class EmailService {
  constructor(config) {
    this.client = new EmailClient(config);
  }
  
  async sendWelcomeEmail(user) {
    const template = await this.loadTemplate('welcome');
    const content = this.renderTemplate(template, { name: user.name });
    return await this.client.send(user.email, 'Welcome!', content);
  }
  
  async loadTemplate(name) {
    // 加载邮件模板
  }
  
  renderTemplate(template, data) {
    // 渲染模板
  }
}

// user-repository.js - 只负责数据持久化
export class UserRepository {
  constructor(database) {
    this.db = database;
  }
  
  async save(user) {
    return await this.db.collection('users').insert(user);
  }
  
  async findById(id) {
    return await this.db.collection('users').findOne({ _id: id });
  }
  
  async findByEmail(email) {
    return await this.db.collection('users').findOne({ email });
  }
}

// user-service.js - 协调各个服务
export class UserService {
  constructor(userRepository, emailService) {
    this.userRepository = userRepository;
    this.emailService = emailService;
  }
  
  async createUser(userData) {
    // 验证用户数据
    UserValidator.validate(userData);
    
    // 检查邮箱是否已存在
    const existingUser = await this.userRepository.findByEmail(userData.email);
    if (existingUser) {
      throw new Error('Email already exists');
    }
    
    // 保存用户
    const user = await this.userRepository.save(userData);
    
    // 发送欢迎邮件
    await this.emailService.sendWelcomeEmail(user);
    
    return user;
  }
}
```

### 2. 开闭原则 (Open/Closed Principle)

模块应该对扩展开放，对修改关闭。

```javascript
// 抽象基类
// payment-processor.js
export class PaymentProcessor {
  async process(payment) {
    throw new Error('Must implement process method');
  }
  
  async validate(payment) {
    if (!payment.amount || payment.amount <= 0) {
      throw new Error('Invalid payment amount');
    }
  }
}

// 具体实现 - 不修改基类，只扩展
// stripe-processor.js
import { PaymentProcessor } from './payment-processor.js';

export class StripeProcessor extends PaymentProcessor {
  constructor(apiKey) {
    super();
    this.stripe = new Stripe(apiKey);
  }
  
  async process(payment) {
    await this.validate(payment);
    
    return await this.stripe.charges.create({
      amount: payment.amount,
      currency: payment.currency,
      source: payment.token
    });
  }
}

// paypal-processor.js
import { PaymentProcessor } from './payment-processor.js';

export class PayPalProcessor extends PaymentProcessor {
  constructor(config) {
    super();
    this.paypal = new PayPal(config);
  }
  
  async process(payment) {
    await this.validate(payment);
    
    return await this.paypal.payment.create({
      intent: 'sale',
      transactions: [{
        amount: {
          total: payment.amount,
          currency: payment.currency
        }
      }]
    });
  }
}

// payment-service.js - 使用策略模式
export class PaymentService {
  constructor() {
    this.processors = new Map();
  }
  
  registerProcessor(type, processor) {
    this.processors.set(type, processor);
  }
  
  async processPayment(type, payment) {
    const processor = this.processors.get(type);
    if (!processor) {
      throw new Error(`Unsupported payment type: ${type}`);
    }
    
    return await processor.process(payment);
  }
}

// 使用示例
const paymentService = new PaymentService();
paymentService.registerProcessor('stripe', new StripeProcessor(apiKey));
paymentService.registerProcessor('paypal', new PayPalProcessor(config));

// 添加新的支付方式不需要修改现有代码
paymentService.registerProcessor('alipay', new AlipayProcessor(config));
```

### 3. 里氏替换原则 (Liskov Substitution Principle)

子类必须能够替换其父类而不影响程序的正确性。

```javascript
// cache-interface.js
export class CacheInterface {
  async get(key) {
    throw new Error('Must implement get method');
  }
  
  async set(key, value, ttl = 3600) {
    throw new Error('Must implement set method');
  }
  
  async delete(key) {
    throw new Error('Must implement delete method');
  }
}

// memory-cache.js
import { CacheInterface } from './cache-interface.js';

export class MemoryCache extends CacheInterface {
  constructor() {
    super();
    this.cache = new Map();
    this.timers = new Map();
  }
  
  async get(key) {
    return this.cache.get(key) || null;
  }
  
  async set(key, value, ttl = 3600) {
    this.cache.set(key, value);
    
    // 清除旧的定时器
    if (this.timers.has(key)) {
      clearTimeout(this.timers.get(key));
    }
    
    // 设置新的过期定时器
    const timer = setTimeout(() => {
      this.cache.delete(key);
      this.timers.delete(key);
    }, ttl * 1000);
    
    this.timers.set(key, timer);
  }
  
  async delete(key) {
    this.cache.delete(key);
    if (this.timers.has(key)) {
      clearTimeout(this.timers.get(key));
      this.timers.delete(key);
    }
  }
}

// redis-cache.js
import { CacheInterface } from './cache-interface.js';

export class RedisCache extends CacheInterface {
  constructor(client) {
    super();
    this.redis = client;
  }
  
  async get(key) {
    return await this.redis.get(key);
  }
  
  async set(key, value, ttl = 3600) {
    return await this.redis.setex(key, ttl, JSON.stringify(value));
  }
  
  async delete(key) {
    return await this.redis.del(key);
  }
}

// user-service.js - 可以无缝替换缓存实现
export class UserService {
  constructor(userRepository, cache) {
    this.userRepository = userRepository;
    this.cache = cache; // 可以是 MemoryCache 或 RedisCache
  }
  
  async getUser(id) {
    const cacheKey = `user:${id}`;
    
    // 先从缓存获取
    let user = await this.cache.get(cacheKey);
    if (user) {
      return JSON.parse(user);
    }
    
    // 缓存未命中，从数据库获取
    user = await this.userRepository.findById(id);
    if (user) {
      await this.cache.set(cacheKey, JSON.stringify(user), 1800);
    }
    
    return user;
  }
}
```

### 4. 接口隔离原则 (Interface Segregation Principle)

不应该强迫客户端依赖于它们不使用的接口。

#### ❌ 违反接口隔离的示例

```javascript
// 臃肿的接口
export class DatabaseService {
  // 用户相关
  async createUser(user) { /* ... */ }
  async updateUser(id, user) { /* ... */ }
  async deleteUser(id) { /* ... */ }
  
  // 订单相关
  async createOrder(order) { /* ... */ }
  async updateOrder(id, order) { /* ... */ }
  async deleteOrder(id) { /* ... */ }
  
  // 产品相关
  async createProduct(product) { /* ... */ }
  async updateProduct(id, product) { /* ... */ }
  async deleteProduct(id) { /* ... */ }
  
  // 报表相关
  async generateUserReport() { /* ... */ }
  async generateOrderReport() { /* ... */ }
  async generateProductReport() { /* ... */ }
}
```

#### ✅ 遵循接口隔离的改进

```javascript
// 细分的接口
// user-operations.js
export class UserOperations {
  constructor(database) {
    this.db = database;
  }
  
  async create(user) {
    return await this.db.collection('users').insert(user);
  }
  
  async update(id, user) {
    return await this.db.collection('users').update({ _id: id }, user);
  }
  
  async delete(id) {
    return await this.db.collection('users').delete({ _id: id });
  }
  
  async findById(id) {
    return await this.db.collection('users').findOne({ _id: id });
  }
}

// order-operations.js
export class OrderOperations {
  constructor(database) {
    this.db = database;
  }
  
  async create(order) {
    return await this.db.collection('orders').insert(order);
  }
  
  async findByUserId(userId) {
    return await this.db.collection('orders').find({ userId });
  }
  
  async updateStatus(id, status) {
    return await this.db.collection('orders').update(
      { _id: id }, 
      { $set: { status, updatedAt: new Date() } }
    );
  }
}

// report-generator.js
export class ReportGenerator {
  constructor(database) {
    this.db = database;
  }
  
  async generateUserReport(startDate, endDate) {
    return await this.db.collection('users').aggregate([
      { $match: { createdAt: { $gte: startDate, $lte: endDate } } },
      { $group: { _id: null, count: { $sum: 1 } } }
    ]);
  }
  
  async generateOrderReport(startDate, endDate) {
    return await this.db.collection('orders').aggregate([
      { $match: { createdAt: { $gte: startDate, $lte: endDate } } },
      { $group: { _id: '$status', count: { $sum: 1 }, total: { $sum: '$amount' } } }
    ]);
  }
}

// 客户端只依赖需要的接口
// user-service.js
import { UserOperations } from './user-operations.js';

export class UserService {
  constructor(database) {
    this.userOps = new UserOperations(database);
  }
  
  async createUser(userData) {
    // 只需要用户操作接口
    return await this.userOps.create(userData);
  }
}
```

### 5. 依赖倒置原则 (Dependency Inversion Principle)

高层模块不应该依赖低层模块，两者都应该依赖于抽象。

```javascript
// 抽象层
// logger-interface.js
export class LoggerInterface {
  info(message, meta = {}) {
    throw new Error('Must implement info method');
  }
  
  error(message, error = null, meta = {}) {
    throw new Error('Must implement error method');
  }
  
  warn(message, meta = {}) {
    throw new Error('Must implement warn method');
  }
}

// notification-interface.js
export class NotificationInterface {
  async send(recipient, message, options = {}) {
    throw new Error('Must implement send method');
  }
}

// 具体实现
// console-logger.js
import { LoggerInterface } from './logger-interface.js';

export class ConsoleLogger extends LoggerInterface {
  info(message, meta = {}) {
    console.log(`[INFO] ${new Date().toISOString()} - ${message}`, meta);
  }
  
  error(message, error = null, meta = {}) {
    console.error(`[ERROR] ${new Date().toISOString()} - ${message}`, {
      error: error?.stack || error,
      ...meta
    });
  }
  
  warn(message, meta = {}) {
    console.warn(`[WARN] ${new Date().toISOString()} - ${message}`, meta);
  }
}

// file-logger.js
import { LoggerInterface } from './logger-interface.js';
import fs from 'fs/promises';

export class FileLogger extends LoggerInterface {
  constructor(logFile) {
    super();
    this.logFile = logFile;
  }
  
  async info(message, meta = {}) {
    await this.writeLog('INFO', message, meta);
  }
  
  async error(message, error = null, meta = {}) {
    await this.writeLog('ERROR', message, { error: error?.stack || error, ...meta });
  }
  
  async warn(message, meta = {}) {
    await this.writeLog('WARN', message, meta);
  }
  
  async writeLog(level, message, meta) {
    const timestamp = new Date().toISOString();
    const logEntry = JSON.stringify({ timestamp, level, message, meta }) + '\n';
    await fs.appendFile(this.logFile, logEntry);
  }
}

// email-notification.js
import { NotificationInterface } from './notification-interface.js';

export class EmailNotification extends NotificationInterface {
  constructor(emailService) {
    super();
    this.emailService = emailService;
  }
  
  async send(recipient, message, options = {}) {
    return await this.emailService.send({
      to: recipient,
      subject: options.subject || 'Notification',
      body: message,
      html: options.html || false
    });
  }
}

// 高层模块依赖抽象
// order-service.js
export class OrderService {
  constructor(orderRepository, logger, notificationService) {
    this.orderRepository = orderRepository;
    this.logger = logger; // 依赖抽象，不是具体实现
    this.notificationService = notificationService; // 依赖抽象
  }
  
  async createOrder(orderData) {
    try {
      this.logger.info('Creating new order', { orderData });
      
      const order = await this.orderRepository.create(orderData);
      
      await this.notificationService.send(
        orderData.customerEmail,
        `Your order ${order.id} has been created successfully`,
        { subject: 'Order Confirmation' }
      );
      
      this.logger.info('Order created successfully', { orderId: order.id });
      return order;
      
    } catch (error) {
      this.logger.error('Failed to create order', error, { orderData });
      throw error;
    }
  }
}

// 依赖注入配置
// app.js
import { OrderService } from './order-service.js';
import { ConsoleLogger } from './console-logger.js';
import { EmailNotification } from './email-notification.js';

// 可以轻松切换不同的实现
const logger = new ConsoleLogger(); // 或 new FileLogger('./app.log')
const notificationService = new EmailNotification(emailService);

const orderService = new OrderService(orderRepository, logger, notificationService);
```

## 模块耦合和内聚

### 高内聚的模块设计

模块内部的元素应该紧密相关，共同完成一个明确的功能。

```javascript
// 高内聚的用户认证模块
// auth-module.js
export class AuthModule {
  constructor(userRepository, tokenService, passwordService) {
    this.userRepository = userRepository;
    this.tokenService = tokenService;
    this.passwordService = passwordService;
  }
  
  // 所有方法都与认证相关
  async login(email, password) {
    const user = await this.userRepository.findByEmail(email);
    if (!user) {
      throw new Error('User not found');
    }
    
    const isValidPassword = await this.passwordService.verify(password, user.hashedPassword);
    if (!isValidPassword) {
      throw new Error('Invalid password');
    }
    
    return this.tokenService.generate(user.id);
  }
  
  async register(userData) {
    const hashedPassword = await this.passwordService.hash(userData.password);
    const user = await this.userRepository.create({
      ...userData,
      hashedPassword
    });
    
    return this.tokenService.generate(user.id);
  }
  
  async verifyToken(token) {
    return this.tokenService.verify(token);
  }
  
  async refreshToken(refreshToken) {
    const userId = await this.tokenService.verifyRefresh(refreshToken);
    return this.tokenService.generate(userId);
  }
  
  async logout(token) {
    return this.tokenService.revoke(token);
  }
}
```

### 低耦合的模块间通信

模块之间的依赖应该最小化，通过明确的接口进行通信。

```javascript
// 事件驱动的低耦合架构
// event-bus.js
export class EventBus {
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
    callbacks.forEach(callback => {
      try {
        callback(data);
      } catch (error) {
        console.error(`Error in event listener for ${event}:`, error);
      }
    });
  }
}

// order-service.js - 发布事件，不直接依赖其他服务
export class OrderService {
  constructor(orderRepository, eventBus) {
    this.orderRepository = orderRepository;
    this.eventBus = eventBus;
  }
  
  async createOrder(orderData) {
    const order = await this.orderRepository.create(orderData);
    
    // 发布事件而不是直接调用其他服务
    this.eventBus.emit('order.created', {
      orderId: order.id,
      customerId: order.customerId,
      amount: order.amount,
      createdAt: order.createdAt
    });
    
    return order;
  }
}

// notification-service.js - 监听事件
export class NotificationService {
  constructor(emailService, eventBus) {
    this.emailService = emailService;
    this.eventBus = eventBus;
    
    // 订阅相关事件
    this.eventBus.on('order.created', this.handleOrderCreated.bind(this));
    this.eventBus.on('user.registered', this.handleUserRegistered.bind(this));
  }
  
  async handleOrderCreated(orderData) {
    await this.emailService.send(
      orderData.customerEmail,
      'Order Confirmation',
      `Your order ${orderData.orderId} has been confirmed.`
    );
  }
  
  async handleUserRegistered(userData) {
    await this.emailService.send(
      userData.email,
      'Welcome!',
      `Welcome to our platform, ${userData.name}!`
    );
  }
}

// analytics-service.js - 独立的分析服务
export class AnalyticsService {
  constructor(analyticsRepository, eventBus) {
    this.analyticsRepository = analyticsRepository;
    this.eventBus = eventBus;
    
    // 订阅所有感兴趣的事件
    this.eventBus.on('order.created', this.trackOrderCreated.bind(this));
    this.eventBus.on('user.registered', this.trackUserRegistered.bind(this));
    this.eventBus.on('user.login', this.trackUserLogin.bind(this));
  }
  
  async trackOrderCreated(orderData) {
    await this.analyticsRepository.recordEvent('order_created', {
      orderId: orderData.orderId,
      amount: orderData.amount,
      timestamp: orderData.createdAt
    });
  }
  
  async trackUserRegistered(userData) {
    await this.analyticsRepository.recordEvent('user_registered', {
      userId: userData.id,
      timestamp: userData.createdAt
    });
  }
  
  async trackUserLogin(loginData) {
    await this.analyticsRepository.recordEvent('user_login', {
      userId: loginData.userId,
      timestamp: loginData.timestamp
    });
  }
}
```

## 模块命名和组织

### 清晰的命名约定

```javascript
// 好的命名约定
// services/user-authentication.service.js
export class UserAuthenticationService { }

// repositories/user.repository.js
export class UserRepository { }

// models/user.model.js
export class User { }

// utils/date.utils.js
export const DateUtils = { };

// config/database.config.js
export const databaseConfig = { };

// types/user.types.js
export interface User { }

// constants/http-status.constants.js
export const HTTP_STATUS = { };
```

### 目录结构组织

```
src/
├── components/           # 可复用组件
│   ├── ui/              # UI组件
│   │   ├── button/
│   │   ├── input/
│   │   └── modal/
│   └── business/        # 业务组件
│       ├── user-profile/
│       └── order-summary/
├── services/            # 业务服务层
│   ├── user.service.js
│   ├── order.service.js
│   └── payment.service.js
├── repositories/        # 数据访问层
│   ├── user.repository.js
│   └── order.repository.js
├── models/             # 数据模型
│   ├── user.model.js
│   └── order.model.js
├── utils/              # 工具函数
│   ├── date.utils.js
│   ├── validation.utils.js
│   └── format.utils.js
├── config/             # 配置文件
│   ├── app.config.js
│   ├── database.config.js
│   └── api.config.js
├── types/              # TypeScript 类型定义
│   ├── user.types.ts
│   └── api.types.ts
├── constants/          # 常量定义
│   ├── http-status.constants.js
│   └── error-codes.constants.js
└── tests/              # 测试文件
    ├── unit/
    ├── integration/
    └── e2e/
```

## 模块版本管理

### 语义化版本控制

```json
// package.json
{
  "name": "@mycompany/user-service",
  "version": "1.2.3",
  "description": "User management service",
  "main": "dist/index.js",
  "exports": {
    ".": {
      "import": "./dist/esm/index.js",
      "require": "./dist/cjs/index.js",
      "types": "./dist/types/index.d.ts"
    },
    "./types": {
      "import": "./dist/esm/types.js",
      "require": "./dist/cjs/types.js",
      "types": "./dist/types/types.d.ts"
    }
  },
  "files": [
    "dist/"
  ],
  "engines": {
    "node": ">=16.0.0"
  }
}
```

### 向后兼容的 API 设计

```javascript
// v1.0.0 - 初始版本
export class UserService {
  async getUser(id) {
    return await this.userRepository.findById(id);
  }
}

// v1.1.0 - 添加新功能，保持向后兼容
export class UserService {
  async getUser(id) {
    return await this.userRepository.findById(id);
  }
  
  // 新增方法
  async getUserWithProfile(id) {
    const user = await this.userRepository.findById(id);
    const profile = await this.profileRepository.findByUserId(id);
    return { ...user, profile };
  }
}

// v2.0.0 - 破坏性变更，新的主版本
export class UserService {
  // 修改方法签名，返回结构变化
  async getUser(id, options = {}) {
    const user = await this.userRepository.findById(id);
    
    if (options.includeProfile) {
      const profile = await this.profileRepository.findByUserId(id);
      return { user, profile };
    }
    
    return { user };
  }
  
  // 废弃的方法标记
  /**
   * @deprecated Use getUser with options.includeProfile instead
   */
  async getUserWithProfile(id) {
    console.warn('getUserWithProfile is deprecated. Use getUser with options.includeProfile instead.');
    return this.getUser(id, { includeProfile: true });
  }
}
```

## 总结

良好的模块设计原则包括：

### 🎯 **核心原则**
- **SOLID原则**: 单一职责、开闭、里氏替换、接口隔离、依赖倒置
- **高内聚低耦合**: 模块内部紧密相关，模块间依赖最小
- **明确的接口**: 清晰的输入输出和职责边界

### 📝 **命名和组织**
- **一致的命名约定**: 清晰表达模块用途
- **合理的目录结构**: 按功能和层次组织
- **语义化版本**: 明确的版本变更策略

### 🔄 **持续改进**
- **定期重构**: 保持代码质量
- **代码审查**: 确保设计原则的执行
- **文档维护**: 保持文档与代码同步

遵循这些设计原则可以构建出易于维护、测试和扩展的模块化应用。

---

**下一章**: [性能优化](./performance.md) →
