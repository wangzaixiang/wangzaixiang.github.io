//! 第 1 章配套：布局决定 ABI
//!
//! 观察方法（macOS / aarch64）：
//!   cargo rustc --release --bin ch01_abi -- --emit asm -C debuginfo=0
//!   然后看 target/release/deps/ch01_abi-*.s 里 take_* 的符号
//!
//! 结论：小于等于 16 字节的聚合值走寄存器（x0/x1），更大的走隐藏指针（sret）。

#![allow(dead_code)]

#[unsafe(no_mangle)]
#[inline(never)]
pub fn take_pair(p: (u64, u64)) -> u64 {
    p.0.wrapping_add(p.1)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn take_triple(t: (u64, u64, u64)) -> u64 {
    t.0.wrapping_add(t.1).wrapping_add(t.2)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn take_opt_ref(o: Option<&u8>) -> u64 {
    match o {
        Some(v) => u64::from(*v),
        None => u64::MAX,
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn take_opt_u32(o: Option<std::num::NonZeroU32>) -> u64 {
    match o {
        Some(v) => u64::from(v.get()),
        None => 0,
    }
}

fn main() {
    println!(
        "{}",
        take_pair((1, 2)) + take_triple((1, 2, 3)) + take_opt_u32(None)
    );
}
