use core::num;
use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    i32,
};

fn main() {
    let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
    let s2 = "c".to_string();
    println!("{:?}", find_error_nums(vec![1, 2, 2, 4]));
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

pub fn reverse_degree(s: String) -> i32 {
    s.chars()
        .enumerate()
        .map(|(i, ch)| (26 + ('a' as i32 - ch as i32)) * (i + 1) as i32)
        .sum()
}

pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    for num1 in &nums1 {
        for num2 in &nums2 {
            if num1 % (num2 * k) == 0 {
                ans += 1;
            }
        }
    }
    ans
}

pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 1..nums.len() {
        let left = nums[0..i].iter().sum::<i32>();
        let right = nums[i..nums.len()].iter().sum::<i32>();
        if (left - right) % 2 == 0 {
            ans += 1;
        }
    }
    ans
}

pub fn is_balanced(num: String) -> bool {
    let mut x = 0;
    let mut y = 0;
    for (i, ch) in num.chars().enumerate() {
        if i % 2 == 0 {
            x += ch as u8 - b'0';
        } else {
            y += ch as u8 - b'0';
        }
    }
    x == y
}

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut index = [0; 26];
    let mut ans = 0;
    for (i, ch) in s.chars().enumerate() {
        index[(ch as u8 - b'a') as usize] = i as i32;
    }
    for (i, ch) in t.chars().enumerate() {
        let r = index[(ch as u8 - b'a') as usize] - i as i32;
        ans += r.abs();
    }
    ans
}

pub fn is_array_special(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 1 {
        return true;
    }
    for i in 1..nums.len() {
        if (nums[i] - nums[i - 1]) % 2 == 0 {
            return false;
        }
    }
    true
}

pub fn min_element(nums: Vec<i32>) -> i32 {
    let mut min = 10001;
    for mut n in nums {
        let mut sum = 0;
        while n != 0 {
            sum += n % 10;
            n /= 10;
        }
        min = min.min(sum);
    }

    min
}

pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
    nums.sort_unstable();
    let mut ans = f64::MIN;
    for i in 0..nums.len() / 2 {
        ans = ans.min((nums[i] + nums[nums.len() - i - 1]) as f64 / 2.0);
    }
    ans
}

pub fn sum_of_the_digits_of_harshad_number(mut x: i32) -> i32 {
    let ans = x;
    let mut sum = 0;
    while x != 0 {
        sum += x % 10;
        x /= 10;
    }
    if ans % sum == 0 {
        sum
    } else {
        -1
    }
}

pub fn count_key_changes(mut s: String) -> i32 {
    let chars: Vec<char> = s.to_lowercase().chars().collect();
    let mut ans = 0;
    for i in 0..s.len() - 1 {
        if chars[i] != chars[i + 1] {
            ans += 1;
        }
    }
    ans
}

pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
    let x = (num1 % 10).min(num2 % 10).min(num3 % 10);
    let y = (num1 % 100).min(num2 % 100).min(num3 % 100) / 10;
    let z = (num1 % 1000).min(num2 % 1000).min(num3 % 1000) / 100;
    let w = (num1 % 10000).min(num2 % 10000).min(num3 % 10000) / 1000;
    println!("{},{},{}", x, y, z);
    x + y * 10 + z * 100 + w * 1000
}

pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut sum = 0;
    println!("{:?}", map);
    for key in map.keys() {
        let v = *map.get(key).unwrap();
        if v % k == 0 {
            sum += key * v
        }
    }
    sum
}

pub fn clear_digits(s: String) -> String {
    let mut ss = VecDeque::new();

    for ch in s.chars() {
        if ch.is_numeric() {
            ss.pop_back();
        } else {
            ss.push_back(ch);
        }
    }

    ss.into_iter().collect()
}

pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();

    for i in 1..mountain.len() - 1 {
        if mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1] {
            ans.push(i as i32);
        }
    }
    ans
}

pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    for i in 0..grid.len() {
        let sum = grid[i].iter().sum::<i32>();
        if sum == (grid.len() - 1) as i32 {
            return i as i32;
        }
    }
    0
}

