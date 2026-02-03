use std::collections::BTreeMap;

fn main() {}

pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
    let k = (k - 1) as usize; // k--，表示需要保留 k 个元素在 L 中

    let mut sum_l: i64 = nums[0] as i64;
    let mut l: BTreeMap<i32, i32> = BTreeMap::new();
    let mut r: BTreeMap<i32, i32> = BTreeMap::new();

    // 初始化窗口 [1, dist+2)
    for i in 1..=(dist as usize + 1).min(nums.len() - 1) {
        sum_l += nums[i] as i64;
        *l.entry(nums[i]).or_insert(0) += 1;
    }
    let mut size_l = (dist as usize + 1).min(nums.len() - 1);

    // 维护 L 的大小为 k
    while size_l > k {
        l2r(&mut l, &mut r, &mut sum_l, &mut size_l);
    }

    let mut ans = sum_l;

    // 滑动窗口
    for i in (dist as usize + 2)..nums.len() {
        let out = nums[i - dist as usize - 1];
        // 移除 out
        if l.contains_key(&out) {
            sum_l -= out as i64;
            size_l -= 1;
            remove_one(&mut l, out);
        } else {
            remove_one(&mut r, out);
        }

        // 添加 in
        let in_val = nums[i];
        let l_max = l.keys().next_back().copied().unwrap_or(i32::MIN);
        if in_val < l_max {
            sum_l += in_val as i64;
            size_l += 1;
            *l.entry(in_val).or_insert(0) += 1;
        } else {
            *r.entry(in_val).or_insert(0) += 1;
        }

        // 维护平衡
        if size_l == k.saturating_sub(1) && !r.is_empty() {
            r2l(&mut l, &mut r, &mut sum_l, &mut size_l);
        } else if size_l == k + 1 {
            l2r(&mut l, &mut r, &mut sum_l, &mut size_l);
        }

        ans = ans.min(sum_l);
    }

    ans
}

fn l2r(
    l: &mut BTreeMap<i32, i32>,
    r: &mut BTreeMap<i32, i32>,
    sum_l: &mut i64,
    size_l: &mut usize,
) {
    let x = *l.keys().next_back().unwrap();
    remove_one(l, x);
    *sum_l -= x as i64;
    *size_l -= 1;
    *r.entry(x).or_insert(0) += 1;
}

fn r2l(
    l: &mut BTreeMap<i32, i32>,
    r: &mut BTreeMap<i32, i32>,
    sum_l: &mut i64,
    size_l: &mut usize,
) {
    let x = *r.keys().next().unwrap();
    remove_one(r, x);
    *sum_l += x as i64;
    *size_l += 1;
    *l.entry(x).or_insert(0) += 1;
}

fn remove_one(m: &mut BTreeMap<i32, i32>, x: i32) {
    let cnt = *m.get(&x).unwrap();
    if cnt > 1 {
        m.insert(x, cnt - 1);
    } else {
        m.remove(&x);
    }
}
