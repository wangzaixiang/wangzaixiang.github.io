//! 第 5 章：堆上的所有权
//!
//! 运行：cargo run --bin ch05_heap

use std::mem::{align_of, size_of};

fn layout_row<T>(name: &str) {
    println!(
        "{name:<28} size={:>2} align={:>2}",
        size_of::<T>(),
        align_of::<T>()
    );
}

struct DropSpy {
    name: &'static str,
}

impl Drop for DropSpy {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}

fn dump_vec(label: &str, values: &Vec<u8>) {
    println!(
        "{label:<12} ptr={:p} len={:>2} cap={:>2}",
        values.as_ptr(),
        values.len(),
        values.capacity()
    );
}

fn main() {
    println!("== 1. 堆句柄布局 ==");
    layout_row::<Box<u64>>("Box<u64>");
    layout_row::<Box<[u64]>>("Box<[u64]>");
    layout_row::<Vec<u8>>("Vec<u8>");
    layout_row::<String>("String");
    layout_row::<Option<Box<u64>>>("Option<Box<u64>>");
    layout_row::<Option<Vec<u8>>>("Option<Vec<u8>>");
    layout_row::<Option<String>>("Option<String>");

    println!("\n== 2. Box：移动指针，不移动 pointee ==");
    let original = Box::new([7_u64; 4]);
    let heap_before = original.as_ptr() as usize;
    let moved = original;
    let heap_after = moved.as_ptr() as usize;
    println!(
        "same pointee after move = {}, value[3] = {}",
        heap_before == heap_after,
        moved[3]
    );
    drop(moved);

    {
        let _boxed_drop = Box::new(DropSpy {
            name: "boxed value",
        });
        println!("before end of box scope");
    }
    println!("after end of box scope");

    println!("\n== 3. Vec：空缓冲区与容量增长 ==");
    let mut values = Vec::<u8>::new();
    dump_vec("new", &values);

    let mut previous_capacity = values.capacity();
    for value in 0..20_u8 {
        values.push(value);
        if values.capacity() != previous_capacity {
            dump_vec("grew", &values);
            previous_capacity = values.capacity();
        }
    }
    dump_vec("final", &values);

    println!("\n== 4. Vec 移动与 realloc ==");
    let mut moved_vec = Vec::<u64>::with_capacity(2);
    moved_vec.extend([10, 20]);
    let heap_before = moved_vec.as_ptr() as usize;
    let moved_vec_2 = moved_vec;
    println!(
        "same heap after Vec move = {}, len = {}",
        heap_before == moved_vec_2.as_ptr() as usize,
        moved_vec_2.len()
    );

    let mut growing = Vec::<u64>::with_capacity(1);
    growing.push(1);
    let address_before = growing.as_ptr() as usize;
    growing.reserve(4096);
    let address_after = growing.as_ptr() as usize;
    println!(
        "after reserve(4096): cap={}, pointer_changed={}",
        growing.capacity(),
        address_before != address_after
    );

    let mut boxed_source = Vec::<u64>::with_capacity(3);
    boxed_source.extend([1, 2, 3]);
    let boxed_ptr = boxed_source.as_ptr() as usize;
    let boxed_slice = boxed_source.into_boxed_slice();
    println!(
        "Vec(len=cap=3) -> Box<[u64]>: same allocation={}, len={}",
        boxed_ptr == boxed_slice.as_ptr() as usize,
        boxed_slice.len()
    );

    println!("\n== 5. 精确容量与 ZST ==");
    let exact = Vec::<u64>::with_capacity(3);
    println!(
        "Vec<u64>::with_capacity(3): len={}, cap={}",
        exact.len(),
        exact.capacity()
    );

    let mut zst = Vec::<()>::new();
    let zst_ptr_before = zst.as_ptr();
    zst.extend(std::iter::repeat_n((), 1024));
    println!(
        "Vec<()>: size={} len={} cap={} ptr_unchanged={}",
        size_of::<()>(),
        zst.len(),
        zst.capacity(),
        zst_ptr_before == zst.as_ptr()
    );

    println!("\n== 6. set_len：只登记已初始化前缀 ==");
    let mut initialized = Vec::<u32>::with_capacity(4);
    {
        let spare = initialized.spare_capacity_mut();
        spare[0].write(11);
        spare[1].write(22);
        spare[2].write(33);
    }
    unsafe {
        initialized.set_len(3);
    }
    println!(
        "after explicit initialization: values={initialized:?} len={} cap={}",
        initialized.len(),
        initialized.capacity()
    );

    println!("\n== 7. String：UTF-8 字节序列 ==");
    let text = String::from("A💖");
    let codepoints = text.chars().collect::<Vec<_>>();
    println!(
        "text={text:?} bytes={} chars={} cap={} byte_len(u32)={}",
        text.len(),
        text.chars().count(),
        text.capacity(),
        size_of::<char>()
    );
    println!("bytes = {:?}", text.as_bytes());
    println!("chars = {codepoints:?}");

    let invalid = vec![b'h', 0xff];
    match String::from_utf8(invalid) {
        Ok(value) => println!("unexpected valid UTF-8: {value:?}"),
        Err(error) => {
            let valid_up_to = error.utf8_error().valid_up_to();
            let original = error.into_bytes();
            println!(
                "invalid UTF-8 rejected: valid_up_to={valid_up_to}, original_len={}",
                original.len()
            );
        }
    }
    println!(
        "lossy conversion = {:?}",
        String::from_utf8_lossy(&[b'h', 0xff])
    );

    println!("\n== 8. Vec drop 只析构 initialized 前缀 ==");
    {
        let _dropped = vec![
            DropSpy { name: "element 0" },
            DropSpy { name: "element 1" },
            DropSpy { name: "element 2" },
        ];
        println!("before Vec drop");
    }
    println!("after Vec drop");
}
