//! 第五章 所有权系统
use std::borrow::Cow;

// cow copy on write 写时复制
// Cow<T> 是一个枚举体的智能指针
// Borrowed, 用于包裹引用
// Owned 用于包裹所有者
// 以不可变的方式借用内容，以及在需要可变借用或所有权的时候再克隆一份数据。
fn cow() {
    // 没有可变需求，所以不会克隆
    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);

    // 有可变需求，所以会克隆
    // 注意：借用数据被克隆成了新的对象
    // s2 != i2, 实际上，s2不可变，也不会被改变
    let s2 = [1, 2, 3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);
    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
}
