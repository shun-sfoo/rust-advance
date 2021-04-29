# 知识点记录

`::` 语法表明 **关联函数** ， 关联函数是针对类型实现的，一些语言把它称为 **静态方法** 。

## rand 随机库使用

```rust
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..101);
```

`use rand::Rng`。`Rng` 是一个 trait ，它定义了随机数生成器应实现的方法，
想使用这些方法的话，此 trait 必须在作用域中。第十章会详细介绍 trait。

`rand::thread_rng` 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，
并从操作系统获取 seed。接下来，调用随机数生成器的 gen_range 方法。
这个方法由刚才引入到作用域的 Rng trait 定义。

## Ordering

从标准库引入了一个叫做 `std::cmp::Ordering` 的类型。同 Result 一样，
Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。
这是比较两个值时可能出现的三种结果。

## 转换

```rust
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```
