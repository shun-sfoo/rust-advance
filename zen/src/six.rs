//! 第六章

/**

    ### 按值传递的参数使用

    ```
    fn main() {
    let v = vec![1, 2, 3];
    let v = modify(v);
    println!("{:?}", v);
    }

    fn modify(mut v: Vec<i32>) -> Vec<i32> {
    v.push(42);
    v
    }

    ```
*/
pub fn modify(mut v: Vec<i32>) -> Vec<i32> {
    v.push(42);
    v
}
