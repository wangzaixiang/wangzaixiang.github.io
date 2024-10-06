+++
title = "LLVM 学习笔记"
description = "LLVM 学习笔记"
date = 2024-10-06
draft = false
template = "blog/page.html"
+++

# Why learn LLVM?
在阅读 duckdb 的源代码，对 duckdb 执行向量计算的方式有初步的了解之后，思考如何在 OLAP 引擎中高效的进行向量计算，感觉引入 JIT 会是一个更佳
的选择。

尝试阅读 x86 的 SIMD 汇编代码，感觉是非常困难的，SIMD 的汇编指令非常复杂，可读性很差，而对应编译生成的 LLVM-IR 则可读性、可理解性高了很多，
基本上，对照 LLVM-IR 文档，一遍下来就可以理解其含义。再考虑到跨 CPU 体系的兼容支持，LLVM-IR 无疑是一个更高level的，更容易理解的，同时也有
更好的可移植性的中间表示。

1. 初读 LLVM-IR 示例代码
   
   自己动手编写一段 Rust 代码，然后编译生成 LLVM-IR，再对照 LLVM-IR 文档，逐一阅读 IR 代码，相比直接阅读 IR 文档来说，是一个很好的场景化
   学习方式，对 IR 有了一些初步的了解后，后续再系统的学习 LLVM-IR 的知识，就显得没那么枯燥了。

   - Rust 源代码
        ```rust
        #[inline(never)]
        pub fn select(v1: &[i32], v2: &[i32], result: &mut [bool]) {
            assert!(v1.len() == v2.len() && v1.len() == result.len());
            for i in 0..v1.len() {
                if v1[i] > v2[i] {
                    result[i] = true
                } else {
                    result[i] = false
                }
            }
        }
        ```     
   - LLVM-IR 代码
        ```
        ; l_asm::demo1::select
        ; Function Attrs: noinline uwtable
        define internal fastcc void @_ZN5l_asm5demo16select17h43db37ec056aed21E(
            ptr noalias nocapture noundef nonnull readonly align 4 %v1.0, i64 noundef %v1.1,
            ptr noalias nocapture noundef nonnull readonly align 4 %v2.0, i64 noundef %v2.1,
            ptr noalias nocapture noundef nonnull writeonly align 1 %result.0, i64 noundef %result.1) unnamed_addr #0 {
        start:
          %_4 = icmp eq i64 %v1.1, %v2.1
          %_7 = icmp eq i64 %v1.1, %result.1
          %or.cond = and i1 %_4, %_7  -- i1: bit
          br i1 %or.cond, label %bb5.preheader.split, label %bb4  -- br type iftrue ifalse

        bb5.preheader.split:                              ; preds = %start
          %_218.not = icmp eq i64 %v1.1, 0
          br i1 %_218.not, label %bb15, label %bb13.preheader

        bb13.preheader:                                   ; preds = %bb5.preheader.split
          %min.iters.check = icmp ult i64 %v1.1, 32       -- unsigned less than
          br i1 %min.iters.check, label %bb13.preheader17, label %vector.ph

        vector.ph:                                        ; preds = %bb13.preheader
          %n.vec = and i64 %v1.1, -32
          br label %vector.body

        vector.body:                                      ; preds = %vector.body, %vector.ph
          %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]  -- TODO? what is phi
          %0 = getelementptr inbounds [0 x i32], ptr %v1.0, i64 0, i64 %index  -- %0 = %v1.0
          %1 = getelementptr inbounds i32, ptr %0, i64 8                       -- %1 = %0 + 32byte
          %2 = getelementptr inbounds i32, ptr %0, i64 16
          %3 = getelementptr inbounds i32, ptr %0, i64 24
          %wide.load = load <8 x i32>, ptr %0, align 4                          -- 8x32 from v1
          %wide.load10 = load <8 x i32>, ptr %1, align 4
          %wide.load11 = load <8 x i32>, ptr %2, align 4
          %wide.load12 = load <8 x i32>, ptr %3, align 4
          
          %4 = getelementptr inbounds [0 x i32], ptr %v2.0, i64 0, i64 %index
          %5 = getelementptr inbounds i32, ptr %4, i64 8
          %6 = getelementptr inbounds i32, ptr %4, i64 16
          %7 = getelementptr inbounds i32, ptr %4, i64 24
          %wide.load13 = load <8 x i32>, ptr %4, align 4                        -- 8x32 from v1
          %wide.load14 = load <8 x i32>, ptr %5, align 4
          %wide.load15 = load <8 x i32>, ptr %6, align 4
          %wide.load16 = load <8 x i32>, ptr %7, align 4
          
          %8 = icmp sgt <8 x i32> %wide.load, %wide.load13                   -- signed greater than
          %9 = icmp sgt <8 x i32> %wide.load10, %wide.load14
          %10 = icmp sgt <8 x i32> %wide.load11, %wide.load15
          %11 = icmp sgt <8 x i32> %wide.load12, %wide.load16
          
          %12 = zext <8 x i1> %8 to <8 x i8>                                 -- zero extend 8x1 to 8x8
          %13 = zext <8 x i1> %9 to <8 x i8>
          %14 = zext <8 x i1> %10 to <8 x i8>
          %15 = zext <8 x i1> %11 to <8 x i8>
          
          %16 = getelementptr inbounds [0 x i8], ptr %result.0, i64 0, i64 %index
          %17 = getelementptr inbounds i8, ptr %16, i64 8
          %18 = getelementptr inbounds i8, ptr %16, i64 16
          %19 = getelementptr inbounds i8, ptr %16, i64 24
          
          store <8 x i8> %12, ptr %16, align 1                               -- store 8x8 to result
          store <8 x i8> %13, ptr %17, align 1
          store <8 x i8> %14, ptr %18, align 1
          store <8 x i8> %15, ptr %19, align 1
          
          %index.next = add nuw i64 %index, 32
          %20 = icmp eq i64 %index.next, %n.vec
          br i1 %20, label %middle.block, label %vector.body, !llvm.loop !17  -- TODO what's !llvm.loop !17

        middle.block:                                     ; preds = %vector.body
          %cmp.n = icmp eq i64 %n.vec, %v1.1
          br i1 %cmp.n, label %bb15, label %bb13.preheader17

        bb13.preheader17:                                 ; preds = %bb13.preheader, %middle.block
          %iter.sroa.0.09.ph = phi i64 [ 0, %bb13.preheader ], [ %n.vec, %middle.block ]
          br label %bb13

        bb4:                                              ; preds = %start
        ; call core::panicking::panic
          tail call void @_ZN4core9panicking5panic17h2a3e12572053020cE(ptr noalias noundef nonnull readonly align 1 @alloc_882a6b32f40210455571ae125dfbea95, i64 noundef 66, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_649ca88820fbe63b563e38f24e967ee7) #12
          unreachable

        bb15:                                             ; preds = %bb13, %middle.block, %bb5.preheader.split
          ret void

        bb13:                                             ; preds = %bb13.preheader17, %bb13
          %iter.sroa.0.09 = phi i64 [ %_0.i, %bb13 ], [ %iter.sroa.0.09.ph, %bb13.preheader17 ]
          %_0.i = add nuw i64 %iter.sroa.0.09, 1
          %21 = getelementptr inbounds [0 x i32], ptr %v1.0, i64 0, i64 %iter.sroa.0.09
          %_13 = load i32, ptr %21, align 4, !noundef !4
          %22 = getelementptr inbounds [0 x i32], ptr %v2.0, i64 0, i64 %iter.sroa.0.09
          %_15 = load i32, ptr %22, align 4, !noundef !4
          %_12 = icmp sgt i32 %_13, %_15
          %spec.select = zext i1 %_12 to i8
          %23 = getelementptr inbounds [0 x i8], ptr %result.0, i64 0, i64 %iter.sroa.0.09
          store i8 %spec.select, ptr %23, align 1
          %exitcond.not = icmp eq i64 %_0.i, %v1.1
          br i1 %exitcond.not, label %bb15, label %bb13, !llvm.loop !20
        }
        ```
   
