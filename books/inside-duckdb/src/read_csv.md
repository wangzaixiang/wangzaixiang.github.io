# read_csv 表函数分析

- ReadCSVBind        -- 返回表的 schema 信息，主要是列名 和 类型信息。
  ```cpp
    typedef unique_ptr<FunctionData> (*table_function_bind_t)(
        ClientContext &context, 
        TableFunctionBindInput &input,
        vector<LogicalType> &return_types, 
        vector<string> &names
    );
  ```

  在 Plan 阶段，会调用, 返回值 FunctionData 会传递到后续调用中，例如 TableFunctionInput.bind_data;

- ReadCSVFunction    -- main function
  ```cpp
   typedef void (*table_function_t)(
        ClientContext &context, 
        TableFunctionInput &data, 
        DataChunk &output
   );
  ```
  在 Pipeline 执行阶段，有 Source Operator 调用。
  1. 为什么是3个线程在执行？
  2. 这三个线程是如何协作的？例如并行处理文件的行数？

- ReadCSVInitGlobal   -- 可选
- ReadCSVInitLocal    -- 可选
  
  CVS Scanner: 按照每 8M 一个 scanner , 每个 scanner 分配一个扫描范围:
  - 如果一个 scanner 扫描完了当前block，需要继续扫描下一个 scanner 的数据，直到遇到一个换行符。
  - 对非第一个 scanner，会先忽略直到第一个换行符的数据

  优化： CSV Scanner 是否可以考虑使用 SIMD 加速？
  
  StringValueScanner: BaseScanner:
  - BaseScanner
    - iterator: CSVIterator
      - pos: CSVPosition( buffer_idx,  buffer_pos)  -- 当前位置
      - boundary: CSVBoundary( buffer_idx, buffer_pos, boundary_idx, end_pos ) -- 边界
    - CSVFileHandle
      - CSVOptions
      
  ```
  duckdb::CSVBuffer::CSVBuffer(duckdb::CSVFileHandle &, duckdb::ClientContext &, unsigned long long, unsigned long long, unsigned long long, unsigned long long) csv_buffer.cpp:28
  duckdb::CSVBuffer::CSVBuffer(duckdb::CSVFileHandle &, duckdb::ClientContext &, unsigned long long, unsigned long long, unsigned long long, unsigned long long) csv_buffer.cpp:25
  duckdb::make_shared_ptr<…>(duckdb::CSVFileHandle &, duckdb::ClientContext &, unsigned long long &, unsigned long long &&, unsigned long long &, unsigned long long &&) helper.hpp:73
  duckdb::CSVBuffer::Next(duckdb::CSVFileHandle &, unsigned long long, unsigned long long, bool &) csv_buffer.cpp:44
  duckdb::CSVBufferManager::ReadNextAndCacheIt() csv_buffer_manager.cpp:42
  duckdb::CSVBufferManager::GetBuffer(unsigned long long) csv_buffer_manager.cpp:71
  duckdb::CSVIterator::Next(duckdb::CSVBufferManager &) scanner_boundary.cpp:54
  duckdb::CSVGlobalState::Next(duckdb::optional_ptr<…>) global_csv_state.cpp:143
  duckdb::ReadCSVInitLocal(duckdb::ExecutionContext &, duckdb::TableFunctionInitInput &, duckdb::GlobalTableFunctionState *) read_csv.cpp:212
     -- 会调用 TableFunction.init_local 进行初始化，调用 global_state.Next() 获取 当前任务的一个 CSV-Scanner
  duckdb::TableScanLocalSourceState::TableScanLocalSourceState(duckdb::ExecutionContext &, duckdb::TableScanGlobalSourceState &, const duckdb::PhysicalTableScan &) physical_table_scan.cpp:75
  duckdb::TableScanLocalSourceState::TableScanLocalSourceState(duckdb::ExecutionContext &, duckdb::TableScanGlobalSourceState &, const duckdb::PhysicalTableScan &) physical_table_scan.cpp:71
  duckdb::make_uniq<…>(duckdb::ExecutionContext &, duckdb::TableScanGlobalSourceState &, const duckdb::PhysicalTableScan &) helper.hpp:65
  
  duckdb::PhysicalTableScan::GetLocalSourceState(duckdb::ExecutionContext &, duckdb::GlobalSourceState &) const physical_table_scan.cpp:84
     -- 每个 PipelineExecutor 中的 Source/Sink 都会有 local state
  duckdb::PipelineExecutor::PipelineExecutor(duckdb::ClientContext &, duckdb::Pipeline &) pipeline_executor.cpp:27
  
  duckdb::PipelineExecutor::PipelineExecutor(duckdb::ClientContext &, duckdb::Pipeline &) pipeline_executor.cpp:14
  duckdb::make_uniq<…>(duckdb::ClientContext &, duckdb::Pipeline &) helper.hpp:65
  duckdb::PipelineTask::ExecuteTask(duckdb::TaskExecutionMode) pipeline.cpp:34
  duckdb::ExecutorTask::Execute(duckdb::TaskExecutionMode) executor_task.cpp:44
  duckdb::TaskScheduler::ExecuteForever(std::__1::atomic<…> *) task_scheduler.cpp:189
  duckdb::ThreadExecuteTasks(duckdb::TaskScheduler *, std::__1::atomic<…> *) task_scheduler.cpp:279
  ```
  
  CSVGlobalState:
    - fileScans: `vector<shared_ptr<CSVFileScan>>` ; 每一个 csv 文件一个 CSVFileScan
      - file_path
      - file_size: 文件大小
      - buffer_manager: 管理文件的多个 buffer
        - cacheed_buffers: `vector<CSVBuffer>`  -- 每个 CSVBuffer 对应一个 PipelineExecutor
          - buffer_idx
          - actual_buffer_size
          - handle  -- 指向 buffer 数据
  每次 CSVGlobalState.Next() 获取下一个 StringValueScanner 时，会分配一个 Buffer(8M)，将文件数据读取到 buffer 中。在后面 scan 处理
  时，从内存中进行读取。
  - 这一块的代码写得很复杂。 


