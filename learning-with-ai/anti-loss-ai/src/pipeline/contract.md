# 事件契约：类型 τ 与载荷 X

$f$ 与外界唯一的接口就是**事件契约**。定死它，模型才能被替换，系统才能被测试。

## 事件的基本结构

一条事件是 $(t_{event}, \tau, X, p, \text{prov})$：

```json
{
  "t_event": 12.345,
  "t_emit": 12.780,
  "type": "object_crossed_scan_window",
  "entities": {
    "object": { "track_id": 42, "class": "banana", "box": [x1, y1, x2, y2] },
    "hand":   { "track_id": 7,  "handedness": "right" }
  },
  "predicates": { "in_scan_window": true, "motion": "left_to_right" },
  "confidence": 0.91,
  "provenance": { "model": "det_v3+track_v2+hoi_v1", "calib": "cam_A_2026-09" }
}
```

## 类型 $\tau$ 的分层枚举

推荐三层本体（ontology），类似 ECS / 关系模型：

### 实体事件（entity）

| $\tau$ | 语义 | 关键字段 |
| --- | --- | --- |
| `person_detected` | 收银台前有人 | `person_id` |
| `hand_detected` | 检测到手 | `hand_id`, `handedness` |
| `object_detected` | 检测到商品 | `object_id`, `class` |

### 关系 / 谓词事件（relation）

| $\tau$ | 语义 | 关键字段 |
| --- | --- | --- |
| `hand_holds` | 手持有物体 | `hand_id`, `object_id`, `contact` |
| `object_in_region` | 物体处于某区域 | `object_id`, `region`（`scan_window`/`bagging_area`） |
| `object_crossed_scan_window` | 物体掠过扫描窗 | `object_id`, `direction` |
| `object_in_basket` | 物体在购物篮 / 车底 | `object_id`, `basket_id` |

### 动作事件（action）

| $\tau$ | 语义 | 关键字段 |
| --- | --- | --- |
| `scan_attempt` | 疑似扫码动作 | `hand_id`, `object_id`, `duration` |
| `bagging` | 装袋动作 | `hand_id`, `object_id` |
| `session_start` / `session_end` | 会话边界 | `session_id` |

> $\tau$ 的粒度直接决定下游状态机复杂度：过细 → 状态爆炸；过粗 → 下游要重做识别。

## 载荷 $X$ 的组成

| 组成 | 说明 | 完整性约束 |
| --- | --- | --- |
| 实体引用 | `track_id` / `object_id` / `hand_id` | 会话内有效且不复用 |
| 类别 | `class`（含置信度） | 闭集 + 开放集兜底 |
| 空间 | `box`、ROI 归属、世界坐标 | 供复核与存证 |
| 关系 | `predicates`：布尔 / 枚举谓词 | 关系事件需连两个实体 |

## 不变量（写成可执行测试）

| 不变量 | 断言 |
| --- | --- |
| Schema 合规 | 每条事件的 $\tau$ 与 $X$ 结构匹配（判别式联合） |
| 事件时间有序 | 输出按 $t_{event}$ 非降 |
| 会话内 id 持久 | 同一实体在一个 session 内 id 不复用 |
| 回放确定性 | 固定 $f_\theta$ 与 cfg，同一段视频输出逐位一致 |
| 空输出合法 | `[]` 不是错误 |
| 幂等 | 重复投递同一事件不改变下游状态 |

## 版本化与兼容

- 事件 schema 与 $f_\theta$ 的版本**绑定**，`prov.model` 可追溯。
- 新增 $\tau$ 必须向后兼容（旧消费者忽略未知类型）。
- 建议用 Protobuf / JSON Schema 固化契约，并在 CI 中做**契约测试**。

下一篇：[检测组件](../components/detection.md)。
