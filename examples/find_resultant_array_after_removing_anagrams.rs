fn main() {
    // println!(
    //     "{:?}",
    //     remove_anagrams(vec![
    //         "abba".to_string(),
    //         "baba".to_string(),
    //         "bbaa".to_string(),
    //         "cd".to_string(),
    //         "cd".to_string()
    //     ])
    // );
    println!(
        "{:?}",
        remove_anagrams(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ])
    )
}

pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    let mut i = 0;

    while i < words.len() {
        let mut temp: Vec<char> = words[i].chars().collect();
        ans.push(words[i].clone());
        temp.sort_unstable();

        while i + 1 < words.len() {
            let mut temp1: Vec<char> = words[i + 1].chars().collect();
            temp1.sort_unstable();

            if temp == temp1 {
                i += 1
            } else {
                println!("temp:{:?},temp1:{:?},i:{}", temp, temp1, i);
                break;
            }
        }
        i += 1;
    }

    ans
}
