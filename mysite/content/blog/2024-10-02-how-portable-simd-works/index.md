+++
title = "Portable SIMD: How it works"
description = "portable-simd 是一个 Rust 库，提供了一种跨平台的 SIMD 操作接口。相比在 C++ 中使用 SIMD 库的复杂，portable-simd 显得简单很多，本文探索 portable-simd 是如何实现这个编译魔法的呢？"
date = 2024-10-02
draft = false
template = "blog/page.html"
+++

# Portable SIMD: How it works

一直很好奇，portable simd 库是如何实现的，难道是修改了 rustc 的编译期，从而生成支持 SIMD 的机器代码？但在 portable-simd 的源代码中，
并没有看到与 rustc 相关的修改。那么，portable-simd 是如何实现的呢？

要探究这个问题，可以编写一个简单的示例，然后跟踪一下编译过程，就可以知道 portable-simd 是如何工作的了。

## 示例

1. 创建一个新的 Rust 项目
   - cargo new hellosimd
   - cd hellosimd
   - add cargo-toolchain.toml, 这样，后续就不需要每次添加 +nightly 选项了。
     ```toml
     [toolchain]
     channel = "nightly"
     ```
   - add src/main.rs as  
   ```rust
    #![feature(portable_simd)]
    use std::simd::f32x4;
    use std::simd::num::SimdFloat;

    fn main() {
        let v1: f32 = std::env::args().nth(1).unwrap().parse().unwrap(); 
        let result = test(v1);
        println!("{:?}", result);
    }

    #[inline(never)] 
    fn test(num: f32) -> f32 {
        let a = f32x4::splat(num);
        let b = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
        let c = a + b;
        c.reduce_sum()
    }
    ```

2. 查看 MIR 代码: `cargo rustc --release -- -Zunpretty=mir`
   MIR 代码比较难以阅读，大概可以看到这里调用了 simd_shuffle, simd_add, simd_reduce_add_ordered 函数。
   ```
       bb0: {
        _5 = [copy _1];
        StorageLive(_6);
        _6 = copy _5;
        StorageLive(_7);
        _7 = &raw const _6;
        StorageLive(_11);
        StorageLive(_8);
        _8 = MaybeUninit::<Simd<f32, 1>> { uninit: const () };
        StorageLive(_9);
        _11 = &raw mut _8;
        _9 = copy _11 as *mut [f32; 1] (PtrToPtr);
        copy_nonoverlapping(dst = copy _9, src = copy _7, count = const 1_usize);
        StorageDead(_9);
        StorageLive(_10);
        _10 = copy _8;
        _17 = move ((_10.1: std::mem::ManuallyDrop<std::simd::Simd<f32, 1>>).0: std::simd::Simd<f32, 1>);
        StorageDead(_10);
        StorageDead(_8);
        StorageDead(_11);
        StorageDead(_7);
        StorageDead(_6);
        _2 = simd_shuffle::<Simd<f32, 1>, swizzle::{constant#0}::SimdShuffleIdx<4>, Simd<f32, 4>>(copy _17, move _17, const <std::simd::Simd<T, N>::splat::Splat as std::simd::Swizzle<4>>::swizzle::<f32, 1>::{constant#0}) -> [return: bb1, unwind unreachable];
    }

    bb1: {
        StorageLive(_3);
        _3 = [const 1f32, const 2f32, const 3f32, const 4f32];
        StorageLive(_12);
        _12 = &raw const _3;
        StorageLive(_16);
        StorageLive(_13);
        _13 = MaybeUninit::<Simd<f32, 4>> { uninit: const () };
        StorageLive(_14);
        _16 = &raw mut _13;
        _14 = copy _16 as *mut [f32; 4] (PtrToPtr);
        copy_nonoverlapping(dst = copy _14, src = copy _12, count = const 1_usize);
        StorageDead(_14);
        StorageLive(_15);
        _15 = copy _13;
        _18 = move ((_15.1: std::mem::ManuallyDrop<std::simd::Simd<f32, 4>>).0: std::simd::Simd<f32, 4>);
        StorageDead(_15);
        StorageDead(_13);
        StorageDead(_16);
        StorageDead(_12);
        StorageDead(_3);
        _4 = simd_add::<Simd<f32, 4>>(move _2, move _18) -> [return: bb2, unwind unreachable];
    }

    bb2: {
        _0 = simd_reduce_add_ordered::<Simd<f32, 4>, f32>(move _4, const 0f32) -> [return: bb3, unwind unreachable];
    }

   ```
   在 MIR 这个层次，可以看到 splat 函数被 inline 成了一次 simd_shuffle 调用。这与 portable-simd 源代码是一致的。
   ```rust
    #[inline]
    pub fn splat(value: T) -> Self {
        // This is preferred over `[value; N]`, since it's explicitly a splat:
        // https://github.com/rust-lang/rust/issues/97804
        struct Splat;
        impl<const N: usize> Swizzle<N> for Splat {
            const INDEX: [usize; N] = [0; N];
        }
        Splat::swizzle::<T, 1>(Simd::<T, 1>::from([value]))
    }
   ```
   
   而 simd_shuffle 函数定义如下：
   ```rust
   -- file stdlib/core/src/intrinsics/simd.rs
   extern "rust-intrinsic" {
    #[rustc_nounwind]
    pub fn simd_shuffle<T, U, V>(x: T, y: T, idx: U) -> V;

    #[rustc_nounwind]
    pub fn simd_add<T>(x: T, y: T) -> T;
   
    #[rustc_nounwind]
    pub fn simd_reduce_add_ordered<T, U>(x: T, y: U) -> U;
   } 
   ```
   这些函数都是 rustc 的 intrinsics 函数，编译器会将这些函数调用转换为对应的 LLVM SIMD 指令。

3. 继续查看 LLVM-IR 代码：`cargo rustc --release -- --emit llvm-ir`
   ```
    ; hellosimd::test
    ; Function Attrs: mustprogress nofree noinline norecurse nosync nounwind willreturn memory(none) uwtable
    define internal fastcc noundef float @_ZN9hellosimd4test17h3d900c9e1094a824E(float noundef %num) unnamed_addr #4 {
    start:
    %0 = insertelement <1 x float> poison, float %num, i64 0
    %1 = shufflevector <1 x float> %0, <1 x float> poison, <4 x i32> zeroinitializer
    %2 = fadd <4 x float> %1, <float 1.000000e+00, float 2.000000e+00, float 3.000000e+00, float 4.000000e+00>
    %3 = tail call float @llvm.vector.reduce.fadd.v4f32(float 0.000000e+00, <4 x float> %2)
    ret float %3
    }
   ```
   此时，生成的 LLVM-IR 已经是 LLVM 的 SIMD 指令了。

## 结论
1. portable-simd 本身仅仅是一个普通的 Rust 库，不过，其内部依赖了 rustc 的 intrinsics 函数，而这些函数在 LLVM 下会转换为 LLVM 的 指令。
2. rustc 内部有很多与 CPU 架构相关的 intrinsics 函数，这些函数在编译后（可能）会直接转换为目标 CPU 上的指令，而非函数调用。显然， intrinsics 函数
   的可移植能力对编译期来说，会是一个挑战。rustc 目前是建立在 LLVM 的基础之上，把这个责任转嫁给了 LLVM。
3. 我们也可以参考 portable-simd 的实现，来利用 rustc 的 intrinsics 函数，来实现自己的某些特定操作。