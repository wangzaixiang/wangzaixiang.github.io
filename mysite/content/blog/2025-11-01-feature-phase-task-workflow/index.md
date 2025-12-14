+++
title = "FPT workflow"
description = "这是我在使用 claude code 过程中，和 code agent 一起定义的开发流程规范，非常适合于使用后 vibe coding 进行复杂项目的开发"
data = "2025-11-01"
dfaft = false
template = "blog/page.html"

[extra]
toc = true
+++

# Feature-Phase-Task Workflow

本文档定义了 Vibe Coding 实施项目的流程和规范。

## 术语表

| 术语      | 定义             | 示例                     |
|---------|----------------|------------------------|
| Feature | 独立的功能模块        | scheme_v2, file_writer |
| Phase   | Feature 内的开发阶段 | Phase 1, Phase 2       |
| Task    | 最小工作单元（0.5-1天） | Task 1, Task 2         |
| TDD     | 测试驱动开发         | Red-Green-Refactor     |
| Stub    | API 占位实现       | `fn foo() { todo!() }` |

## 角色
- **Code Agent**：负责和 Developer 进行交互，理解需求、编写文档、代码、测试
- **Developer**：对 Code Agent 提出指令，审阅产出物，确认关键决策

## 状态符号规范

- ✅ 完成 (Completed)
- ⏳ 进行中 (In Progress)
- ⏸️ 待开始 (Pending)
- ⚠️ 受阻 (Blocked)
- 🔄 需重做 (Need Rework)

**使用规则**：
- 每个 Phase/Task 必须有明确的状态符号
- Code Agent 必须在完成后立即更新状态
- Developer 可以将任务标记为 ⚠️ 或 🔄

---

## 0. Feature 和层级结构

### Feature 定义

Feature 是一个**独立的功能特性**，可独立规划、开发、测试和交付，对应于 Git Workflow 中 Pull Request。

**Feature 示例**：
- `scheme_v1`, `scheme_v2`, `scheme_v3`：不同的存储方案
- `file_writer`：文件写入模块
- `file_reader`：文件读取模块
- `cli`：命令行工具
- `lsm_compaction`：LSM 合并逻辑

### 三层组织结构

```
Feature → Phase → Task
```

- **Feature**：独立功能模块，有独立的 设计文档, task_list.md 和测试文件
- Feature 的设计文档在 fpt/features/{feature}/design.md 下。
- **Phase**：Feature 内的开发阶段，相当于1个小的迭代，编号在 **Feature 内独立**（每个 Feature 从 Phase 1 开始）
- **Task**：可在 0.5-1 天完成的最小工作单元

如果与 Git Workflow 相对比：
- Feature 对应于 Pull Request，一般是一个完整的特性、场景。
- Phase 对应于一个 Feature 内的多个 iteration，每个 phase 完成一定的功能，并为下一个 phase 提供准备。部份代码可能再后续的 phase 清理、重构
  如果后续重构/删除了这个 phase 的代码，则也相应的重构/删除原有的测试用例。
- Task 则对应于 phase 内的字任务拆解，与TDD 的 test case 有较好的对应关系。

**示例**：
```
Feature: file_writer
  Phase 1: API 定义
    Task 1: 创建核心数据结构
    Task 2: 导出 API
  Phase 2: Mmap 初始化
    Task 1: 文件预分配
    Task 2: Mmap 映射

Feature: scheme_v2
  Phase 1: Dictionary 实现
    Task 1: DimDict API
    Task 2: Encoding 逻辑
```

---

## 1. 开发流程总览

```
规划 → 测试设计 → API 定义 → 测试评审 → 实现 → 验证 → 归档
  ↓        ↓          ↓          ↓        ↓      ↓       ↓
Phase   Test Cases  Signatures  Review   Code   Pass   Commit
```

### 流程说明

