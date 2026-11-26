//! 第 3 章：引用与生命周期
//!
//! 运行：cargo run --bin ch03_refs
//!
//! 本探针展示引用布局、NLL、reborrow、two-phase borrow 和 lifetime erasure。

#![allow(dead_code)]

use std::marker::PhantomData;
use std::mem::size_of;

struct SliceView<'a> {
    data: &'a [u32],
    offset: usize,
}

struct PhantomPtr<'a, T> {
    ptr: *const T,
    marker: PhantomData<&'a T>,
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn lifetime_identity<'a>(value: &'a u32) -> &'a u32 {
    value
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn lifetime_identity_static(value: &'static u32) -> &'static u32 {
    value
}

#[inline(never)]
fn nll_local(mut value: u32) -> u32 {
    let first = &mut value;
    *first += 1;

    // first 仍在词法作用域内，但最后一次使用已经结束，所以这里可以再次借用。
    let second = &mut value;
    *second *= 2;

    value
}

#[inline(never)]
fn sequential_reborrow(values: &mut Vec<u32>) {
    let first = &mut *values;
    first.push(1);

    // first 的最后一次使用已经结束，因此可以重新借用同一个 Vec。
    let second = &mut *values;
    second.push(2);
}

#[inline(never)]
fn two_phase_borrow(values: &mut Vec<usize>) {
    // 接收者的 &mut Vec 先被“预留”；
    // 参数中的共享借用先读取 len，方法真正调用时才激活可变借用。
    values.push(values.len());
}

#[inline(never)]
fn shorten_mut_reference<'long, 'short>(value: &'long mut u32) -> &'short mut u32
where
    'long: 'short,
{
    value
}

#[inline(never)]
fn compare_shared_lifetimes<'a>(s1: &'a str, s2: &'a str) -> bool {
    s1.len() == s2.len()
}

fn main() {
    println!("== 1. 引用的布局 ==");
    println!("&u32              size = {}", size_of::<&u32>());
    println!("&mut u32          size = {}", size_of::<&mut u32>());
    println!("&[u32]            size = {}", size_of::<&[u32]>());
    println!("&str              size = {}", size_of::<&str>());
    println!(
        "&dyn Debug        size = {}",
        size_of::<&dyn std::fmt::Debug>()
    );
    println!(
        "SliceView<'a>     size = {}  (fat pointer + offset)",
        size_of::<SliceView<'static>>()
    );
    println!(
        "PhantomPtr<'a,T>  size = {}  (lifetime 只存在于类型系统)",
        size_of::<PhantomPtr<'static, u32>>()
    );
    println!("Option<&u32>      size = {}", size_of::<Option<&u32>>());

    println!("\n== 2. NLL：最后一次使用决定借用结束点 ==");
    println!("nll_local(20) = {}", nll_local(20));

    println!("\n== 3. 顺序 reborrow ==");
    let mut values = vec![0];
    sequential_reborrow(&mut values);
    println!("values = {values:?}");

    println!("\n== 4. two-phase borrow ==");
    let mut values = vec![10_usize, 20];
    two_phase_borrow(&mut values);
    println!("values = {values:?}");

    println!("\n== 5. &mut 在生命周期参数上协变 ==");
    let mut number = 7_u32;
    {
        let short = shorten_mut_reference(&mut number);
        *short += 1;
    }
    println!("number = {number}");

    println!("\n== 6. 只读引用可以缩短到共同 lifetime ==");
    static STATIC_NUMBER: u32 = 11;
    let static_text: &'static str = "static";
    let local_text = String::from("local");
    println!(
        "compare_shared_lifetimes(&'static str, &'local str) = {}",
        compare_shared_lifetimes(static_text, &local_text)
    );

    println!("\n== 7. 生命周期擦除后，两个 identity 函数产生相同机器行为 ==");
    println!(
        "lifetime_identity = {}, lifetime_identity_static = {}",
        lifetime_identity(&number),
        lifetime_identity_static(&STATIC_NUMBER)
    );
}
