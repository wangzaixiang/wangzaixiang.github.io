# RequireJS详解

RequireJS是AMD（Asynchronous Module Definition）规范最流行的实现，也是早期浏览器端模块化开发的重要工具。本章将深入探讨RequireJS的使用方法、配置选项、最佳实践和实际应用。

## RequireJS简介

RequireJS是一个JavaScript文件和模块加载器，具有以下特点：

- **异步加载**: 异步加载JavaScript文件，提升页面性能
- **依赖管理**: 自动解析和加载模块依赖
- **跨浏览器**: 支持所有现代浏览器
- **插件系统**: 丰富的插件生态，支持加载各种资源
- **优化工具**: 提供r.js优化器进行代码合并和压缩

## 安装和基本使用

### 1. 引入RequireJS

```html
<!DOCTYPE html>
<html>
<head>
    <title>RequireJS Demo</title>
</head>
<body>
    <div id="app"></div>
    
    <!-- 引入RequireJS -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/require.js/2.3.6/require.min.js"></script>
    
    <!-- 或者从本地引入 -->
    <!-- <script src="js/lib/require.js"></script> -->
    
    <!-- 指定主模块 -->
    <script src="js/lib/require.js" data-main="js/app"></script>
</body>
</html>
```

### 2. 基本模块定义

```javascript
// js/utils/math.js
define(function() {
    'use strict';
    
    return {
        add: function(a, b) {
            return a + b;
        },
        
        subtract: function(a, b) {
            return a - b;
        },
        
        multiply: function(a, b) {
            return a * b;
        },
        
        divide: function(a, b) {
            if (b === 0) {
                throw new Error('Division by zero');
            }
            return a / b;
        }
    };
});
```

### 3. 使用模块

```javascript
// js/app.js - 主应用文件
require(['utils/math'], function(math) {
    console.log('2 + 3 =', math.add(2, 3));
    console.log('10 - 4 =', math.subtract(10, 4));
    console.log('5 * 6 =', math.multiply(5, 6));
    console.log('15 / 3 =', math.divide(15, 3));
});
```

## 配置系统

### 1. 基础配置

```javascript
// js/config.js
require.config({
    // 设置基础路径
    baseUrl: 'js',
    
    // 路径映射
    paths: {
        // 第三方库
        'jquery': 'lib/jquery-3.6.0.min',
        'underscore': 'lib/underscore-1.13.1.min',
        'backbone': 'lib/backbone-1.4.0.min',
        
        // 应用模块
        'app': 'app',
        'models': 'app/models',
        'views': 'app/views',
        'collections': 'app/collections',
        'templates': '../templates'
    },
    
    // 超时设置（秒）
    waitSeconds: 30,
    
    // URL查询参数（用于缓存破坏）
    urlArgs: 'v=' + (new Date()).getTime()
});

// 启动应用
require(['app/main'], function(main) {
    main.init();
});
```

### 2. Shim配置（兼容非AMD库）

```javascript
require.config({
    baseUrl: 'js',
    
    paths: {
        'jquery': 'lib/jquery-3.6.0.min',
        'underscore': 'lib/underscore-1.13.1.min',
        'backbone': 'lib/backbone-1.4.0.min',
        'bootstrap': 'lib/bootstrap-4.6.0.min',
        'd3': 'lib/d3-7.3.0.min'
    },
    
    // 为非AMD库配置依赖和导出
    shim: {
        'underscore': {
            exports: '_'
        },
        'backbone': {
            deps: ['underscore', 'jquery'],
            exports: 'Backbone'
        },
        'bootstrap': {
            deps: ['jquery']
        },
        'd3': {
            exports: 'd3'
        }
    }
});
```

### 3. 高级配置选项

```javascript
require.config({
    baseUrl: 'js',
    
    // 映射配置 - 为不同上下文使用不同版本
    map: {
        '*': {
            'jquery': 'jquery-3.6.0'
        },
        'legacy-module': {
            'jquery': 'jquery-1.12.4'
        }
    },
    
    // 包配置
    packages: [
        {
            name: 'dojo',
            location: 'lib/dojo',
            main: 'main'
        }
    ],
    
    // 配置文件
    config: {
        'app/config': {
            apiUrl: 'https://api.example.com',
            debug: true
        }
    },
    
    // 强制定义
    enforceDefine: true,
    
    // Node.js适配
    nodeIdCompat: true
});
```

