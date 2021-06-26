#[macro_export]
macro_rules! hashmap {
    // 内部宏必须放到真正的匹配规则之前
    // @开头是惯用法,也可不要
    (@unit $($x:tt)*) => (());

    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));

    ($($key:expr => $value:expr),* $(,)* ) => {
        {

            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);

            $(
                _map.insert($key, $value);
                )*

                _map

        }
    };
}

#[macro_export]
macro_rules! hashmap_recursion {
    ($($key:expr => $value:expr,)*) => {
        hashmap_recursion!($($key => $value),*)
    };

    ($($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::HashMap::new();

            $(
                _map.insert($key, $value);
                )*

                _map

        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_hash_map() {
        let map = hashmap!(
        "a" => 1,
        "b" => 2,
        "c" => 3,
        );

        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map["c"], 3);
    }

    #[test]
    fn test_hash_map_recursion() {
        let map = hashmap_recursion!(
        "a" => 1,
        "b" => 2,
        "c" => 3,
        );

        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map["c"], 3);
    }
}
