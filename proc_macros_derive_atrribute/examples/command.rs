use proc_macros_derive_atrribute::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
struct Command {
    execute: String,
    #[builder(each = "arg", default = "Default::default()")]
    args: Vec<String>,
    #[builder(each = "env", default = "vec![\"RUST_LOG=info\".into()]")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .execute("find")
        // .arg("-c")
        // .arg("-vvv")
        // .env("RUST_LOG=info")
        // .current_dir("/Users/liuyang/StudySpaces/rust-advance/proc_macros_derive/")
        .finish()
        .unwrap();

    println!("{:?}", command);
}
