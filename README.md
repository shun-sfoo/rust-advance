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