## 插件系统

### 1. text插件 - 加载文本文件

```javascript
// 安装：下载text.js到js/lib/目录

// 配置
require.config({
    paths: {
        'text': 'lib/text'
    }
});

// 使用text插件加载HTML模板
define(['text!templates/user.html'], function(userTemplate) {
    return {
        render: function(user) {
            var html = userTemplate
                .replace('{{name}}', user.name)
                .replace('{{email}}', user.email);
            return html;
        }
    };
});
```

### 2. domReady插件 - DOM就绪

```javascript
// 使用domReady插件
require(['domReady!'], function() {
    // DOM已经准备就绪
    console.log('DOM is ready');
    
    // 可以安全地操作DOM
    document.getElementById('app').innerHTML = 'Hello, World!';
});

// 或者在模块中使用
define(['domReady!', 'jquery'], function(doc, $) {
    // DOM和jQuery都已准备就绪
    return {
        init: function() {
            $('#app').text('Application initialized');
        }
    };
});
```

### 3. i18n插件 - 国际化

```javascript
// 配置i18n
require.config({
    config: {
        i18n: {
            locale: 'zh-cn' // 设置默认语言
        }
    }
});

// 创建语言文件
// nls/messages.js
define({
    root: {
        greeting: 'Hello',
        goodbye: 'Goodbye'
    },
    'zh-cn': true,
    'en-us': true
});

// nls/zh-cn/messages.js
define({
    greeting: '你好',
    goodbye: '再见'
});

// 使用i18n
define(['i18n!nls/messages'], function(messages) {
    return {
        greet: function() {
            return messages.greeting;
        }
    };
});
```

### 4. css插件 - 加载CSS

```javascript
// 使用css插件
define(['css!styles/main.css'], function() {
    // CSS已经加载
    return {
        init: function() {
            console.log('Styles loaded');
        }
    };
});
```

### 5. 自定义插件

```javascript
// plugins/json.js - 加载JSON文件的插件
define(function() {
    return {
        load: function(name, req, onload, config) {
            if (config.isBuild) {
                // 构建时的处理
                onload(null);
                return;
            }
            
            var xhr = new XMLHttpRequest();
            xhr.open('GET', name + '.json', true);
            xhr.onreadystatechange = function() {
                if (xhr.readyState === 4) {
                    if (xhr.status === 200) {
                        try {
                            var data = JSON.parse(xhr.responseText);
                            onload(data);
                        } catch (e) {
                            onload.error(e);
                        }
                    } else {
                        onload.error(new Error('HTTP ' + xhr.status));
                    }
                }
            };
            xhr.send();
        }
    };
});

// 使用自定义插件
define(['json!data/config'], function(config) {
    console.log('Loaded config:', config);
});
```

## 实际项目应用

### 1. 大型单页应用架构

```javascript
// js/app/main.js - 应用入口
define([
    'jquery',
    'backbone',
    'app/router',
    'app/models/user',
    'app/views/layout'
], function($, Backbone, Router, User, LayoutView) {
    
    var App = {
        Models: {},
        Views: {},
        Collections: {},
        Router: null,
        
        init: function() {
            // 初始化布局
            this.layout = new LayoutView({
                el: '#app'
            });
            
            // 初始化路由
            this.router = new Router();
            
            // 启动应用
            this.start();
        },
        
        start: function() {
            // 渲染布局
            this.layout.render();
            
            // 启动Backbone路由
            Backbone.history.start({
                pushState: true,
                root: '/'
            });
            
            console.log('Application started');
        }
    };
    
    return App;
});
```

