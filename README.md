# README

## 使用 Rust 的模块与文档系统构建学习笔记

Rust 的模块分割非常直观，可以清晰地区分各个模块
Rust 的注释系统可以很方便地生成本地文档，
利用这两点，代码和注释都可以很方便地集成在一起

## Rust 注释

| 注释符号         | 说明                                     |
| ---------------- | ---------------------------------------- |
| `//!`            | 模块级文档注释，置于模块头部             |
| `//!! `          | 模块级文档注释，但是和上面注释置于同一行 |
| `/*! comment */` | 模块级文档注释                           |
| `///`            | 行级文档注释                             |
| `//`             | 普通行级注释                             |
| `/* comment */`  | 普通块级注释                             |
| `/** comment */` | 块级文档注释                             |

## 使用

`cargo doc --open` 打开本地生成的文档阅读

添加如下片段，可以在文档中使用 rust playground 运行代码

```rust
#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]
```

## eidtion 2015 和 2018 申明模块的不同方式

2015 中在模块文件下创建 `mod.rs`来组织此文件下的模块,
2018 兼容上面的形式，同时支持在上一级目录创建和此同名的 rs 文件来组织模块

```
edition = "2015"
 .
├──  Cargo.toml
├──  README.md
└──  src
   ├──  advance
   │  ├──  mod.rs
   │  ├──  one.rs
   │  └──  two.rs
   └──  lib.rs

edition = "2018"
 .
├──  Cargo.toml
├──  README.md
└──  src
   ├──  advance
   │  ├──  one.rs
   │  └──  two.rs
   ├──  advance.rs
   └──  lib.rs
```

## 使用子 crate 方式重构项目结构

在主目录的`cargo.toml`中只用申明子 crate 名称。

```toml
[workspace]
members = [
  "algorithms",
  "data-struct",
  "live",
  "zen",
  "meta",
]

```

目录结构如下：这样做的好处是单个子模块可以单独发布。

```
 .
├──  algorithms
│  ├──  Cargo.toml
│  └──  src
│     ├──  advance.rs
│     ├──  basic.rs
│     └──  lib.rs
├──  Cargo.toml
├──  data-struct
│  ├──  Cargo.toml
│  └──  src
│     ├──  lib.rs
│     ├──  queue.rs
│     ├──  segment_tree.rs
│     └──  stack.rs
├──  live
│  ├──  Cargo.toml
│  └──  src
│     ├──  actor.rs
│     ├──  encoder.rs
│     ├──  fibonacci.rs
│     ├──  lib.rs
│     ├──  serialization.rs
│     ├──  strok.rs
│     └──  ticket.rs
├──  meta
│  ├──  Cargo.toml
│  └──  src
│     ├──  hashmap_macro.rs
│     └──  lib.rs
├──  README.md
├──  toml-parse
│  ├──  Cargo.toml
│  └──  src
│     └──  lib.rs
└──  zen
   ├──  Cargo.toml
   └──  src
      ├──  eleven.rs
      ├──  five.rs
      ├──  guessing_game.rs
      ├──  lib.rs
      ├──  pow.rs
      └──  six.rs
```
