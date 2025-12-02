# 跟着 AI 学习 prosemirror 框架

本项目是通过 claude code 学习论文 [prosemirror](~/workspaces/github.com/prosemirror/) ，相关的学习记录将整理成为 mdbook。

## 学习过程
- 我会通过提问的方式，向 Teacher（Cluade Code CLI） 提出问题。
- Teacher 可以将有关的问答先记录到 qa/目录下。例如：
  -  [001-论文概述.md](./qa/001-论文概述.md) 其中 001 为编号，递增。
  - 如果提问明确为对 001 号问题的追问，请将问答内容作为补充追加到该文件中，而非创建一个新的文件。
  - 请保持一问一答的方式，不要在回答完问题后，继续教授无关的内容。这些可以作为回答中的“参考信息”予以说明。
- 在一定的阶段，我会要求根据对话更新 mdbook，作为学习记录和总结。
- 回答内容可以使用更高层次的模型抽象，例如基于数学公式的的概念等。

## 本地实验工程
1. 在 `/Users/wangzaixiang/workspaces/wangzaixiang/l_prosemirror` 这个目录下，可以创建用于学习目的的代码实验。
2. prosemirror 本地源代码。如果需要分析源代码，请从如下目录读取。
   - prosemirror-model:  /Users/wangzaixiang/workspaces/github.com/prosemirror/prosemirror-model
   - prosemirror-state:  /Users/wangzaixiang/workspaces/github.com/prosemirror/prosemirror-state
   - prosemirror-transform:  /Users/wangzaixiang/workspaces/github.com/prosemirror/prosemirror-transform
   - prosemirror-view:  /Users/wangzaixiang/workspaces/github.com/prosemirror/prosemirror-view

在文章中，图表类使用 mermaid 格式编写。
