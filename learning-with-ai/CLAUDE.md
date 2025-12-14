# 跟着 AI 学习 attention is all you need 论文

本目录记录我通过 Claude Code 来进行学习的记录和整理。

## 学习过程
- 我会通过提问的方式，向 Teacher（Cluade Code CLI） 提出问题。
- Teacher 可以将有关的问答先记录到 qa/目录下。例如：
  -  [001-论文概述.md](./qa/001-论文概述.md)
- 在一定的阶段，我会要求根据对话更新 mdbook，作为学习记录和总结。
- 我期望从 数学的角度理解模型的输入、输出、和计算过程。

数学公式使用 mdBook 支持的 LaTeX 格式：
1. 行内公式：使用 `$...$`
   - 例如：$O(1)$、$n < d$、$\frac{1}{\sqrt{d_k}}$
2. 独立公式块：使用 `$$...$$`
   - 例如：注意力机制的核心公式

图表类使用 mermaid 格式编写。

1. [attention is all you need 论文学习](./attention-is-all-you-need)
2. [Lora 论文学习](./lora)