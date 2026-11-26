# 参考资料

> 访问日期：2026-11-02。标注“原文”“官方仓库”“对照阅读”以便区分证据等级。

## 一、Super-agent 原文与官方实现

### 1. 原文

DeepSeek Harness 团队（Yujia Qiao 等），
《Super-agent 设计探索：迈向更自主的 Agent 系统》，2026-10-31。

- 微信公众号原文：<https://mp.weixin.qq.com/s/zoS7GlReWPhfCjENGxXCdg>
- 知乎链接（用户提供，当前环境返回 HTTP 403）：<https://zhuanlan.zhihu.com/p/2099987320443237060>
- 全文镜像（第三方，用于核对原文）：<https://www.uied.cn/posts/922127>
- 报道（第三方，用于交叉核对）：智东西 <https://zhidx.com/p/606311.html>；
  IT之家/网易 <https://c.m.163.com/news/a/LAJOI3N40511B8LM.html>

原文的核心结构：四个特征（连续、高效、主动、持久）与五个模块（双层架构、事件驱动、
面向人的交互、计算/状态/环境分离、Watchdog）。

### 2. 官方原型仓库

`deepseek-ai/dsh-superagent-prototype`，MIT License，TypeScript。

- 仓库：<https://github.com/deepseek-ai/dsh-superagent-prototype>
- README：研究原型，非正式产品；为公开发布做过适配；部分文章设计未实现
- 领域模型：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/001-domain.md>
- 授权：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/002-authorization.md>
- 执行：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/003-execution.md>
- 提示词：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/004-prompts.md>
- 部署：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/006-deployment.md>
- 架构：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/007-architecture.md>
- 哲学：<https://github.com/deepseek-ai/dsh-superagent-prototype/blob/main/docs/000-philosophy.md>

### 3. DeepSeek Harness

- 产品页：<https://www.deepseek.com/en/harness/>
- 仓库：<https://github.com/deepseek-ai/deepseek-harness>
- 文档：<https://deepseek-harness.github.io/deepseek-harness/en/guide/quickstart>

## 二、对照阅读：Anthropic 的 Agent 工程文章

### 4. Scaling Managed Agents: Decoupling the brain from the hands

Anthropic Engineering，2026-04-08。

- 正文：<https://www.anthropic.com/engineering/managed-agents>
- 规范 URL（当前重定向）：<https://www.anthropic.com/research/managed-agents>

要点：session / harness / sandbox 三接口；无状态 harness；`execute`、`provision`、`wake`、
`getSession`、`getEvents`、`emitEvent`；凭据不进入 sandbox；TTFT p50 下降约 60%、p95 下降超过 90%。
Super-agent 原文明确将其“脑手分离”作为对照，但场景不同（托管服务 vs 用户机器/BYOM）。

### 5. Building effective agents

Anthropic，2024-12-19（2026 年页面有更新说明）。

- 正文：<https://www.anthropic.com/engineering/building-effective-agents>

要点：workflow 与 agent 的区别；prompt chaining、routing、parallelization、
orchestrator-workers、evaluator-optimizer；从简单开始，只在可证明有效时增加复杂度。

### 6. How we built our multi-agent research system

Anthropic，2025-06-13。

- 正文：<https://www.anthropic.com/engineering/multi-agent-research-system>

要点：orchestrator-worker；多 Agent 在广度优先研究上的优势；token 消耗约为聊天的 15 倍；
同步执行造成瓶颈；持久执行、恢复、可观测性、彩虹部署等生产经验。

## 三、协议与基础设施

### 7. A2A（Agent2Agent）Protocol

Linux Foundation，v1.0.0。

- 规范：<https://a2a-protocol.org/latest/specification/>

要点：独立、可能不透明的 Agent 系统之间的通信；Agent Card、Task、Message、Artifact；
异步、流式、推送通知；不要求共享内部状态/记忆/工具。

### 8. MCP（Model Context Protocol）

- 官网：<https://modelcontextprotocol.io/>
- 规范：<https://modelcontextprotocol.io/specification>

要点：模型与工具/上下文源之间的接口；不解决 Agent 之间的任务分派与协作。

## 四、本书内部的交叉引用

- 问题与定义：[为什么需要 Super-agent](../overview/why.md)
- 架构总览：[四个特征，五个模块](../overview/architecture.md)
- 调度：[Router 与 Worker](../design/router-worker.md)
- 事件：[事件驱动](../design/event-driven.md)
- 交互：[全双工与多信道](../design/interaction.md)
- 持久：[存算分离与 Watchdog](../design/durability.md)
- 实现：[dsh-superagent](../implementation/prototype.md)
- 实验：[数字说明了什么](../evaluation/experiments.md)
- 对比：[Agent 版图](../analysis/comparison.md)
- 边界：[开放问题与安全](../analysis/open-problems.md)

## 五、引用说明

- 原文中的实验数字只来自原文第 7 节；本书没有独立复现。
- 原型行为以官方仓库 `docs/000-008` 与源码为准；文章与原型不一致处，本书明确标注。
- 知乎链接在当前环境无法访问（HTTP 403）；本书不以该页面作为独立信源。
- 第三方报道仅用于交叉核对，不作为设计结论的主要依据。
