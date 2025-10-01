use std::{
    cmp,
    collections::{HashMap, VecDeque},
};

fn main() {
    let s1 = "abc".to_string();
    let s2 = "c".to_string();
    println!("{:?}", earliest_time(vec![vec![1, 6], vec![2, 3]],));
}

pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut ans = Vec::new();
    for (index, words) in words.iter().enumerate() {
        for ch in words.chars() {
            if ch == x {
                ans.push(index as i32);
                break;
            }
        }
    }
    ans
}

pub fn convert_date_to_binary(date: String) -> String {
    date.split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|n| format!("{:b}", n))
        .collect::<Vec<_>>()
        .join("-")
}

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut quenu = VecDeque::<char>::new();
    for ch in chars {
        if ch == '(' || ch == '{' || ch == '[' {
            quenu.push_back(ch);
        }
        if ch == ')' && quenu.pop_back() != Some('(') {
            return false;
        }
        if '}' == ch && quenu.pop_back() != Some('{') {
            return false;
        }
        if ']' == ch && quenu.pop_back() != Some('[') {
            return false;
        }
    }
    quenu.is_empty()
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack == needle {
        return 0;
    }
    if !haystack.contains(&needle) {
        return -1;
    }
    for i in 0..haystack.len() {
        println!("{}", haystack.get(i..i + needle.len()).unwrap());
        if haystack.get(i..i + needle.len()) == Some(&needle) {
            return i as i32;
        }
    }
    -1
}

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    sum % k
}

pub fn subarray_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        let start = cmp::max(0, i as i32 - nums[i]) as usize;

        let n = nums[start..=i].iter().sum::<i32>();

        ans += n;
    }
    ans
}

pub fn min_costs(mut cost: Vec<i32>) -> Vec<i32> {
    for i in 1..cost.len() {
        cost[i] = cmp::min(cost[i], cost[i - 1])
    }
    cost
}

pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    if (x - z).abs() > (y - z).abs() {
        1
    } else if (x - z).abs() == (y - z).abs() {
        0
    } else {
        2
    }
}

pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut num1 = 0;
    let mut num2 = 0;
    for i in 1..=n {
        if i % m == 0 {
            println!("num2:{}", i);
            num2 += i;
        } else {
            println!("num1:{}", i);
            num1 += i;
        }
    }
    num1 - num2
}

pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut friends_index = vec![0; order.len()];
    for i in friends {
        friends_index[i as usize - 1] += 1;
    }
    for i in order {
        if friends_index[i as usize - 1] != 0 {
            ans.push(i);
        }
    }
    ans
}

pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    let mut values: Vec<i32> = Vec::new();
    let mut ans = Vec::new();
    for i in 0..score.len() {
        map.insert(score[i][k as usize], i);
        values.push(score[i][k as usize]);
    }
    values.sort_unstable();
    for i in values.iter().rev() {
        if let Some(&idx) = map.get(i) {
            ans.push(score[idx].clone());
        }
    }
    ans
}

pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
    tasks.iter().map(|v| v[0] + v[1]).min().unwrap()
}
