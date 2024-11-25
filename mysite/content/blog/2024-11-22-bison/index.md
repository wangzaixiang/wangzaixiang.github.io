+++
title = "bison(yacc) 源代码分析"
description = "bison(yacc) 源代码分析"
date = 2024-11-22
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# 背景
"每个程序员都应该写一个 parser"。在大学时，编译原理是我学得最糟糕的一门课程，尤其是 LR 文法构造的那一章，当时是完全没有理解，毕业后有机会
了解了 yacc，动手编写了一个小的 parser， 是一个 TUI 下的 UI界面描述语言。（当时，我们主要使用 curses library 来构建 TUI 程序，这个过程
是一个相当枯燥无趣的编码过程），相比使用 curses 库进行编码，使用我自创的 UI 描述语言，原本需要1周的的开发工作的任务，大部份我都可以在1天内完成，
这在当时是很有成就感的。后面再结合 Yacc 学习编译原理，并自己动手完成了 LR 文法的规则表构建过程之后，对 LR 文法的理解有了很大的提升，也对 Yacc
/Bison这类的工具有更好的驾驭度，所以后面陆续使用 Yacc 编写了几个小的 语言parser。

在最近公司的一个数据分析项目中，我们引入了一个 SQL Parser 的构建工作。因为我们在做数据分析的过程中，主要的工作就是在与 SQL 打交道：包括将
我们的前端查询语言，转译成SQL执行，这里涉及到很多的挑战：
- 查询语义的表述，尤其是计算指标的表述能力。参考我之前的系列文章：[DAX](@/blog/2024-09-04-DAX/index.md)、[Tabulau](@/blog/2024-09-04-tabular/index.md)、
  [MDX](@/blog/2024-08-24-MDX-optimize.md)。将这些表达能力转译成 SQL 执行是数据分析引擎中最有挑战和价值的工作之一。
- 数据库的兼容性。不同的数据库在 SQL 支持能力、支持语法、或者实现细节上有很大的差异。如果你的产品需要兼容数十个数据库产品时，构建 SQL 的
  工程挑战性变得巨大。

传统的基于字符串处理的SQL构建、加工过程，在稍微复杂一些的场景中，会变得力不从心，因此，有必要引入 SQL Parser，将对 SQL 语句的操作转变为对
AST 得操作，这会带来：
- 精确性。使用正则表达式或者其他简单的方式对 SQL 代码进行加工，都会失去精确性。基本上只胜任最简单的场景。
- 语义化优化。对 AST 进一步的 Type Resolver 后，我们可以对 SQL 中的符号关系进行更为精确的描述，为 SQL 的优化提供丰富的支撑（我们产品中
  内置了类似于 apache calcite 的 Relation Algebra Optimize 的自研模块）
- 数据库方言适配。类似于关系代数优化，不过，这里更多的是对一些数据库系统的兼容性适配，
  - 语法差异：比如有的数据库(SQL Server) 不支持 limit 子句， 有的数据库 in 子句有元素数量限制（Oracle）、有的数据库不支持 with CTE 子句，有的数据库不支持 order by/group expression 或者
  order by/group by field-no等。
  - 词法差异。qualified identity 不同，字符串常量中的转义规则不同等。
  - 函数差异。这方面的差异是相当大的，每款数据库都有自己的函数定义，即使对完全相同的功能，大家都有不同的函数签名。

处理这些差异，AST 都会带来明显的价值。

我们刚开始的时候，使用 antlr 构建了一个 Parser，不过后面发现性能不佳，而且由于我对 LR 文法（以及Yacc、Bison等工具）更为熟悉，
而对 LL文法 及 antlr 并不够熟悉，于是指导工程师基于Bison 重新构建了一个 SQL Parser。当然，效果也是很不错的：
- 团队人员对 SQL 语法有了更深的了解。尤其是结合 数据库的查询计划，了解 SQL 的执行过程、优化过程、物理计划。
- 对 windows 函数等高级概念有了更为深入的理解，并应用到复杂指标计算的过程中，提高了计算指标的支持能力和速度。
- 相比 antlr LL 文法的解析器， LR 版本的性能提升显著，可以在 ms 量级处理 1M 的 SQL 语句。（LR自身性能优秀，我们的文法完全无需回溯，
  我们也能驾驭。而 antlr 我们 驾驭能力较差，或者并非 antlr 就不能达到这么快的速度）

在这个过程中，我协助工程师解决了多个问题，本文记录在这个过程中，我对 Bison 的源代码的分析，以及 Bison 的使用技巧，以供后续参考。

# LR 规则表相关数据结构
在 LR 文法中，核心的数据结构包括：
1. Shift 表：(state, terminal) -> state
2. goto 表: (state, non_terminal) -> state
3. default reduce 表: state -> ruleNo

一般来说，1，2 数据结构均可以表述为二维矩阵，对我们的 SQL Parser 来说，这个矩阵大小为 337 * 155 = 52,235 个元素，如果使用 short 类型，
则大概占用 100K 的内存。而实际上，二维矩阵中大部份的元素都是无效的值，因此是一个非常稀疏的矩阵，通过采用压缩技术，则可大幅度减少内存的占用。
（例如，在bison版本中，实际占用内存为 337(yypact) + 61(yypgoto) + 61(yydefgoto) + 702(yytable) + 702(yycheck) = 1863*2 = 3.7K )
大幅压缩后，可以保证这些数据结构可以放入 CPU 的 L1 Cache 中，从而提高解析的性能。

