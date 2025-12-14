# LoRA 中的 $W_0$ 是什么?

## 简短回答

**$W_0$ 不是指某一个特定的矩阵,而是指 Transformer 中任意一个预训练的权重矩阵**。

LoRA 可以应用到 Transformer 的多个权重矩阵上,论文中主要关注 **Self-Attention 模块中的 4 个投影矩阵**。

---

## Transformer 中可以应用 LoRA 的权重矩阵

### 1. Self-Attention 模块的权重矩阵

在 Transformer 的 Self-Attention 机制中,有 4 个关键的权重矩阵:

```
输入: x ∈ ℝ^(d_model)

Query投影:  Q = x · Wq,  其中 Wq ∈ ℝ^(d_model × d_model)
Key投影:    K = x · Wk,  其中 Wk ∈ ℝ^(d_model × d_model)
Value投影:  V = x · Wv,  其中 Wv ∈ ℝ^(d_model × d_model)
输出投影:   O = attn · Wo, 其中 Wo ∈ ℝ^(d_model × d_model)
```

这 4 个矩阵 **$W_q, W_k, W_v, W_o$ 都可以作为 LoRA 中的 $W_0$**。

### 2. MLP (Feed-Forward) 模块的权重矩阵

Transformer 的每一层还包含一个两层的前馈网络:

```
FFN(x) = max(0, x·W1 + b1)·W2 + b2

其中:
W1 ∈ ℝ^(d_model × d_ff)     # 第一层,通常 d_ff = 4 × d_model
W2 ∈ ℝ^(d_ff × d_model)     # 第二层
```

这两个权重矩阵 **$W_1, W_2$ 也可以作为 LoRA 中的 $W_0$**。

---

## LoRA 论文的实际选择

### 论文的策略

LoRA 论文在实验中主要 **只对 $W_q$ 和 $W_v$ 应用 LoRA**,而冻结其他权重矩阵。

### 为什么这样选择?

论文在 Section 7.1 做了消融实验,对比了不同权重矩阵组合的效果:

| 应用 LoRA 的矩阵 | 秩 r | WikiSQL 准确率 | MultiNLI 准确率 |
|-----------------|------|---------------|----------------|
| 只有 $W_q$      | 8    | 70.4%         | 91.0%          |
| 只有 $W_k$      | 8    | 70.0%         | 90.8%          |
| 只有 $W_v$      | 8    | 73.0%         | 91.0%          |
| 只有 $W_o$      | 8    | 73.2%         | 91.3%          |
| $W_q + W_k$     | 4    | 71.4%         | 91.3%          |
| **$W_q + W_v$** | **4**| **73.7%**     | **91.3%**      |
| $W_q + W_k + W_v + W_o$ | 2 | 73.7%    | 91.7%          |

**结论**:
- 只适配 $W_q$ 或 $W_k$ 效果较差
- 适配 $W_q + W_v$ 在相同参数量下效果最好
- 适配所有 4 个矩阵提升有限,但参数量翻倍

因此论文选择 **在 $W_q$ 和 $W_v$ 上应用 LoRA**,作为性能和效率的平衡。

---

## LoRA 的具体应用方式

### 以 $W_q$ 为例

假设原始的 Query 投影是:

$$
Q = x \cdot W_q^{(0)}
$$

其中 $W_q^{(0)} \in \mathbb{R}^{d_{model} \times d_{model}}$ 是预训练的权重矩阵。

### 应用 LoRA 后

$$
Q = x \cdot (W_q^{(0)} + \Delta W_q) = x \cdot W_q^{(0)} + x \cdot B_q A_q
$$

其中:
- $W_q^{(0)}$ 被**冻结**,不参与训练
- $B_q \in \mathbb{R}^{d_{model} \times r}$ 是**可训练**的
- $A_q \in \mathbb{R}^{r \times d_{model}}$ 是**可训练**的
- $r \ll d_{model}$ (论文中 r = 1~8)

### 参数量对比

**原始参数量**: $|W_q| = d_{model}^2$

**LoRA 参数量**: $|B_q| + |A_q| = d_{model} \cdot r + r \cdot d_{model} = 2 \cdot d_{model} \cdot r$

**压缩比例**: $\frac{2r}{d_{model}}$

**实例 (GPT-3)**:
- $d_{model} = 12288$
- $r = 4$
- 压缩比 = $\frac{2 \times 4}{12288} = \frac{8}{12288} \approx 0.065\%$

---

## 更详细的例子:GPT-3 中的应用

### GPT-3 175B 的结构
- 层数: $L = 96$
- 模型维度: $d_{model} = 12288$
- 每层有 4 个 attention 权重矩阵

### 完整微调的参数量
如果微调所有 attention 矩阵:

$$
4 \times L \times d_{model}^2 = 4 \times 96 \times 12288^2 \approx 58 \text{ billion 参数}
$$

### LoRA 微调的参数量
只对 $W_q$ 和 $W_v$ 应用 LoRA,且 $r = 4$:

$$
2 \times L \times 2 \times d_{model} \times r = 2 \times 96 \times 2 \times 12288 \times 4 \approx 18.9 \text{ million 参数}
$$

**压缩比**: $\frac{18.9M}{58B} \approx 0.03\%$ (约 **3000 倍**压缩)

---

## 总结

1. **$W_0$ 是通用符号**,代表 Transformer 中任意一个预训练的权重矩阵

2. **可应用 LoRA 的矩阵**:
   - Self-Attention: $W_q, W_k, W_v, W_o$
   - Feed-Forward: $W_1, W_2$

3. **论文的选择**: 主要对 $W_q$ 和 $W_v$ 应用 LoRA

4. **应用方式**:
   $$W_0 \rightarrow W_0 + \Delta W = W_0 + BA$$
   - 冻结 $W_0$
   - 只训练低秩矩阵 $B$ 和 $A$

5. **为什么有效**:
   - 模型适配只需要修改少量"任务相关"的特征方向
   - 这些方向可以用低秩矩阵表示

---

## 与 Transformer 知识的连接

基于你已学习的 Transformer 知识,可以这样理解:

1. **Attention 机制中的 $W_q, W_k, W_v$**: 你应该知道这些是如何计算 Query, Key, Value 的

2. **LoRA 的改动**: 不改变 attention 的计算流程,只是让投影矩阵 = 原矩阵 + 低秩更新

3. **结果**: Attention 的输出会略微调整,以适应下游任务

这就像是给每个 attention 投影矩阵加了一个"微调旋钮",而这个旋钮只有很少的参数。

---

## 下一步学习建议

1. **复习 Transformer 的权重矩阵**:确保理解每个矩阵的作用

2. **手工计算参数量**:选择一个具体的模型(如 GPT-2 Medium),计算 LoRA 的参数量

3. **理解为什么 $W_q + W_v$ 最优**:思考这与 attention 机制的关系

4. **可视化前向传播**:画出应用 LoRA 后的计算图

需要我详细讲解某个部分吗?
