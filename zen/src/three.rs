#[derive(Debug)]
struct Foo;
trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self)
    }
}

fn static_dispatch<T>(t: &T)
where
    T: Bar,
{
    t.baz();
}

fn dynamic_dispatch(t: &Bar) {
    t.baz();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 对比 trait object 和 trait限定
    fn three_trait() {
        let foo = Foo;
        static_dispatch(&foo);
        dynamic_dispatch(&foo);
    }
}
