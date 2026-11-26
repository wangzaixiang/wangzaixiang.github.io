//! 第 2 章：移动、Copy、Drop
//!
//! 运行：cargo run --bin ch02_move
//!
//! 本探针只记录安全代码中可观察的行为。局部变量地址、参数传递方式等
//! 会受 ABI 和优化影响，留到 MIR / ASM 小节解释。

#![allow(dead_code)]

use std::mem::{ManuallyDrop, forget, size_of};
use std::sync::Mutex;

static EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn record(event: impl Into<String>) {
    EVENTS.lock().unwrap().push(event.into());
}

fn flush(title: &str) {
    let mut events = EVENTS.lock().unwrap();
    println!("{title}: {:?}", *events);
    events.clear();
}

#[derive(Clone, Copy)]
struct CopyWord(u64);

pub struct Tracked(&'static str);

impl Drop for Tracked {
    fn drop(&mut self) {
        record(format!("drop Tracked({})", self.0));
    }
}

struct Outer {
    first: Tracked,
    second: Tracked,
}

impl Drop for Outer {
    fn drop(&mut self) {
        record("drop Outer");
    }
}

#[inline(never)]
fn consume(value: Tracked) {
    record(format!("consume Tracked({})", value.0));
}

#[inline(never)]
fn pass_through(value: Tracked) -> Tracked {
    record(format!("pass_through Tracked({})", value.0));
    value
}

// 专供 MIR 观察：非 Copy 参数先移入局部变量，再作为返回值移出。
#[unsafe(no_mangle)]
#[inline(never)]
pub fn mir_move_into_local(value: Tracked) -> Tracked {
    let moved = value;
    std::hint::black_box(&moved);
    moved
}

// 专供 MIR 观察：参数在正常返回路径和 unwind 路径上都要 drop。
#[unsafe(no_mangle)]
#[inline(never)]
pub fn mir_drop_on_return(value: String) -> usize {
    value.len()
}

fn main() {
    println!("== 1. 移动 String 时移动的是哪些字节 ==");
    let original = String::from("a heap-allocated string");
    let heap_ptr = original.as_ptr();
    let moved = original;
    let cloned = moved.clone();
    println!(
        "String size = {}, same heap pointer after move = {}, after clone = {}",
        size_of::<String>(),
        heap_ptr == moved.as_ptr(),
        heap_ptr == cloned.as_ptr()
    );
    println!("moved value = {moved:?}");
    println!("cloned value = {cloned:?}");

    println!("\n== 2. 移动到函数参数：源位置不 drop，目的地 drop 一次 ==");
    consume(Tracked("argument"));
    flush("after consume");

    println!("\n== 3. 移动作为返回值：所有权继续传给调用者 ==");
    let returned = pass_through(Tracked("returned"));
    record("returned to caller");
    drop(returned);
    flush("after pass_through");

    println!("\n== 4. 对已初始化变量赋值：旧值在赋值点立即 drop ==");
    let mut current = Tracked("old");
    record(format!("initial Tracked({})", current.0));
    flush("before reassignment");
    current = Tracked("new");
    flush("after reassignment");
    drop(current);
    flush("after explicit drop");

    println!("\n== 5. 局部变量离开作用域：按声明逆序 drop ==");
    {
        let _first = Tracked("first local");
        let _second = Tracked("second local");
    }
    flush("after local scope");

    println!("\n== 6. 自定义 Drop 先于字段 drop，字段按声明顺序 drop ==");
    drop(Outer {
        first: Tracked("outer.first"),
        second: Tracked("outer.second"),
    });
    flush("after Outer drop");

    println!("\n== 7. ManuallyDrop 禁止自动 drop ==");
    {
        let _manual = ManuallyDrop::new(Tracked("manual"));
        record("leaving ManuallyDrop scope");
    }
    flush("after ManuallyDrop scope");

    println!("\n== 8. mem::forget 主动泄漏值，不运行析构 ==");
    forget(Tracked("forgotten"));
    flush("after mem::forget");

    println!("\n== 9. Copy：源值仍然可用，没有所有权转移 ==");
    let copy_source = CopyWord(7);
    let copy_destination = copy_source;
    println!(
        "CopyWord size = {}, source = {}, destination = {}",
        size_of::<CopyWord>(),
        copy_source.0,
        copy_destination.0
    );

    println!("\n== 10. 所有权转移不等于深拷贝 ==");
    let boxed = Box::new([1_u64, 2, 3, 4]);
    let box_heap_ptr = &*boxed as *const [u64; 4];
    let boxed_moved = boxed;
    println!(
        "Box<[u64; 4]> size = {}, same pointee after move = {}",
        size_of::<Box<[u64; 4]>>(),
        box_heap_ptr == &*boxed_moved
    );
}
