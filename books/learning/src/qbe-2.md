# QBE 源代码阅读

| dir    | file    | lines | details |
|--------|---------|-------|---------|
| root   | main.c  | 198   | 主流程     |
| root   | parse.c | 1428  |         |
| root   | rega.c  | 698   |         |
| root   | util.c  | 653   |         |
| root   | spill.c | 538   |         |
| root   | fold.c  | 535   |         |
| root   | load.c  | 493   |         |
| root   | mem.c   | 488   |         |
| root   | ssa.c   | 434   |         |
| root   | cfg.c   | 331   |         |
| root   | emit.c  | 254   |         |
| root   | alias.c | 222   |         |
| root   | copy.c  | 217   |         |
| root   | live.c  | 144   |         |
| root   | simpl.c | 126   |         |
| root   | abi.c   | 25    |         |
| total  |         | 6,366 |         |

## main.c

```c
int main(){

   // parse options hd:o:t: 
   
   for each input file {
     parse(FILE *f, char *path, 
        void dbgfile(char *),  // 处理 -d 选项
        void data(Dat *),      // emit data section
        void func(Fn *)        // 通过一系列的 pass 来处理 fn(IR), 最终 emit IR.
     ); 
   }
   
   T.emitfin( outf )          // target emit .s file
}
```

`-d` 提供的如下选项可以帮助我们更好的理解编译过程：（提高可视化，有助于理解内部结构）
- P: print IR after parsing
- M: memory optimization: slot promotion, load elimination, slot coalescing,
- N: print IR after SSA construction
- C: copy elimination
- F: constant folding
- A: abi lowering
- L: liveness
- S: spilling
- R: register allocation


- [ ] 后续对这些过程按照顺序进行逐一分析。
- [ ] struct Fn 是对一个函数的 IR 表达，是核心的数据结构

阅读完成度：198/6366 = 3.1%

## parse.c

1. 一大堆的全局变量

2. 主要的数据结构：
    - [Lnk](https://c9x.me/compile/doc/il.html#Linkage) 修饰 function 和 data
    - Fn
      - Blk
        - Ins
        - Phi
      - Tmp:  %name 局部变量定义
      - Con
      - Mem

    - [ ] Fn 等数据结构注释较少，且不便于调试查看，考虑增加 toString 功能方便调试
    - [ ] 如何遍历 Fn 数据结构，不便于在调试器中查看数据
    - 使用 qbe -d P 来查看 IR 的输出

    parse 的源代码阅读本身没有太大的挑战，主要的挑战是在于对 qbe 的数据结构的理解，这一块是一个挑战，如果，把上述的TODO
    解决了，那么整个代码的可读性会打幅度提升。

    接下来就是对 IR 的 多个 pass 处理了。

    阅读完成度：(198 + 1428)/6366 = 1626/6366 = 25.5%
3. 改造 main.c 增加 -d 9 选项支持，在该选项下，打印每一个 pass 后的 IR 输出。
   然后，对测试的输入进行分析，理解各个 pass 的作用。

   ```text
   export function w $ifelse(w %t0) {
   @l0
       %n =l alloc4 4
       storew %t0, %n
       %result =l alloc4 4
       %t3 =w loadw %n
       %t1 =w csltw 0, %t3
       jnz %t1, @l1, @l2
   @l1
       %t6 =w loadw %n
       %t5 =w mul %t6, 2
       storew %t5, %result
       jmp @l3
   @l2
       %t10 =w loadw %n
       %t11 =w sub 0, 3
       %t9 =w mul %t10, %t11
       storew %t9, %result
   @l3
       %t14 =w loadw %result
       ret %t14
   }
   ```
   1. 主要的 pass 有：
      - promote: 将 slot(局部变量) 转换为 register, 消除 alloc, store, load 操作
      - ssa: 通过 phi 函数，将 IR 转换为 SSA（在此之前的 register 可以多次赋值）
      - copy：消除 copy 操作，减少 register 的使用
      - abi: 引入目标平台的寄存器分配（参数、返回值），对寄存器分配目前还不是很清楚，是如何在IR上进行的
      - isel: 选择指令。 目前还不清楚，是如何和 IR 协调工作的。
   
