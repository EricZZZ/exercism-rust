use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    i32,
};

fn main() {
    let s1 = "abc".to_string();
    let s2 = "c".to_string();
    println!(
        "{:?}",
        final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string(),])
    );
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

pub fn equal_frequency(word: String) -> bool {
    let mut values = [0; 26];
    for ch in word.chars() {
        values[(ch as u8 - b'a') as usize] += 1;
    }

    for i in 0..values.len() {
        if values[i] == 0 {
            continue;
        }
        values[i] -= 1;
        let mut set = HashSet::new();
        for v in values {
            if v > 0 {
                set.insert(v);
            }
        }
        if set.len() == 1 {
            return true;
        }
        values[i] += 1;
    }

    false
}

pub fn can_alice_win(nums: Vec<i32>) -> bool {
    let mut g = 0;
    let mut s = 0;
    for n in nums {
        if n > 9 {
            s += n;
        } else {
            g += n;
        }
    }
    !g == s
}

pub fn count_tested_devices(mut battery_percentages: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..battery_percentages.len() {
        if battery_percentages[i] > 0 {
            for j in i + 1..battery_percentages.len() {
                if battery_percentages[j] > 0 {
                    battery_percentages[j] -= 1;
                }
            }
            ans += 1;
        }
    }
    ans
}

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut v = [0; 101];
    println!("{:?}", v);
    for n in nums {
        println!("{}", n);
        if v[n as usize] >= 1 {
            ans.push(n);
        } else {
            v[n as usize] += 1;
        }
    }
    ans
}

pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let mut n_max = vec![0; grid.len()];
    let mut m_max = vec![0; grid[0].len()];
    let mut ans = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            n_max[i] = n_max[i].max(c);
            m_max[j] = m_max[j].max(c)
        }
    }
    for (i, row) in grid.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            ans += n_max[i].min(m_max[j]) - x;
        }
    }
    ans
}

pub fn alternating_sum(nums: Vec<i32>) -> i32 {
    // let mut ans = 0;
    // for i in 0..nums.len() {
    //     if i % 2 == 0 {
    //         nums[i] = -nums[i];
    //     }
    //     ans += nums[i];
    // }
    // ans
    nums.into_iter()
        .enumerate()
        .map(|(i, num)| if i % 2 == 1 { -num } else { num })
        .sum()
}

pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    let mut ans = 0;
    for command in commands {
        match command.chars().next().unwrap() {
            'U' => ans -= n,
            'R' => ans += 1,
            'D' => ans += n,
            _ => ans -= 1,
        }
    }
    ans
}
