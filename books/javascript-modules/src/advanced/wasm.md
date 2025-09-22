# WASM模块集成

WebAssembly (WASM) 是一种二进制指令格式，可以在现代浏览器中以接近原生的速度运行。本章将探讨如何将WASM模块与JavaScript模块系统集成。

## WASM模块基础

### WASM模块加载

```javascript
// wasm-loader.js - WASM模块加载器
class WasmModuleLoader {
  constructor() {
    this.moduleCache = new Map();
    this.loadingPromises = new Map();
  }

  // 基础WASM模块加载
  async loadWasmModule(url, importObject = {}) {
    if (this.moduleCache.has(url)) {
      return this.moduleCache.get(url);
    }

    if (this.loadingPromises.has(url)) {
      return this.loadingPromises.get(url);
    }

    const loadingPromise = this._loadModule(url, importObject);
    this.loadingPromises.set(url, loadingPromise);

    try {
      const module = await loadingPromise;
      this.moduleCache.set(url, module);
      this.loadingPromises.delete(url);
      return module;
    } catch (error) {
      this.loadingPromises.delete(url);
      throw error;
    }
  }

  async _loadModule(url, importObject) {
    // 获取WASM字节码
    const response = await fetch(url);
    const bytes = await response.arrayBuffer();

    // 编译和实例化WASM模块
    const wasmModule = await WebAssembly.compile(bytes);
    const wasmInstance = await WebAssembly.instantiate(wasmModule, importObject);

    return {
      module: wasmModule,
      instance: wasmInstance,
      exports: wasmInstance.exports
    };
  }

  // 流式加载（适用于大型WASM模块）
  async loadWasmStreamingly(url, importObject = {}) {
    const response = await fetch(url);
    
    if (!WebAssembly.compileStreaming) {
      // 降级到普通加载
      return this.loadWasmModule(url, importObject);
    }

    try {
      const wasmModule = await WebAssembly.compileStreaming(response.clone());
      const wasmInstance = await WebAssembly.instantiate(wasmModule, importObject);

      return {
        module: wasmModule,
        instance: wasmInstance,
        exports: wasmInstance.exports
      };
    } catch (error) {
      console.warn('Streaming compilation failed, falling back to regular loading:', error);
      return this.loadWasmModule(url, importObject);
    }
  }

  // 预编译WASM模块
  async precompileWasm(url) {
    try {
      const response = await fetch(url);
      const wasmModule = await WebAssembly.compileStreaming(response);
      
      // 将预编译的模块存储在缓存中
      this.moduleCache.set(url, { module: wasmModule, precompiled: true });
      return wasmModule;
    } catch (error) {
      console.error(`Failed to precompile WASM module ${url}:`, error);
      throw error;
    }
  }

  // 从预编译模块创建实例
  async instantiatePrecompiled(url, importObject = {}) {
    const cached = this.moduleCache.get(url);
    
    if (!cached || !cached.precompiled) {
      throw new Error(`No precompiled module found for ${url}`);
    }

    const wasmInstance = await WebAssembly.instantiate(cached.module, importObject);
    
    return {
      module: cached.module,
      instance: wasmInstance,
      exports: wasmInstance.exports
    };
  }
}

// 全局WASM加载器实例
const wasmLoader = new WasmModuleLoader();

// 使用示例
async function loadMathWasm() {
  try {
    // 定义导入对象
    const importObject = {
      env: {
        console_log: (value) => console.log('WASM Log:', value),
        Math_pow: Math.pow,
        Date_now: Date.now
      }
    };

    const wasmModule = await wasmLoader.loadWasmStreamingly('./math.wasm', importObject);
    
    // 使用WASM导出的函数
    const result = wasmModule.exports.fibonacci(40);
    console.log('Fibonacci result:', result);
    
    return wasmModule;
  } catch (error) {
    console.error('Failed to load math WASM module:', error);
  }
}
```

### ES模块与WASM集成

