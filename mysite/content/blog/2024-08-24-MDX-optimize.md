+++
title = "MDX 执行引擎优化思路 ..."
description = "本文记录在MDX优化过程中的一些思考"
date = 2024-08-10T12:00:00+00:00
draft = true
template = "blog/page.html"
+++

## EvaluationContext 优化
   - AllMembers 
   - Member(u32)  -- native member
   - CalcMemberInCube(u32)
   - CalcMemberInQuery(u32)
   - SlicedMembers -- 全局的 slice 筛选，会同时跨多个维度
   - FilteredMembers(u32) -- 单维度的筛选上下文
   - ReservedLevel(u32)   -- pseudo 
   - _ReservedOther(u32)
   通过扩展 MemberFilter 的能力，支持更好的 filter pushdown 能力。解决目前，不能有效或正确的下沉。

当 MDX 查询中涉及到 filter 轴时，如果简单的展开集合，filter 集合会放大 MDX 的计算。
例如： filter 集合 为 1000 时，会导致每个 cell 的计算都会进行1000次的计算。

## 优化1: slice
当 filter 集合不是一个正交集合时，会在 Context 中保存为 sliced: Set[Tuple]

1. 如果当前 Context 中 filter 的所有维度，其值为 SlicedMembers 时， 会将完整的集合下沉。
2. 如果filter轴中部分维度的值为 SlicedMembers 时，会根据这些维度，生成一个新的下沉条件。

## 优化2: 保存为 多个 FilteredMember。
filterMember 会直接传递到 cellRequest，而无需展开。

## 下沉按需展开
当计算 expr 时， 在表达式中，需要获取 dim1 维度的当前值进行计算时，而这个维度在 slice 或 FilterMember中时，需要对其展开：

此时， slice 和 filter 仍然会传递到 cellRequest 中 (计算度量 与 计算成员的优先级问题，需要有 expr 的再聚合方式)。

## filter 函数优化
尝试将 filter 函数优化成为支持如下能力的函数，可以快速执行一个 tuple load.
- loadTuple 函数设计方案，能尽量下称到 SQL 执行


## 优化相关
1. member cache
2. tuple cache
3. segment cache
4. compute cache
5. 公共表达式消除
6. batch-fetch buffer optimization
7. profile support
8. 向量化优化

## segment cache

1. Cell Request
   ```rust
   
   // 存储 查询生命周期相关的信息， slice, filter 信息可以在这里保存
   struct QueryContext {
   }
   
   // 存储 模型模型生命周期相关的信息，例如 member 可以在这里进行 cache
   struct CubeContext {
   }
   
   struct EvalContext {
        members: Vec<MemberValue>,
        slicer:  Slicer, 
   }
   
   struct CellRequest {
       members: Vec<MemberValue>,
       pattern: BitMap,     // 1: equals for AllMember, SlicedMember, FilterMember, 0: same shape for PrimaryMember, HierarchyMember
   }
   
   enum MemberValue {
       AllMember,
       SlicedMember(&Slicer),    // 在 slice 中指定
       PrimaryMember(usize),   // 对应于非层次维度下的一个基础成员（sql 绑定单个字段）
       HierarchyMember(usize), // 对应于层次维度下的一个成员（sql 绑定多个字段）
       FilterMember(&Filter),    // 对应于一个 filter member, 例如 it > '2023-01-01' and it < '2023-12-31'
       CalcMember(&CalcMember),  // 对应于一个计算成员， 在 CellRequest 中，不会出现
   }
   
   // MemberValue 可以编码为 MemberValue16, MemberValue32
   
   
   trait CellRequestBuffer {
       fn cellRequest_exists(&self, request: &CellRequest) -> bool;
       fn add_cellRequest(&mut self, request: &CellRequest, value);
   }
   
   trait SegmentLoader {
      fn load(requestBuffer: CellRequestBuffer) -> Segment;
   }
   
   trait Segment {
       fn cellRequest_matches(&self, request: &CellRequest) -> bool;
       fn get_cell(&self, request: &CellRequest) -> Option[Value];
   }
   ```

    1. MemberValue16
       bit0: 0: 1: ALL(0)/PrimaryMember(n)  最多可表示 32K 成员
       bit1: 1:
       10_b13 SliceMember(0)/FilteredMember(n) -- 最多 16K 个切片成员
       11_b13 CalMember(n) -- 最多 16K 个计算成员
    2. MemberValue32
       做多可表示 2^31（ 2G 个成员 ）

   在一个查询中，可能会构造当量的 CellRequest, 且出现重复请求的情况很多，因此，需要一个非常高效的 hashmap 来进行缓存。
    1. MemberValue 尽可能编码为 i16/i32, 减少内存占用，并且机制 hash 计算。
    2. cellRequest 以 trait 设计，提供几个精简的实现，最小化内存使用：
        - CompactCellRequest32: 32 字节大小， 最多支持 15个 维度， 每个 member 支持 2字节编码或 4字节编码。 pattern 使用 2 字节。
          其中 2字节可表示 2^15 个 成员（Slice/Filter/ALL 保留若干成员）， 可处理大部份的小维度。 4字节可表示 2^31 个 member。
        - CompactCellRequest64: 64 字节大小，最多支持 30 个 维度。 bitmap 使用 4字节。
        - CompactCellRequest128: 128 字节大小，最多支持 60 个维度。 bitmap 使用 8字节。
        - CellRequestN: 支持任意的维度数量，但是内存占用较大，不适合大部份的场景。

   结合 Rust 的 HashMap 实现，可以实现一个高效的缓存：
    1. 存储结构紧凑，内存使用少
    2. hash 效率高
    3. 内存连续性好（大部份的 CellRequest 是一个纯值对象，没有二次引用）
    4. 结合 HashMap 的 SIMD 实现，查找效率高
    5. [ ] cell request 是从 Context 演变来的，需要设计 Context 的存储机制，使得这个转换几乎0成本

   与 Java实现对比：
    1. RolapEvaluator 可以通过 savepoint/restore 机制来实现共享一个 evaluator， 以节省内存使用。更换一个 member 需要在 history 中增加
       SET_CONTEXT index member 3个对象， 至少24字节，当有2-3个字段更新时，这个成本还不如直接创建一个新的 Context 轻量

