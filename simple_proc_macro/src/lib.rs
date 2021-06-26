#![feature(proc_macro_hygiene)]
extern crate proc_macro;
use self::proc_macro::TokenStream;

#[proc_macro_derive(A)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    assert!(input.contains("struct A"));
    r#"
        impl A {
            fn a(&self) -> String {
                format!("hello from impl A")
            }
        }
    "#
    .parse()
    .unwrap()
}

#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let _input = input.to_string();
    format!("fn foo() -> &'static str {{ {} }} ", args)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim_end_matches(",");
    let input: Vec<String> = input
        .split(",")
        .map(|n| {
            let mut data = if n.contains(":") {
                n.split(":")
            } else {
                n.split("=>")
            };

            let (key, value) = (data.next().unwrap(), data.next().unwrap());
            format!("map.insert({},{})", key, value)
        })
        .collect();

    let count: usize = input.len();
    println!("count: {}", count);

    let tokens = format!(
        "
        {{
        let mut map = ::std::collections::HashMap::with_capacity({});
        {}
        map
        }}",
        count,
        input.iter().map(|n| format!("{};", n)).collect::<String>()
    );
    tokens.parse().unwrap()
}
