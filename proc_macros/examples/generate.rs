pub mod generated {
    use proc_macros::generate;
    generate!("proc_macros/fixtures/person.json");
}

use generated::*;

pub fn main() {
    let p = Person {
        first_name: "law".into(),
        last_name: "z".into(),
        skill: Skill {
            name: "rust".into(),
        },
    };

    println!("{:?}", p);
}
