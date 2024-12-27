# 阅读 李成栋-从现代CPU 特性和编译的角度分析C++ 代码优化

1. 示例代码：
    ```cpp
    void compute(int *input, int *output){

      if(*input > 10) *output = 1;
      if(*input > 5) *output *= 2;

    }

    # include <stdio.h>
    int main(int args, char **argv){

      int i = 20;
      int o = 0;

      for(int j = 0; j < 800000000; j++){
        compute(&i, &o);
      }

      printf("out = %d\n", o);

    }
    ```

2. ASM
    ```asm
    compute:
            mov     eax, dword ptr [rdi]
            cmp     eax, 11
            jge     .LBB0_1
            cmp     eax, 6
            jge     .LBB0_3
    .LBB0_4:
            ret
    .LBB0_1:
            mov     dword ptr [rsi], 1
            mov     eax, dword ptr [rdi]
            cmp     eax, 6
            jl      .LBB0_4
    .LBB0_3:
            shl     dword ptr [rsi]
            ret
    ```

对比 
1. 在同一个模块中，直接内连优化。循环被消除掉了，直接产生结果。
2. 在不同模块中，除非使用 LTO，否则无法进行内联优化。

对比：
1. rust 在 inline 方面更为激进
2. rust 在 alias 方面更有利于优化。
3. likely 导致分支预测，性能偏差约 25%。

```rust
#![feature(core_intrinsics)]

#[inline(never)]
unsafe fn compute(input: &i32, output: &mut i32) {
    if std::intrinsics::unlikely(*input > 10) {
         *output = 1;
    }
    if *input > 5 {
         *output *= 2;
    }
} 

pub fn main(){
    let i = 20i32;
    let mut o = 0i32;

    let mut j = 0u32;
    while j < 1_000_000_000u32 {
        unsafe { compute(&i, &mut o); }
        j += 1;
    }

    println!("out = {}\n", o);

}
```
1. likely(0.76s) 版本相比 unlikely(1.26s) 版本，性能提升约 40%。