```javascript
// js/app/router.js - 路由器
define([
    'backbone',
    'app/views/home',
    'app/views/users',
    'app/views/profile'
], function(Backbone, HomeView, UsersView, ProfileView) {
    
    var Router = Backbone.Router.extend({
        routes: {
            '': 'home',
            'users': 'users',
            'users/:id': 'profile',
            '*notfound': 'notFound'
        },
        
        home: function() {
            var view = new HomeView();
            this.showView(view);
        },
        
        users: function() {
            var view = new UsersView();
            this.showView(view);
        },
        
        profile: function(id) {
            var view = new ProfileView({ userId: id });
            this.showView(view);
        },
        
        notFound: function() {
            console.log('404 - Page not found');
        },
        
        showView: function(view) {
            // 清理之前的视图
            if (this.currentView) {
                this.currentView.remove();
            }
            
            // 显示新视图
            this.currentView = view;
            view.render();
        }
    });
    
    return Router;
});
```

### 2. 模块化组件系统

```javascript
// js/components/base-component.js - 基础组件
define(['jquery'], function($) {
    
    function BaseComponent(options) {
        this.options = $.extend(true, {}, this.defaults, options);
        this.element = null;
        this.initialized = false;
        
        this.init();
    }
    
    BaseComponent.prototype = {
        defaults: {},
        
        init: function() {
            this.createElement();
            this.bindEvents();
            this.initialized = true;
        },
        
        createElement: function() {
            throw new Error('createElement must be implemented');
        },
        
        bindEvents: function() {
            // 子类可以重写
        },
        
        render: function() {
            throw new Error('render must be implemented');
        },
        
        destroy: function() {
            if (this.element) {
                this.element.remove();
            }
            this.initialized = false;
        }
    };
    
    return BaseComponent;
});

// js/components/data-table.js - 数据表组件
define([
    'components/base-component',
    'text!templates/data-table.html'
], function(BaseComponent, tableTemplate) {
    
    function DataTable(options) {
        BaseComponent.call(this, options);
    }
    
    // 继承BaseComponent
    DataTable.prototype = Object.create(BaseComponent.prototype);
    DataTable.prototype.constructor = DataTable;
    
    DataTable.prototype.defaults = {
        columns: [],
        data: [],
        sortable: true,
        pageable: true,
        pageSize: 10
    };
    
    DataTable.prototype.createElement = function() {
        this.element = $(tableTemplate);
    };
    
    DataTable.prototype.bindEvents = function() {
        var self = this;
        
        // 排序事件
        if (this.options.sortable) {
            this.element.on('click', 'th[data-sortable]', function() {
                var column = $(this).data('column');
                self.sort(column);
            });
        }
        
        // 分页事件
        if (this.options.pageable) {
            this.element.on('click', '.pagination a', function(e) {
                e.preventDefault();
                var page = $(this).data('page');
                self.goToPage(page);
            });
        }
    };
    
    DataTable.prototype.render = function() {
        this.renderHeader();
        this.renderBody();
        this.renderPagination();
        return this;
    };
    
    DataTable.prototype.renderHeader = function() {
        var headerHtml = '<tr>';
        this.options.columns.forEach(function(column) {
            var sortable = column.sortable !== false ? 'data-sortable data-column="' + column.field + '"' : '';
            headerHtml += '<th ' + sortable + '>' + column.title + '</th>';
        });
        headerHtml += '</tr>';
        
        this.element.find('thead').html(headerHtml);
    };
    
    DataTable.prototype.renderBody = function() {
        var self = this;
        var bodyHtml = '';
        
        var data = this.getCurrentPageData();
        data.forEach(function(row) {
            bodyHtml += '<tr>';
            self.options.columns.forEach(function(column) {
                var value = row[column.field];
                if (column.formatter) {
                    value = column.formatter(value, row);
                }
                bodyHtml += '<td>' + value + '</td>';
            });
            bodyHtml += '</tr>';
        });
        
        this.element.find('tbody').html(bodyHtml);
    };
    
    DataTable.prototype.getCurrentPageData = function() {
        if (!this.options.pageable) {
            return this.options.data;
        }
        
        var start = (this.currentPage - 1) * this.options.pageSize;
        var end = start + this.options.pageSize;
        return this.options.data.slice(start, end);
    };
    
    DataTable.prototype.sort = function(column) {
        // 排序逻辑
        console.log('Sorting by:', column);
    };
    
    DataTable.prototype.goToPage = function(page) {
        this.currentPage = page;
        this.render();
    };
    
    return DataTable;
});
```

