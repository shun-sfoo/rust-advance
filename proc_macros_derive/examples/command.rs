use proc_macros_derive::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    execute: String,
    args: Vec<String>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .execute("find")
        .args(vec!["v".into(), "-vvv".into()])
        .env(vec![])
        .current_dir("/Users/liuyang/StudySpaces/rust-advance/proc_macros_derive/")
        .finish()
        .unwrap();

    println!("{:?}", command);
}
