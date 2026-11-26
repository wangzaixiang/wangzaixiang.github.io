# 场景与问题

## 场景

在超市 / 便利店的**收银台**（人工通道或自助收银 SCO）上方部署摄像头，实时判断**商品是否被正确结算**，即检测"漏扫"及其变体。系统对可疑交易给出实时提醒或事后存证。

## 要检测的漏损模式

不同模式的检测方法完全不同，是设计的第一手输入：

| 模式 | 英文 | 本质 | 视频侧信号 | POS 侧信号 |
| --- | --- | --- | --- | --- |
| 漏扫 / 空扫 | skip-scan | 商品过扫描区无条码录入 | 物体穿越扫描窗、无"嘀" | 该时刻无商品行 |
| 假扫 | fake scan | 扫码动作没对准条码 | 手 + 商品动作但无解码 | 无条码事件 |
| 错扫 | banana trick / PLU 替换 | 高价商品按廉价生鲜计价 | 视觉见到的是牛排 | 录入的是香蕉 PLU |
| 换码 / 套码 | ticket switching | 贴假条码 | 视觉商品 ≠ 条码商品 | SKU 不匹配 |
| 底筐 | bottom of basket | 购物车底部商品不过台 | 商品在篮里未上台 | 无对应商品行 |
| 撤单滥用 | void abuse | 扫了又撤 | 动作 + 时序 | void 事件 |
| 内盗合谋 | sweethearting | 收银员不扫朋友商品 | 商品过扫描窗无录入 | T-log 无销售 |

> 关键：**每一行都需要"视频事件"与"POS 事件"对齐**才能判定。只看视频会误判（顾客可能只是在整理购物袋），只看 POS 也看不出原因（收银小票上一切正常）。

## 为什么这个问题不简单

1. **正样本稀疏。** 真实数据里漏扫事件占比极低，类别极不平衡。
2. **误报的代价不对称。** 漏报的代价是货损，误报的代价是顾客被冤枉、店员疲惫（alert fatigue）。
3. **时间同步是命门。** 视频事件与 POS 流水必须毫秒级对齐，否则判定全部漂移。
4. **长尾商品。** 生鲜、季节性商品、促销 SKU 无穷无尽，闭集分类器必然崩。
5. **硬件异构。** 已有相机视角、光照、POS 系统五花八门。
6. **隐私合规。** 人流量大、涉及生物特征的红线不能碰。

## 市场与量级

- 美国 shrink 规模：2022 年约 **\$1121 亿**（[NRF 2023](https://nrf.com/research-insights/retail-security-survey)），2025 年约 **\$1420 亿**。
- 自助收银的未知损耗率约为人工通道的 **~5 倍**（[Beck, ECR/Leicester 2016](https://www2.le.ac.uk/departments/criminology/people/adp5/documents/self-checkout-in-the-retail-environment.pdf)）。
- 主导厂商：**Everseen**（覆盖 30+ 漏损模式，[Soft Nudge](https://everseen.com/blog/retail-loss-prevention-technology) 干预、声称 80%+ 案例无需店员介入即可自纠）；国内有**元芒 iDetector**、万店掌、海康/大华、Telpo 等。

## 行业的血的教训

- **误报 → alert fatigue 是第一杀手。** 沃尔玛早期部署因误报过高，店员直接忽略提示，2023–24 年花大力气调误报权衡。
- **单次拦截成本可能只是勉强打平。** Lowe's 试点的 cost-per-incident-prevented 第一年接近 break-even。
- **全场景无人结算经济性不成立。** Standard AI 裁员收缩、Amazon 将 Just Walk Out 从自营店撤下；可落地的是"收银台叠加检测 + 流程改造"（[pdpspectra 2026](https://pdpspectra.com/blog/ai-retail-loss-prevention-2026/)）。

## 对我们的启示

- 系统的护城河是**事件契约 + 误报调优经验 + 数据闭环 + POS 适配广度**，而不是某个更强的模型。
- 因此本书把**"AI 部分"抽象为 $f$**，先把它的接口和内部组件设计清楚，再谈训练与部署。

下一篇：[把 AI 抽象为 f](./abstraction.md)。