### 3. API集成

```javascript
// js/services/api.js - API服务
define(['jquery'], function($) {
    
    var Api = {
        baseUrl: '/api/v1',
        
        request: function(options) {
            var defaults = {
                url: '',
                method: 'GET',
                data: null,
                headers: {
                    'Content-Type': 'application/json'
                },
                timeout: 30000
            };
            
            var config = $.extend(true, {}, defaults, options);
            config.url = this.baseUrl + config.url;
            
            // 添加认证头
            var token = this.getAuthToken();
            if (token) {
                config.headers['Authorization'] = 'Bearer ' + token;
            }
            
            return $.ajax(config).fail(function(xhr, status, error) {
                console.error('API request failed:', error);
                
                // 处理认证失败
                if (xhr.status === 401) {
                    Api.handleAuthError();
                }
            });
        },
        
        get: function(url, params) {
            return this.request({
                url: url,
                method: 'GET',
                data: params
            });
        },
        
        post: function(url, data) {
            return this.request({
                url: url,
                method: 'POST',
                data: JSON.stringify(data)
            });
        },
        
        put: function(url, data) {
            return this.request({
                url: url,
                method: 'PUT',
                data: JSON.stringify(data)
            });
        },
        
        delete: function(url) {
            return this.request({
                url: url,
                method: 'DELETE'
            });
        },
        
        getAuthToken: function() {
            return localStorage.getItem('auth_token');
        },
        
        setAuthToken: function(token) {
            localStorage.setItem('auth_token', token);
        },
        
        clearAuthToken: function() {
            localStorage.removeItem('auth_token');
        },
        
        handleAuthError: function() {
            this.clearAuthToken();
            window.location.href = '/login';
        }
    };
    
    return Api;
});

// js/models/user.js - 用户模型
define(['services/api'], function(Api) {
    
    function User(data) {
        this.id = data.id || null;
        this.name = data.name || '';
        this.email = data.email || '';
        this.role = data.role || 'user';
        this.createdAt = data.createdAt || null;
    }
    
    User.prototype.save = function() {
        var url = this.id ? '/users/' + this.id : '/users';
        var method = this.id ? 'put' : 'post';
        
        return Api[method](url, this.toJSON()).then(function(response) {
            if (!this.id) {
                this.id = response.id;
            }
            return this;
        }.bind(this));
    };
    
    User.prototype.destroy = function() {
        if (!this.id) {
            return Promise.reject(new Error('Cannot delete user without ID'));
        }
        
        return Api.delete('/users/' + this.id);
    };
    
    User.prototype.toJSON = function() {
        return {
            id: this.id,
            name: this.name,
            email: this.email,
            role: this.role
        };
    };
    
    // 静态方法
    User.findById = function(id) {
        return Api.get('/users/' + id).then(function(data) {
            return new User(data);
        });
    };
    
    User.findAll = function(params) {
        return Api.get('/users', params).then(function(data) {
            return data.map(function(userData) {
                return new User(userData);
            });
        });
    };
    
    return User;
});
```

## 性能优化

### 1. r.js构建优化

```javascript
// build.js - r.js构建配置
({
    appDir: '../src',
    baseUrl: 'js',
    dir: '../dist',
    
    // 模块配置
    modules: [
        {
            name: 'app/main',
            include: [
                'app/router',
                'app/models/user',
                'app/views/layout'
            ]
        }
    ],
    
    // 路径配置
    paths: {
        'jquery': 'lib/jquery-3.6.0.min',
        'underscore': 'lib/underscore-1.13.1.min',
        'backbone': 'lib/backbone-1.4.0.min'
    },
    
    // 文件排除
    fileExclusionRegExp: /^(r|build)\.js$/,
    
    // 优化选项
    optimize: 'uglify2',
    preserveLicenseComments: false,
    generateSourceMaps: true,
    
    // 移除console.log
    uglify2: {
        output: {
            beautify: false
        },
        compress: {
            drop_console: true
        }
    }
})
```

