#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Offline,
    Message,
    Other,
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "OFFLINE" => EventType::Offline,
            "MESSAGE" => EventType::Message,
            _ => EventType::Other,
        }
    }
}

fn main() {
    println!(
        "{:?}",
        count_mentions(
            2,
            vec![
                vec!["MESSAGE".to_string(), "70".to_string(), "HERE".to_string()],
                vec!["OFFLINE".to_string(), "10".to_string(), "0".to_string()],
                vec!["OFFLINE".to_string(), "71".to_string(), "0".to_string()],
            ]
        )
    )
}

pub fn count_mentions(number_of_users: i32, mut events: Vec<Vec<String>>) -> Vec<i32> {
    let mut online = vec![1_i32; number_of_users as usize];
    let mut offline = vec![0_i32; number_of_users as usize];
    let mut ans = vec![0; number_of_users as usize];

    events.sort_by(|a, b| {
        // 1. 第一优先级：按第二个元素（数字）从小到大排序
        let a_num = a[1].parse::<i32>().unwrap_or(0);
        let b_num = b[1].parse::<i32>().unwrap_or(0);
        let num_cmp = a_num.cmp(&b_num);

        if num_cmp != std::cmp::Ordering::Equal {
            return num_cmp;
        }

        // 2. 第二优先级：数字相同时，按类型排序
        // OFFLINE > MESSAGE > 其他
        let type_order = |s: &str| match s {
            "OFFLINE" => 0,
            "MESSAGE" => 1,
            _ => 2,
        };

        type_order(&a[0]).cmp(&type_order(&b[0]))
    });

    for event in events {
        let time = event[1].parse::<i32>().unwrap_or(0);
        for id in 0..offline.len() {
            if offline[id] > 0 && offline[id] <= time {
                offline[id] = 0;
                online[id] = 1;
            }
        }
        if event[0] == "MESSAGE" {
            if event[2] == "ALL" {
                for id in 0..online.len() {
                    if online[id] == 1 {
                        ans[id] += 1;
                    }
                }
                for id in 0..offline.len() {
                    if offline[id] > 0 {
                        ans[id] += 1;
                    }
                }
            } else if event[2] == "HERE" {
                for id in 0..online.len() {
                    if online[id] == 1 {
                        ans[id] += 1;
                    }
                }
            } else {
                for id in event[2].split_whitespace().filter_map(|s| {
                    // 移除 "id" 前缀
                    let num_str = s.trim_start_matches("id");
                    num_str.parse::<usize>().ok()
                }) {
                    ans[id] += 1;
                }
            }
        } else {
            let id = event[2].parse::<usize>().unwrap_or(0);
            let time = event[1].parse::<i32>().unwrap_or(0) + 60;
            offline[id] = time;
            online[id] = 0;
        }
    }
    ans
}

pub fn count_mentions_by_ai(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
    let mut online = vec![true; number_of_users as usize];
    let mut offline = vec![0; number_of_users as usize]; // 存储离线结束时间
    let mut ans = vec![0; number_of_users as usize];

    // 预处理事件：解析时间并按类型和时间排序
    let mut processed_events: Vec<(EventType, i32, String)> = events
        .into_iter()
        .map(|e| {
            let time = e[1].parse::<i32>().unwrap_or(0);
            (EventType::from(e[0].as_str()), time, e[2].clone())
        })
        .collect();

    processed_events.sort_by(|a, b| {
        a.1.cmp(&b.1) // 先按时间排序
            .then_with(|| a.0.cmp(&b.0)) // 时间相同则按类型排序
    });

    // 使用优先队列或类似结构跟踪即将恢复在线的用户
    let mut upcoming_online = Vec::new();

    for (event_type, time, target) in processed_events {
        // 处理即将恢复在线的用户
        upcoming_online.retain(|&(user_id, end_time)| {
            if end_time <= time {
                online[user_id] = true;
                offline[user_id] = 0;
                false // 移除已处理的用户
            } else {
                true // 保留尚未恢复在线的用户
            }
        });

        match event_type {
            EventType::Message => {
                if target == "ALL" {
                    // 所有在线和离线用户都收到消息
                    for (i, &is_online) in online.iter().enumerate() {
                        if is_online || offline[i] > 0 {
                            ans[i] += 1;
                        }
                    }
                } else if target == "HERE" {
                    // 只有在线用户收到消息
                    for (i, &is_online) in online.iter().enumerate() {
                        if is_online {
                            ans[i] += 1;
                        }
                    }
                } else {
                    // 特定用户收到消息
                    for id in target.split_whitespace().filter_map(|s| {
                        let num_str = s.trim_start_matches("id");
                        num_str.parse::<usize>().ok()
                    }) {
                        if id < ans.len() {
                            ans[id] += 1;
                        }
                    }
                }
            }
            EventType::Offline => {
                if let Ok(id) = target.parse::<usize>() {
                    if id < online.len() {
                        let end_time = time + 60;
                        online[id] = false;
                        offline[id] = end_time;
                        upcoming_online.push((id, end_time));
                    }
                }
            }
            EventType::Other => {} // 忽略其他类型事件
        }
    }

    ans
}
