//! 第 4 章：Cell 家族
//!
//! 运行：cargo run --bin ch04_cells

#![allow(dead_code)]

use std::cell::{Cell, LazyCell, OnceCell, RefCell, UnsafeCell};
use std::hint::black_box;
use std::mem::{align_of, needs_drop, size_of};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static LAZY_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

#[inline(never)]
fn observe_plain(value: &u32) {
    black_box(value);
}

#[inline(never)]
fn observe_cell(value: &Cell<u32>) {
    black_box(value);
}

#[inline(never)]
fn poke_cell(value: &Cell<u32>) {
    value.set(22);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn freeze_read_plain(value: &u32) -> u64 {
    let before = u64::from(*value);
    observe_plain(black_box(value));
    let after = u64::from(*value);
    (after << 32) | before
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn freeze_read_cell(value: &Cell<u32>) -> u64 {
    let before = u64::from(value.get());
    observe_cell(black_box(value));
    let after = u64::from(value.get());
    (after << 32) | before
}

fn layout_row<T>(name: &str) {
    println!(
        "{name:<30} size={:>2} align={:>2} needs_drop={}",
        size_of::<T>(),
        align_of::<T>(),
        needs_drop::<T>()
    );
}

fn main() {
    println!("== 1. 布局：包装成本与运行时状态 ==");
    layout_row::<u8>("u8");
    layout_row::<Cell<u8>>("Cell<u8>");
    layout_row::<RefCell<u8>>("RefCell<u8>");
    layout_row::<UnsafeCell<u8>>("UnsafeCell<u8>");
    layout_row::<OnceCell<u8>>("OnceCell<u8>");
    println!();
    layout_row::<String>("String");
    layout_row::<Cell<String>>("Cell<String>");
    layout_row::<RefCell<String>>("RefCell<String>");
    layout_row::<UnsafeCell<String>>("UnsafeCell<String>");
    layout_row::<OnceCell<String>>("OnceCell<String>");
    layout_row::<LazyCell<String, fn() -> String>>("LazyCell<String, fn()>");

    println!("\n== 2. UnsafeCell：零检查的底层写入口 ==");
    let unsafe_cell = UnsafeCell::new(10_i32);
    let shared = &unsafe_cell;
    unsafe {
        *shared.get() += 1;
    }
    println!("UnsafeCell value = {}", unsafe_cell.into_inner());

    println!("\n== 3. Freeze：共享引用下的可观察读取 ==");
    let plain = 11_u32;
    let framed = Cell::new(11_u32);
    println!("plain before/after = {:016x}", freeze_read_plain(&plain));
    println!("Cell  before/after = {:016x}", freeze_read_cell(&framed));
    let before = framed.get();
    poke_cell(&framed);
    let after = framed.get();
    println!("Cell mutation before/after = {before}/{after}");

    println!("\n== 4. Cell：通过 &Cell<T> 整体替换值 ==");
    let number = Cell::new(5_u32);
    let number_ref = &number;
    number_ref.set(number_ref.get() + 1);
    let old = number_ref.replace(20);
    println!(
        "number = {}, replaced old = {}, take = {}",
        number.get(),
        old,
        number.take()
    );

    let text = Cell::new(String::from("old"));
    let old_text = text.replace(String::from("new"));
    println!(
        "Cell<String> old = {old_text:?}, current = {:?}",
        text.take()
    );

    println!("\n== 5. RefCell：运行时共享/独占借用检查 ==");
    let values = RefCell::new(vec![1_u32, 2, 3]);
    let first = values.borrow();
    let second = values.borrow();
    println!(
        "shared borrows = {}, try_borrow_mut is_err = {}",
        first.len() + second.len(),
        values.try_borrow_mut().is_err()
    );
    drop(second);
    drop(first);

    {
        let mut exclusive = values.borrow_mut();
        exclusive.push(4);
    }
    println!("values after RefMut = {:?}", values.borrow());

    println!("\n== 6. Rc<RefCell<T>>：单线程共享可变 owner ==");
    let shared_values = Rc::new(RefCell::new(vec![10_u32]));
    let another_owner = Rc::clone(&shared_values);
    another_owner.borrow_mut().push(20);
    println!(
        "Rc count = {}, shared values = {:?}",
        Rc::strong_count(&shared_values),
        shared_values.borrow()
    );

    println!("\n== 7. OnceCell：只初始化一次，之后只读 ==");
    let once = OnceCell::new();
    println!("before initialization = {:?}", once.get());
    let initialized = once.get_or_init(|| String::from("initialized"));
    let rejected = once.set(String::from("second"));
    println!(
        "value = {initialized:?}, second set rejected = {}, stored = {:?}",
        rejected.is_err(),
        once.get()
    );

    println!("\n== 8. LazyCell：第一次解引用时初始化 ==");
    let lazy = LazyCell::new(|| {
        LAZY_INIT_COUNT.fetch_add(1, Ordering::Relaxed);
        String::from("lazy")
    });
    println!(
        "before deref: count = {}",
        LAZY_INIT_COUNT.load(Ordering::Relaxed)
    );
    println!("first = {:?}", &*lazy);
    println!("second = {:?}", &*lazy);
    println!(
        "after derefs: count = {}",
        LAZY_INIT_COUNT.load(Ordering::Relaxed)
    );
}