### 2. 运行时优化

```javascript
// 懒加载策略
define(['jquery'], function($) {
    
    var LazyLoader = {
        cache: {},
        
        loadModule: function(moduleName) {
            if (this.cache[moduleName]) {
                return Promise.resolve(this.cache[moduleName]);
            }
            
            return new Promise(function(resolve, reject) {
                require([moduleName], function(module) {
                    this.cache[moduleName] = module;
                    resolve(module);
                }.bind(this), reject);
            }.bind(this));
        },
        
        preloadModules: function(moduleNames) {
            var promises = moduleNames.map(function(name) {
                return this.loadModule(name);
            }.bind(this));
            
            return Promise.all(promises);
        }
    };
    
    return LazyLoader;
});

// 代码分割
require.config({
    bundles: {
        'bundle-admin': ['app/admin/dashboard', 'app/admin/users'],
        'bundle-public': ['app/public/home', 'app/public/about']
    }
});
```

## 调试和测试

### 1. 开发环境配置

```javascript
// js/config-dev.js
require.config({
    baseUrl: 'js',
    
    paths: {
        'jquery': 'lib/jquery-3.6.0',  // 使用未压缩版本
        'underscore': 'lib/underscore-1.13.1',
        'backbone': 'lib/backbone-1.4.0'
    },
    
    // 禁用缓存
    urlArgs: 'bust=' + (new Date()).getTime(),
    
    // 启用详细错误
    enforceDefine: true,
    
    // 缩短超时以快速发现问题
    waitSeconds: 7
});

// 全局错误处理
requirejs.onError = function(err) {
    console.error('RequireJS Error:', err);
    console.error('Error type:', err.requireType);
    console.error('Error modules:', err.requireModules);
    
    if (err.requireType === 'timeout') {
        console.error('Modules that timed out:', err.requireModules);
    }
};
```

### 2. 单元测试

```javascript
// test/test-main.js - 测试配置
require.config({
    baseUrl: '../js',
    
    paths: {
        'jasmine': '../test/lib/jasmine-2.9.1/jasmine',
        'jasmine-html': '../test/lib/jasmine-2.9.1/jasmine-html',
        'jasmine-boot': '../test/lib/jasmine-2.9.1/boot',
        
        // 应用模块
        'utils': 'utils',
        'models': 'app/models'
    },
    
    shim: {
        'jasmine-html': ['jasmine'],
        'jasmine-boot': ['jasmine', 'jasmine-html']
    }
});

// 加载测试
require(['jasmine-boot'], function() {
    require([
        'test/utils/math-test',
        'test/models/user-test'
    ], function() {
        // 启动测试
        window.onload();
    });
});

// test/utils/math-test.js
define(['utils/math'], function(math) {
    describe('Math Utils', function() {
        it('should add two numbers', function() {
            expect(math.add(2, 3)).toBe(5);
        });
        
        it('should subtract two numbers', function() {
            expect(math.subtract(5, 3)).toBe(2);
        });
        
        it('should throw error on division by zero', function() {
            expect(function() {
                math.divide(10, 0);
            }).toThrow();
        });
    });
});
```

## 总结

RequireJS作为AMD规范的主要实现，提供了完整的模块化解决方案：

- ✅ **成熟稳定**: 经过大量项目验证的可靠工具
- ✅ **功能丰富**: 完整的配置系统和插件生态
- ✅ **工具完善**: 提供构建优化和调试支持
- ✅ **文档详细**: 有详细的文档和社区支持

- ❌ **语法复杂**: 相比现代模块系统语法较为复杂
- ❌ **体积较大**: 运行时库增加了页面负担
- ❌ **生态衰落**: 现代开发更倾向于ES模块和构建工具

虽然RequireJS在现代开发中逐渐被新技术取代，但它为JavaScript模块化发展做出了重要贡献，理解其原理和使用方法对于掌握前端发展历程很有价值。

---

**下一章**: [UMD通用模块](../umd/universal.md) →
