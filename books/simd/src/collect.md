
# 前言

最近，花了比较多的时间来学习 SIMD，感觉有必要把 SIMD 的一些资料、算法进行整理。因此，考虑使用 mdbook 来进行这个系列的编写工作。

计划：
1. 一些 SIMD 算法的实现收集（重点关注在关系计算相关的算法）。包括源代码、性能测试（使用 criterion）等。
   - 排序
   - 求和、求平均、求最大值、求最小值
   - hashmap
   - hash join
   - hash aggregation
   - JSON parser
   - CSV parser

2. portable-simd 与 intel intrinsics 的对比。目前，有关 portable-simd 的资料比较少，各个 API 缺少文档、示例描述。
   intel intrinsics 的官方资料也不够直观。
   - intel intrinsics 与 portable-simd 的对应关系
   - 示例代码
   - 计算图示。