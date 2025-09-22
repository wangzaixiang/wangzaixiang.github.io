# AMD模块系统基础

AMD（Asynchronous Module Definition，异步模块定义）是一种为浏览器环境设计的JavaScript模块格式。它支持异步加载、依赖管理和模块化开发，是早期解决浏览器端模块化问题的重要方案。

## 什么是AMD

AMD是一个用于定义模块的JavaScript API，具有以下特点：

- **异步加载**: 支持模块的异步加载，不阻塞页面渲染
- **依赖管理**: 明确声明模块依赖，自动处理加载顺序
- **浏览器友好**: 专为浏览器环境设计，无需构建工具
- **插件系统**: 支持加载各种类型的资源
- **配置灵活**: 提供丰富的配置选项

## AMD API

### 1. define函数

`define`是AMD的核心函数，用于定义模块：

```javascript
// 基本语法
define(id?, dependencies?, factory);
```

#### 无依赖模块

```javascript
// math.js - 简单的数学工具模块
define(function() {
    function add(a, b) {
        return a + b;
    }
    
    function multiply(a, b) {
        return a * b;
    }
    
    // 返回模块的公共接口
    return {
        add: add,
        multiply: multiply,
        PI: 3.14159
    };
});
```

#### 有依赖的模块

```javascript
// calculator.js - 依赖math模块的计算器
define(['math'], function(math) {
    function calculate(operation, a, b) {
        switch(operation) {
            case 'add':
                return math.add(a, b);
            case 'multiply':
                return math.multiply(a, b);
            case 'circle-area':
                return math.PI * a * a;
            default:
                throw new Error('Unknown operation: ' + operation);
        }
    }
    
    return {
        calculate: calculate
    };
});
```

#### 命名模块

```javascript
// 指定模块ID
define('utils/string', [], function() {
    function capitalize(str) {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }
    
    function trim(str) {
        return str.replace(/^\s+|\s+$/g, '');
    }
    
    return {
        capitalize: capitalize,
        trim: trim
    };
});
```

### 2. require函数

`require`用于动态加载和使用模块：

```javascript
// 异步加载模块
require(['calculator', 'utils/string'], function(calc, stringUtils) {
    var result = calc.calculate('add', 5, 3);
    var message = stringUtils.capitalize('result is: ' + result);
    console.log(message); // "Result is: 8"
});
```

#### 错误处理

```javascript
require(['module1', 'module2'], 
    function(mod1, mod2) {
        // 成功回调
        console.log('Modules loaded successfully');
    },
    function(error) {
        // 错误回调
        console.error('Failed to load modules:', error);
    }
);
```

### 3. require.config配置

```javascript
require.config({
    // 基础路径
    baseUrl: 'js/lib',
    
    // 路径映射
    paths: {
        'jquery': 'jquery-3.6.0.min',
        'underscore': 'underscore-1.13.1.min',
        'backbone': 'backbone-1.4.0.min',
        'utils': '../utils',
        'components': '../components'
    },
    
    // 模块依赖配置（用于非AMD库）
    shim: {
        'backbone': {
            deps: ['underscore', 'jquery'],
            exports: 'Backbone'
        },
        'underscore': {
            exports: '_'
        }
    },
    
    // 超时设置
    waitSeconds: 15,
    
    // URL参数
    urlArgs: 'bust=' + (new Date()).getTime()
});
```

## 常见模式

### 1. 简单工厂模式

```javascript
// logger.js
define(function() {
    function createLogger(name) {
        return {
            name: name,
            log: function(message) {
                console.log('[' + this.name + '] ' + message);
            },
            error: function(message) {
                console.error('[' + this.name + '] ERROR: ' + message);
            }
        };
    }
    
    return {
        create: createLogger
    };
});
```

### 2. 单例模式

```javascript
// config.js
define(function() {
    var instance = null;
    
    function Config() {
        if (instance) {
            return instance;
        }
        
        this.settings = {
            apiUrl: '/api',
            timeout: 5000,
            debug: false
        };
        
        instance = this;
        return this;
    }
    
    Config.prototype.get = function(key) {
        return this.settings[key];
    };
    
    Config.prototype.set = function(key, value) {
        this.settings[key] = value;
    };
    
    return new Config();
});
```

### 3. 模块继承

