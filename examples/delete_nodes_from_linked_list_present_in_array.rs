use std::collections::HashSet;

fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut set = HashSet::new();
    for num in nums {
        set.insert(num);
    }

    // 哨兵
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev = &mut dummy;

    while let Some(ref mut current) = prev.next {
        if set.contains(&current.val) {
            // remove current by linking prev.next to current.next
            prev.next = current.next.take();
        } else {
            // advance prev
            prev = prev.next.as_mut().unwrap();
        }
    }

    dummy.next
}
