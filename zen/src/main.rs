// 第11章， 安全并发
mod eleven;
mod pow;
// 第3章, 类型系统
mod three;
// 第五章，所有权系统
mod five;

fn main() {
    eleven::test_unsafe_seq();
    // eleven::build_thread();
    eleven::local_thread();
    eleven::park_thread();
    eleven::lock_sync();
    eleven::poison();
    eleven::coins();
    eleven::rw_lock();
    eleven::barrier();
    eleven::condvar();
    eleven::atom();
    eleven::mpsc();
    pow::pow_result();
}