```javascript
// base-view.js
define(function() {
    function BaseView(element) {
        this.element = element;
        this.initialize();
    }
    
    BaseView.prototype.initialize = function() {
        // 基础初始化逻辑
    };
    
    BaseView.prototype.render = function() {
        throw new Error('render method must be implemented');
    };
    
    BaseView.prototype.destroy = function() {
        this.element = null;
    };
    
    return BaseView;
});

// user-view.js
define(['base-view'], function(BaseView) {
    function UserView(element, user) {
        BaseView.call(this, element);
        this.user = user;
    }
    
    // 继承BaseView
    UserView.prototype = Object.create(BaseView.prototype);
    UserView.prototype.constructor = UserView;
    
    UserView.prototype.render = function() {
        this.element.innerHTML = '<h1>' + this.user.name + '</h1>';
        return this;
    };
    
    return UserView;
});
```

### 4. 插件模式

```javascript
// text插件 - 加载文本文件
define('text', function() {
    return {
        load: function(name, req, onload, config) {
            var xhr = new XMLHttpRequest();
            xhr.open('GET', name, true);
            xhr.onreadystatechange = function() {
                if (xhr.readyState === 4) {
                    if (xhr.status === 200) {
                        onload(xhr.responseText);
                    } else {
                        onload.error(xhr);
                    }
                }
            };
            xhr.send();
        }
    };
});

// 使用text插件
define(['text!templates/user.html'], function(userTemplate) {
    return {
        render: function(user) {
            return userTemplate.replace('{{name}}', user.name);
        }
    };
});
```

## 实际应用示例

### 1. 模块化应用结构

```javascript
// app.js - 应用入口
require.config({
    baseUrl: 'js',
    paths: {
        'jquery': 'lib/jquery-3.6.0.min',
        'router': 'app/router',
        'views': 'app/views',
        'models': 'app/models',
        'utils': 'app/utils'
    }
});

require(['jquery', 'router'], function($, Router) {
    $(document).ready(function() {
        var router = new Router();
        router.start();
    });
});
```

```javascript
// app/router.js
define(['jquery', 'views/home', 'views/user'], function($, HomeView, UserView) {
    function Router() {
        this.routes = {
            '': 'home',
            'user/:id': 'user'
        };
        this.views = {
            home: HomeView,
            user: UserView
        };
    }
    
    Router.prototype.start = function() {
        this.bindEvents();
        this.navigate(window.location.hash);
    };
    
    Router.prototype.bindEvents = function() {
        var self = this;
        $(window).on('hashchange', function() {
            self.navigate(window.location.hash);
        });
    };
    
    Router.prototype.navigate = function(hash) {
        var route = hash.replace('#', '');
        var viewName = this.matchRoute(route);
        if (viewName && this.views[viewName]) {
            var View = this.views[viewName];
            new View().render();
        }
    };
    
    Router.prototype.matchRoute = function(route) {
        // 简化的路由匹配
        for (var pattern in this.routes) {
            if (pattern === route || pattern === '') {
                return this.routes[pattern];
            }
        }
        return null;
    };
    
    return Router;
});
```

### 2. 组件化开发

```javascript
// components/modal.js
define(['jquery'], function($) {
    function Modal(options) {
        this.options = $.extend({
            title: '',
            content: '',
            closable: true,
            width: 400,
            height: 300
        }, options);
        
        this.element = null;
        this.isVisible = false;
    }
    
    Modal.prototype.create = function() {
        if (this.element) {
            return this;
        }
        
        this.element = $('<div class="modal">')
            .css({
                position: 'fixed',
                top: '50%',
                left: '50%',
                width: this.options.width,
                height: this.options.height,
                marginTop: -this.options.height / 2,
                marginLeft: -this.options.width / 2,
                backgroundColor: 'white',
                border: '1px solid #ccc',
                boxShadow: '0 4px 8px rgba(0,0,0,0.1)',
                zIndex: 1000,
                display: 'none'
            });
        
        var header = $('<div class="modal-header">')
            .css({
                padding: '10px',
                borderBottom: '1px solid #eee',
                fontWeight: 'bold'
            })
            .text(this.options.title);
        
        var content = $('<div class="modal-content">')
            .css({
                padding: '10px',
                height: this.options.height - 80
            })
            .html(this.options.content);
        
        this.element.append(header, content);
        
        if (this.options.closable) {
            var closeBtn = $('<button>×</button>')
                .css({
                    position: 'absolute',
                    top: '5px',
                    right: '10px',
                    border: 'none',
                    background: 'none',
                    fontSize: '18px',
                    cursor: 'pointer'
                })
                .click(this.hide.bind(this));
            header.append(closeBtn);
        }
        
        $('body').append(this.element);
        return this;
    };
    
    Modal.prototype.show = function() {
        if (!this.element) {
            this.create();
        }
        this.element.fadeIn();
        this.isVisible = true;
        return this;
    };
    
    Modal.prototype.hide = function() {
        if (this.element) {
            this.element.fadeOut();
        }
        this.isVisible = false;
        return this;
    };
    
    Modal.prototype.destroy = function() {
        if (this.element) {
            this.element.remove();
            this.element = null;
        }
        this.isVisible = false;
        return this;
    };
    
    return Modal;
});

// 使用Modal组件
require(['components/modal'], function(Modal) {
    var modal = new Modal({
        title: 'Welcome',
        content: '<p>Welcome to our application!</p>',
        width: 500,
        height: 200
    });
    
    $('#open-modal').click(function() {
        modal.show();
    });
});
```

