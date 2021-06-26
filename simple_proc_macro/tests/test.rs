#![feature(proc_macro_hygiene)]
use simple_proc_macro::hashmap;

#[macro_use]
extern crate simple_proc_macro;
#[derive(A)]
struct A;

use simple_proc_macro::attr_with_args;

#[test]
fn test_derive_a() {
    assert_eq!("hello from impl A".to_string(), A.a());
}

#[attr_with_args("Hello, Rust!")]
fn foo() {}

#[test]
fn test_foo() {
    assert_eq!(foo(), "Hello, Rust!");
}

#[test]
fn test_hashmap() {
    let map = hashmap! {"a":1, "b":2,};
    assert_eq!(map["a"], 1);
    let map = hashmap! {"a"=>1, "b"=>2, "c"=>3};
    assert_eq!(map["c"], 3);
}
