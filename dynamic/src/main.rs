use std::cmp::max;

fn main() {
    println!("Hello, world!");
    for i in 1..10 {
        println!("fib {} is {}", i, fib(i));
    }

    println!("max fib({}) is {}", 46, fib(46));

    work_earings();
    max_sum();

    let arr = [3, 34, 4, 12, 5, 2];
    let n = arr.len() - 1;
    println!("{} can sum in arr:{}", 9, rec_subset(&arr, n, 9));
    println!("{} can sum in arr:{}", 10, rec_subset(&arr, n, 10));
    println!("{} can sum in arr:{}", 11, rec_subset(&arr, n, 11));
    println!("{} can sum in arr:{}", 12, rec_subset(&arr, n, 12));
    println!("{} can sum in arr:{}", 13, rec_subset(&arr, n, 13));
}

fn fib(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;

    for _i in 1..n {
        let n = a + b;
        a = b;
        b = n;
    }
    a
}

// 利用动态规划解决收益问题
// 根据选择当前和满足除当前外可选收益 ，与不选当前所获得收益比较最大值
// 状态转移方程 opt[i] = max(opt[i].v + opt[prev(i)] , opt[i-1])
// 由低向上，可避免overlay 重复地计算相同子问题
fn work_earings() {
    // 使用元组表示时间开始，时间结束，时间收益
    let mut opt_list = [(0, 0, 0); 8];
    opt_list[0] = (1, 4, 5);
    opt_list[1] = (3, 5, 1);
    opt_list[2] = (0, 6, 8);
    opt_list[3] = (4, 7, 4);
    opt_list[4] = (3, 8, 6);
    opt_list[5] = (5, 9, 3);
    opt_list[6] = (6, 10, 2);
    opt_list[7] = (8, 11, 4);

    // 选择1-8 策略的最大收益
    // opt[0] 表示为0 代表没有收益
    let mut opt = [0; 9];

    for i in 1..opt.len() {
        // opt_list 与 opt 存在序号差异
        // opt[2] 对应 max(opt_list[2 - 1].2 + opt[prev(2 - 1, opt_list)], opt[i-1])
        // TODO 保存结果
        opt[i] = max(opt_list[i - 1].2 + opt[prev(i - 1, &opt_list)], opt[i - 1]);
    }

    for i in 1..opt.len() {
        println!("opt[{}] is {}", i, opt[i]);
    }

    fn prev(n: usize, list: &[(u32, u32, u32)]) -> usize {
        for i in (0..n).rev() {
            if list[i].1 <= list[n].0 {
                return i + 1;
            }
        }
        0
    }
}

// 选出不相邻数字最大的值
// 选和不选
// sum = max(opt[i].v + opt[i - 2], opt[i - 1])
fn max_sum() {
    const LIST: [i32; 7] = [1, 2, 4, 1, 7, 8, 3];
    let mut sum = [0; LIST.len()];

    sum[0] = LIST[0];
    sum[1] = max(LIST[0], LIST[1]);

    for i in 2..LIST.len() {
        sum[i] = max(LIST[i] + sum[i - 2], sum[i - 1]);
    }

    for i in 0..LIST.len() {
        println!("sum({}) is {}", i, sum[i]);
    }
}

// 在一组数组中是否存在相加得到某数的集合
// arr = [3, 34, 4, 12, 5, 2]; S = 9
// 递归的状态转移方程
//  subset(arr, i, S) = subset(arr, i-1, S - arr[i]) | subset(arr, i - 1, S)
//  终止条件
//  if S == 0 return true
//  if i == 0  return  arr[0] == S
//  if arr[i] > S return subset(arr, i-1, S)
// TODO 自底向上的实现
// https://www.bilibili.com/video/BV12W411v7rd    29:20
fn rec_subset(arr: &[i32], i: usize, s: i32) -> bool {
    if s == 0 {
        return true;
    } else if i == 0 {
        return arr[0] == s;
    } else if arr[i] > s {
        return rec_subset(arr, i - 1, s);
    } else {
        return rec_subset(arr, i - 1, s - arr[i]) || rec_subset(arr, i - 1, s);
    }
}