Bison 的这个数据结构设计很有意思，之前我也阅读过代码，但对这个压缩的数据结构没有细化的分析，这对调试 Bison 生成的 Parser 代码还是会构成
一些障碍，乘着这次在调试一个 SQL 编译期的过程，花了一些时间来彻底理解它的数据结构设计，感觉有必要进行记录，一来是理解 Bison  的数据结构，方便后续
调试 Parser 代码时，能够更加的得心应手。二来也觉得这个数据结构的设计，非常精巧，值得学习。

在我们测试的案例中，
1. terminal 共94个，编号为 0..93 (不同与 Lexer 中的编码，在 Lexer 中，为 1-255 的字符保留了 Lexer 编码，如果某个编码没有对应的 terminal,
   则会映射为 YYUNDEF)
2. non termial 共 61 个，编号为 94..154。 (由此可见， terminal 和 non terminal 使用了相同的编码空间)
3. states: 共337个状态， 编号为 0..336。
4. rules: 共192 条产生式，编号为 1..192。 (规则编号从1开始)

## yytable, yycheck
yytable 中存储了两部份的信息：
- shift(and go to): 其逻辑为对 (state1, terminal) -> state2 稀疏矩阵的压缩
  - state -> `yycheck[ terminal1 .. terminalN ]`  这个在 yypact 中描述，如果 yycheck( yypact(state) + terminal ) == terminal，则表示有效的 shift
  - state -> `yytable[ terminal1 .. terminalN]`, 如果是有效的 shift,则 yytable( yypact(state) + terminal ) 是shift 后的新状态。
- goto: 其逻辑为对 (non_terminal, state) -> state 稀疏矩阵的压缩。
  - non_terminal -> `yycheck[ state1 .. stateN ]` 这个在 yypgoto 中描述，如果 yycheck( yypgoto(non_terminal) + state ) == state，则表示有效的 goto
  - non_terminal -> `yytable[ state1 .. stateN]`, 如果是有效的 goto,则 yytable( yypgoto(non_terminal) + state ) 是goto 后的新状态。
yytable 实际上是上述两个压缩表的合并。


## `short[] yypact, yytable, yycheck`
这三个表结合起来，构成了 (state, terminal) -> state 的映射关系。

在我们示例的 SQL Parser 中，如果使用完全矩阵，完整描述 (state, terminal) -> state 的映射关系，则需要 337 * 94 = 31678 个元素。而
通过这3个数组来模拟的一个稀疏矩阵，则可以大大减少内存使用。

1. 如果某个状态下，没有 shift 操作， 只有 reduce 操作。 则 `yypact(STATE)` 分配一个最小的负数值,  `yytable(yyn + terminal.code)` 
   对所有的 terminal code 都是无效的 yytable 索引值，不占用 yytable 中的任何空间。
2. 对有 shift 操作的状态，需要在 yytable 中分配 n 个空间： n = state.max_terminal - state.min_terminal + 1 个空间。
   即使这个空间中的某个 terminal 没有对应的 shift 操作，则 yytable(yyn) 设置为-1， yycheck(yyn) 设置为 
3. 如果某两个状态的 shift 完全相同（包括目标状态），可以共享空间。
4. 理论上可以如果 state1 与 state2 对应的 shift 操作如果没有交集，则可以共享空间。这样可以通过算法，进一步对 yytable 压缩。

## `short[] yydefact`
- `yydefact[stateNo]` 的长度与状态数量一致，每个状态对应一个值，表示该状态的 default reduce 操作(从1开始）。
  为 0 则表示该状态没有 reduce 操作（accept reduce 的编号为 1）.  
- `yyr2[ruleNo]` 记录每个 产生式的右侧符号的数量。 为了节省空间，当最大长度 < 127 时，使用 byte 类型。

## `yypgoto, yytable, yycheck, short yydefgoto`
goto table 用于描述 (state, non_terminal) -> state 的映射关系。 
goto_table 由两部份组成：
- yypgoto、yytable、yycheck：通过 yypgoto 建立 non_terminal -> yyn_index 的映射，然后通过 state -> yyn_index + state 检查
  yycheck 和 yytable 来获取 goto 的状态。
- 如果上述检查失败，则通过 yydefgoto(non_terminal) 获取默认的 goto 状态。(当 non_terminal 在多个状态下的 goto 状态相同时，可以共享空间。)

## `short[] yystos_`
在我们的 SQL parser 中，这个表没有使用到。暂时不分析。 其对应每个状态下 前导终结符，如果没有确定的终结符，则对应的前导非终结符。 

## `sort[] yyr1, yyr2`
- yyr1: 对应于每一个产生式的左侧符号编码。
- yyr2: 对应于每一个产生式的右侧符号的数量。

# Bison 的异常处理

未完成，待续

# Location 

# Comments 处理
