# 术语表

| 术语 | 英文 / 缩写 | 含义 |
| --- | --- | --- |
| $f$ | — | 本书核心抽象：视频窗 + 配置 → 结构化事件流 |
| 时间窗 | $W_t$ | 用于判定动作的一段连续帧 |
| 事件时间 | $t_{event}$ | 事件实际发生的时刻 |
| 处理时间 | $t_{emit}$ | 系统输出事件的时刻；与 $t_{event}$ 之差为延迟 |
| 事件类型 | $\tau$ | 事件的语义类别（本体 / ontology） |
| 载荷 | $X$ | 事件的结构化内容（实体 + 谓词） |
| 置信度 | $p$ | 事件的分数（注意：非校准概率） |
| 溯源 | prov | 模型版本、标定、流水线组成等，用于复现与审计 |
| 漏扫 | skip-scan | 商品过扫描区但未录入 |
| 假扫 | fake scan | 有扫码动作但未解码 |
| 错扫 | banana trick | 高价商品按廉价 PLU 计价 |
| 换码 | ticket switching | 贴假条码 |
| 底筐 | bottom of basket | 车底商品不上台 |
| 内盗合谋 | sweethearting | 收银员不扫（亲友）商品 |
| 自助收银 | SCO (Self-Checkout) | 顾客自助结算终端 |
| 交易流水 | T-log | POS 的交易日志 |
| 扫描窗 | scan window | 条码扫描器覆盖的物理区域 |
| 感兴趣区域 | ROI | **Region of Interest（视觉）**：标定出的几何区域（扫描区 / 打包区 / 货架区），属 `cfg`。⚠️ 与下一条同缩写 |
| 投资回报 | ROI | **Return on Investment（业务）**：见[分期落地路线](../engineering/roadmap.md) |
| 目标检测 | object detection | 逐帧定位物体 |
| 多目标跟踪 | MOT | 跨帧关联检测，分配稳定 id |
| 手物交互 | HOI | 手与物体的交互识别 |
| 重识别 | ReID | 跨窗 / 跨相机保持同一实体身份 |
| 误报疲劳 | alert fatigue | 误报过多导致人工忽视告警 |
| 域适应 | domain adaptation | 合成 / 跨场景到目标的迁移 |
| 多示例学习 | MIL | 只用视频级标签的弱监督 |
| 训练后量化 | PTQ | 训练后 INT8 量化 |
| 量化感知训练 | QAT | 训练中模拟量化 |
| 知识蒸馏 | distillation | 大模型教小模型 |

参考：[参考资料](./references.md)。
