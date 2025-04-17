fn main() {
    let numbers = [1, 2, 3, 4, 5];
    // iterator.fold(init, |acc, x| { /* 处理逻辑 */ }) init: 初始累积值，acc: 累积器（不断更新的中间结果），x: 当前迭代元素
    let sum_f = numbers.iter().fold(0, |acc, &x| acc + x);
    // .sum 计算numbers的和
    let sum_s: u32 = numbers.iter().sum();
    let sum_r = numbers.iter().cloned().reduce(|acc, x| acc + x).unwrap();
    assert_eq!(sum_f, sum_s);
    assert_eq!(sum_f, sum_r);

    let new_numbers = numbers
        .into_iter()
        // .enumerate()
        .map(|x| if x % 2 == 0 { x } else { x + 2 })
        .map(|x| if x > 2 { x * 4 } else { x + 1 })
        .map(|x| if x > 10 { x - 4 } else { x - 1 })
        .collect::<Vec<u32>>();

    println!("new_numbers:{:?}", new_numbers);

    let str = "Complementary metal-oxide semiconductor";
    let change = str
        .split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
        .collect::<Vec<&str>>();
    println!("change:{:?}", change);
}
