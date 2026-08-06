+++
title = "AI时代的商业软件编程语言选择"
description = "从零售 ERP 后端这个具体领域出发，讨论 AI 时代的编程语言选型：类型系统、表达能力、DSL 与宏、OO 与 FP、Direct Style、ORM-less，以及 Scala 3 在其中的位置"
date = 2026-08-06
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["wangzx"]

[extra]
toc = true
+++

# 背景

语言之争永远都是话题。近期 21CTO
的一篇文章[《从性能、生产力和 AI 实现的角度，Go、Rust 和 Zig 谁才是最终赢家？》](https://mp.weixin.qq.com/s/iSdP6DedB-MTZQsO4WMxkQ)
提出一个很有意思的观点：当开发范式从"人类手写代码"转向"AI 生成 + 自动 Review +
人类架构审批"，编程语言的选型标准也随之改变——除了性能和生态，一个新的核心指标正在决定成败： **AI
到底能不能高效、准确地写好一门语言**。

这篇文章的结论是：在 AI 驱动的开发中，Go
展现出压倒性优势——语法极简、编译极快、标准库强大、幻觉率低，形成了"生成-验证-修复"的完美闭环。而 Rust 受困于借用检查的反馈时延，Zig
则因语料匮乏而难以被 AI 拟合。

这个论点在微服务、API、CLI 场景下是成立的。但如果我们聚焦到 **商业软件**这个具体领域——以零售 ERP
后端为代表、以数据库为核心的业务流程与业务逻辑——结论会被大幅改写。这篇文章试图从六个维度回答：在这个领域，什么才是真正重要的选型标准？

需要先声明我的立场： **我是 Scala 爱好者，函数式编程的实践者**。这篇文章不是中立的产品对比文档，而是一个在商业软件领域摸爬滚打多年、把
Scala 3 用于核心产品重构的人的选型思考。我会尽量给出证据，但偏好是明牌的。

# 商业软件的特征

以零售 ERP 为例：进销存、POS、财务核算、促销、多门店、供应链。这类系统的技术特征是什么？

## 1. 瓶颈在数据库，不在 CPU

高并发写（订单、扣库存、流水）、复杂查询（报表、对账、毛利分析）。Rust/Zig 那点性能优势在这里几乎用不上——网络和 SQL 才是大头，GC
的开销无关痛痒。 **性能不是这个领域的选型主导因素。**

## 2. 业务规则密集

代码量里 80% 是 CRUD + 规则编排：促销叠加、定价、税务、库存分配、门店调拨、退换货。业务规则的正确性、可维护性，远比执行效率重要。

## 3. 长期演进

ERP 是 10~20 年的系统（SAP 跑了 50 年）。可维护性、人才供给、生态稳定性，权重远高于语言的新鲜度。

## 4. 多租户

SaaS 化的商业软件几乎必然走向多租户。租户隔离是生死题——漏写一个隔离条件就是跨租户数据泄露。

结论先行： **这个领域的选型主导因素是"类型系统表达能力 + 业务逻辑表达密度 + 编译期能拦住多少业务错误 + 长期可维护性"
，不是性能，也不是 AI 反馈时延。** 下面从六个维度展开。

# 1. 类型系统

很多语言之争，本质上是类型系统之争。而类型系统有两个正交的维度，常被混为一谈：

- **静态 vs 动态**：类型检查发生在编译期，还是运行时；
- **强 vs 弱**：类型约束的严格程度——隐式转换是否泛滥，类型错误能否被容忍。

**"强 vs 弱"是对"静态 vs 动态"的一次升级**：静态/动态只决定检查的 *时机*，强/弱决定约束的 *质量*。静态语言可能弱（Java
的隐式转换泛滥），动态语言可能强（Python 的严格报错）——两者并不绑定。

## 语言的强弱对比

| 语言       | 静态/动态 | 强/弱          | 弱在哪里（证据）                                  |
|------------|-----------|----------------|---------------------------------------------------|
| Rust       | 静态      | 强（顶格）     | 所有权 + 借用，零隐式转换，整数溢出都不静默       |
| Haskell    | 静态      | 强             | 纯函数 + 类型类                                   |
| Scala 3    | 静态      | 强             | opaque type、ADT，转换必须显式（given/implicit）  |
| Kotlin     | 静态      | 强             | 少数妥协：平台类型、`!!` 非空断言                 |
| C#         | 静态      | 强             | 仅受限的隐式数值转换                              |
| Go         | 静态      | 强（有逃逸口） | 类型系统本身严格，但 `any`/`interface{}` 随处逃逸 |
| Java       | 静态      | **半强**       | 隐式 widening、自动装箱、null 黑洞、泛型擦除      |
| C / C++    | 静态      | 弱             | 指针隐式转换、整型提升，无类型安全                |
| Python     | 动态      | 强             | `"1" + 1` 直接报错                                |
| Ruby       | 动态      | 较强           | duck typing，约束弱于 Python                      |
| JavaScript | 动态      | 弱             | `"1" + 1 == "11"`，`==` 宽松相等                  |

两个反直觉的结论：

1. **Java 是静态语言，却只是"半强"**。隐式 widening（Int 自动转 Long）、自动装箱、null
   黑洞、泛型擦除，让大量类型错误绕过了编译器，延迟到运行时才暴露。商业软件里大量"看起来编译过了、运行时 ClassCastException"的
   bug，根源就在这里。
2. **Python 是动态语言，类型约束却相当严格**。`"1" + 1` 直接报错，而 JavaScript 返回 `"11"`——同样是动态语言，强弱差异巨大。

## 强类型能力全景

更进一步，把"类型能力"的视野放宽—— **凡是类型系统能在编译期强制的约束，都是类型能力**。除了强弱之外，还包括：

- **ADT（代数数据类型）**：sum type + product type + 穷尽匹配，非法状态不可表示（后文详述）；
- **字面量类型（literal type）**：把业务取值空间写进类型（`"CNY" | "USD"`）（后文详述）；
- **opaque / newtype**：给同一底层类型建立语义不同的新类型（Money ≠ BigDecimal），编译期禁止混用；
- **null 安全**：用 Option/Maybe 显式建模可空性，消灭 null 黑洞（后文详述）；
- **生命周期（lifetime）**：引用有效性由类型系统证明，杜绝悬垂引用（Rust 独有）；
- **只读 / 不可变**：类型层面强制 `val`/`readonly`，让不可变性成为编译期约束而非编码约定；
- **capability（能力）**：把权限、副作用、错误通道变成类型化的"能力"传递，调用方拿不到能力就做不了对应的事（Scala 3 的
  can-cc/CanThrow 目前仍是 *实验特性*，capability 传递更多是社区实践；Haskell 的 IO 类型化副作用是成熟形态）；
- **并发安全**：编译期排除数据竞争（Rust 的 Send/Sync 是类型化保证；Haskell 靠纯函数 + STM；JVM
  系靠锁与并发工具，靠约定而非类型，数据竞争要等运行时工具才暴露）；
- **类型化错误**：错误路径进入类型签名（Result/Either/checked exception），强迫调用方处理；
- **类型级计算**：类型本身参与计算（match types、GADT、const generics）；
- **宏 / 编译期元编程**：把约束从类型进一步扩展到业务规则（后文详述）。

| 能力             | Scala 3 | Rust | Haskell | Kotlin | C# | Java | Go | TypeScript |
|------------------|---------|------|---------|--------|----|------|----|------------|
| ADT + 穷尽匹配   | ✅      | ✅   | ✅      | ✅     | ⚠️ | ⚠️   | ❌ | ✅         |
| 字面量类型       | ✅      | ❌   | ❌      | ❌     | ❌ | ❌   | ❌ | ✅         |
| opaque / newtype | ✅      | ✅   | ✅      | ✅     | ⚠️ | ❌   | ❌ | ⚠️         |
| null 安全        | ✅      | ✅   | ✅      | ✅     | ⚠️ | ❌   | ❌ | ✅         |
| 生命周期         | —       | ✅   | —       | —      | —  | —    | —  | —          |
| 并发安全         | ⚠️      | ✅   | ✅      | ⚠️     | ⚠️ | ⚠️   | ⚠️ | ⚠️         |
| 只读 / 不可变    | ✅      | ✅   | ✅      | ✅     | ⚠️ | ⚠️   | ❌ | ✅         |
| capability       | ✅\*    | ⚠️   | ✅      | ❌     | ❌ | ❌   | ❌ | ❌         |
| 类型化错误       | ⚠️      | ✅   | ✅      | ❌     | ⚠️ | ⚠️   | ⚠️ | ⚠️         |
| 类型级计算       | ✅      | ⚠️   | ✅      | ❌     | ❌ | ❌   | ❌ | ⚠️         |
| 宏 / 元编程      | ✅      | ✅   | ⚠️      | ❌     | ⚠️ | ❌   | ❌ | ❌         |

图例：✅ 原生支持；⚠️ 部分支持或需妥协；❌ 无；— 无此概念（GC 语言不需要生命周期，Rust 独有）。`\*` 表示该能力目前为实验特性（Scala
3 的 can-cc/CanThrow 尚未稳定）。并发安全一行的 ⚠️ 语义不同：JVM 系靠锁与并发工具、Go 靠 goroutine +
channel，但都不是编译期类型保证；TypeScript 单线程 + worker 消息传递，无共享内存数据竞争问题。

几个值得注意的观察：

1. **Rust 的"强"是独占路径**——生命周期与并发安全两行，Rust 都是类型级保证（Haskell 的并发安全来自纯函数 +
   STM），代价是其他行（宏之外的开发体验、类型化错误之外的学习曲线）与商业软件需求错位；且并发安全的类型化保证在商业软件里的权重低于系统软件——ERP
   的并发仲裁在数据库层（行锁/事务），进程内数据竞争相对次要；
2. **Go 是弱化版的 Java**——在 ADT、null 安全、opaque、只读上全面 ❌，其"静态 + 强"的定位主要靠"无隐式转换"撑着，业务建模能力是空白的；
3. **Scala 3 是唯一全表拉满的**——除生命周期（GC 语言不需要）外无一处 ❌（并发安全是 ⚠️ 而非 ❌），且宏/元编程 ✅
   意味着它的能力矩阵还能继续扩展；
4. **动态语言（Python/Ruby/JS）在绝大多数能力上为 ❌**，仅 TypeScript 例外（ADT、字面量、null 安全、只读四项 ✅）——这也是
   TypeScript 在 AI 时代受欢迎的类型学原因。

**能力越多，编译器替业务兜的底越厚。** 这张表也回答了"为什么选 Scala 3"：不是某单一特性，而是能力矩阵的整体覆盖。

## ADT：业务模型的强表达能力

ADT（代数数据类型）是"强类型"从"防类型错误"升级到"防业务错误"的第一块基石。它的价值不是多一种语法，而是
**把业务语义直接编码为类型**——业务模型本身成为类型系统的一部分，编译器因此能替你检查业务规则。

**1. 业务词汇即类型。** 订单状态、促销规则、税务类别、支付方式——业务领域的概念直接成为 `enum`/`case class`。`OrderStatus`
就是"订单状态"这个词本身，业务代码里不再有散落各处的字符串魔法值。

**2. 业务不变量进入类型：非法状态不可表示。** 这是 ADT 表达能力的核心——业务规则"不允许"的组合，在类型层面根本构造不出来：

```scala
enum OrderStatus: Draft, Submitted, Paid, Shipped, Completed, Cancelled
case class Order(status: OrderStatus, total: Money)  // 不存在 "PAIDD"; 状态取值空间封闭

def ship(o: Order): Order = o.status match
  case Paid => o.copy(status = Shipped)   // 只有 Paid 才能发货
  case _    => throw IllegalStateException // 编译器逼你处理其余每个状态
```

**3. 业务分支变化的穷尽覆盖。**
业务加一种促销类型、一种支付方式——编译器立刻列出所有需要处理的位置，漏一处就是编译错误而非线上事故。在业务高度易变的时代，这是对"变更"最强的保险。

**4. 业务模型的演进追踪。** 改一个字段、加一个状态，编译器把所有影响点指出来——重构的安全网是类型系统本身，而不是文档和记忆。

商业软件里最易错的状态机、促销规则、税务类别，正是 ADT
的主场。我在[《使用 ADT 代数数据类型 进行数据建模》](@/blog/2024-09-01-ADT/index.md)
中有更完整的讨论——包括数据与行为分离（数据稳定、行为易变，以静制动）、JSON 与 ADT
的映射，以及"动态语言一时爽，一直动态一直爽，重构代码火葬场"的教训。

## 字面量类型

字面量类型把业务取值空间写进类型：`"CNY" | "USD"`、`1 | 2 | 3`，比 String/Int 精确一个量级——货币代码拼错、状态值写错，编译期就拦下：

```scala
// 字面量类型: 取值空间进入类型
def fx(c: "CNY" | "USD" | "EUR"): BigDecimal = ...
fx("RMB")   // 编译错误: RMB 不在字面量联合中
```

与 ADT 配合，业务语义（状态、枚举、单位、货币）全部成为编译期事实，运行时不可能出现类型签名之外的取值。

## Null 安全：消灭十亿美元的错误

null 是类型系统的后门。类型签名说"这里是 String"，实际运行时可能是 null—— **类型承诺被静默违背**，编译器无能为力，错误延迟到运行时才炸。Tony
Hoare 在 1965 年发明 null 引用后，称它为"十亿美元的错误"（billion-dollar mistake）——这个数字在商业软件领域只多不少：空指针异常是生产环境最高频的故障类型之一。

```scala
// Scala: Option 显式建模可空性, 编译器强制处理 None 分支
def findUser(id: Id): Option[User] = ...
findUser(id) match
  case Some(u) => u.name
  case None    => "unknown"

// Scala 3 explicit nulls: 引用类型默认非空, null 必须显式联合
def findUser2(id: Id): User | Null = ...
findUser2(id) match
  case u: User => u.name
  case null    => "unknown"
// 或直接剥离: findUser2(id).nn

// Java: 签名说"非空", 实际可能是 null, 编译器不帮你
String name = user.getName();   // 运行时 NPE
```

各语言的解法，强制程度差异明显：

| 语言               | null 安全机制                                  | 强制程度                                                                                                 |
|--------------------|------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| Scala 3            | Option + explicit nulls（`T\|Null`，实验特性） | 编译器强制，None 分支必须处理；开启 explicit nulls 后引用类型默认非空，null 必须显式联合                 |
| Rust               | `&T` 引用永不为 null + Option                  | 语言级保证：引用类型天然非空，可空只能显式 `Option<&T>`；裸指针 `*const T` 可为 null 但解引用必须 unsafe |
| Haskell            | Maybe                                          | 编译器强制，Just/None 分支必须处理                                                                       |
| Kotlin             | 可空类型 `T?`                                  | 编译器强制（与 Java 互操作处有漏洞）                                                                     |
| TypeScript         | strictNullChecks                               | 编译期检查，生态可选                                                                                     |
| C#                 | 可空引用类型（NRT）                            | 警告级别，非强制                                                                                         |
| Java               | Optional（仅是容器）                           | 无——null 到处合法，Optional 当字段/参数反而是反模式                                                      |
| Go                 | 无，零值惯例                                   | 无                                                                                                       |
| Python / Ruby / JS | None / nil / undefined                         | 运行时                                                                                                   |

Scala 3 的 explicit nulls 值得单独说明：开启 `-Yexplicit-nulls` 后，`User` 类型 **默认不含 null**，可空值必须显式写成
`User | Null`，读取时必须剥离（`.nn`）或模式匹配—— **null 从隐式威胁变成显式类型**。它与 Option 互为补充：Option 提供容器抽象，
`T\|Null` 提供精确的可空建模，后者尤其适合数据库映射（数据库 NULL 天然是"值或空"，而非"容器"）。

对比 Scala 的"显式化"路径，Rust 走的是"结构性"路径：`&T` 引用在类型层面 **就不可能为 null**（语言规范保证，编译器还据此做非空优化——
`Option<&T>` 与 `&T` 同尺寸零开销），空值只能通过 `Option` 或需要 `unsafe` 的裸指针表达。一个是"把 null 显式化"，一个是"让
null 在引用上不存在"——两种哲学殊途同归：null 都从"隐式威胁"变成了"显式类型"。

**商业软件里 null 的最大来源是数据库**。SQL 的 NULL
自带三值逻辑（TRUE/FALSE/UNKNOWN），语义模糊（是"未知"还是"不存在"？），列可空性五花八门。落到 ERP 场景，语言层的 null
安全本质就是"数据库 NULL 与领域模型映射"的正确性——这是我做 wsql 时最花心思的地方：

```scala
// wsql 中的实证: T|Null 与 Option 显式建模数据库可空列
case class Order(id: Long, remark: Option[String], paidAt: Option[Timestamp])
// 列值为 NULL → None, 编译器保证访问前必须处理
// 值类型/引用类型分别走 _opt_nn / _opt_null 两条路径, 避免 wasNull 判断泄漏到业务层
```

null 安全的本质，是 **类型系统是否允许 null 绕过类型承诺**
。强类型语言把可空性显式化（Option/T?），半强语言（Java/C#）靠约定和警告凑合，弱语言干脆放弃——这一行差异，决定了"空指针"是编译错误，还是凌晨三点的报警。

## 为什么 AI 时代看重"强类型"

**对 AI 编程，真正重要的是"强"，不是"静态"。** 理由有三：

1. **强类型 = 给 AI 的免费验证器**。AI
   生成的代码类型不匹配、参数传错，强类型语言在编译期就拦下，形成"生成-验证-修复"闭环——21CTO
   文章所说的反馈闭环，其质量本质上取决于"编译能拦住多少错"，而不只是"编译有多快"。
2. **弱类型 = AI 幻觉放大器**。JavaScript 的隐式转换、Java 的自动装箱，让"编译通过了但语义错了"成为常态，AI
   无法预测这些隐式行为，生成结果更加不可控。
3. **类型标注是 AI 的监督信号**。语料中的类型信息越丰富，AI 对结构的把握越准。

一个极端的证据来自 Rust。Rust 社区有句流传的话： **"If it compiles, it works"（一旦编译通过，基本就能正确运行）**
——这不是营销话术，而是机制使然：所有权/借用/生命周期把内存错误推到了编译期，Send/Sync
把数据竞争推到了编译期，强类型把类型错误推到了编译期，剩下的大多是纯逻辑错误。对 AI 编程这意味着：AI 生成的 Rust
代码，"编译通过"这个验收标准本身就携带了极高的正确性置信度——生成-验证-修复的闭环终点是可信的。对比弱类型语言，"编译通过"几乎不携带任何语义保证（隐式转换、null、类型擦除让"编译过但语义错"成为常态）。
**编译从"语法检查"升级为"语义检查"——AI 时代最有价值的不是编译快，而是编译能证明什么。**

当然要诚实限定：编译期拦不住的是逻辑错误（业务规则算错、边界条件漏判），而这恰恰是商业软件的主要风险领域——所以对商业软件而言，Rust
的"编译即正确"覆盖的是内存与类型层，业务逻辑层的正确性要靠 ADT 与宏补上。这正是后文的伏笔： **Scala 3 用 ADT 把"
非法状态不可表示"、用宏把"租户条件/表结构校验"推进编译期，是在业务规则层逼近"编译通过 ≈ 正确"**。

这个维度也修正了 21CTO 文章的一个隐含归因： **Go 的 AI 优势不只是"语法极简、编译快"，更是"静态 + 强类型"给 AI 的强约束**
——而在这个维度上，Scala、Kotlin、C#、Rust 并不输 Go。静态 + 强类型，是 AI 时代商业软件语言的第一门槛。

# 2. 表达能力与简洁性

如果说类型系统决定"编译器能替你检查多少"，表达能力与简洁性则决定"业务逻辑写起来像不像需求本身"。我倾向"有很简洁的类型支持和表达式能力"的语言——一个
`price * qty * discount` 能直接写出来的语言，和一个要 `price.multiply(qty).multiply(discount)` 的语言，在业务代码的可读性上是数量级的差距。
**业务逻辑的表达密度，直接决定商业软件的可维护性。**

## 运算符重载是否必要？

先给反对者的立场：运算符重载容易被滥用（C++ 的 `<<` 流运算符被批评为可读性灾难），Java/Go 因此干脆禁止。但商业软件的实情是：
**金额运算是领域核心，`price * qty * discount` 就是业务语言本身**。Java 里同一段逻辑被迫写成方法调用串：

```java
BigDecimal total = price.multiply(qty)
    .multiply(discount)
    .add(price.multiply(qty).multiply(discount).multiply(taxRate))
    .setScale(2, RoundingMode.HALF_UP);
```

对比 C#——decimal 是原生类型，运算符直接可用：

```csharp
decimal total = Math.Round(price * qty * discount * (1m + taxRate), 2, MidpointRounding.AwayFromZero);
```

同一段业务逻辑，一个像数学表达式，一个像方法调用串——表达密度的差距直接转化为可读性和出错率的差距。 **问题的关键不是"
要不要重载"，而是"重载的克制"**：Scala 3 允许任意符号重载，但工程约定把它限制在领域类型上（Money、Quantity），滥用是工程纪律问题，不是语言能力问题：

```scala
// 领域类型 + 运算符重载: 业务表达式一行数学, 顺带解决单位安全
opaque type Money = BigDecimal
object Money:
  def apply(v: BigDecimal): Money = v
  extension (m: Money)
    def *(r: BigDecimal): Money = Money((m * r).setScale(2, RoundingMode.HALF_UP))
    def +(o: Money): Money = Money(m + o)

val total = price * qty * discount * (1 + TAX_RATE)  // 业务表达式一行数学
val x: BigDecimal = total   // 编译错误: Money 不能当 BigDecimal 用
```

注意单位安全是重载的伴生价值：C# 的 `decimal` 原生但没有单位安全（金额 × 金额编译器不拦你，财务大忌）；Scala 的 opaque type 把
Money 建成独立类型，重载的 `*`/`+` 内聚舍入逻辑，业务层写不出金额 × 金额。Kotlin 的 value class 也能做类似的事，但建模能力弱一档；Java
只能退回 BigDecimal 加人肉纪律。

## 表达式能力：表达还是语句

定点数解决"怎么算"，表达式能力决定业务逻辑的 *形状*。表达式（expression）代表一个计算过程并产生值；语句（statement）执行副作用但不返回值——
`1 + 2 * 3` 是表达式，`if (flag) a = 1 else a = 0` 是语句。

Java 中 if-else、for、while、switch、try-catch 全部是语句（唯一的表达式是 `?:` 三目运算符）——业务逻辑被迫写成"步骤序列 +
可变变量赋值"，这是 Java 代码啰嗦的一个隐藏根源。而 Kotlin、Scala 中，if-else、match/switch、for、try-catch **全部是表达式**：

```scala
// Scala: if-else 是表达式, 有返回值
val discount = if (member) 0.9 else 1.0

// Scala: match 是表达式
val fee = order.status match
  case Draft => 0
  case Paid  => order.total * 0.01
  case _     => 0

// Java 的等价物: 语句 + 必须先声明再赋值的可变变量
double discount;
if (member) discount = 0.9; else discount = 1.0;
```

表达式风格的意义：业务逻辑从"命令序列"变成"值转换"——每个分支产生一个值，没有中间可变状态，天然契合后文的 FP
分层与不可变数据。再叠加运算符重载（见上一节）与字符串插值（`sql"..."`，见 DSL 一章），业务代码才能写成"需求本身的样子"。

## 集合操作能力

ERP 的数据变换（订单行聚合、库存过滤、报表统计）本质上都是集合操作——map/filter/fold/groupBy
的组合。集合操作能力决定"数据流水线"写起来多直接：

```scala
// Scala: 组合子直接表达
val totalQty = orderLines.filter(_.valid).map(_.qty).sum
val byStatus = orders.groupBy(_.status).view.mapValues(_.map(_.total).sum).toMap

// Java 8+: Stream 两段式 + Collectors 噪音
int totalQty = orderLines.stream()
    .filter(Line::isValid)
    .mapToInt(Line::getQty)
    .sum();
```

加上不可变集合默认（呼应 #1 的只读能力），集合变换天然无副作用、可组合、可测试。Go
没有泛型集合组合子，这类逻辑退化为手写循环——表达力差距在报表、对账这种数据密集代码里是数量级的。

## 模式匹配

模式匹配是"表达能力之王"：它是表达式（产生值）+ 解构（拆开数据结构）+ 穷尽（编译器检查所有分支）三者的合一：

```scala
val amount = payment match
  case Cash(paid)          => paid
  case Card(_, fee, paid)  => paid - fee   // 解构
  case Wechat(discount, _) => ...
```

配合 #1 的 ADT，新增一种支付方式时编译器立刻指出所有遗漏的 match——业务分支的变化被模式匹配变成编译期事件。对比：Java 21
才勉强有 instanceof 模式匹配（雏形），Go 完全没有。

## 扩展方法：能力而不动类型

给既有类型加领域能力，而不修改类型定义本身——Scala 3 的 extension、Kotlin 的扩展函数、C# 的扩展方法。上面的 Money 运算符、
`date + 1.month` 这类业务化操作，都挂在既有类型上（BigDecimal、LocalDate），领域代码与基础类型解耦：

```scala
// Scala 3: 给 BigDecimal 挂业务运算符, 不动 BigDecimal 本身
extension (bd: BigDecimal)
  def *(rate: BigDecimal): Money = Money(bd * rate)
```

Java 无此能力——只能包装类（Wrapper）或工具类（XxxUtils），前者侵入类型，后者散落逻辑。

## 类型表达 × 表达式对比

| 语言    | 类型表达能力 | 表达式能力 | 定点数体验                   |
|---------|--------------|------------|------------------------------|
| Java    | ★★         | ★         | BigDecimal 无重载            |
| C#      | ★★★★     | ★★★★★ | decimal 原生                 |
| Kotlin  | ★★★★★   | ★★★★   | value class 自建             |
| Scala 3 | ★★★★★   | ★★★★★ | opaque type 自建，可做到最强 |
| Go      | ★★★       | ★★       | 比 Java 还啰嗦               |
| Rust    | ★★★★★   | ★★★★   | rust_decimal                 |

## 动态语言：表达力的另一面

业务高度易变的商业环境里，动态语言靠谱吗？我的观点是： **"不靠谱"的不是动态语言，是"没有纪律的动态语言"
——缺测试、缺类型标注、缺模块边界。**

反例是硬证据：Shopify（Ruby on Rails）承载了全球最大的零售电商平台之一，多商户、促销、定价规则之复杂不亚于任何 ERP，活了近 20
年；Odoo（Python）是完整的开源 ERP。它们的业务规则变更频率比大多数 Java 系统都高——动态语言的表达力是真实的。

但正例也有：SAP、Oracle EBS、用友、金蝶全部是静态语言（ABAP/Java/.NET）。长期演进 + 大团队 +
高频重构时，编译期保障确实值钱——编译器是免费的"变更影响面扫描器"，动态语言把正确性责任全部转移给测试，而业务系统的测试覆盖率通常赶不上变更速度。

## Go / Rust：表达力与定位

回到 21CTO 那篇文章。Go 的优势（极简语法、秒级编译、AI 友好）是真实的，但它恰好解决的是微服务场景的问题。放在商业软件里：

- Go 没有运算符重载，decimal 全靠方法调用（shopspring/decimal），比 Java 还啰嗦；
- Go 的 CRUD + 规则代码异常啰嗦（`if err != nil` 洪流）；
- Rust 的借用检查在这个领域没有用武之地——ERP 几乎没有内存安全问题，却天天被编译等待拖累。

所以这两门语言在这个领域的定位是： **Go 适合高吞吐周边服务（订单、库存扣减、价格计算），Rust
适合毫秒级定价引擎这类热路径，但不是核心业务逻辑的答案。**

## 对 Java 的观感

可能是我有偏见：使用 Java，太容易受不良习惯的左右，很难编写出好的代码；而应用
Scala，我们通过施加一些简单的限制原则，就可以强迫开发人员改变习惯，编写成可读性、可维护性显著改进的代码。这句话我在[Scala3 浅尝](@/blog/2022-07-10-Trying%20Scala3/index.md)
中写过——那是用 Scala 3 重构核心产品后的总结，两年过去，这个判断依然成立。

## AI 编程与表达能力的关系

表达力与 AI 编程是双刃剑，两个方向都真实：

**方向一：高表达力 = 低生成成本。** 同一业务逻辑，表达密度高意味着代码量少——AI 生成的 token 少、出错空间小、Review 成本低。
`price * qty * discount` 一行 vs `price.multiply(qty).multiply(discount)` 三行，AI 生成前者的正确率天然更高：更少的代码 =
更少的犯错机会。

**方向二：高表达力 = 写法多样 = AI 拟合更难。** 表达力强的语言，同一逻辑的合法写法更多（运算符重载、扩展方法、given/implicit、集合组合子的多种组合），AI
缺乏强统计先验；而 Java/Go 写法单一、语料统计性强，AI"照着写"几乎不会错。这正是 21CTO 文章里 Go 胜出的深层原因—— **极简 =
少样 = 高可预测性**。

**关键：张力被编译期反馈补偿。** 表达力带来的拟合难度，被强类型 + 宏的编译期检查对冲——AI 生成的代码错了，编译器立刻指出并进入修复闭环（呼应
#1 与 #3）。于是两种语言的 AI 策略分野：

|              | 低表达力（Java/Go） | 高表达力（Scala 3） |
|--------------|---------------------|---------------------|
| AI 生成      | 写法单一，近对率高  | 写法多样，近对率中  |
| 错误发现     | 运行时 / 测试       | 编译期（类型 + 宏） |
| 校验成本     | 高（测试 + 人工）   | 低（编译器兜底）    |
| 人类可维护性 | 低                  | 高                  |

**结论：不要因为"AI 拟合度"选低表达力语言**——那是用人类十年的可维护性换 AI 短期的便利。正确姿势是高表达力 + 编译期校验闭环：AI
负责生成，编译器负责把关，人类负责架构——这正是 Scala 3 的定位。

# 3. DSL 与 Macro

商业软件的 bug 几乎不是"算法错"，而是"规则漏分支、字段拼错、SQL 漏条件、类型错位"—— **这四个恰好全是编译期可拦的**
。而"把业务不变式推进编译期"这件事，正是宏（编译期元编程）的价值。

Scala 3 的宏（quotes/splicing）本质上是一个"编译期 DSL 引擎"：验证型宏（编译期解析 + 报错）、生成型宏（编译期生成代码）、求值型宏（inline
计算）。它们遵守一个关键分工： **编译期做结构分析和注入计划，生成"带注入点的代码"，运行时只填值**。编译期可知的是 SQL
文本结构、表列名、case class 字段、类型；编译期不可知的是租户值、连接、参数值——后者永远只能生成注入点。

我在自己的库 [scala-sql（wsql）](https://github.com/wangzaixiang/scala-sql) 中实践了这些特性。按场景组织：

| DSL 场景                          | 宏机制                                   | 商业软件的价值                                         |
|-----------------------------------|------------------------------------------|--------------------------------------------------------|
| SQL 语法检查                      | `SQL""` 宏编译期解析校验                 | 表名/字段名/租户条件拼错，编译期报错而非线上事故       |
| ResultSetMapper：强类型的轻度 ORM | 宏生成 case class 行映射                 | 零反射、编译期字段类型检查，替代 ORM 的可变实体        |
| Batch 语法改写                    | `createMySqlBatch` 宏重写 `set → values` | 可读语法 + 批处理性能兼得，语法转换编译期完成          |
| BeanBuilder                       | 宏生成对象转换代码                       | 模型层（行模型 → 领域模型 → DTO）转换零样板、零反射    |
| StringInterpolation + 子语言      | `sql"..."` 插值                          | SQL 成为类型安全子语言，参数自动绑定、防注入是结构性的 |

## SQL 即 DSL

`sql""` 字符串插值把 SQL 变成类型安全的一等 DSL——参数自动参数化（防注入是结构性的，不存在 `${}` 拼接的危险通道），参数类型编译期检查（
`JdbcValueAccessor` 隐式约束），宏生成 case class 的行映射（零反射）：

```scala
// wsql 的真实用法: 宏按字段名自动映射 case class, 零反射, 返回 Option
def loadOrder(id: OrderId): Option[Order] =
  dataSource.row[Order](sql"SELECT * FROM orders WHERE id = $id")

// 上面的宏展开等价于手写映射 (Row API 只是兜底路径, 用于自定义类型映射):
//   dataSource.row[Row](sql"...").map { row =>
//     Order(row.get[Long]("id"), row.get[String]("status"), row.get[BigDecimal]("total"))
//   }
// 自定义类型 (OrderStatus/Money) 提供 JdbcValueAccessor 即可接入, 列名/字段名默认按名对应
```

## 编译期 SQL 检查与多租户注入

`SQL""` 宏更进一步：编译期解析 SQL 并校验。这分两层——静态 SQL 在宏层面检查（多租户表必须带租户谓词，缺则编译报错），动态拼接的
SQL 走独立的租户作用域 API 强制注入：

```scala
// 静态: 编译期检查
val a = SQL"select * from orders where tenant_id = $t"   // ✅ 编译过
val b = SQL"select * from orders where status = 'PAID'"  // ❌ 编译失败: 缺租户条件

// 动态: 租户作用域强制, 运行时自动注入
dataSource.withTenant(TenantId("t_42")) { ds =>
  ds.rows[Order](sql"select * from orders")              // 自动注入 and tenant_id = ?
}
ds.rows[Order](sql"select * from orders")   // ❌ 无租户上下文, 拒绝执行
```

## BeanBuilder：编译期对象转换

模型层转换（数据库行模型 → 领域模型 → API DTO）是 ERP 里最频繁的样板。BeanBuilder 用宏在编译期生成转换代码，零反射、零运行时开销——
**转换规则（字段匹配、类型转换）在编译期确定，错误在编译期暴露**：

```scala
// 基础: 字段同名自动映射, 缺失字段用目标默认值
case class UserRow(name: String, age: Int)
case class User(name: String, age: Int, status: String = "active")
val user = BeanBuilder.build[User](row)

// 类型转换 + Option + 集合: 数据库列 → 领域类型
given Conversion[String, PhoneNumber] = PhoneNumber.parse
val user = BeanBuilder.build[User](row)   // String → PhoneNumber, 集合 Seq → List, 可空列 → Option 自动

// 多源合并: 一个目标从多个源组装
val order = BeanBuilder.build[Order](orderRow, summary)

// additions: 显式覆盖/补字段 (最高优先级)
val user = BeanBuilder.build[User](row)("status" -> "vip")
```

### 与 Java 生态的对比

Java 生态处理同样问题的工具链：

| 维度       | BeanBuilder (Scala 3)                        | MapStruct               | ModelMapper   | Spring BeanUtils |
|------------|----------------------------------------------|-------------------------|---------------|------------------|
| 机制       | 语言内建宏，内联生成代码                     | APT 编译期生成实现类    | 运行时反射    | 运行时反射       |
| 编译期检查 | ✅ 字段名/类型/缺失全查                      | ✅ 字段名/类型          | ❌ 运行时才错 | ❌ 运行时才错    |
| 目标模型   | 不可变 case class（直接构造）                | Java Bean（setter）     | Java Bean     | Java Bean        |
| 转换能力   | 类型转换/Option/集合/嵌套/多源合并/additions | 类型转换/嵌套/集合      | 灵活但隐式    | 仅同名同类型     |
| 性能       | 零反射，内联                                 | 零反射，生成类          | 反射，慢      | 反射，慢         |
| 使用方式   | 直接调用 `build[T](src)`                     | 声明 mapper 接口 + 注解 | 配置式        | 调用式           |

四个关键差异：

1. **模型假设不同**：MapStruct/ModelMapper 面向可变 Java Bean（setter、无参构造），BeanBuilder 面向不可变 case class——后者与
   ADT/immutable 建模（见 #4）一致，目标类型不需要 setter；
2. **编译期检查的深度**：MapStruct 与 BeanBuilder 都能编译期报错，但 MapStruct 的错误定位在生成代码里（APT 阶段），BeanBuilder
   的错误直接指向调用点；ModelMapper/BeanUtils 则是"运行时才错"——类型改了漏改映射 = 线上事故；
3. **零框架依赖**：BeanBuilder 是语言特性（宏），不引入注解处理器、不生成额外类；MapStruct 需要 APT 配置、生成代码要纳入构建；
4. **多源合并与 additions**：BeanBuilder 原生支持（一个目标从多个源组装 + 显式覆盖），MapStruct 需要逐字段 `@Mapping` 或
   `@AfterMapping` 处理。

一句话：MapStruct 是 Java 生态"编译期安全映射"的最优解，而 BeanBuilder 把这个能力以语言内建的方式给了 Scala 3——且因为 case
class 不可变，它服务的模型层（ADT）比 Java Bean 更接近业务语义。这是"宏生成代码"在业务层的落点。

## Batch：执行策略的编译期重写

前面几种 DSL 用法都是"用户写 DSL 文本/类型，宏校验或生成代码"。Batch 展示了另一种方式：
**用户写普通函数，宏在编译期分析函数结构，重写执行策略**：

```scala
// 用户写的是最自然的形式: 一条数据 → 一条 SQL (普通函数, 不是批处理语法)
val batch = conn.createMySqlBatch[User] { u =>
  val name = u.name.toUpperCase()          // 函数体可以自由计算
  sql"insert into users set name = ${name}, age = ${u.age}, email = ${u.email}"
}
users.foreach(batch.addBatch)
batch.close()
```

宏在编译期对这个函数做了什么：

1. **提取 SQL 结构**：分析函数体，拿到表名、列名、参数位——预编译 PreparedStatement，`addBatch` 时只计算参数值，不再重复构建
   SQL（消除"每次 addBatch 重新插值"的运行时开销）；
2. **语法转换**：`createMySqlBatch` 把 `insert into t set a = ?, b = ?`（可读性好，但 MySQL JDBC 不支持批处理）在编译期转成
   `insert into t(a, b) values(?, ?)`（批处理要求的形式）—— **语法转换在编译期完成，运行时零成本**；
3. **执行策略重写**：逐条执行的语义被重写为批量执行（配合 `rewriteBatchedStatements=true` 获得真实性能提升）。

用户全程无感——写的是"一条数据怎么变成一条 SQL"的自然代码，宏把它编译成"高效批量执行 N 条"的执行计划。 **DSL
的三种使用方式至此齐全：文本校验（`SQL""`）、代码生成（BeanBuilder）、执行策略重写（Batch）——共同点是"
把运行时成本与运行时错误前移到编译期"。**

## 宏与 AI：幻觉拦截器

这是 AI 时代被低估的维度：AI 生成的 SQL 是幻觉重灾区——编造不存在的表名、字段名，运行时才炸。而编译期 SQL 检查（`SQL""`
宏）把这类幻觉在编译时拦住。 **宏让"AI 生成 + 编译期校验"成为闭环**，这比"生成 + 人工 Review"可靠一个量级。这也正是"强类型适合
AI"的延伸：强类型语言用类型约束拦住幻觉，而宏进一步把约束从类型扩展到业务规则（租户条件、表结构）—— **约束越多，AI
的发挥空间被框得越准，幻觉越少**。

# 4. OO 与 FP

## 图灵机 vs λ演算

计算有两个模型。命令式编程的根隐喻是"图灵机模型"：程序 = 控制流 +
共享内存。控制流的复杂性和数据流的复杂性相互纠缠——某个数据的当前状态依赖于之前的执行流程，复杂的控制流让数据流难以证明正确，一旦出错就蔓延放大。

另一个是 λ演算（Alonzo Church，1930 年建立，比图灵机理论早 6 年——Church 正是 Turing 的老师）。它的核心只有三行：

```
E = x        -- 变量
  | λx.E     -- 函数抽象
  | (E1 E2)  -- 函数调用
```

图灵机与 λ演算已被证明计算能力等价，但编程体验截然不同：图灵机模型是"共享内存 + 跳转"，λ模型是"函数组合 + 数据流"。函数式编程的核心是
**简单数据流**——请求参数流经一个个纯函数，不断产生新数据，直到达成结果。简单数据流有两种形态：pipeline（管道：每个 step
是一个函数，首尾相接，如 `cat log | awk | sort | uniq -c`）与 DAG（单向无环：有分支有汇聚，但绝不形成回路）——
**数据一旦形成回路，复杂性就回来了**。数据流单向无环，逻辑可以自我证明，这是 FP
相对命令式最本质的优势。我在[《应用函数式编程》](@/blog/2021-07-10-applied%20function%20progamming/index.md)中有完整的讨论。

## 以静制动：数据与行为分离

商业系统中，数据结构（实体、数据库结构）是相对稳定的，行为则是高度不稳定的——随业务、时间、空间不断变化。OOP
把数据与行为强行绑定，一个订单类会为了发邮件、发短信、打印而不断膨胀；而 ADT 让数据保持封闭、稳定，行为通过函数独立演化——数据以静制动，行为动而不乱。OOP
的另一重问题是继承：现实中很少有真正的 is-a 关系，更多的是 has-a——大部份继承都是错误的设计，所以 Rust/Go
干脆不再提供继承，改用组合。这也是我在 ADT 建模实践中的核心体会。

## 分层：规则层 FP，基础设施层命令式

不是风格之争，是分层之争。商业软件的逻辑分三层：

| 层                                       | 本质   | 最佳风格                         |
|------------------------------------------|--------|----------------------------------|
| 领域规则层（定价、促销、税务、库存分配） | 纯计算 | FP：纯函数 + ADT + 穷尽匹配      |
| 流程层（订单流转、审批、对账）           | 状态机 | ADT 状态机，非法迁移编译期不可写 |
| 基础设施层（CRUD、事务、集成）           | 副作用 | 命令式，承认副作用               |

最脏、最贵、最易错的部分（业务规则）用 FP 保护；最机械的部分（CRUD/事务）用命令式，别让抽象妨碍干活。

这里有一个要诚实面对的悖论： **"以数据库为核心"意味着状态源是共享可变的外部世界**。库存数在数据库里，几十个并发事务在改它——immutable
只保护进程内视图，数据库层的仲裁者是行锁和事务隔离级别。FP 的纯度在这个系统里做不到，也不该做。FP 的价值在于
**把最复杂的纯计算部分与副作用隔离**，而不是消灭副作用。

## 计算-解释模型：隔离副作用的落地方法

"把纯计算与副作用隔离"怎么落地？我在 FP 实践中用的是 **计算-解释模型**：每个服务拆成三段——fetch（只读数据）、logic（纯函数计算，产出
ActionModel）、interpret（执行副作用，把计算结果落地）：

```scala
def service(request: Request): Response =
  val initial = fetch(request)        // 只读: 取数据
  val action  = logic(initial)        // 纯函数: 计算要做什么
  interpret(action)                   // 副作用: 统一落地
```

三条实践约定（完整讨论见[《应用函数式编程》](@/blog/2021-07-10-applied%20function%20progamming/index.md)）：

1. **副作用聚合**：逻辑计算保持纯函数，副作用集中到 interpret，取代散落在业务逻辑各处的 IO——服务的目的性明确，业务逻辑可复用、可单测；
2. **命名约定**：有副作用的方法用 `doXXX` 命名，混合方法（串联计算与解释）作为服务最外层并标注 `@effect`，代码中的副作用块用
   `DoEffects!` 标记——让"哪里在改世界"一眼可见；
3. **解释的简单性**：避免在 interpret 里依赖 insert 返回的自增 ID（改为先取全局 ID 再显式赋值）、避免依赖 update 命中数（在
   fetch 阶段先检查存在性）—— **解释阶段只落地，不决策**。

这个模型与 #5 的 Direct Style 天然契合：`withTransaction` 块就是 interpret
的载体，逻辑保持纯函数，副作用在事务边界内落地——代码层的副作用从"四处散落"变成"一处聚合"。

# 5. Direct Style 与 Effect Monad

## Effect System 的税

ZIO / Cats Effect 这类 effect system，我明确不采用。CRUD + 事务密集型的商业应用，effect 的收益（类型化错误、可组合性）接近零，成本却真实存在：每个
DB 调用包进 IO、组合子切碎业务逻辑、调用栈失真、团队要先学 monad。

```scala
// Effect System (ZIO): 一个促销应用要穿过 service、flatMap、类型错误通道
def applyPromo(p: Promotion, id: OrderId): ZIO[OrderRepo & Pricing, PricingError, Money] =
  for
    order <- ZIO.service[OrderRepo].flatMap(_.load(id))
    base  <- ZIO.service[Pricing].flatMap(_.basePrice(order))
    _     <- ZIO.logInfo(s"applying $p")
  yield p.discount(base)

// Direct Style: 就是业务需求本身的样子 (wsql 真实 API)
def applyPromo(p: Promotion, id: OrderId): Money =
  dataSource.withTransaction { conn =>
    val order = conn.row[Order](sql"SELECT * FROM orders WHERE id = $id").get
    val base  = pricing.basePrice(order)
    log.info(s"applying $p")
    p.discount(base)
  }
```

## 不必因为 FP 而迷恋 Monad

FP 的核心价值是纯函数、不可变、简单数据流、副作用隔离—— **这些都不需要 monad**。Monad 只是 FP
生态中处理特定问题（副作用顺序、可失败性、状态传递）的一种工具，不是 FP 本身。

很多人引入 ZIO/Cats Effect 的心理动机是"FP 就应该用 monad"——但这是把"工具"当成了"信仰"。实际业务里：

- **纯函数、不可变、表达式风格（见 #2）、简单数据流（见 #4）——这些 FP 核心收益，Direct Style 全都能拿到**，一条不少；
- monad 解决的"副作用顺序与组合"问题，在 CRUD + 事务密集的商业软件里，用 `withTransaction` 块 + 顺序代码表达得更直接（见下节）；
- monad 的税（学习曲线、组合子、调用栈失真、调试困难）是真实的，而它的收益（类型化错误、可组合性）在大多数业务代码里用不到。

我的判断：FP 是一种思维方式和建模原则，monad 只是其中一种实现手段。 **在商业软件里，FP 的精神（数据流清晰、副作用隔离）用
Direct Style 表达，比用 monad 表达更贴合业务**——这也是我在 wsql
中的实践选择。除非你的领域真的是"可组合的计算管道"（解析器、流处理、协议栈），否则 monad 的收益撑不起它的成本。

## 事务是块

Direct Style 里，事务就是块——`withTransaction { ... }` 一个块就是事务，异常自动回滚。一个反直觉的事实是：Direct Style 正是
Rails 和 Odoo 的默认风格（`transaction do ... end`），FP 爱好者的 Direct Style 与动态语言 ERP 的实践殊途同归：
**都让业务代码看起来像普通代码**，区别只是我们用类型系统把动态语言丢掉的那部分正确性保障补了回来。

```scala
def createOrder(userId: UserId, items: List[CartItem]): Order =
  dataSource.withTransaction { conn =>
    val price = pricing.compute(items)   // 纯函数: 促销 + 税务 + 舍入
    val order = Order.create(userId, price) // ADT
    conn.executeUpdate(
      sql"insert into orders(id, customer_id, total) values(${order.id}, $userId, ${order.total})")
    stock.allocate(conn, order)          // 同一事务
    order
  }
```

## Virtual Threads：Direct Style 的并发答案

Direct Style 过去被嫌弃"阻塞线程"，Java 21+ 的 Virtual Threads 让这个理由消失了：阻塞变廉价，一请求一线程一事务的经典模型重新拥有高并发。JDK
24（JEP 491）修复了 synchronized pinning 后，这个问题彻底干净。

于是 Scala 3 + Virtual Threads 成为 JVM 上这个领域逻辑最自洽的组合：类型系统给正确性，Direct Style 给可读性，Virtual
Threads 给并发，JDBC 生态现成可用。

# 6. ORM or ORM-less

## ORM 与 ADT 的冲突

FP + ADT + immutable 与 ORM 必然冲突：ORM 的实体是可变、可空、带隐式 dirty tracking 和懒加载代理的，与不可变 ADT 直接矛盾。而且
ERP 的现实是：读路径全是复杂聚合（报表、对账），写路径全是事务——恰好是 ORM 最弱的两个区域。

| ORM 的机制           | 与 FP/ADT 的冲突                          |
|----------------------|-------------------------------------------|
| 实体 mutable、setter | 与 immutable ADT 直接矛盾                 |
| Dirty tracking       | 副作用隐藏，行为不可预测                  |
| 会话/Session         | 隐式全局状态，测试要 mock 一片            |
| 懒加载代理           | 运行时才炸（LazyInitializationException） |
| 复杂查询             | 终究掉头写原生 SQL——ORM 白背了对象图      |

## SQL 当一等 DSL：wsql 实证

替代品不是没有，是更好： **SQL 当一等 DSL**，类型类 + 宏做映射，显式行映射为 ADT（见 #3 的 `loadOrder` 示例）。映射由宏生成、零反射；可空列用
`T|Null`/`Option` 显式建模（见 #1 的 null 安全）；事务是块（见 #5）——数据访问层与领域模型的隔离由类型系统保证，而不是靠框架魔法。

## 多租户数据访问：DB + schema + 共享表

SaaS 化的商业软件，多租户隔离是架构核心。我的设计主张是混合模式： **DB + schema + 共享表**。

- **instance** 是物理资源单位：大客户独占，小微客户共享；
- **schema** 是容量单元和迁移单元：一组租户放一个 schema，扩容/迁移以 schema 为粒度整体搬迁（逻辑复制 + 映射切换，零停机）；
- **tenant_id 是安全隔离单元**：同一 schema 内多租户数据共存，靠 tenant_id 列区分——这是唯一的隔离边界。

关键收益： **schema 数量与租户数量解耦**。schema 按容量开（比如每 100
租户一个），租户从几百到几万，表总量可控，autovacuum、统计信息、备份等维护成本不会随租户数线性膨胀。

但代价是： **隔离责任全部压在 tenant_id 上**。漏写 tenant_id 条件 = 同一 schema 内跨租户泄露。这不能靠人肉自觉，必须分层强制：

1. **连接路由层**：tenant_id → (instance, schema)，连接时设置 search_path（资源定位，错即显式失败）；
2. **数据库兜底层**：PostgreSQL RLS，事务开始时 `SET LOCAL app.current_tenant = ?`，policy 强制行级过滤——应用层漏了也挡得住；
3. **编译期防线**：静态 SQL 在宏层面检查多租户表必须带租户谓词（见 #3）。

一个关键的安全语义：`SET LOCAL` 只作用于当前事务，事务结束自动清空—— **事务边界就是租户边界**
，连接池复用零风险。而"未设置租户"必须是安全失败（读空/写拒），而不是全量访问。MySQL 没有原生的 RLS，共享表模式只能靠应用层强制——这是
MySQL 在 SaaS 多租户场景的一个真实短板。

# AI 时代的重新评估

回到 21CTO 那篇文章的框架，重新评估这个领域的语言：

**1. 训练语料**。文章说"AI 准确率依赖训练语料"——这条标准指向的是 Java/C#，不是 Go。GitHub 上 Java/C# 的业务代码语料是 Go
的几十倍，AI 生成 Spring/JPA 代码的准确率远高于生成 Go 的 CRUD + 规则代码。Scala 的语料虽然少，但 Direct Style + SQL 插值的
API 形状贴近 MyBatis/JDBC 主流写法，AI 生成业务层代码时可以参照主流语料。

**2. 反馈闭环**。Go 的秒级编译确实无敌，但 Scala 3 的编译速度（虽慢于 Go/Java）在可接受范围，且宏集中在少量基建代码上，业务层的反馈时延影响有限。

**3. 表达力与 AI 拟合的张力**。一个诚实的判断：表达力越强的语言（运算符重载、自定义 DSL），AI 越难拟合——同样逻辑的合法写法太多样。Scala
3 在这条轴上是"人类生产力顶配、AI 拟合度中等"。这是必须接受的权衡： **你用表达力换业务可读性，代价是 AI 生成时多花一点
Review 成本。**

（编译期检查作为 AI 幻觉拦截器，已在 #3 详述。）

# 结论：决策框架

回到最初的问题：AI 时代的商业软件，选什么语言？

我的回答是： **语言之争在这个领域有个被严重低估的维度——编译期能替你拦多少业务错误。** 在这个维度上，Scala 3
是唯一拉满的（宏 + 类型系统 + ADT + Direct Style + Virtual Threads），C# 凭借 decimal 原生和成熟生态拿到约 90% 的收益，Kotlin
是 JVM 存量团队的务实替代。

```
选择 Scala 3 + Direct Style + Virtual Threads 的决策框架:
✓ 系统生命周期 10 年以上, 正确性 > 开发速度 (商业软件天然如此)
✓ 团队有 1-2 个能写宏的核心维护者 (宏是基建, 业务层不需要会宏)
✓ 业务规则复杂密集 (促销/税务/库存——商业软件天然如此)
✓ 接受"基建自己造" (自研数据访问层, 而非依赖生态)

否则 → C#/Kotlin 拿 90% 收益, 同样是合理的选择
```

剩下的问题不是"Scala 3 行不行"，而是"你的团队愿不愿意为 10 年的正确性付招聘和生态的税"。而 AI 时代的到来，正在降低这笔税：AI
可以补齐业务层的大量编码工作，让团队的核心竞争力从"写代码的人数"转向"架构定义与编译期不变式设计的能力"——这恰恰是宏、ADT、类型系统这些工具的主场。

最后，回到我那句话：我可能是有偏见的。但二十年的商业软件生涯告诉我，
**这个领域的胜负手从来不是语言的性能，而是语言能帮你把多少错误挡在编译期之外**——而这，正是 Scala 3 的用武之地。
