+++
title = "2025 agent 技术汇编"
description = "本文是对 agent 相关技术的一个汇编"
data = "2025-12-21"
dfaft = false
template = "blog/page.html"

[extra]
toc = true
+++

作为一个 IT 从业者，我们自身是最能感受到 AI 技术对日常工作的影响的群里：claude code/gemini/codex 已经成为我们日常编程的主力助手，是 agent 技术的
深度使用者。

# Context Engineering

究竟是 prompt engineering 还是 context engineering？我一度也纠结过这个概念，不过从本质的角度来看，二者的本质区别并不大，只是随着应用的复杂度提升，在工程
领域需要不断的优化：本质上还是提供给 LLM 一个更合理的上下文（或者描述为 input 更为朴素）。

本质：
- LLM context 长度限制，导致我们不能将所有知识都放入 context 中
  - 每个 LLM 都有自己的 context 长度限制（趋势是，这个门槛在逐渐提升，很多都是 256K ）
  - 越长的输入，LLM 的成本也显著提升，响应速度也会变慢。（）
  - 知识越多，带给 LLM 的干扰、歧义也越多, LLM 就越会迷失注意力。
- context 包括：
  - rules: 大部份的 Prompt 都是围绕规则展开的：Should do, Should not do
  - examples: 典型的 few-shot learning 的例子
  - knowledge, references: 具体的知识、参考资料
  - tools: tool's reference, examples, and rules
  - user input: 用户的具体输入
  - history
    - tool's invocation history
    - conversation history

分类：
1. knowledge representation
   - Agent Skills ( metadata, Skill.md,  scripts, references, assets) 本质上是一种知识的分层表示。
2. RAG

3. memory management

## Agent Skills
[Agent Skills](https://agentskills.io/) 本质上是一种针对技能知识的分层表示方式, Skills 就像给 agent 安装的"技能包"，每个 skill 包含：
- Metadata: 技能的名称、描述、触发条件
- Instructions (skill.md): 详细的执行指令和规则 (包括规则、指令，以及对 scripts/references/assets 内容的索引)
- Scripts: 可执行的脚本文件
- References: 相关文档和示例
- Assets: 配置文件、模板等资源

解决的问题

1. 知识复用: 将常见任务（如 code review、测试生成）封装成 skill，避免重复编写 prompt
2. Context 优化: 按需加载相关 skill 的 context，而非每次都加载所有指令
3. 领域专业化: 针对特定场景（如特定框架、代码规范）定制 agent 行为

工作方式
```text
/review-code  →  触发 "review-code" skill
              →  加载 skill.md 到 context
              →  执行 skill 定义的流程
              →  使用 skill 中的 tools/scripts
```

典型应用

- /commit: 智能生成 git commit message
- /review-pr: PR 代码审查
- /test: 自动生成测试用例
- 自定义业务 skills（如特定框架的最佳实践检查）

> 使用 Knowledge Graph 来组织 Skills ？
方案 1: Hybrid 混合模式（推荐）

文件系统存储 + Graph 元数据

```
skills/
├── code-review/
│   ├── skill.md
│   └── metadata.yaml  ← 定义 graph 关系
├── security-scan/
└── graph.json         ← 全局关系图

metadata.yaml:
name: code-review
dependencies:
- security-scan
- performance-check
triggers:
- auto: ["*.pr", "pull_request"]
- command: ["/review", "/pr"]
tags: [code-quality, automation]
related_skills:
- refactoring (similarity: 0.8)
- test-generation (often_used_together)
```

实现：
```python
class SkillGraph:
def __init__(self):
self.graph = nx.DiGraph()  # NetworkX

      def load_from_filesystem(self, skills_dir):
          for skill_dir in skills_dir.iterdir():
              metadata = yaml.load(skill_dir / "metadata.yaml")
              self.add_skill_node(skill_dir, metadata)
              self.add_edges(metadata.dependencies)

      def resolve_dependencies(self, skill_name) -> List[Skill]:
          """返回依赖的 skills（拓扑排序）"""
          return nx.topological_sort(
              self.graph.subgraph(nx.ancestors(self.graph, skill_name))
          )

      def recommend_skills(self, task_context) -> List[Skill]:
          """基于 graph 推荐相关 skills"""
          # 使用 graph embedding 或 PageRank
```          

方案 2: Pure Graph 存储

使用 Graph Database（如 Neo4j）：

```
// 创建 skill
CREATE (review:Skill {
name: "code-review",
path: "/skills/code-review"
})

// 创建关系
CREATE (review)-[:DEPENDS_ON]->(security:Skill {name: "security-scan"})
CREATE (review)-[:USES_REFERENCE]->(guide:Reference {path: "style-guide.md"})
CREATE (review)-[:APPLICABLE_TO]->(lang:Language {name: "Python"})

// 查询：找到所有 Python 相关的 review skills
MATCH (s:Skill)-[:APPLICABLE_TO]->(l:Language {name: "Python"})
(s)-[:DEPENDS_ON*0..2]->(dep)  // 包括间接依赖
RETURN s, collect(dep)
```

何时使用 Graph？

| 场景                 | 文件系统     | Graph  |
|--------------------|----------|--------|
| 简单独立 skills (<10个) | ✅ 足够     | ❌ 过度设计 |
| Skills 有依赖关系       | ⚠️ 需手动管理 | ✅ 自动解析 |
| 需要智能推荐             | ❌ 难实现    | ✅ 适合   |
| 多维度检索 (语言+领域+工具)   | ❌ 只能按目录  | ✅ 灵活查询 |
| Skill 组合编排         | ⚠️ 硬编码   | ✅ 动态组合 |
| 版本演化追踪             | ❌        | ✅      |

