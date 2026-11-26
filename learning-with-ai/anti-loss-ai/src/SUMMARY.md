# Summary

[介绍](./intro.md)

# 问题与抽象

- [场景与问题](./overview/scenario.md)
- [把 AI 抽象为 f](./overview/abstraction.md)
- [f 的要素逐条解读](./overview/elements.md)

# 流水线与契约

- [f 的内部分解：级联 vs 端到端](./pipeline/overview.md)
- [事件契约：类型 τ 与载荷 X](./pipeline/contract.md)

# 组件设计

- [检测组件](./components/detection.md)
- [手部关键点组件](./components/hand-pose.md)
- [跟踪组件（非神经网络）](./components/tracking.md)
- [动作 / 交互组件](./components/action.md)
- [细粒度商品识别](./components/recognition.md)
- [规则引擎与会话状态机](./components/rule-engine.md)

# 训练

- [数据与标注](./training/data.md)
- [训练范式](./training/paradigms.md)
- [训练配方](./training/recipe.md)
- [评测体系](./training/evaluation.md)

# 工程化

- [端侧部署与模型压缩](./engineering/deployment.md)
- [边缘架构](./engineering/edge-architecture.md)
- [与 POS 的融合和时间同步](./engineering/integration.md)
- [隐私与合规](./engineering/privacy.md)
- [分期落地路线](./engineering/roadmap.md)

# 附录

- [模型选型一览](./appendix/model-zoo.md)
- [术语表](./appendix/glossary.md)
- [参考资料](./appendix/references.md)
