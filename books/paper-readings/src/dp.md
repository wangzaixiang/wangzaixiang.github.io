# Dynamic Programming Strikes Back (动态规划)

本文介绍了一种改进的 DPccp 算法，更好的支持复杂的 join predicate 和 outer joins，可以带来数量级上的性能提升。

## 1. 简介
CBO：Cost Based Optimization
RBO: Rule Based Optimization
Join Order 在 CBO 中非常重要。
经典论文：[access path selection](access_path_selection.md): 
- 按照数据规模从小到大的顺序（这一条并没有在该文中明确提到）
- left deep tree (虽然简单，但并不一定是最佳的)
- DPsize


## 参考：
1. [Dynamic Programming Strikes Back - MySQL8.0的新优化器](https://developer.aliyun.com/article/789923)