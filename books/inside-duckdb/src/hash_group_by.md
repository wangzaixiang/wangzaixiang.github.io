# hash_group_by

我们使用示例 SQL 语句来学习 hash_group_by 算子的原理：
```sql
select name, count(freight), sum(freight) 
from sale_orders so 
left join customers c on c.customer_id = so.customer_id 
where gender = 'M' and name like 'abc%'  and freight > 10 and freight < 50 
group by name
```

在执行计划中，可以看到如下的 hash_group_by 算子， 该算子是 pipeline 的 Sink 节点，其谦虚处理节点为： join |> projection |> hash_group_by

```
┌───────────────────────────┐
│       HASH_GROUP_BY       │
│    ────────────────────   │
│         Groups: #0        │
│                           │
│        Aggregates:        │
│        count_star()       │
│          sum(#1)          │
│                           │
└─────────────┬─────────────┘ 
```

对应的源代码为： `src/execution/operator/aggregate/physical_hash_aggregate.cpp`

```cpp
class PhysicalHashAggregate : public PhysicalOperator {

	//! The grouping sets, SQL 中分组+聚合的设置信息
	GroupedAggregateData grouped_aggregate_data;
	
            vector<unique_ptr<Expression>> groups;  // 分组字段， 在这里例子中是 name
            vector<vector<idx_t>> grouping_functions;  // 这个例子中没有使用分组函数
            vector<LogicalType> group_types;        // 分组字段的类型， 在这里例子中是 VARCHAR


            vector<unique_ptr<Expression>> aggregates;  // 聚合表达式， 这里是 count(freight), sum(freight)
            vector<LogicalType> payload_types;         // freight 字段的类型
            vector<LogicalType> aggregate_return_types; // 聚合函数的返回类型, 这里是 BIGINT, BIGINT
            vector<BoundAggregateExpression *> bindings; // 对应聚合函数的 binding 信息
            idx_t filter_count;
        
	vector<GroupingSet> grouping_sets;
	//! The radix partitioned hash tables (one per grouping set)
	vector<HashAggregateGroupingData> groupings;
	unique_ptr<DistinctAggregateCollectionInfo> distinct_collection_info;
	//! A recreation of the input chunk, with nulls for everything that isnt a group
	vector<LogicalType> input_group_types;

	// Filters given to Sink and friends
	unsafe_vector<idx_t> non_distinct_filter;
	unsafe_vector<idx_t> distinct_filter;

	unordered_map<Expression *, size_t> filter_indexes;
}

class HashAggregateLocalSinkState: public LocalSinkState {
	DataChunk aggregate_input_chunk;
	
	vector<HashAggregateGroupingLocalState> grouping_states;  // 保存对每个grouping的状态信息
	// operator.local_state.grouping_states[group_idx].table_state.ht 存储一个 hashtable
	
	AggregateFilterDataSet filter_set;
}

class HashAggregateGlobalSinkState: public GlobalSinkState {
    vector<HashAggregateGroupingGlobalState> grouping_states;
	vector<LogicalType> payload_types;
	//! Whether or not the aggregate is finished
	bool finished = false;
}

class AggregateFunction: public BaseScalarFunction {
    //! The hashed aggregate state sizing function
	aggregate_size_t state_size;  // 
	//! The hashed aggregate state initialization function
	aggregate_initialize_t initialize;
	//! The hashed aggregate update state function
	aggregate_update_t update;
	//! The hashed aggregate combine states function
	aggregate_combine_t combine;
	//! The hashed aggregate finalization function
	aggregate_finalize_t finalize;
	//! The simple aggregate update function (may be null)
	aggregate_simple_update_t simple_update;
	//! The windowed aggregate custom function (may be null)
	aggregate_window_t window;
	//! The windowed aggregate custom initialization function (may be null)
	aggregate_wininit_t window_init = nullptr;

	//! The bind function (may be null)
	bind_aggregate_function_t bind;
	//! The destructor method (may be null)
	aggregate_destructor_t destructor;

	//! The statistics propagation function (may be null)
	aggregate_statistics_t statistics;

	aggregate_serialize_t serialize;
	aggregate_deserialize_t deserialize;
	//! Whether or not the aggregate is order dependent
	AggregateOrderDependent order_dependent;
	//! Additional function info, passed to the bind
	shared_ptr<AggregateFunctionInfo> function_info;
}

```

分别在 Sink 方法, Combine 方法， Finalize 方法中添加断点，调试执行该算子的代码，跟踪其执行流程，进一步理解该算子的数据结构、算法。

TODO:
- 理解 duckdb 是如何通过 template 来实现不同类型的聚合函数的处理的。
- sum(x) 是如何映射到特定版本的函数的？