1. **规划**：将功能需求分解为 Feature、Phase 和 Task
2. **测试设计**：编写测试用例，定义预期行为
3. **API 定义**：定义数据结构和接口签名，使用 `todo!()` 占位
4. **测试评审**：Developer 确认测试用例、API 设计是否符合需求
5. **实现**：实现功能逻辑，使测试通过
6. **验证**：运行测试，确保所有测试通过
7. **归档**：更新 `fpt/features/{feature}/task_list.md`，提交代码

---

## 2. Phase 划分

Phase 划分由 Code Agent 根据 Developer 指令进行，规划后需与 Developer 确认。

### Phase 规范
- Phase 编号在 **Feature 内独立**，每个 Feature 从 Phase 1 开始
- 每个 Phase 有清晰的开发目标，可作为后续 Phase 的基础（视为一个 Iteration）
- 包含 3-8 个 Task，每个 Task 0.5-1 天可完成
- 有明确的验收标准和产出物

### 命名规范
```
Feature: file_writer
  Phase 1: API 定义
  Phase 2: Mmap 初始化
  Phase 3: Schema 定义

Feature: scheme_v2
  Phase 1: Dictionary 实现
  Phase 2: Encoding 逻辑
```

### phase 0: 端到端的API 设计阶段
部份复杂性的任务需要插入 phase 0 的阶段，这个阶段的目的是：采用 TDD 的方式，通过定义端到端的场景，来定义相关的 API 和数据结构，评审通过后
再开启下一阶段的的开发任务。

- 定义端到端的API，即模块对外提供的 pub API + struct，而非内部的、实现细节粒度。
- 编写测试用例来模拟使用场景，演示 API 的使用。
- 仅编写最小化的代码（todo!() 替代实现），使其通过编译。

一般的，在 phase 0 完成后，应该交给 developer 来进行评审，评审确认后，再开始工作：
- 更新 task_list.md （需要根据评审中的变化而调整计划）

---

## 3. Task 分解规则

### Task 规范
- 单个 Task 不超过 0.5-1 天工作量
- 可独立测试和验证
- 有明确的输入和输出

### 命名规范
```
Feature: file_writer
  Phase 1: API 定义
    Task 1: 创建核心数据结构
    Task 2: 导出 API
  Phase 2: Mmap 初始化
    Task 1: 文件预分配
    Task 2: Mmap 映射
    Task 3: Drop 清理逻辑
```

---

## 4. 测试用例规范

### 4.1 命名规范

**测试文件**: `{crate}/tests/{feature}_phase_{seq}_description.rs`
- `crate`: 所在 crate（如 molap_file, lsm）
- `feature`: Feature 名称（如 file_writer, scheme_v2）
- `seq`: Phase 序号（在 Feature 内独立编号，**仅1级编号**，如 1, 2, 3）
- `description`: 简短描述

**测试函数**: `task_{phase}_{task}_description`
- `phase`: Phase 序号（**仅1级编号**，如 1, 2, 3）
- `task`: Task 序号（**仅1级编号**，如 1, 2, 3）
- `_`: 下划线分隔符
- `description`: 简短描述

**示例**:
```rust
// 文件名: molap_file/tests/file_writer/phase_2_mmap_init.rs

/// Task 2-1: 文件预分配逻辑
#[test]
fn task_2_1_basic_file_creation() { ... }

/// Task 2-3: Drop 清理逻辑
#[test]
fn task_2_3_drop_cleanup() { ... }
```

```rust
// 文件名: molap_file/tests/scheme_v2/phase_1_dictionary.rs

/// Task 1-1: DimDict API 定义
#[test]
fn task_1_1_dim_dict_api() { ... }

/// Task 1-2: Encoding 逻辑
#[test]
fn task_1_2_encoding_logic() { ... }
```

### 4.2 文件组织

**按 Feature 分目录**：每个 Feature 有独立的测试子目录

