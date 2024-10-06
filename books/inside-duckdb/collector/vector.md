# Vector

## Vector 数据结构
```cpp
class Vector {
    VectorType vector_type;  // enum FLAT_VECTOR, FSST_, CONSTANT_, DICTIONARY_, SEQUENCE_
    LogicalType type;  //

    data_ptr_t data;   // 
    ValidityMask validity;
    buffer_ptr<VectorBuffer> buffer;  // 一般的，存储当前 Vector 的基础数据
    buffer_ptr<VectorBuffer> auxiliary;  // 存储动态部份的数据，例如非 inlined string, dictionary data etc.
};

class LogicType {
    LogicTypeId id_;   // INVALID, SQLNULL, ...,  BOOLEAN, INTEGER, ... STRUCT, LIST, MAP, ...
    PhysicalType physical_type_;  // BOOL, UINT8, INT8, ..., LIST, STRUCT, ARRAY, ...
    shared_ptr<ExtraTypeInfo> type_info_;
};

class ValidityMask {
    uint64_t *validity_mask;
    buffer_ptre<ValidityBuffer> validity_data;
    idx_t target_count;
};

class VectorBuffer {  // 使用 rust enum 来表示
    VectorBufferType buffer_type;  // STANDARD_BUFFER， DICTIONAY_BFUFER, ....
    unique_ptr<VectorAuxiliaryData> aux_data;   // 不同的类型对 data, aux_data 的理解不同
    unique_ptr<data_t> data;
};

class VectorChildBuffer: VectorBuffer {
    Vector data;  // 对 DictionaryVectory 来说，编码向量，{ index : value }
};
class DictionaryBuffer: VectorBuffer {
    SelectionVector sel_vector;  // 
};

struct SelectionVector {
    sel_t * sel_vector;  // type sel_t = uint32_t
    buffer_ptr<SelectionData> selection_data;   // type buffer_ptr = shared_ptr
};

struct SelectionData {
    unique_ptr<uint32_t[]> owned_data;
};

```

- [ ] Vector 居然没有 count 信息， 在 DataChunk 中？
- Vector 操作
 
## Flat Vector  内存布局
1. FlatVector Int32 的内存布局
   ```
    vector_type: i8
    type: 
        - id: i8
        - physical_type: i8
        _ type_info : nullptr  [i8,16]
    data: i32*  
    validity: [i8, 32] 
        - validity_mask: u64*  -- 其 owner 由 validity_data 持有
        - validity_data: shared_ptr<ValidityBuffer>
        - target_count: u64 = 2048
    buffer: shared_ptr<VectorBuffer> -- owner data
            - VectorBuffer.buffer_type
            - VectorBuffer.aux_data: nullptr
            - VectorBuffer.data: nullptr -- 一般的，与 data 是一致的。
            - 
    auxliary: nullptr
   ```
2. FlatVector string_t 的内存布局
   ```
    vector_type: i8 = FLAT_VECTOR
    type: 
        - id: i8 = VARCHAR 
        - physical_type: i8 = VARCHAR
        _ type_info = nullptr  [i8,16]
    data: i32*  
    validity: [i8, 32] 
        - validity_mask: u64* = NULL -- 其 owner 由 validity_data 持有
        - validity_data: shared_ptr<ValidityBuffer>
        - target_count: u64 = 2048 -- 实际数量
    buffer: shared_ptr<VectorBuffer> -- owner data  VectorCacheBuffer
            - VectorBuffer.buffer_type = OPAQUE_BUFFER
            - VectorBuffer.aux_data = nullptr
            - VectorBuffer.data: shared_ptr<[i8]> // 持有 vector.data 的所有权
            - type: LogicalType [i8;24]
            - owned_data
            - child_caches
            - auxilary
            - capacity: u64 = 2048
    auxliary: nullptr  -- 对非 inlined string，在这里进行存储
   ```   

# Dictionary Vector
TODO 补充几个 DictionaryVector 的例子

# Contract 整理
1. Vector 没有 length 的存储，由外部管理
2. 当一个 Vector referernce 另外一个 Vector 时，对一个的修改是否会影响到另一个？

# 思考
1. duckdb 的 Vector 数据结构定义还是比较复杂，如果使用 rust enum 来表示，感觉会清晰很多。
2. 接下来整理 Vector 的操作，运算。
3. 补充讲解一下 Flat Vector(varchar) 的 setValue 过程
4. 补充讲解一下 Dictionary Vector 的 setValue 过程
5. Resize 的过程
   1. vector.buffer/ vector.data 调整
6. 还需要阅读理解 VectorBuffer 的多种子类型
   - DictionaryBuffer : hold a SelectionVector
   - ManagedVectorBuffer
   - ParquetStringVectorBuffer
   - VectorArrayBuffer
   - VectorChildBuffer
   - VectorListBuffer
   - VectorStringBuffer: 对应 FLAT_VECTOR(VARCHAR) 的 auxliary
   - VectorStructBuffer
   - VectorCacheBuffer