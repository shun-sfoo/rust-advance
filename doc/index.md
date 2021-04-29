# The book

## 第二章 猜猜看游戏教程

[guessing_game](guessing_game)

使用某个外部 crate 时，cargo search 找到最新语义化版本 (Semantic Versioning)

### 语义化版本 (也称为 SemVer)

[语义化版本](https://semver.org/lang/zh-CN/)

这是一种定义版本号的标准， `0.5.5` 实际上是 `^0.5.5` 的
简写，它表示 『任何与 `0.5.5` 版本公有 API 相兼容的版本』。

### 使用库

你不可能凭空就知道应该 use 哪个 trait 以及该从 crate 中调用哪个方法。
crate 的使用说明位于其文档中。Cargo 有一个很棒的功能是：
运行 `cargo doc --open` 命令来构建所有本地依赖提供的文档，并在浏览器中打开。
例如，假设你对 rand crate 中的其他功能感兴趣，
你可以运行 `cargo doc --open` 并点击左侧导航栏中的 rand。

## 第三章 常见编程概念

[concept](concept)

### 关键字

[中文](http://120.78.128.153/rustbook/appendix-01-keywords.html)
[英文](https://doc.rust-lang.org/book/appendix-01-keywords.html)

### 变量与可变性

除了防止出现 bug 外，还有很多地方需要权衡取舍。
例如，使用大型数据结构时，适当地使用可变变量，可能比复制和返回新分配的实例更快。
对于较小的数据结构，总是创建新实例，采用更偏向函数式的编程风格，
可能会使代码更易理解，为可读性而牺牲性能或许是值得的。
