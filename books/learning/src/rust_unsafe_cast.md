# 测试 unsafe cast

# 1. unsafe cast
```rust
fn main(){
    
    let i = 10u32;
    
    let p1 = &i as *const u32 as usize;
    

    // let p2 = do1(p1);    // OK
    // let p2: &mut u32 = unsafe { &mut *(p1 as *mut u32) }; // error: casting &T to &mut T is UB
    let p2: &mut u32 = unsafe { &mut *((p1+0) as *mut u32) }; // OK
    
    *p2 = 20;   // change immutable i, so it is UB

    println!("p1 = {}", p1);
    println!("i = {}, p2 = {:?}", i, p2);

}

fn do1(p1: usize) -> &'static mut u32{
    unsafe { &mut *(p1 as *mut u32) }
} 
```

1. 简单的 `unsafe { &mut *(&i as *const u32 as uszie as *mut u32) }` 会被编译器检查
   （实际上这里应该是 warning 而不是 error,因为本来就是 unsafe）
2. 进行变换后可以绕过编译器检查。
   - 封装成一次函数调用
   - 加上偏移量
3. Rust 中的 raw pointer 自身没有生命周期。