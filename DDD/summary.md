---
layout: ddd
title: 目录
---

# 目录

* [导言](readme.html)
* [领域建模](about-ddd.html)
  * [领域模型的基础要素](construct-blocks.html)
  * [贫血 vs 充血](anemia.html)
  * [微服务与康威定律](microservice.html)
  * [Entity](entity.html)
    * 模型一致性，统一概念模型
  * [日志模式与事件溯源](eventsourcing.html)
  * [Vaule Object](vaule-object.md)
  * [Invariants](invariants.md)
  * [Event](event.md)
  * [Package](package.md)
  * [Command](command.md)
  * [原子操作 与 聚合操作、状态图](atomic.html)
  * Query
  * [最终一致性](consistence.html) 强事务一致性 与 最终一致性
* [服务构建块](fu-wu-gou-jian-kuai.md)
  * Service
  * [Tasks](fu-wu-gou-jian-kuai/tasks.md)
  * [MQ Receiver](fu-wu-gou-jian-kuai/mq-receiver.md)
  * EventBus
  * Data Access
* [服务设计原则](patterns.html)
  * [服务分层](layer.html)
      * 原子服务
      * 聚合服务
  * [策略分离、规则引擎、流程引擎](stratage.html)
  * [面向契约设计](dbc.html)
      * 严进宽出
      * 使用层次的、语义化的数据结构
      * 前置条件、后置条件定义
      * 无状态设计
      * null 处理
      * 数值类型
      * 枚举类型
      * 返回码与异常
  * [面向性能设计](performance.html)
      * 动静分离
      * 读写分离
      * 分库分表
      * 缓存
      * 异步处理
  * [面向监控设计](monitor.html)
      * 日志设计
      * 业务指标设计
      * JMX
  * [面向运维设计]()
      * 自适应设计
      * 参数配置
      * 资源依赖
  * [面向柔性设计](resilent.html)
      * 无状态设计
      * 可丢失缓存
      * 依赖分析
      * 限流、熔断、降级
  * [Restful风格 vs RPC风格](rest-rpc.html)
  * [一致性设计原则]()
      * 幂等性设计
      * 最终一致性
      * 分布式事务、TCC
      * 不变量
  * [缓存设计原则]()
      * 领域化，隐藏在服务接口之内
      * 穿透模式
      * 超时设置
      * 刷新策略
  * [服务版本升级模式](upgrade.html)
      * 向下兼容
      * 双写
  * [CQRS](cqrs.html)
* [识别伪需求引起的模型变化]
* [中台 与 DDD]()
* [UML 与 DDD]()
* [应用函数式编程](functional.html)
  * [如何学习scala](learn-scala.html)
  * [第一步：熟悉语法](learn-scala-1.html)
  * [进阶2: 作为更好的Java](learn-scala-2.html)
  * [进阶3: 挑战不变性](learn-scala-3.html)
  * [进阶3: 隔离副作用](learn-scala-4.html)
  * [应用函数式编程](applyfp.html)
  * 
* [编程风格](bian-cheng-feng-ge.md)
* [收集箱](drafts.html)
