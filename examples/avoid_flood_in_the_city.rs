use std::collections::{BTreeSet, HashMap};
use std::ops::Bound::{Included, Unbounded};

fn main() {
    println!("{:?}", avoid_flood(vec![1, 2, 3, 4]))
}

pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![1; rains.len()];
    let mut dry_days = BTreeSet::new();
    let mut last_rain_day = HashMap::new();
    for i in 0..rains.len() {
        if rains[i] == 0 {
            dry_days.insert(i);
        } else {
            ans[i] = -1;
            if let Some(&prev_rain_day) = last_rain_day.get(&rains[i]) {
                let dry_day_iter = dry_days.range((Included(prev_rain_day), Unbounded));
                let next_dry_day = dry_day_iter.into_iter().next();
                match next_dry_day {
                    Some(&dry_day_index) => {
                        ans[dry_day_index] = rains[i];
                        dry_days.remove(&dry_day_index);
                    }
                    None => {
                        return vec![];
                    }
                }
            }
            last_rain_day.insert(rains[i], i);
        }
    }
    ans
}