```
molap_file/tests/
├── file_writer/
│   ├── phase_1_v1.rs              # Feature: file_writer, Phase 1
│   ├── phase_2_mmap.rs            # Feature: file_writer, Phase 2
│   └── phase_3_schema.rs          # Feature: file_writer, Phase 3
├── scheme_v1/
│   └── phase_1_basic.rs           # Feature: scheme_v1, Phase 1
└── scheme_v2/
    ├── phase_1_dictionary.rs      # Feature: scheme_v2, Phase 1
    └── phase_2_encoding.rs        # Feature: scheme_v2, Phase 2

lsm/tests/
├── memtable/
│   └── phase_1_basic.rs           # Feature: memtable, Phase 1
└── compaction/
    └── phase_1_merge.rs           # Feature: compaction, Phase 1
```

### 4.2.1 测试文件路径生成规则

Code Agent 创建测试文件时，必须遵循以下规则：

1. **确定 crate**：根据功能所属的 crate（如 molap_file、lsm）
2. **确定 feature**：从 task_list.md 的文件路径中提取 feature 名称
3. **确定 phase**：从当前 Phase 标题中提取序号
4. **生成路径**：`{crate}/tests/{feature}/phase_{seq}_{description}.rs`

**示例**：
- task_list 路径：`fpt/features/scheme_v2/task_list.md`
- 当前 Phase：`Phase 3: Data Encoding`
- 生成测试文件：`molap_file/tests/scheme_v2/phase_3_encoding.rs`

**文件内部按 Task 分组**：
```rust
//! Feature: file_writer
//! Phase 2: 文件预分配 + Mmap 初始化
//!
//! # Tasks
//! - Task 1: 文件预分配逻辑
//! - Task 2: Mmap 初始化
//! - Task 3: Drop 清理逻辑

use molap_file::{FileWriter, FileConfig};
use std::fs;

// ============================================================================
// 辅助函数
// ============================================================================

fn cleanup_test_file(path: &str) { ... }

// ============================================================================
// Task 1: 文件预分配
// ============================================================================

/// Task 2-1: 文件预分配逻辑
#[test]
fn task_2_1_basic_file_creation() { ... }

// ============================================================================
// Task 2: Mmap 初始化
// ============================================================================

/// Task 2-2: Mmap 初始化
#[test]
fn task_2_2_mmap_initialization() { ... }
```

### 4.3 测试覆盖

每个 Phase 至少包含：
- 正常流程测试
- 边界情况测试
- 错误处理测试

每个测试函数对应一个或多个 Task，在文档注释中明确说明覆盖范围。

---

## 5. 实现流程（TDD 循环）

### Red → Green → Refactor（严格执行顺序）

**重要：Code Agent 必须严格按照以下顺序执行，禁止跳过或颠倒步骤**

1. **定义测试（Red）**：
    - 先在 `tests/{feature}/phase_{seq}_*.rs` 中编写测试函数
    - 测试函数命名必须遵循 `task_{phase}_{task}_description` 格式
    - 验证测试失败（因为功能尚未实现）

2. **定义 API（Stub）**：
    - 定义满足测试的最小接口签名
    - 函数体使用 `todo!()` 或 `unimplemented!()`
    - 确保 `cargo test --no-run` 编译通过

3. **测试评审点**：
    - Code Agent 向 Developer 展示测试用例
    - 等待 Developer 确认后再继续实现

4. **实现功能（Green）**：
    - 实现逻辑，使测试通过
    - 优先最简单的实现，不过度设计

5. **验证测试通过**：
    - 运行 `cargo test task_{phase}_{task}`
    - 所有测试必须通过，无 panic 或 warnings

6. **重构（Refactor）**：
    - 优化代码结构，保持测试通过
    - 每次重构后重新运行测试

### Code Agent 操作检查清单

**在每个 Task 开始前，Code Agent 必须**：
- 确认已读取 `fpt/features/{feature}/task_list.md`
- 确认当前 Task 的编号和描述

**在每个 Task 完成后，Code Agent 必须**：
- 更新 `fpt/features/{feature}/task_list.md` 中的状态
- 运行 `cargo test task_{phase}_{task}` 验证测试通过

---

## 6. 进度跟踪规范

每个 Feature 的 Phase 和 Task 信息写入到 `fpt/features/{feature}/task_list.md` 文件。

