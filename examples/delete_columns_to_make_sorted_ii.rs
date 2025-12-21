fn main() {
    println!(
        "{}",
        min_deletion_size(vec![
            "zyx".to_string(),
            "wvu".to_string(),
            "tsr".to_string()
        ])
    )
}

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let m = strs[0].len();

    // 存储目前为止保留下来的字符片段
    let mut kept_strs = vec![String::new(); n];
    let mut deletions = 0;

    // 将 String 转换为字节数组，方便通过索引访问，提高性能
    let byte_strs: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();

    for j in 0..m {
        let mut is_sorted = true;

        // 检查如果加上第 j 列，当前的 kept_strs 是否依然保持非递减字典序
        for i in 0..n - 1 {
            // 模拟 Java 的 (a[i] + char).compareTo(a[i+1] + char)
            // 在 Rust 中，我们可以先比较原始字符串，如果相等再比较当前列字符
            if kept_strs[i] > kept_strs[i + 1]
                || (kept_strs[i] == kept_strs[i + 1] && byte_strs[i][j] > byte_strs[i + 1][j])
            {
                is_sorted = false;
                break;
            }
        }

        if is_sorted {
            // 如果符合升序要求，将该列字符更新到 kept_strs 中
            for i in 0..n {
                kept_strs[i].push(byte_strs[i][j] as char);
            }
        } else {
            // 否则，这一列必须删除
            deletions += 1;
        }
    }

    deletions
}
