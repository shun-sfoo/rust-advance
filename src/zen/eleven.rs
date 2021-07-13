//! 安全并发

use std::cell::RefCell;
use std::panic;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread::{self, current, Builder};
use std::time::Duration;

static mut V: i32 = 0;

pub fn test_unsafe_seq() {
    fn unsafe_seq() -> i32 {
        unsafe {
            V += 1;
            V
        }
    }

    let child = thread::spawn(move || {
        for _ in 0..10 {
            unsafe {
                unsafe_seq();
                println!("child: {}", V)
            }
        }
    });

    for _ in 0..10 {
        unsafe {
            unsafe_seq();
            println!("local : {}", V)
        }
    }

    child.join().unwrap();
}

pub fn build_thread() {
    let mut v = vec![];
    for id in 0..5 {
        let thread_name = format!("child-{}", id);
        let size: usize = 3 * 1024;
        let builder = Builder::new().name(thread_name).stack_size(size);
        let child = builder
            .spawn(move || {
                println!("in child : {}", id);
                if id == 3 {
                    let _p = panic::catch_unwind(|| {
                        panic!("oh no!");
                    });
                    println!("in {} do sm", current().name().unwrap());
                }
            })
            .unwrap();

        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }
}

// 线程本地存储，是每个线程独有的存储空间，在这里可以放其他线程无法访问的本地数据
pub fn local_thread() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    thread::spawn(|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    })
}

// park/unpark 阻塞/重启线程 park_timeout 指定阻塞超时时间
pub fn park_thread() {
    let parked_thread = Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    thread::sleep(Duration::from_millis(10));
    println!("Unpark the thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}

// 使用锁进行线程同步 Mutex<T>
// 消除了数据竞争， 存在竞态条件 (push_str操作会乱序执行)
pub fn lock_sync() {
    let s = Arc::new(Mutex::new("Hello".to_string()));
    let mut v = vec![];
    for _ in 0..3 {
        let s_clone = s.clone();
        let child = thread::spawn(move || {
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(" world!");
        });
        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }
}

pub fn poison() {
    let mutex = Arc::new(Mutex::new(1));
    let c_mutex = mutex.clone();
    let _ = thread::spawn(move || {
        let mut data = c_mutex.lock().unwrap();
        *data = 2;
        panic!("oh no");
    })
    .join();

    assert_eq!(mutex.is_poisoned(), true);

    match mutex.lock() {
        Ok(_) => unreachable!(),
        Err(p_err) => {
            let data = p_err.get_ref();
            println!("recovered: {}", data);
        }
    };
}

// 投硬币 8个线程统计连续透出10次硬币需要的次数，算出平均值
pub fn coins() {
    fn flip_simulate(target_fips: u64, total_flips: Arc<Mutex<u64>>) {
        let mut continue_positive = 0;
        let mut iter_counts = 0;
        while continue_positive <= target_fips {
            iter_counts += 1;
            let pro_or_con = rand::random();
            if pro_or_con {
                continue_positive += 1;
            } else {
                continue_positive = 0;
            }
        }
        println!("iter_counts: {}", iter_counts);
        let mut total_flips = total_flips.lock().unwrap();
        *total_flips += iter_counts;
    }

    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_fips = 10;

    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate(target_fips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }

    loop {
        // 利用loop循环来等待所有自线程完成投硬币任务
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average : {}", *total_flips / *completed);
            break;
        }
    }
}

// rwlock 允许多个读者或一个写者
pub fn rw_lock() {
    let lock = RwLock::new(5);

    {
        let _r1 = lock.read().unwrap();
        let _r2 = lock.read().unwrap();
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}

// 屏障(barrier)示例
// wait 方法阻塞了5个线程， 等全部线程执行完前半部分后，再开始后半部分操作
// 屏障一般用于实现线程同步
pub fn barrier() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));

    for id in 0..5 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait id :{}", id);
            c.wait();
            println!("after wait id : {}", id);
        }));
    }

    for handle in handles {
        handle.join().unwrap()
    }
}

// 条件变量
pub fn condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = pair.clone();
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair_clone;
        let mut start = lock.lock().unwrap();
        *start = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut start = lock.lock().unwrap();

    while !*start {
        println!("{}", start); // false
        start = cvar.wait(start).unwrap();
        println!("{}", start) // true
    }
}

pub fn atom() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = spinlock.clone();

    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    while spinlock.load(Ordering::SeqCst) != 0 {} // 自旋锁
    if let Err(panic) = thread.join() {
        println!("Thread had an error : {:?}", panic);
    }
}

pub fn mpsc() {
    let (tx, rx) = channel();
    for id in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(id).unwrap());
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        assert!(0 <= j && j < 10);
    }
}
