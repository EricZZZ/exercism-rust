use std::collections::BTreeSet;

fn main() {
    println!("{}", minimum_pair_removal(vec![5, 2, 3, 1]))
}

pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }

    // pairs 存储: (相邻元素和, 左侧下标)
    let mut pairs = BTreeSet::new();
    let mut dec = 0;
    let mut a: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();

    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            dec += 1;
        }
        pairs.insert((a[i] + a[i + 1], i));
    }

    // 维护活跃的下标
    let mut idx: BTreeSet<usize> = (0..n).collect();
    let mut ans = 0;

    while dec > 0 {
        ans += 1;

        // 1. 取出当前和最小的一对
        let (s, i) = *pairs.iter().next().unwrap();
        pairs.remove(&(s, i));

        // 获取当前 i 及其后继 nxt 的位置
        let nxt = *idx.range((i + 1)..).next().unwrap();

        if a[i] > a[nxt] {
            dec -= 1;
        }

        // 处理前驱：(pre, i)
        let pre = idx.range(..i).next_back().cloned();
        if let Some(p_idx) = pre {
            // 旧的贡献与 pair
            if a[p_idx] > a[i] {
                dec -= 1;
            }
            pairs.remove(&(a[p_idx] + a[i], p_idx));

            // 新的贡献与 pair (pre, i_new) 其中 i_new 的值变为 s
            if a[p_idx] > s {
                dec += 1;
            }
            pairs.insert((a[p_idx] + s, p_idx));
        }

        // 处理后继的后继：(nxt, nxt2)
        let nxt2 = idx.range((nxt + 1)..).next().cloned();
        if let Some(n2_idx) = nxt2 {
            // 旧的贡献与 pair
            if a[nxt] > a[n2_idx] {
                dec -= 1;
            }
            pairs.remove(&(a[nxt] + a[n2_idx], nxt));

            // 新的贡献与 pair (i_new, nxt2)
            if s > a[n2_idx] {
                dec += 1;
            }
            pairs.insert((s + a[n2_idx], i));
        }

        // 更新数值和下标
        a[i] = s;
        idx.remove(&nxt);
    }

    ans
}
