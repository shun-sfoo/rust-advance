fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is :{}", x);
    x = 6;
    println!("The value of x is :{}", x);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let o = x.0;
    let t = x.1;
    let r = x.2;

    let w = x.0;
}
