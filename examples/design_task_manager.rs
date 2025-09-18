use std::collections::{BinaryHeap, HashMap};

fn main() {}

#[derive(PartialEq, Eq)]
struct Task {
    task_id: i32,
    priority: i32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.task_id.cmp(&other.task_id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

/**
 * BinaryHeap 实现优先队列。其设计目标是快速访问和移除最大（或最小）的元素。
 */
struct TaskManager {
    task_info: HashMap<i32, (i32, i32)>,
    heap: BinaryHeap<Task>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_info = HashMap::new();
        let mut heap = BinaryHeap::new();

        for task in tasks {
            let user_id = task[0];
            let task_id = task[1];
            let priority = task[2];
            task_info.insert(task_id, (user_id, priority));
            heap.push(Task { task_id, priority });
        }
        TaskManager { task_info, heap }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_info.insert(task_id, (user_id, priority));
        self.heap.push(Task { task_id, priority });
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(info) = self.task_info.get_mut(&task_id) {
            info.1 = new_priority;
            self.heap.push(Task {
                task_id,
                priority: new_priority,
            });
        }
    }

    fn rmv(&mut self, task_id: i32) {
        self.task_info.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        while let Some(task) = self.heap.pop() {
            if let Some(&(user_id, priority)) = self.task_info.get(&task.task_id) {
                if priority == task.priority {
                    self.task_info.remove(&task.task_id);
                    return user_id;
                }
            }
        }
        -1
    }
}