2. Expr cache
   当 EvalContext 中包括至少1个计算成员时， 可以对 Expr 进行缓存，以减少计算成员的计算成本。

   对当前 MDX， 每一个 AST node 都可以生成一个 计算度量，有一个查询级的 ExprId，以及 Expr 的依赖维度列表。
   eval(Expr)时， 会根据 (ExprId, EvalContext.with(expr.dependencies)) 来查找缓存。

   不是所有的 Expr 都会进入这个缓存，一般的，系统中部分 expr 会根据规则，进入这个缓存。可以在 MDX 中使用特殊的函数来控制这个规则。例如：
    ```
      (/*expr-cache*/ expr )
    ```

3. segment cache
    1. 可以在多个 MDX query之间共享 segment cache.
       ```scala
       
       key: (cubeId, cubeVersion, shape, CellRequestCrossJoin):
         cubeId
         cubeVersion： 每次抽取后，cubeVersion + 1
         shape
         requests: 仅对 CrossJoin Bag 进行缓存，存储各个维度的 值
       value: SegmentData( HashMap[ compact_cell_data, Value] )
       ```
       segment cache：(or using rocksdb as storage.)
        1. in memory
        2. in files

        1个shape 可能对应多个 Segment, 是否需要进行合并。
   
    2. 自动的 aggregation
       对宽表进行查看时，可以识别每个查询使用到的维度，预先按这些维度进行聚合。再在聚合后的结果上进行查询。


## 求值优化

1. 公共表达式提取

   关于公共表达式提取，有两种方式：
    1. 简单提取：如果两个 AST 结构相同并且具有相同的类型，则可以共享同一个 AST tree。
       evaluate( tree1, context ) 完成后，可以将  contetx 和 value 保存到 tree中。当再次执行这 evaluate 时，如果 context 没有变化，则无需重复计算。
    2. 优化提取：当前仅当检测到 两个 AST 会共享同一个 context 时，才共享同一个 AST Tree。
       做一个改写， 在 第一个 AST 中， 改写为 { val x = ..., x }，后续直接引用 x 即可。

2. 表达式是调用者求值，还是函数求值。
    1. 可以标记 函数的参数，是否可以由调用者求值（在调用者的 context 中），还是由函数求值，如果由函数求值，则函数可能会改变 context。
       进行这个标记，可以帮助进行公共表达式的提取优化。

3. 表达式相关维度。
   部分的 tree，求值时，仅引来 context 的部分维度，这时，即使context 的其他维度发生变化时，其值仍然不变。
   将这个 mask 存储在 tree 中，则求值的时候， 可以更好的进行优化。
    ```scala
      class Expr:
        val mask: Set[Dimension] = Set.empty
        var lastContext: Context 
        var lastResult: Value
        
        def evaluation(implicit context) = 
          if context == lastContext then lastResult
          else
              val depContext = context.keepOnly(mask) // 清理无关维度
              cache[depContext] match 
                case x => x
                case _ => realyEvaluation(context)
    ```