### 3. 数据模型

```javascript
// models/user.js
define(['utils/ajax'], function(ajax) {
    function User(data) {
        this.id = data.id || null;
        this.name = data.name || '';
        this.email = data.email || '';
        this.createdAt = data.createdAt || null;
    }
    
    User.prototype.save = function() {
        var url = this.id ? '/api/users/' + this.id : '/api/users';
        var method = this.id ? 'PUT' : 'POST';
        
        return ajax.request({
            url: url,
            method: method,
            data: this.toJSON()
        }).then(function(response) {
            this.id = response.id;
            return this;
        }.bind(this));
    };
    
    User.prototype.destroy = function() {
        if (!this.id) {
            return Promise.reject(new Error('Cannot delete user without ID'));
        }
        
        return ajax.request({
            url: '/api/users/' + this.id,
            method: 'DELETE'
        });
    };
    
    User.prototype.toJSON = function() {
        return {
            id: this.id,
            name: this.name,
            email: this.email,
            createdAt: this.createdAt
        };
    };
    
    // 静态方法
    User.findById = function(id) {
        return ajax.request({
            url: '/api/users/' + id,
            method: 'GET'
        }).then(function(data) {
            return new User(data);
        });
    };
    
    User.findAll = function() {
        return ajax.request({
            url: '/api/users',
            method: 'GET'
        }).then(function(data) {
            return data.map(function(userData) {
                return new User(userData);
            });
        });
    };
    
    return User;
});
```

## AMD vs 其他模块系统

### 1. AMD vs CommonJS

```javascript
// CommonJS (同步)
var math = require('./math');
var result = math.add(2, 3);

// AMD (异步)
define(['math'], function(math) {
    var result = math.add(2, 3);
});
```

### 2. AMD vs ES模块

```javascript
// ES模块
import { add } from './math.js';
const result = add(2, 3);

// AMD
define(['math'], function(math) {
    var result = math.add(2, 3);
});
```

## 优缺点分析

### 优点

- ✅ **异步加载**: 不阻塞页面渲染，提升用户体验
- ✅ **依赖管理**: 自动处理模块依赖顺序
- ✅ **浏览器原生**: 无需预编译，直接在浏览器中运行
- ✅ **插件系统**: 支持加载各种资源类型
- ✅ **配置灵活**: 丰富的配置选项满足不同需求

### 缺点

- ❌ **语法复杂**: 相比ES模块语法较为繁琐
- ❌ **回调嵌套**: 复杂依赖可能导致回调地狱
- ❌ **性能开销**: 运行时依赖解析有一定性能成本
- ❌ **调试困难**: 异步加载增加调试复杂度
- ❌ **生态衰落**: 现代开发更倾向于ES模块

## 总结

AMD作为早期的模块化解决方案，为浏览器端JavaScript模块化开发做出了重要贡献：

- 🎯 **适用场景**: 需要浏览器端异步加载的传统项目
- 🎯 **学习价值**: 理解模块化发展历程和设计思想
- 🎯 **现状**: 逐渐被ES模块和现代构建工具取代

虽然AMD在现代开发中使用较少，但理解其设计原理对于掌握JavaScript模块化发展历程仍然很有价值。

---

**下一章**: [RequireJS详解](./requirejs.md) →