### 文件路径规范

```
fpt/features/
├── file_writer/
│   └── design.md             # file_writer feature 的设计文档
│   └── task_list.md          # file_writer feature 的任务清单
└── cli/
    └── task_list.md
```

### 更新职责
- **Code Agent**：每个 Task/Phase 完成后自动更新
- **Developer**：审阅并调整优先级或计划

### 文件格式示例

```markdown
# file_writer Task List

**Git Branch**: `feature/file_writer`
**Base Branch**: `main`
**Status**: ⏳ 进行中
**Created**: 2025-01-10
**Last Updated**: 2025-01-15

---

## Phase 1: API 定义 + 测试框架 (✅ 完成)
- **状态**: ✅ 完成
- **时间**: 2025-01-XX
- **产出**: file_writer.rs, tests/file_writer/phase_1_v1.rs
- **Commits**: `a1b2c3d` - `e4f5g6h`
- **验收标准**: 所有 API 编译通过，测试框架就绪

| Task | 描述 | 状态 |
|------|------|------|
| 1 | 创建核心数据结构 | ✅ 完成 |
| 2 | 导出 API | ✅ 完成 |

## Phase 2: 文件预分配 + Mmap 初始化 (⏳ 进行中)
- **状态**: ⏳ 进行中
- **产出**: FileWriter::new(), tests/file_writer/phase_2_mmap.rs
- **验收标准**: 文件预分配正确，Mmap 初始化成功，资源正确清理

| Task | 描述 | 状态 |
|------|------|------|
| 1 | 文件创建与预分配 | ✅ 完成 |
| 2 | Mmap 初始化 | ⏳ 进行中 |
| 3 | Drop 清理逻辑 | ⏸️ 待开始 |
```

注意事项：
- 在 task_list 中，不要写入太多的技术细节，包括方案描述、代码等，重要在于描述任务目标，以维持该文件的简洁性。
- phase 题中包括该 phase 的当前状态，方便在 IDE 中快速通过 outline 查看进度。
- task table 需要包括： Task、Description, Status 三列。
- 完成 task 或 phase 后，及时更新该文档的进度状态。
- 在每个 phase 下，根据需要，可以补充上相关文档的链接。

---

## 7. 验收标准

### Task 完成标准
- ✅ 对应测试通过（`cargo test task_{phase}_{task}`）
- ✅ 代码有清晰注释
- ✅ 无新增 warnings（除合理的 `unused`）

### Phase 完成标准
- ✅ 所有 Task 测试通过
- ✅ 更新 `fpt/features/{feature}/task_list.md`
- ✅ Developer 确认验收

---

## 8. 沟通模式

### 8.1 Phase 开始前

**Code Agent**：
1. 展示完整的 Phase 计划（Task 清单、时间估算）
2. 说明技术方案和设计决策
3. 等待 Developer 确认后开始实施

**Developer**：
- 审阅 Phase 计划
- 提出调整建议或优化方案
- 明确确认后 Code Agent 开始实施

### 8.2 实施过程中

**Code Agent**：
- 每完成一个 Task 向 Developer 汇报进度
- 遇到设计问题及时询问 Developer

**Developer**：
- 及时回复问题
- 确认关键设计决策

### 8.3 Phase 完成后

**Code Agent**：
1. 总结完成内容和测试结果
2. 更新 `doc/tdd/{feature}/task_list.md`
3. 说明下一步计划

**Developer**：
- 验证产出物
- 确认是否进入下一 Phase

### 8.4 完整对话示例

**场景：开始 scheme_v2 Phase 1 Task 1**

**Code Agent**:
> 我将开始 Feature `scheme_v2` 的 Phase 1 Task 1: "DimDict API 定义"
>
> 我的计划是：
> 1. 创建测试文件 `molap_file/tests/scheme_v2/phase_1_dictionary.rs`
> 2. 编写测试函数 `task_1_1_dim_dict_api()`
> 3. 定义 DimDict 的数据结构和接口签名
> 4. 使用 `todo!()` 占位实现
> 5. 验证编译通过
>
> 测试用例将验证：
> - DimDict::new() 创建空字典
> - DimDict::insert() 插入维度值
> - DimDict::get() 查询维度 ID
>
> 是否开始？