- table_function.pushdown_complex_filter 可选
  
  在 Optimizer 阶段，会调用。
  - [ ] 在前面应该有一个处理 bind_data 的阶段，获取表的 types, names 等信息。 

[ ] 需要整理出一个 table_function 的调用时序图，帮助理解每一个可选函数存在的价值。

阅读了上述信息后，可以开始评估编写我们自己的 table function 了，接下来，我们需要的一个场景是：
1. 在 Java 中使用 JDBC 读取业务数据，将 ResultSet 写入到一个 DataChunk 中。（提供API 写 data chunk）
2. 创建一个 table function，将 DataChunk 中的数据返回给 DuckDB。

```java
    
    DataChunk chunk = new DataChunk();  
    // prvoide 
    connection.registerDataChunk("asdf", chunk);
    
    connection.executeQuery("select * from read_asdf");
```

或者：
jdbc ResultSet -> arrow.vector.VectorSchemaRoot -> ArrowStreamReader -> arrow.c.ArrowArrayStream -> duckdb

```java
import org.apache.arrow.memory.BufferAllocator;
import org.apache.arrow.memory.RootAllocator;
import org.apache.arrow.vector.VectorSchemaRoot;
import org.apache.arrow.vector.ipc.ArrowStreamWriter;
import org.apache.arrow.vector.ipc.message.ArrowRecordBatch;
import org.apache.arrow.vector.types.pojo.Schema;
import org.apache.arrow.c.ArrowArrayStream;
import org.apache.arrow.c.ArrowArrayStreamListener;

import java.io.ByteArrayOutputStream;
import java.io.IOException;

public class ArrowConversionExample {

    public static ArrowArrayStream convertToArrowArrayStream(VectorSchemaRoot root) throws IOException {
        BufferAllocator allocator = new RootAllocator(Long.MAX_VALUE);
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        ArrowStreamWriter writer = new ArrowStreamWriter(root, null, out);

        // Write the VectorSchemaRoot to the output stream
        writer.start();
        writer.writeBatch();
        writer.end();

        // Create an ArrowArrayStream from the output stream
        ArrowArrayStream arrayStream = new ArrowArrayStream();
        arrayStream.setListener(new ArrowArrayStreamListener() {
            @Override
            public void onNext(ArrowRecordBatch batch) {
                // Handle the ArrowRecordBatch
            }

            @Override
            public void onError(Throwable t) {
                // Handle the error
            }

            @Override
            public void onCompleted() {
                // Handle completion
            }
        });

        // Initialize the ArrowArrayStream with the written data
        arrayStream.init(out.toByteArray(), allocator);

        return arrayStream;
    }

    public static void main(String[] args) throws IOException {
        // Example usage
        BufferAllocator allocator = new RootAllocator(Long.MAX_VALUE);
        Schema schema = new Schema(/* define your schema here */);
        VectorSchemaRoot root = VectorSchemaRoot.create(schema, allocator);

        // Populate the VectorSchemaRoot with data
        // ...

        ArrowArrayStream arrayStream = convertToArrowArrayStream(root);

        // Use the ArrowArrayStream
        // ...
    }
}
```

这个方式是有复制的，需要考虑如何减少复制。

1. https://arrow.apache.org/docs/java/jdbc.html
2. https://duckdb.org/docs/api/java#arrow-import

- ArrowReader
  - Field(name, nullable, type, ...) 
  - Schema(fields, metadata)
  - batches: List[ArrowRecordBatch]
    - ArrowRecordBatch( length, nodes, buffers) // each field encode in a buffer

这一块需要研究一下，整体成本相比自己实现可能会简单一些。

- Pipeline::Schedule 负责创建 PipelineTask 并提交调度。
  对 TableScan, 会调用 source_state->MaxThreads() 获取最大线程数，然后创建对应的 PipelineTask，一般会调用 对应 operator.globalState.MaxThreads() 
  对 CSV 文件，取文件大小 / 8M 
- TODO 理解 PipelineEvent 
