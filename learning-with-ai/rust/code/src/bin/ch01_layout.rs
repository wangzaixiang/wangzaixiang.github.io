//! 第 1 章：值的布局（size / align / offset / niche）
//!
//! 运行：cargo run --bin ch01_layout
//!
//! 所有数字都是本机（aarch64-apple-darwin, rustc 1.100）实测结果。
//! 布局细节属于实现细节，但稳定得足以作为心智模型使用。

#![allow(dead_code)]

use std::mem::{align_of, offset_of, size_of};
use std::num::NonZeroU32;

// ---------- 1. repr(Rust) vs repr(C)：字段重排 ----------

struct Reordered {
    a: u8,
    b: u32,
    c: u8,
}

#[repr(C)]
struct PaddedC {
    a: u8,
    b: u32,
    c: u8,
}

// ---------- 2. niche optimization 的载体 ----------

struct NonZeroNewtype(NonZeroU32);

// ---------- 3. enum 的 tag + payload ----------

enum Three {
    A,
    B,
    C,
}

enum Payload {
    A(u32),
    B(u64),
}

enum Mixed {
    A,
    B(u32),
}

#[derive(Clone, Copy)]
enum NotReprC {
    A(u8, u8, u8),
    B,
}

fn main() {
    println!("== 1. 标量：size / align ==");
    macro_rules! row {
        ($t:ty) => {
            println!(
                "{:<28} size = {:>2}, align = {:>2}",
                stringify!($t),
                size_of::<$t>(),
                align_of::<$t>()
            );
        };
    }
    row!(u8);
    row!(u32);
    row!(u64);
    row!(u128);
    row!(bool);
    row!(char);
    row!(usize);
    row!(f64);

    println!("\n== 2. 结构体：repr(Rust) 编译器重排，repr(C) 保持声明顺序 ==");
    println!(
        "Reordered  size = {}, align = {}, offsets: a = {}, b = {}, c = {}",
        size_of::<Reordered>(),
        align_of::<Reordered>(),
        offset_of!(Reordered, a),
        offset_of!(Reordered, b),
        offset_of!(Reordered, c),
    );
    println!(
        "PaddedC    size = {}, align = {}, offsets: a = {}, b = {}, c = {}",
        size_of::<PaddedC>(),
        align_of::<PaddedC>(),
        offset_of!(PaddedC, a),
        offset_of!(PaddedC, b),
        offset_of!(PaddedC, c),
    );

    println!("\n== 3. niche optimization：Option<T> 不一定要 tag ==");
    println!(
        "Option<&u8>          size = {:>2}   (裸指针用 0 表示 None)",
        size_of::<Option<&u8>>()
    );
    println!(
        "Option<Box<u8>>      size = {:>2}",
        size_of::<Option<Box<u8>>>()
    );
    println!(
        "Option<fn()>         size = {:>2}",
        size_of::<Option<fn()>>()
    );
    println!(
        "Option<NonZeroU32>   size = {:>2}",
        size_of::<Option<NonZeroU32>>()
    );
    println!(
        "Option<NonZeroNewtype> size = {:>2}  (newtype 可以传递 niche)",
        size_of::<Option<NonZeroNewtype>>()
    );
    println!(
        "Option<bool>         size = {:>2}   (bool 只有 2 个合法字节值)",
        size_of::<Option<bool>>()
    );
    println!(
        "Option<Option<NonZeroU32>> size = {:>2} (niche 只有 1 个值，外层 None 无处安放)",
        size_of::<Option<Option<NonZeroU32>>>()
    );

    println!("\n== 4. 没有 niche 的胖指针 / 容器 ==");
    println!(
        "&[u8]                size = {:>2}   (ptr + len)",
        size_of::<&[u8]>()
    );
    println!("&str                 size = {:>2}", size_of::<&str>());
    println!("Box<[u8]>            size = {:>2}", size_of::<Box<[u8]>>());
    println!(
        "Option<Box<[u8]>>    size = {:>2}   (Box<[T]> 的 ptr 仍是 niche)",
        size_of::<Option<Box<[u8]>>>()
    );
    println!(
        "Vec<u8>              size = {:>2}   (ptr + cap + len)",
        size_of::<Vec<u8>>()
    );
    println!(
        "Option<Vec<u8>>      size = {:>2}   (无 discriminant：容量类型提供 niche)",
        size_of::<Option<Vec<u8>>>()
    );
    println!(
        "Option<Option<Vec<u8>>> size = {:>2} (内层还有多个空位，嵌套后大小仍不变)",
        size_of::<Option<Option<Vec<u8>>>>()
    );
    println!("String               size = {:>2}", size_of::<String>());
    println!(
        "Rc<String>           size = {:>2}   (只是一个指针)",
        size_of::<std::rc::Rc<String>>()
    );
    println!(
        "RcInner<String>      size = {:>2}   (strong + weak + 值，见 rc-layout 笔记)",
        size_of::<RcInner<String>>()
    );
    println!(
        "Option<Rc<String>>   size = {:>2}",
        size_of::<Option<std::rc::Rc<String>>>()
    );

    println!("\n== 5. enum：tag 宽度由 variant 数决定，payload 按最大对齐 ==");
    println!("Three{{A,B,C}}           size = {:>2}", size_of::<Three>());
    println!(
        "Payload{{A(u32),B(u64)}} size = {:>2}",
        size_of::<Payload>()
    );
    println!("Mixed{{A,B(u32)}}        size = {:>2}", size_of::<Mixed>());
    println!(
        "NotReprC{{A(u8,u8,u8),B}} size = {:>2}  (3 字节 payload + tag 挤进 4 字节)",
        size_of::<NotReprC>()
    );
    println!(
        "Option<NotReprC>     size = {:>2}  (4 字节值域只用了 3+1，剩余 252 个空位可做 niche)",
        size_of::<Option<NotReprC>>()
    );

    println!("\n== 6. 关联到 Arrow：一列 u32 + 一个 null bitmap ==");
    println!(
        "一个 PrimitiveArray<u32> 的逻辑载荷 = Vec<u32>(24) + Option<Vec<u8>>(24) + 长度/偏移(8*2)"
    );
}

// 与 std 的 RcInner 等价的内存形状：strong: Cell<usize>, weak: Cell<usize>, value: T
#[allow(dead_code)]
struct RcInner<T> {
    strong: std::cell::Cell<usize>,
    weak: std::cell::Cell<usize>,
    value: T,
}
