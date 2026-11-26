# 模型选型一览

以 $f$ 的组件为索引，汇总候选模型、数据形态与端侧友好度。模型水位标注为 2025–2026。

| 子问题 | 候选模型族 | 训练数据形态 | 端侧友好度 |
| --- | --- | --- | --- |
| **目标检测**（人 / 手 / 商品） | RF-DETR / D-FINE / YOLOv12 / YOLO26 | 帧 + box | 高（YOLO nano/s） |
| **手部关键点** | MediaPipe Hands / RTMPose (MMPose) | 帧 + 21 点 | 极高 |
| **人体姿态**（可选） | RTMPose / YOLO-Pose | 帧 + 骨架 | 高 |
| **分割**（可选，离线） | MobileSAM / SAM2 | 掩码 | 低（留给离线） |
| **跟踪**（非 NN） | ByteTrack / BoT-SORT / OC-SORT；ReID：OSNet / FastReID | 轨迹（半自动） | 极高（CPU 级） |
| **动作 / 交互**（可选） | X3D / TSM / UniFormer；HOI 参考 100DOH | 片段 + 类别 / 区间 | 中 |
| **细粒度商品** | ConvNeXt / EfficientNet；或 CLIP / SigLIP embedding + 检索 | 裁剪图 + 类别（长尾） | 中–高 |
| **异常检测**（可选） | 无监督 / 自编码器 | 仅正常样本 | 高 |
| **VLM**（离线增强） | Qwen-VL / InternVL / SigLIP | 图文对 | 低（云 / 离线） |

## 选择原则

- **算力预算反推模型尺寸**：端侧盒（4 路 / INT8 / <1–2 s）通常只能跑 YOLO nano/s 级。
- **长尾用检索而非闭集分类**：CLIP / SigLIP embedding + 近邻，新品入库无需重训。
- **优先现成模型**：MediaPipe Hands、ByteTrack 无需训练即可用于 MVP。
- **别自研 backbone**：复用开源预训练权重。

## 最小可用组合（MVP）

$$
\text{YOLO nano/s} + \text{MediaPipe Hands} + \text{ByteTrack} + \text{规则引擎}
$$

不训练任何模型即可跑通事件流。

## 相关论文（占位，待补充链接）

- YOLOv12（NeurIPS 2025）、YOLO26、RF-DETR、D-FINE、RT-DETR
- ByteTrack、BoT-SORT、OC-SORT
- MediaPipe Hands、RTMPose
- X3D、TSM、UniFormer、VideoMAE
- 100DOH（手物交互）
- SAM2、CLIP、SigLIP

参考：[参考资料](./references.md)。
