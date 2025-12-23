fn main() {}

struct Event {
    ts: i32,
    op: i32,
    val: i32,
}

pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    let mut evs = Vec::new();
    for event in events {
        evs.push(Event {
            ts: event[0],
            op: 0,
            val: event[2],
        });
        evs.push(Event {
            ts: event[1],
            op: 1,
            val: event[2],
        });
    }
    evs.sort_by(|a, b| {
        if a.ts != b.ts {
            a.ts.cmp(&b.ts)
        } else {
            a.op.cmp(&b.op)
        }
    });

    let mut ans = 0;
    let mut best_first = 0;
    for e in evs {
        if e.op == 0 {
            ans = ans.max(e.val + best_first);
        } else {
            best_first = best_first.max(e.val);
        }
    }
    ans
}