**Developer**:
> 确认，开始吧。记得 DimDict 需要支持 Option<String> 类型。

**Code Agent**:
> 收到，我会在测试中包含 None 值的处理。开始实施...

---

## 9. 异常处理流程

### 9.1 测试编译失败

Code Agent 必须立即检查：
- API 签名是否与测试代码匹配
- 依赖关系是否正确
- 模块导出是否完整

### 9.2 测试运行失败

Code Agent 必须：
- 仔细阅读测试输出和断言信息
- 定位失败的具体测试函数
- 修复实现代码，而不是修改测试（除非测试本身有错误）
- 向 Developer 汇报持续失败的测试

### 9.3 测试通过但逻辑有问题

当 Developer 发现问题时：
- Code Agent 必须先增加新的测试用例覆盖该场景
- 然后修复实现使新测试通过

---

## 10. Git 工作流集成

### 10.1 分支策略（Git Worktree）

#### 分支命名规范

每个 Feature 对应一个独立的 Git 分支：

```
main                           # 主分支
├── feature/scheme_v2          # Feature 分支
├── feature/file_writer        # Feature 分支
└── feature/cli                # Feature 分支
```

**规则**：
- 每个 Feature 对应一个 `feature/{feature_name}` 分支
- Feature 分支从 `main` 分支创建
- Phase 和 Task 的 commit 都在 Feature 分支上进行
- Feature 完成后通过 Pull Request 合并回 `main`

#### Git Worktree 并行开发（推荐）

使用 Git Worktree 实现多 Feature 真正并行开发：

```bash
# 主工作目录
cd /path/to/{project}

# 为每个 Feature 创建独立工作目录
git worktree add ../{project}-scheme_v2 feature/scheme_v2
git worktree add ../{project}-file_writer feature/file_writer

# 在不同终端/窗口中并行开发
cd ../{project}-scheme_v2      # Terminal 1
cd ../{project}-file_writer    # Terminal 2
```

**优点**：
- 每个 Feature 有独立的工作目录和编译状态
- 无需频繁切换分支
- 可以同时运行多个测试套件
- 避免分支切换导致的文件变动和重新编译

**清理 Worktree**：
```bash
# Feature 合并后删除 worktree
git worktree remove ../{project}-scheme_v2
```

### 10.2 Commit 规范

#### Task 完成时

```bash
git add .
git commit -m "task: Phase {phase} Task {task} - {description}"
```

**示例**：
```bash
git commit -m "task: Phase 1 Task 2 - DimDict encoding logic"
git commit -m "task: Phase 2 Task 1 - File preallocation"
```

**说明**：Feature 名称已包含在分支名中，无需在 commit message 中重复。

#### Phase 完成时

```bash
git add fpt/features/{feature}/task_list.md
git commit -m "phase: Phase {phase} complete - {summary}"
```

**示例**：
```bash
git commit -m "phase: Phase 1 complete - Dictionary implementation"
```

#### Feature 完成时

```bash
git add .
git commit -m "feature: {feature} complete"
git push origin feature/{feature}
```

### 10.3 完整工作流程示例