对函数的参数，可以分为3类：
1. 在 caller context 中提前求值。
2. 在 caller context 中延迟求值。（例如 iif 函数）
3. 在 callee context 中求值。（例如 filter 函数的 conditional）


1. 在一个表达式求值时，如果求值 f(a, b, c)，在 caller context 中求值的表达式可以进行 公共表达式提取，这个过程可以递归向下。
2. 如果两个表达式完全相同，但可能运行于不同的 context， 则可以按 (mask, mask(context)) 进行cache。
3. 由于伪求值阶段的存在，对 pseudo value 是否 cache? 如果不 cache，则在伪求值阶段会重复计算。如果 cache，则什么时候清理掉这个 cache。

## 批量加载

在 MDX 执行过程中，会涉及到 loadCell 的调用，简单加载模式中，每次 loadCell 都会发起一个SQL语句，并
同步等待结果返回， 这种方式实现简单，但由于一次查询过程中，这样的 loadCell 会发起上百次，上千次，乃至上百万次，上千万次，
会存在很严重的性能问题。

优化方案：
1. 执行过程中，requestCell 并不真实发起请求，而是将其放入到一个buffer中，并返回一个 pseudo value
   同时增加一个 dirtyCount.
2. 如果表达式计算过程中，dirtyCount 没有增加，则这次计算时有效的，可以直接作为返回值，并进行 cache。
   否则这个返回值也是一个 pseudo value，不能进行 cache.
3. 在计算结束后，将 buffer 中的 requestCell 进行批量处理，发起一个 SQL 请求，保存查询结果，并重新发起计算，
   直到 dirtyCount = 0 为止。

每个批次，对应于一个 SegmentLoad，他可能返回超过请求 cell 数量的结果.

```

    def cellRequest( cell: CellRequest):
      cellCache.get(cell) match
        case x => x
        case None =>
          cellRequests.add(cell) // also increase dirty
          PseudoValue
    
    class CellRequestBuffer:
        var dirtyCount: Int = 0
        
        def add(cell: CellRequest): Unit

```

CellRequestBuffer 会按 cell 的 pattern 进行分组记录， 具备一下特性的 cell 可以归入统一分组：
1. 如果 cell1.dim1 的值 为 ALL, 则 cell2.dim1 == cell2.dim1
2. 如果 cell1.dim1 的值 为 PrimaryMember, 则 cell2.dim1 也是 PrimaryMember(两者值不一定相等)
3. 如果 cell1.dim1 为 HierarchyMember, 则 cell2.dim1 也是 HierarchyMember(两者值不一定相等)
4. 如果 cell1.dim1 为 FilterMember，则 cell2.dim1 == cell1.dim1
5. 如果 cell1.slice 不为空，则 cell2.slice == cell1.slice

由两种方式可以组织 buffer:
1. List[CellRequest] 这种方式保存的数量一般会限制在 1000 以内
   对应的SQL 为：
    ```sql
        SELECT d1, d2, d3, SUM(m1) 
        from cube
        where (d1, d2, d3) in ( (v1, v2, v3), (v4, v5, v6) ... )
            and (f1 > f1_0 and f1 < f1_1)  -- filter field
            and (s1, s2, s3) in ( (s1_0, s2_0, s3_0), (s1_1, s2_1, s3_1) ... ) -- slice
    ```
2. Map[ Dim, Set[Member] ] 当 List[CellRequest] 达到一定值时，调整为这个结构。
   对应的SQL 为：
    ```sql
        SELECT d1, d2, d3, SUM(m1)
        from cube
        where d1 in (...) and d2 in (...) and d3 in (...)
            and (f1 > f1_0 and f1 < f1_1)  -- filter field
            and (s1, s2, s3) in ( (s1_0, s2_0, s3_0), (s1_1, s2_1, s3_1) ... ) -- slice
    ```
   后者对应的 segment，可能会返回超过请求的数据，可以匹配更多的 cellRequest 请求。 

## YTD, MTD 等窗口类函数的计算优化。
典型的，例如计算 `stdev` 聚合方式时，可以参照 《Efficient Processing of Window Functions in Analytical SQL Queries
》 论文中的优化方式，极少计算量。