```javascript
// math-wasm.js - WASM模块的ES模块封装
import wasmUrl from './math.wasm?url'; // Vite/Webpack的WASM导入

class MathWasm {
  constructor() {
    this.wasmModule = null;
    this.loaded = false;
  }

  async initialize() {
    if (this.loaded) return;

    try {
      const importObject = {
        env: {
          memory: new WebAssembly.Memory({ initial: 256 }),
          console_log: this._consoleLog.bind(this),
          abort: this._abort.bind(this)
        }
      };

      // 使用ES模块导入WASM
      const wasmModule = await import('./math.wasm');
      
      // 或者使用fetch加载
      // const response = await fetch(wasmUrl);
      // const wasmModule = await WebAssembly.instantiateStreaming(response, importObject);

      this.wasmModule = wasmModule;
      this.loaded = true;

      console.log('Math WASM module loaded successfully');
    } catch (error) {
      console.error('Failed to initialize Math WASM:', error);
      throw error;
    }
  }

  // 包装WASM函数为JS方法
  fibonacci(n) {
    this._ensureLoaded();
    return this.wasmModule.exports.fibonacci(n);
  }

  factorial(n) {
    this._ensureLoaded();
    return this.wasmModule.exports.factorial(n);
  }

  isPrime(n) {
    this._ensureLoaded();
    return Boolean(this.wasmModule.exports.is_prime(n));
  }

  // 处理复杂数据类型
  processArray(arr) {
    this._ensureLoaded();
    
    const memory = this.wasmModule.exports.memory;
    const malloc = this.wasmModule.exports.malloc;
    const free = this.wasmModule.exports.free;
    const processArray = this.wasmModule.exports.process_array;

    // 分配内存
    const inputSize = arr.length * 4; // 假设是32位整数
    const inputPtr = malloc(inputSize);
    
    try {
      // 复制数据到WASM内存
      const inputView = new Int32Array(memory.buffer, inputPtr, arr.length);
      inputView.set(arr);

      // 调用WASM函数
      const resultPtr = processArray(inputPtr, arr.length);
      
      // 读取结果
      const resultView = new Int32Array(memory.buffer, resultPtr, arr.length);
      const result = Array.from(resultView);

      // 释放内存
      free(inputPtr);
      free(resultPtr);

      return result;
    } catch (error) {
      free(inputPtr);
      throw error;
    }
  }

  _ensureLoaded() {
    if (!this.loaded) {
      throw new Error('WASM module not loaded. Call initialize() first.');
    }
  }

  _consoleLog(value) {
    console.log('WASM:', value);
  }

  _abort() {
    throw new Error('WASM execution aborted');
  }
}

// 导出单例实例
export const mathWasm = new MathWasm();

// 也可以导出类供多实例使用
export { MathWasm };

// 默认导出初始化函数
export default async function initMathWasm() {
  await mathWasm.initialize();
  return mathWasm;
}
```

## 高级WASM集成模式

### WASM模块工厂

