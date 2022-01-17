# 过程宏

需要将 lib 在 cargo.toml 中声明为`proc_macro`

```toml
[lib]
proc-macro = true
```

两个重要的库

syn 不仅仅打印 tokenstream 将这些转换成 rust 下描述数据结构的数据结构

queto 使用宏直接操作 tokenstream 的结构

askama 字符串 template
