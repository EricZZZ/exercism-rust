use std::collections::HashMap;

fn main() {}

pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let mut map = [[0u32; 26]; 26];
    for s in allowed {
        let bytes = s.as_bytes();
        let l = (bytes[0] - b'A') as usize;
        let r = (bytes[1] - b'A') as usize;
        let t = (bytes[2] - b'A') as usize;
        map[l][r] |= 1 << t;
    }

    let mut memo = HashMap::new();

    let bottom_vec: Vec<usize> = bottom.bytes().map(|b| (b - b'A') as usize).collect();

    fn dfs(row: Vec<usize>, map: &[[u32; 26]; 26], memo: &mut HashMap<Vec<usize>, bool>) -> bool {
        if row.len() == 1 {
            return true;
        }
        if let Some(&res) = memo.get(&row) {
            return res;
        }

        let mut next_row_options = Vec::new();
        let res = backtrack(&row, 0, &mut next_row_options, map, memo);

        memo.insert(row, res);
        res
    }

    fn backtrack(
        current_row: &[usize],
        idx: usize,
        path: &mut Vec<usize>,
        map: &[[u32; 26]; 26],
        memo: &mut HashMap<Vec<usize>, bool>,
    ) -> bool {
        if idx == current_row.len() - 1 {
            return dfs(path.clone(), map, memo);
        }

        let l = current_row[idx];
        let r = current_row[idx + 1];
        let allowed_tops = map[l][r];

        for i in 0..26 {
            if (allowed_tops >> i) & 1 == 1 {
                path.push(i);
                if backtrack(current_row, idx + 1, path, map, memo) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }

    dfs(bottom_vec, &map, &mut memo)
}
