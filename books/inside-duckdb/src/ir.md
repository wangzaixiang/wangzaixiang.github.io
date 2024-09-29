# Typed IR

使用 Typed IR 来作为 Physical Plan 的输入：
1. 可以将 Executor 作为一个独立的模块。
2. Executor 可以独立嵌入到的应用中，由其他的应用生成 TypedIR，然后交给 Executor 执行。
3. TypedIR 可以手工编写，独立优化、调试，更好的进行新 operator 的开发、测试、优化工作。

