fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3];
    let a = Some(3);
    let p = match a {
        Some(i) => i,
        None => 0,
    };
}
