# Rust Sugars

语法糖具有两面性：
- Pros: 语法糖可以让代码更加简洁，更加易读，更加易写。
- Cons: 语法糖会隐藏实现细节，而如果你并没有理解这些细节，那么可能会导致一些问题：即错误的使用，或者衍生的其他问题。

在这一点上，与抽象具有一定的相似性。

本文收集 Rust 语言的一些语法糖，以帮助加深对其的理解。

## for 循环与 IntoIterator

```rust
for i in 0..10 {
    println!("{}", i);
}

; 上面的代码等效于下面的代码
let iter = (0..10).into_iter();
while let Some(i) = iter.next() {
    println!("{}", i);
}
```

这里以 `collection: Vec<T>` 为例

1. `for elem in collection` 这里的 elem 类型为 `collection.into_iter().Item`, T。 
    - collection 的所有权已经转移给了 `into_iter`。
    - 遍历过程中，elem 的 所有权又从 iterator 转移给了 elem。
    - 可以使用 `for mut elem in collection` 来修饰 elem，这样 elem 就是可变的。
    
2. `for elem in &collection` 这里的 elem 类型为 `(&collection).iter().Item`, 即 &T
    ```rust
    impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
        type Item = &'a T;
        type IntoIter = slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    ```
    - collection 的所有权没有转移
    - iter 过程返回的也是引用，所以 elem 的所有权也没有转移。
3. `for elem in &mut collection` 这里 elem 类型为 `(&mut collection).iter_mut().Item`, 即 &mut T, 
    ```rust
    impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
        type Item = &'a mut T;
        type IntoIter = slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
    ```
    - collection 的所有权没有转移
    - iter_mut 过程返回的是 &mut T
对于所有实现 IntoIterator 的类型 X，都需要参考上述的方式，来分别处理 X, &X, &mut X 的情况。或者根据实现情况，来选择支持其中某一种方式。

## pattern matching

1. literal
2. range: 0..10, 0..=10, 'a'..='z', 0..
3. `_`
4. variable: x, mut x
5. ref variable: ref x, ref mut x
6. enum: Some(x), None, Ok(x), Err(x)
7. tuple: (x, y), (x, y, z)
8. array: `[x, y, z]`
9. slice: `[x,y]`, `[x, _, z]`, `[x, ..., z]`, `[]`
10. struct: `Point { x, y }`, `Point { x: 0, y: 0 }`
11. 引用：`&x`, `&(k,v)`

match 可能引起所有权的转移：
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point({}, {})", self.x, self.y);
    }
}

fn demo(guard: i32){
    let p = Point{x:10, y:20};

    { // block1
        match p {
            v if v.x == guard => {} // p will moved to v
            ref v => {} // p will not moved
        }
    }

    println!("ok");
}

```
1. demo(10) 会打印：
    ```
    Dropping Point(10, 20)  ; // p moved to v, and v will be dropped after block1
    ok
    ```
2. demo(100) 会打印：
    ```
    ok
    Dropping Point(10, 20)   // p will not moved, and will be dropped after demo
    ```
这里就涉及到条件转移，涉及到条件转移时，离开 block 时，都不能再使用 p，因为 p 的所有权可能已经转移了。

```rust
&point match {
    Point{x, y} => {} // x: &i32, y: &i32
    Point{x: ref x1, y: ref y1} => {} // x1: &i32, y1: &i32
    &Point{x, y} => {} // x: i32, y: i32            & 用于从 &struct 中复制数据
    // Point { x: &x1 , y: y1 } => { }  // 编译错误
    // &p2 => {}   //  cannot move out of a shared reference
}

&mut point match {
    Point{x, y} => {} // x: &mut i32, y: &mut i32
    Point{x: ref x1, y: ref y1} => {} // x1: &i32, y1: &i32
    Point{x: ref mut x1, y: ref mut y1 } => { } // x1: &mut i32, y1: &mut i32
    &mut Point {x, y } => { }   // x: i32, y: i32
}
```

Rust 的 pattern match 与 scala 的并不完全相同，scala中，是 unapply 的语法糖，但 rust 显然要复杂很多，都是内置在编译器中。