pub fn possible_string_count(word: String) -> i32 {
    let mut ans = 0;
    let bytes = word.into_bytes();
    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] {
            ans += 1;
        }
    }
    ans + 1
}

pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
    let mut index = 0;
    let mut ans = 0;
    while index < nums.len() {
        let mut pre = 0;
        if index as i32 - k >= 0 {
            pre = nums[index - k as usize];
        }
        let mut after = 0;
        if (index + k as usize) < nums.len() {
            after = nums[index + k as usize];
        }

        let curr = *nums.get(index).unwrap();
        if curr > pre && curr > after {
            ans += curr;
        }
        index += 1;
    }
    ans
}

pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
    for num in nums.iter_mut() {
        if *num % 2 == 0 {
            *num = 0;
        } else {
            *num = 1;
        }
    }
    nums.sort_unstable();
    nums
}

pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let mut ans = 0;

    for i in 0..bytes.len() {
        let mut count = [0, 0];
        for j in i..bytes.len() {
            count[(bytes[j] - b'0') as usize] += 1;
            if count[0] > k && count[1] > k {
                break;
            }
            ans += 1;
        }
    }
    ans
}

pub fn smallest_index(nums: Vec<i32>) -> i32 {
    for (i, num) in nums.iter().enumerate() {
        let mut sum = 0;
        let mut n = *num;
        while n != 0 {
            sum += n % 10;
            n /= 10;
        }
        if sum == i as i32 {
            return i as i32;
        }
    }
    -1
}

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            let mut set = HashSet::new();
            nums[i..=j].iter().for_each(|n| {
                set.insert(*n);
            });
            ans += (set.len() * set.len()) as i32;
        }
    }
    ans % (100_000_000 + 7)
}

pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len() * grid.len();
    let mut vals = vec![0; n];
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            vals[grid[i][j] as usize - 1] += 1;
        }
    }
    let mut ans = vec![0, 0];

    for (i, v) in vals.iter().enumerate() {
        if *v > 1 {
            ans[0] = (i + 1) as i32
        }
        if *v == 0 {
            ans[1] = (i + 1) as i32
        }
    }
    ans
}
pub fn num_different_integers(word: String) -> i32 {
    let mut set: HashSet<String> = HashSet::new();
    let mut cur_num_str = String::new();
    for ch in word.chars() {
        if ch.is_numeric() {
            if cur_num_str == "0" && ch == '0' {
                continue;
            }
            if cur_num_str == "0" && ch != '0' {
                cur_num_str.clear();
            }
            cur_num_str.push(ch);
        } else if !cur_num_str.is_empty() {
            set.insert(cur_num_str.clone());
            cur_num_str.clear();
        }
    }
    if !cur_num_str.is_empty() {
        set.insert(cur_num_str);
    }
    println!("{:?}", set);
    set.len() as i32
}

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut line = 1;
    let mut width = 0;
    for c in s.bytes() {
        let n = widths[(c - b'a') as usize];
        width += n;
        if width > 100 {
            line += 1;
            width = n;
        }
    }
    vec![line, width]
}

pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    let mut ans = (nums[0] - nums[nums.len() - 1]).abs();
    for i in 1..nums.len() {
        ans = ans.max((nums[i] - nums[i - 1]).abs());
    }
    ans
}

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in nums {
        if num % 3 != 0 {
            ans += 1;
        }
    }
    ans
}

pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    fn encrypt(mut n: i32) -> i32 {
        let mut max = 0;
        let mut step = 0;
        if n == 0 {
            return 0;
        }
        while n > 0 {
            max = max.max(n % 10);
            n /= 10;
            step += 1;
        }
        let digit_char = std::char::from_digit(max as u32, 10).unwrap();
        let mut s = String::new();
        for _ in 0..step {
            s.push(digit_char);
        }
        s.parse::<i32>().unwrap_or(0)
    }

    nums.into_iter().map(encrypt).sum()
}

pub fn kth_character(k: i32) -> char {
    let idx = k as usize;
    let mut ans = vec![b'a'];
    while ans.len() < idx {
        let temp = ans.clone();
        for b in temp {
            ans.push(((b - b'a') + 1) % 26 + b'a');
        }
        println!("ans:{:?}", ans);
    }
    ans[idx - 1] as char
}

pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];
    for i in nums.iter().skip(2) {
        if arr1.last() < arr2.last() {
            arr1.push(*i);
        } else {
            arr2.push(*i);
        }
    }
    arr1.extend(arr2);
    arr1
}

pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut min = i32::MAX;
    let mut max = 0;
    let mut ans = Vec::new();
    let mut set = HashSet::new();
    for n in &nums {
        min = min.min(*n);
        max = max.max(*n);
        set.insert(*n);
    }
    for i in min..max {
        if !set.contains(&i) {
            ans.push(i);
        }
    }
    ans
}

pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable();
    let mut ans = 0;
    for i in 1..points.len() {
        ans = ans.max(points[i][0] - points[i - 1][0]);
    }
    ans
}

pub fn min_moves(nums: Vec<i32>) -> i32 {
    let max = nums.iter().max().unwrap();
    let mut ans = 0;
    for num in &nums {
        ans += max - num;
    }
    ans
}

pub fn find_valid_pair(s: String) -> String {
    let mut v = [0i32; 10];
    let bytes: Vec<u8> = s.bytes().collect();
    for b in &bytes {
        v[(b - b'0') as usize] += 1;
    }
    for i in 1..s.len() {
        let pre = bytes[i - 1] - b'0';
        let cur = bytes[i] - b'0';
        if pre != cur && v[pre as usize] == pre as i32 && v[cur as usize] == cur as i32 {
            // return the two-character substring starting at i-1
            return s[i - 1..i + 1].to_string();
        }
    }
    String::new()
}

pub fn min_operations_i(nums: Vec<i32>) -> i32 {
    let pre = nums[0];
    for &n in nums.iter().skip(1) {
        if pre != n {
            return 1;
        }
    }
    0
}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut i = 0;
    let mut count = 0;
    while i < nums.len() {
        if nums[i] == 1 {
            count += 1;
        } else {
            max = max.max(count);
            count = 0;
        }
        i += 1;
    }
    max = max.max(count);
    max
}

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; 2];
    let mut va = vec![0; nums.len()];
    for num in nums.iter() {
        va[*num as usize - 1] += 1;
    }
    println!("{:?}", va);
    for i in 0..nums.len() {
        if va[i] == 2 {
            ans[0] = i as i32 + 1;
        } else if va[i] == 0 {
            ans[1] = i as i32 + 1;
        }
    }
    ans
}
pub fn mirror_distance(n: i32) -> i32 {
    fn reverse(n: i32) -> i32 {
        let s = n.abs().to_string();
        let reverse_s: String = s.chars().rev().collect();
        let mut reverse_num = reverse_s.parse().unwrap_or(0);
        if n < 0 {
            reverse_num = -reverse_num;
        }
        reverse_num
    }
    (n - reverse(n)).abs()
}

struct NeighborSum {
    grid: Vec<Vec<i32>>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        NeighborSum { grid }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let mut ans = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == value {
                    if j > 0 {
                        ans += self.grid[i][j - 1];
                    }
                    if j + 1 < self.grid[0].len() {
                        ans += self.grid[i][j + 1];
                    }
                    if i > 0 {
                        ans += self.grid[i - 1][j];
                    }
                    if i + 1 < self.grid.len() {
                        ans += self.grid[i + 1][j]
                    }
                }
            }
        }
        ans
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let mut ans = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == value {
                    if j > 0 && i > 0 {
                        ans += self.grid[i - 1][j - 1];
                    }
                    if j + 1 < self.grid[0].len() && i + 1 < self.grid[0].len() {
                        ans += self.grid[i + 1][j + 1];
                    }
                    if i > 0 && j + 1 < self.grid[0].len() {
                        ans += self.grid[i - 1][j + 1];
                    }
                    if i + 1 < self.grid.len() && j > 0 {
                        ans += self.grid[i + 1][j - 1]
                    }
                }
            }
        }
        ans
    }
}

pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut count = nums[0];
    for i in 1..nums.len() {
        count += nums[i];
        if count == 0 {
            ans += 1;
        }
    }
    ans
}