```javascript
// wasm-module-factory.js - WASM模块工厂
class WasmModuleFactory {
  constructor() {
    this.modules = new Map();
    this.templates = new Map();
  }

  // 注册WASM模块模板
  registerTemplate(name, config) {
    this.templates.set(name, {
      name,
      url: config.url,
      importObject: config.importObject || {},
      initFunction: config.initFunction,
      exportMappings: config.exportMappings || {},
      memoryConfig: config.memoryConfig
    });
  }

  // 创建WASM模块实例
  async createModule(templateName, instanceId = null) {
    const template = this.templates.get(templateName);
    if (!template) {
      throw new Error(`WASM template ${templateName} not found`);
    }

    const id = instanceId || `${templateName}_${Date.now()}`;
    
    if (this.modules.has(id)) {
      return this.modules.get(id);
    }

    try {
      const wasmInstance = await this._instantiateFromTemplate(template);
      const moduleWrapper = this._createModuleWrapper(wasmInstance, template);
      
      this.modules.set(id, moduleWrapper);
      return moduleWrapper;
    } catch (error) {
      console.error(`Failed to create WASM module ${templateName}:`, error);
      throw error;
    }
  }

  async _instantiateFromTemplate(template) {
    // 准备导入对象
    const importObject = this._prepareImportObject(template);
    
    // 加载和实例化WASM
    const response = await fetch(template.url);
    const wasmModule = await WebAssembly.instantiateStreaming(response, importObject);
    
    return wasmModule;
  }

  _prepareImportObject(template) {
    const importObject = { ...template.importObject };
    
    // 设置内存
    if (template.memoryConfig) {
      importObject.env = importObject.env || {};
      importObject.env.memory = new WebAssembly.Memory(template.memoryConfig);
    }

    // 添加标准环境函数
    importObject.env = importObject.env || {};
    Object.assign(importObject.env, {
      console_log: console.log,
      console_error: console.error,
      Date_now: Date.now,
      Math_random: Math.random,
      Math_floor: Math.floor,
      Math_ceil: Math.ceil,
      performance_now: performance.now.bind(performance)
    });

    return importObject;
  }

  _createModuleWrapper(wasmInstance, template) {
    const wrapper = {
      instance: wasmInstance.instance,
      exports: wasmInstance.instance.exports,
      memory: wasmInstance.instance.exports.memory,
      
      // 原始函数调用
      call: (funcName, ...args) => {
        const func = wasmInstance.instance.exports[funcName];
        if (!func) {
          throw new Error(`Function ${funcName} not found in WASM module`);
        }
        return func(...args);
      },

      // 内存操作辅助函数
      memory: {
        read: (ptr, length, type = 'uint8') => {
          const memory = wasmInstance.instance.exports.memory;
          const TypedArray = this._getTypedArray(type);
          return new TypedArray(memory.buffer, ptr, length);
        },

        write: (ptr, data, type = 'uint8') => {
          const memory = wasmInstance.instance.exports.memory;
          const TypedArray = this._getTypedArray(type);
          const view = new TypedArray(memory.buffer, ptr, data.length);
          view.set(data);
        },

        allocate: (size) => {
          const malloc = wasmInstance.instance.exports.malloc;
          if (!malloc) {
            throw new Error('malloc function not found in WASM module');
          }
          return malloc(size);
        },

        free: (ptr) => {
          const free = wasmInstance.instance.exports.free;
          if (!free) {
            throw new Error('free function not found in WASM module');
          }
          free(ptr);
        }
      }
    };

    // 添加映射的导出函数
    for (const [jsName, wasmName] of Object.entries(template.exportMappings)) {
      const wasmFunc = wasmInstance.instance.exports[wasmName];
      if (wasmFunc) {
        wrapper[jsName] = wasmFunc.bind(wasmInstance.instance.exports);
      }
    }

    // 调用初始化函数
    if (template.initFunction && typeof template.initFunction === 'function') {
      template.initFunction(wrapper);
    }

    return wrapper;
  }

  _getTypedArray(type) {
    const typeMap = {
      'int8': Int8Array,
      'uint8': Uint8Array,
      'int16': Int16Array,
      'uint16': Uint16Array,
      'int32': Int32Array,
      'uint32': Uint32Array,
      'float32': Float32Array,
      'float64': Float64Array
    };

    return typeMap[type] || Uint8Array;
  }

  // 销毁模块实例
  destroyModule(instanceId) {
    const module = this.modules.get(instanceId);
    if (module && module.memory && module.memory.free) {
      // 清理分配的内存
      // 这里需要根据具体的WASM模块实现来清理
    }
    
    this.modules.delete(instanceId);
  }

  // 获取模块统计信息
  getModuleStats() {
    return {
      templateCount: this.templates.size,
      instanceCount: this.modules.size,
      templates: Array.from(this.templates.keys()),
      instances: Array.from(this.modules.keys())
    };
  }
}

// 使用示例
const wasmFactory = new WasmModuleFactory();

// 注册图像处理WASM模块模板
wasmFactory.registerTemplate('imageProcessor', {
  url: './image-processor.wasm',
  memoryConfig: { initial: 1024 }, // 64MB
  exportMappings: {
    blur: 'image_blur',
    sharpen: 'image_sharpen',
    resize: 'image_resize'
  },
  initFunction: (wrapper) => {
    // 模块初始化逻辑
    console.log('Image processor WASM module initialized');
    
    // 添加高级方法
    wrapper.processImage = async (imageData, operations) => {
      const ptr = wrapper.memory.allocate(imageData.length);
      
      try {
        wrapper.memory.write(ptr, imageData, 'uint8');
        
        for (const op of operations) {
          switch (op.type) {
            case 'blur':
              wrapper.blur(ptr, imageData.length, op.radius);
              break;
            case 'sharpen':
              wrapper.sharpen(ptr, imageData.length, op.amount);
              break;
          }
        }
        
        return wrapper.memory.read(ptr, imageData.length, 'uint8');
      } finally {
        wrapper.memory.free(ptr);
      }
    };
  }
});

// 创建和使用模块实例
async function processImageWithWasm(imageData) {
  const processor = await wasmFactory.createModule('imageProcessor', 'main-processor');
  
  const result = await processor.processImage(imageData, [
    { type: 'blur', radius: 5 },
    { type: 'sharpen', amount: 0.8 }
  ]);
  
  return result;
}
```

