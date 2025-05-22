# [Return Position Impl Trait](https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html#return-position-impl-trait-in-trait-rpitit)

```rust
```

# example 1
```rust
fn apply<F>(f: F)
where
    F: Fn(&str),
{
    let s = String::from("hello");
    f(&s);
}
```
