fn main() {
    let numbers = [1, 2, 3, 4, 5];
    // iterator.fold(init, |acc, x| { /* 处理逻辑 */ }) init: 初始累积值，acc: 累积器（不断更新的中间结果），x: 当前迭代元素
    let sum_f = numbers.iter().fold(1, |acc, &x| acc + x);
    // .sum 计算numbers的和
    let sum_s: u32 = numbers.iter().sum();
    let sum_r = numbers.iter().cloned().reduce(|acc, x| acc + x).unwrap();
    assert_eq!(sum_f, sum_s);
    assert_eq!(sum_f, sum_r);
}
