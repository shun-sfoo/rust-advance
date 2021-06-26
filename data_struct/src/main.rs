mod queue;
mod segment_tree;
mod stack;
mod tree_node;

fn main() {
    let a = Box::new(3);
    let mut a_b = Some(a);
    let b = a_b.take();
    println!("{:?}", b);
    println!("{:?}", a_b);
}
