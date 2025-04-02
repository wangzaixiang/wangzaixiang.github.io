# rust type system

## 1. case from arrow-array

```rust
pub struct PrimitiveArray<T: ArrowPrimitiveType> {
    data_type: DataType,
    /// Values data
    values: ScalarBuffer<T::Native>,
    nulls: Option<NullBuffer>,
}

pub trait ArrowPrimitiveType: primitive::PrimitiveTypeSealed + 'static {
    type Native: ArrowNativeTypeOp;

    const DATA_TYPE: DataType;

    /// Returns the byte width of this primitive type.
    #[deprecated(since = "52.0.0", note = "Use ArrowNativeType::get_byte_width")]
    fn get_byte_width() -> usize {
        std::mem::size_of::<Self::Native>()
    }

    /// Returns a default value of this primitive type.
    ///
    /// This is useful for aggregate array ops like `sum()`, `mean()`.
    fn default_value() -> Self::Native {
        Default::default()
    }
}

pub trait ArrowNativeTypeOp: ArrowNativeType {
    const ZERO: Self;
    const ONE: Self;
    const MIN_TOTAL_ORDER: Self;
    const MAX_TOTAL_ORDER: Self;

    fn add_checked(self, rhs: Self) -> Result<Self, ArrowError>;
    fn add_wrapping(self, rhs: Self) -> Self;
    // ...
    fn compare(self, rhs: Self) -> Ordering;
    fn is_eq(self, rhs: Self) -> bool;
}

pub trait ArrowNativeType: Debug + Send + Sync + Copy + PartialOrd + Default + Sealed + 'static {
}



```

在使用时：

```rust
fn test() {
    let array: PrimitiveArray<Int32Type> = PrimitiveArray::<Int32Type>::from(vec![Some(1), None, Some(3)]);
    let birthday: PrimitiveArray<Data32Type> = Date32Array::from(vec![10957, 10958, 10959]);  
}
```

我更期待的写法是：

```rust
fn test() {
    let array = PrimitiveArray::<i32>::from(vec![1, None, 3]);
    let birthday: PrimitiveArray<Data32> = Date32Array::from(vec![Date32::from(10957), Date32::from(10958), Date32::from(10959)]);
}
```

是否可以使用 rust 的类型体系来实现这种写法？

```rust

struct PrimitiveArray<T: ArrowPrimityType> {
}

struct Date32 {
    value: i32
}

impl ArrowPrimityType for i32 { /* ... */ }
impl ArrowPrimityType for f32 { /* ... */ }
impl ArrowPrimityType for Date32 { /* ... */ }

```

1. a type can having Self level values(const) and methods(fn)
2. a type can having self level values(const) and methods(fn)
3. a type can having type members. (associated type)

## trait

```
   unsafe? trait IDENTIFIER  GenericParams? ( : TypeParamBounds? )? WhereClause? {
     InnerAttribute*
     AssociatedItem*
   }
   
   TypeParamBound: Lifetime | TraitBound | UseBound
   
   Lifetime:   
   1. T: 'static， T 的所有生命周期参数都是 'static
   2. T: 'a, T 的所有生命周期参数都是 'a (或者更长)
   3. T: '_， 是一个compiler推断的生成周期。
   
```

example:

```rust
fn capture<'a, 'b, T>(x: &'a (), y: T) -> impl Sized + use<'a, T> {
  //                                      ~~~~~~~~~~~~~~~~~~~~~~~
  //                                     Captures `'a` and `T` only.
  (x, y)
}

impl<'a, T> Trait<'a, T> for &'a T {}

fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

impl<'a> PartialEq<i32> for &'a T {
    // ...
}

```