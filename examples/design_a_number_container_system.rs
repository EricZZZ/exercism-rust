use std::collections::{BTreeSet, HashMap};

fn main() {}

/**
 * 关键点就在这个有序序列数据结构上，插入新值时，自动排序省去很多事情
 * BTreeSet
 */
struct NumberContainers {
    nums: HashMap<i32, i32>,
    indexs: HashMap<i32, BTreeSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            nums: HashMap::new(),
            indexs: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&pre) = self.nums.get(&index) {
            if pre != 0 {
                if let Some(set) = self.indexs.get_mut(&pre) {
                    set.remove(&index);
                }
            }
        }
        self.nums.insert(index, number);
        self.indexs.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        self.indexs
            .get(&number)
            .and_then(|s| s.iter().next().copied())
            .unwrap_or(-1)
    }
}
