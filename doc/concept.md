# 常见编程概念

## 原始标识符

原始标识符（Raw identifiers）允许你使用通常不能使用的关键字，其带有 `r#` 前缀。

```rust
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
} // error

fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
} // ok
```

## 常量

首先，不允许对常量使用 `mut`。常量不光默认不能变，它总是不能变。
声明常量使用 `const` 关键字而不是 `let`，并且 _必须_ 注明值的类型。
常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
最后一个区别是，常量只能被设置为常量表达式，而不能是函数调用的结果，
或任何其他只能在运行时计算出的值。

## 数据类型

两类数据类型子集：标量（scalar）和复合（compound

### 标量

整型、浮点型、布尔类型和字符类

`u32` 该类型声明表明，它关联的值应该是一个占据 32 比特位的无符号整数（有符号整数类型以 i 开头而不是 u）
