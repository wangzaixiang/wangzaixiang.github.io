# zig misc

1. zig 中的传值、传址？
   - zig 中的基础类型如 integer/floats 等是采用 pass by value 的方式传递参数的。
   - 对 struts/unions/array 等数据类型，作为参数传递时，由于 zig 中参数都是 const 的，因此，zig 可以选择使用传值或者传址的方式。一般的，采用
     传址方式会具有更小的效率。
2. zig 的指针
   - *T: 单值指针，不支持指针运算
   - `[*]T`: 多值指针，支持 `ptr[i]` 运算，或者 `ptr[start..end]` 返回一个切片
   - `*[N]T`: 数组指针，sizeof = 8
   - `[]T`: slice, 是一个胖指针，对应 rust中的 `&[T]`, sizeof = 16 
   - `arr[1..4]` 的类型是 `*[3]T`
3. Zig 支持 u3 等小整数类型，但目前来看，其并不会合并到一个字节中（&取址会比较复杂）。