2. 生成 LLVM-IR （这里的 ll 代码是从 LLVM 官方的 Kaleidoscope Chapter 3 生成的，是一个很简单版的 ll 源文件了）
   ```
    ; file a1.ll
    ; ModuleID = 'my cool jit'
    source_filename = "my cool jit"

    define double @foo(double %a, double %b) {
        entry:
        %multmp = fmul double %a, %a
        %multmp1 = fmul double 2.000000e+00, %a
        %multmp2 = fmul double %multmp1, %b
        %addtmp = fadd double %multmp, %multmp2
        %multmp3 = fmul double %b, %b
        %addtmp4 = fadd double %addtmp, %multmp3
        ret double %addtmp4
    }

    declare double @cos(double)
   ```
3. 将上述代码编译为可执行代码。
   1. 编写一个 main 函数，调用 foo 函数
      ```c
        // file a0.c
        # include <stdio.h>

        extern double foo(double a, double b);

        int main(){
        double r = foo(1.0, 2.0);
        printf("foo(1.0,2.0) = %lf", r);
        }
      ```
   2. 编译、执行
      ```shell 
      llc a1.ll -o a1.s
      gcc -o a0 a0.c a1.s
      ./a0   # foo(1.0,2.0) = 9.000000
      ```
4. JIT 方式

   本质上，JIT 执行方式与步骤3 的方式是一样的，只是 JIT 方式在运行时，编译生成的机器码，是在内存中，并 mmap 到可执行内存区域，然后直接执行。
   当然，需要处理的是一些符号连接，包括获取 生成的函数地址，也包括调用宿主环境提供的函数等。

# 工具速查
- LLVM 工具
    - llc : .ll to .s, .bc to .s
    - lli : run .ll, .bc
    - llvm-as: .ll to .bc
    - llvm-dis : .bc to .ll
    - as:  .s to .o
    - ld: link .o to .out
    - cc -v 可以查看完整的编译过程，每一个编译阶段的命令行。
- Rust 编译相关
    1. 使用 `cargo rustc --target x86_64-apple-darwin --release -- --emit asm -C llvm-args=-x86-asm-syntax=intel` 来生成汇编代码。
       生成的汇编代码，可以在 `target/x86_64-apple-darwin/release/deps/` 目录下找到。
    2. 使用选项 -C target-cpu=native 来生成针对当前 CPU 的优化代码。`cargo rustc --target x86_64-apple-darwin --release -- -C target-cpu=native --emit asm`
    3. 使用选项 -C target-feature=+avx2 来生成针对 AVX2 指令集的优化代码。
    4. cargo -vv 可以查看详细的执行命令行参数。 
