# promote 晋级

## what is promote?

在编译器后端中，Promote Pass 的主要作用通常是将变量从内存位置提升（Promote）到寄存器中，以减少内存访问开销并启用更高效的代码优化。
这一过程常见于优化阶段，例如 LLVM 中的 PromoteMemoryToRegisterPass

通过 promote, 我们可以：
1. 消除冗余的内存操作。将局部变量从栈内存分配调整为寄存器分配。
2. 寄存器提升后，更易于 SSA 优化。

```text
# before promote
function $ifelse() {
@l0
	%t0 =w par
	%n =l alloc4 4
	storew %t0, %n
	%result =l alloc4 4
	%t3 =w loadsw %n
	%t1 =w csltw 0, %t3
	jnz %t1, @l1, @l2
@l1
	%t6 =w loadsw %n
	%t5 =w mul %t6, 2
	storew %t5, %result
	jmp @l3
@l2
	%t10 =w loadsw %n
	%t11 =w sub 0, 3
	%t9 =w mul %t10, %t11
	storew %t9, %result
@l3
	%t14 =w loadsw %result
	retw %t14
}

# after promote
function $ifelse() {
@l0
	%t0 =w par
	nop
	%n =w copy %t0        # 将 mem &x 转为 tmp x， store -> copy
	nop
	%t3 =w copy %n        # load -> copy
	%t1 =w csltw 0, %t3
	jnz %t1, @l1, @l2
@l1
	%t6 =w copy %n
	%t5 =w mul %t6, 2
	%result =w copy %t5
	jmp @l3
@l2
	%t10 =w copy %n
	%t11 =w sub 0, 3
	%t9 =w mul %t10, %t11
	%result =w copy %t9
@l3
	%t14 =w copy %result
	retw %t14
}
```

## 原理
1. 识别可提升的变量（哪些可以提升，哪些不可以提升）
    - 函数的局部变量 vs 全局变量/静态变量
    - 变量有明确的控制流路径
    - 无内存逃逸
    - 无别名
    - volatile 修饰的变量

2. 插入 phi 
3. 替换内存操作为 tmp 操作。
