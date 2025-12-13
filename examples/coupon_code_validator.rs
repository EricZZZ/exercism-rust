fn main() {
    println!(
        "{:?}",
        validate_coupons(
            vec![
                "SAVE20".to_string(),
                "".to_string(),
                "PHARMA5".to_string(),
                "SAVE@20".to_string()
            ],
            vec![
                "restaurant".to_string(),
                "grocery".to_string(),
                "pharmacy".to_string(),
                "restaurant".to_string()
            ],
            vec![true, true, true, true]
        )
    )
}

// 排序规则：
// 1.先按照其 businessLine 的顺序排序："electronics"、"grocery"、"pharmacy"、"restaurant"。
// 2.在每个类别内，再按照 标识符的字典序（升序）排序。
pub fn validate_coupons(
    code: Vec<String>,
    business_line: Vec<String>,
    is_active: Vec<bool>,
) -> Vec<String> {
    let mut ans = Vec::new();
    let mut groups: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![]];
    for i in 0..code.len() {
        if is_active[i]
            && !code[i].is_empty()
            && code[i]
                .as_str()
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            match business_line[i].as_str() {
                "electronics" => groups[0].push(code[i].clone()),
                "grocery" => groups[1].push(code[i].clone()),
                "pharmacy" => groups[2].push(code[i].clone()),
                "restaurant" => groups[3].push(code[i].clone()),
                _ => {}
            }
        }
    }

    for group in groups.iter_mut() {
        group.sort();
        ans.extend(group.clone());
    }
    ans
}