### Worker中的WASM

```javascript
// wasm-worker.js - 在Worker中使用WASM
import { WasmModuleFactory } from './wasm-module-factory.js';

class WasmWorker {
  constructor() {
    this.wasmFactory = new WasmModuleFactory();
    this.modules = new Map();
    this.setupWorker();
  }

  setupWorker() {
    // 注册WASM模块模板
    this.wasmFactory.registerTemplate('mathProcessor', {
      url: './math-processor.wasm',
      memoryConfig: { initial: 256 },
      exportMappings: {
        matrixMultiply: 'matrix_multiply',
        fft: 'fast_fourier_transform',
        solve: 'linear_solve'
      }
    });

    // 设置消息处理
    self.onmessage = async (event) => {
      await this.handleMessage(event);
    };
  }

  async handleMessage(event) {
    const { id, type, data } = event.data;

    try {
      let result;

      switch (type) {
        case 'LOAD_WASM_MODULE':
          result = await this.loadWasmModule(data.templateName, data.instanceId);
          break;

        case 'WASM_CALL':
          result = await this.callWasmFunction(data.instanceId, data.funcName, data.args);
          break;

        case 'WASM_PROCESS_ARRAY':
          result = await this.processArrayWithWasm(data.instanceId, data.operation, data.array);
          break;

        case 'DESTROY_WASM_MODULE':
          result = await this.destroyWasmModule(data.instanceId);
          break;

        default:
          throw new Error(`Unknown message type: ${type}`);
      }

      self.postMessage({
        id,
        type: 'SUCCESS',
        data: result
      });

    } catch (error) {
      self.postMessage({
        id,
        type: 'ERROR',
        error: {
          message: error.message,
          stack: error.stack
        }
      });
    }
  }

  async loadWasmModule(templateName, instanceId) {
    const module = await this.wasmFactory.createModule(templateName, instanceId);
    this.modules.set(instanceId, module);
    return { loaded: true, instanceId };
  }

  async callWasmFunction(instanceId, funcName, args) {
    const module = this.modules.get(instanceId);
    if (!module) {
      throw new Error(`WASM module ${instanceId} not found`);
    }

    return module.call(funcName, ...args);
  }

  async processArrayWithWasm(instanceId, operation, array) {
    const module = this.modules.get(instanceId);
    if (!module) {
      throw new Error(`WASM module ${instanceId} not found`);
    }

    // 分配内存并复制数据
    const size = array.length * 4; // 假设32位数字
    const ptr = module.memory.allocate(size);

    try {
      module.memory.write(ptr, array, 'float32');

      // 调用WASM处理函数
      let resultPtr;
      switch (operation) {
        case 'fft':
          resultPtr = module.fft(ptr, array.length);
          break;
        case 'matrixMultiply':
          // 需要额外的矩阵维度参数
          resultPtr = module.matrixMultiply(ptr, Math.sqrt(array.length));
          break;
        default:
          throw new Error(`Unknown operation: ${operation}`);
      }

      // 读取结果
      const result = Array.from(module.memory.read(resultPtr, array.length, 'float32'));
      
      // 清理内存
      module.memory.free(resultPtr);
      
      return result;
    } finally {
      module.memory.free(ptr);
    }
  }

  async destroyWasmModule(instanceId) {
    this.wasmFactory.destroyModule(instanceId);
    this.modules.delete(instanceId);
    return { destroyed: true, instanceId };
  }
}

// 初始化Worker
const wasmWorker = new WasmWorker();

// 主线程使用示例
// wasm-worker-client.js
class WasmWorkerClient {
  constructor() {
    this.worker = new Worker('./wasm-worker.js', { type: 'module' });
    this.taskId = 0;
    this.pendingTasks = new Map();
    this.setupWorker();
  }

  setupWorker() {
    this.worker.onmessage = (event) => {
      const { id, type, data, error } = event.data;
      const task = this.pendingTasks.get(id);

      if (task) {
        this.pendingTasks.delete(id);
        
        if (type === 'SUCCESS') {
          task.resolve(data);
        } else {
          task.reject(new Error(error.message));
        }
      }
    };
  }

  async sendTask(type, data) {
    return new Promise((resolve, reject) => {
      const id = ++this.taskId;
      
      this.pendingTasks.set(id, { resolve, reject });
      
      this.worker.postMessage({ id, type, data });
      
      // 设置超时
      setTimeout(() => {
        if (this.pendingTasks.has(id)) {
          this.pendingTasks.delete(id);
          reject(new Error('Task timeout'));
        }
      }, 30000);
    });
  }

  async loadWasmModule(templateName, instanceId) {
    return this.sendTask('LOAD_WASM_MODULE', { templateName, instanceId });
  }

  async callWasmFunction(instanceId, funcName, args) {
    return this.sendTask('WASM_CALL', { instanceId, funcName, args });
  }

  async processArray(instanceId, operation, array) {
    return this.sendTask('WASM_PROCESS_ARRAY', { instanceId, operation, array });
  }

  terminate() {
    this.worker.terminate();
  }
}

// 使用示例
const wasmClient = new WasmWorkerClient();

async function performHeavyMathComputation() {
  try {
    // 加载WASM模块
    await wasmClient.loadWasmModule('mathProcessor', 'math-1');
    
    // 生成测试数据
    const largeArray = new Float32Array(1024 * 1024);
    for (let i = 0; i < largeArray.length; i++) {
      largeArray[i] = Math.random();
    }
    
    // 在Worker中执行FFT
    const result = await wasmClient.processArray('math-1', 'fft', largeArray);
    
    console.log('FFT computation completed:', result.length);
    return result;
  } catch (error) {
    console.error('Computation failed:', error);
  }
}
```

