use std::cmp;

fn main() {
    println!("{}", compare_version("1.2".to_string(), "1.10".to_string()));
}

// 字符串分割法
// pub fn compare_version(version1: String, version2: String) -> i32 {
//     let nums_1: Vec<i32> = version1
//         .split('.')
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();
//     let nums_2: Vec<i32> = version2
//         .split('.')
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();

//     for i in 0..cmp::max(nums_1.len(), nums_2.len()) {
//         let mut ver_1 = 0;
//         let mut ver_2 = 0;
//         if i < nums_1.len() {
//             ver_1 = nums_1[i];
//         }
//         if i < nums_2.len() {
//             ver_2 = nums_2[i];
//         }
//         if ver_1 < ver_2 {
//             return -1;
//         }
//         if ver_1 > ver_2 {
//             return 1;
//         }
//     }
//     0
// }

// 双指针
pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut index_1: usize = 0;
    let mut index_2: usize = 0;
    let len_1 = version1.len();
    let len_2 = version2.len();
    while index_1 < len_1 || index_2 < len_2 {
        let mut x = 0;
        while index_1 < len_1 && version1.as_bytes()[index_1] != b'.' {
            x = x * 10 + (version1.as_bytes()[index_1] - b'0') as i32;
            index_1 += 1;
        }
        index_1 += 1;
        let mut y = 0;
        while index_2 < len_2 && version2.as_bytes()[index_2] != b'.' {
            y = y * 10 + (version2.as_bytes()[index_2] - b'0') as i32;
            index_2 += 1;
        }
        index_2 += 1;
        if x < y {
            return -1;
        }
        if x > y {
            return 1;
        }
    }
    0
}
