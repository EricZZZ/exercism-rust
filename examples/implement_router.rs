use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut obj = Router::new(2);
    let ret_1: bool = obj.add_packet(1, 4, 90);
    let ret_2: bool = obj.add_packet(2, 5, 90);
    let ret_1: bool = obj.add_packet(1, 4, 90);
    let ret_2: bool = obj.add_packet(3, 5, 95);
    let ret_2: bool = obj.add_packet(4, 5, 105);
    println!("queue:{:?}", obj.queue);
    let v = obj.forward_packet();
    println!("{:?}", v);
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
struct Router {
    memory_limit: usize,
    queue: VecDeque<(i32, i32, i32)>,
    set: HashSet<(i32, i32, i32)>,
    dest_to_timestamps: HashMap<i32, (Vec<i32>, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            memory_limit: memory_limit as usize,
            queue: VecDeque::new(),
            set: HashSet::new(),
            dest_to_timestamps: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let data = (source, destination, timestamp);
        if !self.set.insert(data) {
            return false;
        }
        if self.queue.len() == self.memory_limit {
            self.forward_packet();
        }
        self.queue.push_back(data);
        // self.dest_to_timestamps
        //     .entry(destination)
        //     .or_insert_with(|| (vec![], 0))
        //     .0
        //     .push(timestamp);

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(packet) = self.queue.pop_front() {
            self.set.remove(&packet);
            let (source, destination, timestamp) = packet;
            // self.dest_to_timestamps.get_mut(&destination).unwrap().1 += 1; // 队首下标加一，模拟出队
            vec![source, destination, timestamp]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        // if let Some((timestamps, head)) = self.dest_to_timestamps.get(&destination) {
        //     let left = timestamps[*head..].partition_point(|&x| x < start_time);
        //     let right = timestamps[*head..].partition_point(|&x| x <= end_time);
        //     (right - left) as _
        // } else {
        //     0
        // }
        let mut count = 0;
        for data in self.queue.iter() {
            if data.1 == destination && (data.2 >= start_time && data.2 <= end_time) {
                count += 1;
            }
        }
        count
    }
}