### WASM与现代构建工具集成

```javascript
// vite.config.js - Vite中的WASM配置
import { defineConfig } from 'vite';

export default defineConfig({
  // WASM优化配置
  optimizeDeps: {
    exclude: ['*.wasm']
  },
  
  // 服务器配置
  server: {
    fs: {
      allow: ['..'] // 允许访问上级目录的WASM文件
    }
  },
  
  // 构建配置
  build: {
    target: 'esnext', // 确保支持顶级await
    rollupOptions: {
      external: ['*.wasm'],
      output: {
        // WASM文件处理
        assetFileNames: (assetInfo) => {
          if (assetInfo.name.endsWith('.wasm')) {
            return 'wasm/[name].[hash][extname]';
          }
          return 'assets/[name].[hash][extname]';
        }
      }
    }
  },
  
  // 插件配置
  plugins: [
    // 自定义WASM插件
    {
      name: 'wasm-plugin',
      configureServer(server) {
        server.middlewares.use('/wasm', (req, res, next) => {
          res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
          res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
          next();
        });
      },
      load(id) {
        if (id.endsWith('.wasm')) {
          // 返回WASM模块的ES模块包装
          return `
            const wasmUrl = new URL('${id}', import.meta.url).href;
            let wasmModule;
            
            export default async function(importObject = {}) {
              if (!wasmModule) {
                const response = await fetch(wasmUrl);
                wasmModule = await WebAssembly.instantiateStreaming(response, importObject);
              }
              return wasmModule;
            }
            
            export { wasmUrl };
          `;
        }
      }
    }
  ]
});

// webpack.config.js - Webpack中的WASM配置
module.exports = {
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/async'
      }
    ]
  },
  
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        wasm: {
          test: /\.wasm$/,
          name: 'wasm-modules',
          chunks: 'all'
        }
      }
    }
  }
};

// 现代WASM模块使用示例
// image-processor.js
import wasmInit, { wasmUrl } from './image-processor.wasm';
import { WasmModuleCache } from './wasm-cache.js';

const wasmCache = new WasmModuleCache();

export class ImageProcessor {
  constructor() {
    this.wasmModule = null;
    this.initialized = false;
  }

  async initialize() {
    if (this.initialized) return;

    try {
      // 尝试从缓存获取
      this.wasmModule = await wasmCache.getOrLoad('image-processor', async () => {
        const importObject = {
          env: {
            memory: new WebAssembly.Memory({ initial: 1024 }),
            console_log: console.log
          }
        };
        
        return await wasmInit(importObject);
      });

      this.initialized = true;
    } catch (error) {
      console.error('Failed to initialize ImageProcessor WASM:', error);
      throw error;
    }
  }

  async processImage(imageData, filters = []) {
    await this.initialize();
    
    const { memory, process_image, malloc, free } = this.wasmModule.instance.exports;
    
    // 分配输入内存
    const inputSize = imageData.data.length;
    const inputPtr = malloc(inputSize);
    
    try {
      // 复制图像数据到WASM内存
      const inputView = new Uint8Array(memory.buffer, inputPtr, inputSize);
      inputView.set(imageData.data);
      
      // 处理每个滤镜
      for (const filter of filters) {
        const filterConfig = this._encodeFilter(filter);
        process_image(inputPtr, imageData.width, imageData.height, filterConfig);
      }
      
      // 读取处理后的数据
      const outputData = new Uint8Array(inputSize);
      outputData.set(inputView);
      
      return new ImageData(
        new Uint8ClampedArray(outputData),
        imageData.width,
        imageData.height
      );
      
    } finally {
      free(inputPtr);
    }
  }

  _encodeFilter(filter) {
    // 将JavaScript滤镜配置编码为WASM可理解的格式
    const filterTypes = {
      'blur': 1,
      'sharpen': 2,
      'brightness': 3,
      'contrast': 4
    };
    
    return (filterTypes[filter.type] || 0) | 
           ((filter.intensity || 1.0) * 255) << 8;
  }
}

// WASM缓存管理
class WasmModuleCache {
  constructor() {
    this.cache = new Map();
  }

  async getOrLoad(key, loaderFn) {
    if (this.cache.has(key)) {
      return this.cache.get(key);
    }

    const module = await loaderFn();
    this.cache.set(key, module);
    return module;
  }

  clear() {
    this.cache.clear();
  }
}
```

通过这些技术，可以将高性能的WASM模块无缝集成到JavaScript模块化应用中，实现接近原生性能的计算密集型功能，同时保持良好的开发体验和代码组织。
