# Rust 编程之道

## cargo & rustup

1. rustc 切换到 nightly

`rustup toolchain install nightly`
`rustup override set nightly`

2. cargo test 输出打印

`cargo test -- --nocapture`

## 第三章

### 小技巧

1. 使用单元类型查看数据类型

```rust
let v:() = vec![(); 10];
          ^^^^^^^^^^^^^^^^ expected(), found struct `std::vec::Vec`
```

2. 在一些只需要迭代次数的场合，使用 `Vec<()>` 进行迭代可以获得较高性能，
   Vec 内部迭代器会针对 ZST 类型做一些优化

```rust
let v: Vec<()> = vec![();10];
for i in v {
  println!("{:?}", i);
}
```