```bash
# ========================================
# Developer 启动新 Feature
# ========================================
cd /path/to/{project}
git checkout main
git pull
git checkout -b feature/scheme_v2

# 创建独立工作目录（使用 worktree）
git worktree add ../{project}-scheme_v2 feature/scheme_v2
cd ../{project}-scheme_v2

# 创建 Feature 文档
mkdir -p fpt/features/scheme_v2
# 编写 design.md 和 task_list.md

git add fpt/features/scheme_v2/
git commit -m "docs: Initialize scheme_v2 feature"

# ========================================
# Code Agent 开始 Phase 1
# ========================================
# Task 1: DimDict API 定义
# ... 编写测试、实现 ...
git add .
git commit -m "task: Phase 1 Task 1 - DimDict API definition"

# Task 2: Encoding 逻辑
# ... 编写测试、实现 ...
git add .
git commit -m "task: Phase 1 Task 2 - Encoding logic"

# Phase 1 完成
git add fpt/features/scheme_v2/task_list.md
git commit -m "phase: Phase 1 complete - Dictionary implementation"

# ========================================
# Phase 2, Phase 3 ...
# ========================================
# ... 重复上述流程 ...

# ========================================
# Feature 完成
# ========================================
# 所有测试通过
cargo test --all

# 最终更新
git add .
git commit -m "feature: scheme_v2 complete"

# 推送并创建 PR
git push origin feature/scheme_v2
gh pr create --title "Feature: scheme_v2 - New storage scheme" --base main

# ========================================
# Code Review & Merge
# ========================================
# Developer 审查 PR
# 合并到 main
# 清理 worktree
cd /path/to/{project}
git worktree remove ../{project}-scheme_v2
git branch -d feature/scheme_v2
```

### 10.4 Code Agent 检查清单（Git 相关）

#### 在每个 Feature 开始前
- ✅ 确认当前在正确的 Feature 分支（`git branch --show-current`）
- ✅ 如果使用 worktree，确认在正确的工作目录

#### 在每个 Task 完成后
- ✅ 使用规范的 commit message（`task: Phase X Task Y - description`）
- ✅ Commit 变更到当前 Feature 分支

#### 在每个 Phase 完成后
- ✅ 更新 `task_list.md`（标记 Phase ✅）
- ✅ 创建 Phase 总结 commit（`phase: Phase X complete - summary`）

#### 在 Feature 完成后
- ✅ 所有 Phase 测试通过
- ✅ 更新 `task_list.md`（标记 Feature ✅）
- ✅ 推送到远程分支（`git push origin feature/{name}`）
- ✅ 创建 Pull Request

### 10.5 Pull Request 规范

#### PR 标题格式
```
Feature: {feature_name} - {one-line summary}
```

#### PR 描述模板
```markdown
## Feature 概述
{简短描述功能}

## Phase 清单
- [x] Phase 1: Dictionary 实现
- [x] Phase 2: Encoding 逻辑
- [x] Phase 3: Integration 测试

## 测试结果
```bash
cargo test --all
# 所有测试通过
```

## 相关文档
- 设计文档: `fpt/features/scheme_v2/design.md`
- 任务清单: `fpt/features/scheme_v2/task_list.md`

## Checklist
- [x] 所有 Phase 测试通过
- [x] 更新 task_list.md
- [x] 无新增 warnings
- [x] 代码审查通过
```

### 10.6 多 Feature 冲突处理

#### 策略：定期同步 main 的变更

```bash
# 在 Feature 分支（每完成一个 Phase 后）
git fetch origin main
git rebase origin/main

# 解决冲突（如有）
# ...

git rebase --continue
```

**建议**：
- 优先使用 Rebase 保持线性历史
- 在 Phase 完成后同步 main 的变更
- Code Agent 在 Phase 开始前检查是否需要同步

---

## 11. 常见问题排查

### Q: 测试文件找不到被测试的模块
**A**: 检查 `Cargo.toml` 中的 `[lib]` 配置，确保模块已导出

### Q: Phase 编号冲突
**A**: Phase 编号在 Feature 内独立，不同 Feature 可以有相同的 Phase 编号

### Q: 测试函数命名不符合规范
**A**: 必须使用 `task_{phase}_{task}_description` 格式，phase 和 task 都是单层编号

### Q: `cargo test` 找不到特定测试
**A**: 使用完整的测试名过滤，如 `cargo test task_2_1` 或 `cargo test --test phase_2_mmap`

### Q: 测试通过但 Code Agent 未更新 task_list.md
**A**: 这违反了操作检查清单，Code Agent 必须在每个 Task 完成后立即更新状态