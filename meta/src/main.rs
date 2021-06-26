#![feature(trace_macros)]
// mod hashmap_macro;

use meta::hashmap;

fn main() {
    trace_macros!(true);
    let _map = hashmap!(
    "a" => 1,
    "b" => 2,
    "c" => 3,
    );
}
