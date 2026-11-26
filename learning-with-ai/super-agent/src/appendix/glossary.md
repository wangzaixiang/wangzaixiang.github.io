# 术语表

| 术语 | 英文 | 定义 |
|---|---|---|
| Super-agent | Super-agent | 跨会话保持上下文、并行推进多任务、持续在线、主动响应环境事件的 Agent 形态与系统架构 |
| 会话 | Session | Agent 的一次上下文容器；在 Super-agent 中应是系统实现细节，而非用户认知负担 |
| 路由器层 | Router layer | 常驻的全局交互、记忆与任务分派层；不执行具体任务 |
| 执行层 | Worker layer | 上下文隔离、执行具体任务的会话集合 |
| 触发器 | Trigger | 环境中的一条不可变事件，带来源元数据；审计与幂等的最小单位 |
| 线程 | Thread | 同一话题/PR/邮件串下按时间排列的 Trigger 流 |
| 关联 | Correlation | 一个 Trigger 属于多个 Thread 时的关系；只增加上下文，不路由 |
| 渠道 | Channel | Thread 的写侧；回复、ack、PR、review 等对外写操作 |
| 订阅 | Subscription | Ticket 与 Thread 的多对多关系；决定后续事件直通给谁 |
| 扇出 | Fan-out | 把订阅 Thread 的 Trigger 作为上下文投递给 Ticket |
| 评估 | Eval | 判断一批 Trigger 是否开启新工作的过程，分 Gate 与 Assessment |
| 准入 | Gate | Eval 的确定性第一层：谁可以发起工作、强度如何 |
| 研判 | Assessment | Eval 中由模型做的语义判断 |
| 决定 | Decision | Eval 的输出：accept / stop / ignore / unavailable |
| 授权 | Mandate | accept 授予的内容：范围、强度、依据的 Policy 版本、匹配规则 |
| 工单 | Ticket | 一个被授权的工作单元；只有 Eval 能创建 |
| 派工 | Assignment | Ticket 与一个 Executor 实例的绑定；可多次、可恢复 |
| 执行器 | Executor | 执行 Assignment 的端口；一个 Session + 每个 Ticket 一个目录 |
| 来源 | Source | 事件流；一种可复用实现 + 一个实例配置 |
| 策略 | Policy | 版本化的准入规则与各 Source 的配置 |
| 策略版本 | Policy version | 规则、accept mode、Source 部分的组合标识；每个 Mandate 记录它 |
| 准入模式 | Accept mode | `off` / `observe` / `auto`，控制推断事件是否真正接单 |
| 显式请求 | Explicit request | 白名单内的人明确 @ 或直接请求；可创建 Mandate |
| 推断候选 | Inferred candidate | 非明确请求，需要评估；bot 永远不能产生显式请求 |
| 重验证 | Revalidation | 恢复已结束 Ticket 前，按当前 Policy 检查 Mandate 是否仍成立 |
| 内联 | Inline | 把近期高概率用到的信息直接放入上下文窗口 |
| 溢出 | Spill | 把超预算/低频内容留在窗口外，只保留引用或摘要 |
| 链式召回 | Chain of Recall | 沿对象 ID 引用图逐跳召回上下文 |
| 投机执行 | Speculative Execution | 在结果不确定时后台提前尝试，有价值才交付 |
| 看门狗 | Watchdog | 主系统之外的控制回路，监控基础设施与任务进展并修复 |
| 快路径 | Fast Path | 确定性、低延迟、低成本的代码路径 |
| 慢路径 | Slow Path | 灵活、高成本、需要语义理解的模型路径 |
| BYOM | Bring Your Own Machine | 把用户自己的开发机/个人电脑接入云端 Agent |
| A2A | Agent2Agent | 独立 Agent 系统之间的通信与协作协议 |
| MCP | Model Context Protocol | 模型与工具/上下文源之间的接口协议 |
| Harness | Harness | 调用模型、路由工具调用、推进 Agent Loop 的运行时 |
| 沙箱 | Sandbox | 执行代码、编辑文件的可替换环境 |
| 会话日志 | Session log | append-only 的事件日志，持久保存发生了什么 |
| 工作槽 | Worker Slot | 一个 Ticket 执行时占用的并发名额 |
| 发件箱 | Outbox | 保存消息收据并保证发送顺序的机制 |
| 副作用队列 | Effect Queue | 先持久化、提交后执行的跨边界动作队列 |
| 幂等 | Idempotence | 同一操作重复执行不产生额外副作用 |
| 至少一次 | At-least-once | 允许重复投递，但保证不丢；配合幂等消费 |
| p50 / p95 | 50th/95th percentile | 完成时间分布的中位数/95 分位 |
| 首 token 时间 | TTFT | Time To First Token；用户感受到的首响延迟 |
| 上下文窗口 | Context window | 模型单次调用可接受的最大 token 数 |
| 局部性 | Locality | 近期/相邻信息更可能被再次使用的假设 |
| 控制回路 | Control Loop | 监控—判断—修复的闭环 |
| 单一写者 | Single Writer | 每类状态只有一个写入方，简化并发与审计 |
