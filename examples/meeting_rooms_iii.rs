use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    println!(
        "{}",
        most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]])
    )
}

pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;

    meetings.sort_unstable_by_key(|m| m[0]);

    let mut idle = BinaryHeap::new();
    for i in 0..n {
        idle.push(Reverse(i));
    }

    let mut using = BinaryHeap::new();

    let mut cnt = vec![0; n];

    for m in meetings {
        let start = m[0] as i64;
        let mut end = m[1] as i64;

        while let Some(Reverse((finsh_time, room_idx))) = using.peek() {
            if *finsh_time <= start {
                idle.push(Reverse(*room_idx));
                using.pop();
            } else {
                break;
            }
        }

        let room_i;
        if let Some(Reverse(idx)) = idle.pop() {
            room_i = idx;
        } else {
            let Reverse((finish_time, idx)) = using.pop().unwrap();
            room_i = idx;
            end = finish_time + (end - start);
        }
        using.push(Reverse((end, room_i)));
        cnt[room_i] += 1;
    }
    let mut ans = 0;
    for i in 1..n {
        if cnt[i] > cnt[ans] {
            ans = i;
        }
    }
    ans as i32
}
