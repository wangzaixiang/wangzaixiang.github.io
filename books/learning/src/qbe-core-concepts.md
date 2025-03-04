# QBE core concepts

## SSA
Static Single Assignment (SSA) 是一种 IR 表示方法，其特点是每个变量只被赋值一次。SSA 有助于进行数据流分析，优化等。

1. static: 每个变量在源代码静态（lexical)下只定义一次，而非 dynamic（运行时，因为 loop 的存在有的 assignment 会执行多次）
2. single: 每个变量只被赋值一次。

Why SSA? SSA 简化了数据流分析，只需要使用 def-use 关系就可以追踪数据流，而无需追踪一个完整的变换过程。

非 static single assignment 的 IR 可以通过：
1. 重命名变量，使得每个变量只被赋值一次（这个与CPU中的寄存器重命名概念是相似的）
2. 对 merge point 引入 phi 函数，phi 函数的参数是不同的分支上的变量。

## CFG
Control Flow Graph (CFG) 是一种表示程序控制流的图结构。在 CFG 中，每个基本块（basic block）是一个节点，连接两个基本块的边表示控制流的转移。

1. Basic Block: 一个基本块只有1个入口和1个出口。
2. Edge:
3. 3 种结构：
    - 顺序：BB1 -> BB2
    - 分支：BB1 -> BB2, BB3
    - 合并：BB1, BB2 -> BB3
    - 循环是分支和合并的结合。

4. Reverse Post Order.
   - DFS pre-order
   - DFS post-order
   - DFS reverse post-order
   
5. Dominator Tree
   - Dominator
   - Dominator Tree
   - Dominance 边界
   - Immediate Dominator

## Phi function

## optimizations
1. Dead Code Elimination
2. Constant Propagation
3. Register 重用
