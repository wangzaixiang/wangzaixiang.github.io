# Arrow Array

spec: https://arrow.apache.org/docs/format/Columnar.html

1. Buffer
    1. Buffer 是不共享的值，其持有可共享的 Bytes
    2. ScalarBuffer < Buffer 提供类型化的操作，例如 `scalar_buffer[0..x]` 操作
    3. BooleanBuffer < Buffer 以 bitmap 形式存储的 buffer
    4. NullBuffer < BooleanBuffer 记录 null_count
    5. OffsetBuffer
    6. RunEndBuffer
    7. MutableBuffer 可变的buffer，用于构建 immutable Buffer
2. ArrayData 一个统一的 Array 结构
    ```rust
    struct ArrayData {
      data_type: DataType,
      len: usize,
      offset: usize,
      buffers: Vec<Buffer>,  // value buffer, value offset buffer
      child_data: Vec<ArrayData>, // ListArray, StructArray
      nulls: Option<NullBuffer>
    }
    ```
3. [PrimitiveArray<T>](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)
   ```rust
    struct PrimitiveArray<T: ArrowPrimitiveType> {
        data_type:  DataType,
        values:     ScalarBuffer<T::Native>,
        nulls:      Option<NullBuffer>
    }
    struct ScalarBuffer<T: ArrowNativeType> {
        buffer:     Buffer,
        phantom:    PhantomData<T>
    }
    struct Buffer {
        data: Arc<Bytes>,
        ptr:  *const u8,    // 可能在 data.ptr[0 .. data.len] 之间的某个地址
        length: usize       // ptr + length 不能超越 bytes 的边界
    }
    struct Bytes { // ptr[0..len]
        ptr: NonNull<u8>,
        len: usize,
        deallocation: Deallocation  // when Standard, ptr should be droped using std::alloc::dealloc
    }
   
    enum Deallocation {
        Standard(Layout),
        Custom(Arc<dyn Allocation>, usize)
    }
   ```
4. [BooleanArray]()
   - BooleanBuffer: 位图
   - NullBuffer
5. [GenericBytesArray<T>](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)
   - GenericBytesArray<Utf8Type>
   - GenericStringArray<OffsetSize>
   - buffers
     - offset: OffsetBuffer<32|64>
     - value_data: Buffer
6. [GenericByteViewArray](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout)
   - short strings(<=12>, long strings(>=12)
   - buffers:
    - views: ScalarBuffer<u128>
    - value_data: Buffer
7. [DictionaryArray](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout)
   - buffers: keys
   - child_data: values
8. [GenericListArray](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout)
   - buffers: offsets
   - child_data: values
   1. `List<Int8>` example
   2. `List<List<Int8>>` example
9. [FixedSizeListArray](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-list-layout)
   - buffers
   - child_data: values
10. [MapArray]()
11. [StructArray](https://arrow.apache.org/docs/format/Columnar.html#struct-layout)
    - child_data: fields' array
