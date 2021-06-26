use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::{mpsc, Arc};
use std::thread;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

const BASE: usize = 42;
const THREADS: usize = 8;
static DIFFICULTY: &'static str = "00000";
struct Solution(usize, String);

// 给定一个数字，42 找到应该一个数字，要求该数字和42相乘后经过Hash函数处理后
// 得到满足加密字符串以“00000”开头
fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string());
    let hash = hasher.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

fn find(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>) {
    for number in (start_at..).step_by(THREADS) {
        if is_solution_found.load(Ordering::Relaxed) {
            return;
        }

        if let Some(solution) = verify(number) {
            is_solution_found.store(true, Ordering::Relaxed);
            sender.send(solution).unwrap();
            return;
        }
    }
}

pub fn pow_result() {
    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel();
    for i in 0..THREADS {
        let sender = tx.clone();
        let is_solution_found = is_solution_found.clone();

        thread::spawn(move || {
            find(i, sender, is_solution_found);
        });
    }

    match rx.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution: ");
            println!("The number is : {}, and the hash result is :{}", i, hash);
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}
