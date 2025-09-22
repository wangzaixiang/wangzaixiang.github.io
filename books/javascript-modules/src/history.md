# 模块化发展历史

## 早期的全局变量时代

在 JavaScript 诞生之初，所有的代码都运行在全局作用域中。

```javascript
// 早期的代码组织方式
var userName = 'John';
var userAge = 25;

function getUserInfo() {
    return userName + ' is ' + userAge + ' years old';
}
```

### 存在的问题

- 全局命名空间污染
- 变量名冲突
- 依赖关系不明确
- 代码难以维护和复用

## 命名空间模式

为了解决全局变量冲突问题，开发者开始使用命名空间模式：

```javascript
// 命名空间模式
var MyApp = {
    models: {},
    views: {},
    controllers: {}
};

MyApp.models.User = function(name) {
    this.name = name;
};
```

## IIFE 模式

立即执行函数表达式（IIFE）提供了更好的封装：

```javascript
// IIFE 模式
var MyModule = (function() {
    var privateVar = 'hidden';
    
    return {
        publicMethod: function() {
            return privateVar;
        }
    };
})();
```

## 模块化规范的诞生

### CommonJS (2009)

- 主要用于服务器端（Node.js）
- 同步加载模块
- 简单易用的 `require` 和 `module.exports`

### AMD (2011)

- 异步模块定义
- 主要用于浏览器端
- RequireJS 是主要实现

### UMD (2011)

- 通用模块定义
- 兼容 CommonJS 和 AMD
- 可在多种环境下运行

## ES6 模块系统 (2015)

2015年，ECMAScript 6 引入了原生的模块系统：

```javascript
// ES6 模块
import { userName } from './user.js';
export default class User {
    constructor(name) {
        this.name = name;
    }
}
```

### 主要特性

- 静态结构
- 编译时确定依赖
- 支持树摇（Tree Shaking）：自动移除未使用的代码，减小打包体积
- 原生浏览器支持

## 现代模块化工具

### 打包工具

- **Webpack (2012)**: 强大的模块打包器，丰富的插件生态
- **Rollup (2015)**: 专注于库打包，优秀的Tree Shaking支持
- **Vite (2020)**: 基于原生 ESM 的构建工具，开发体验极佳
- **Bun (2021)**: 极速的JavaScript运行时和打包器
- **Rolldown (2024)**: Rollup的Rust实现，性能大幅提升

### 转译工具

- **Babel**: 将现代 JavaScript 转换为兼容版本
- **TypeScript**: 添加类型系统的 JavaScript 超集

## 时间线总结

| 年份 | 里程碑 | 说明 |
|------|--------|------|
| 1995 | JavaScript 诞生 | 全局变量时代开始 |
| 2009 | CommonJS 规范 | 服务器端模块化标准 |
| 2011 | AMD/UMD 规范 | 浏览器端异步模块化 |
| 2012 | Webpack 发布 | 模块打包工具革命 |
| 2015 | ES6 模块系统 | JavaScript 原生模块支持 |
| 2020 | Vite 发布 | 原生 ESM 开发体验 |

## 发展趋势

现代 JavaScript 模块化正朝着以下方向发展：

1. **原生 ESM 优先**: 越来越多工具支持原生 ES 模块
2. **性能优化**: 更快的构建速度和更小的包体积
3. **开发体验**: 更好的开发工具和调试支持
4. **标准化**: 各种环境间的模块化差异逐渐缩小

---

**下一章**: [为什么需要模块化](./why-modules.md) →