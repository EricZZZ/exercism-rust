fn main() {
    println!(
        "{}",
        maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ])
    )
}

pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let mut st = vec![-1];
        let mut ans = 0;
        for (right, &h) in heights.iter().enumerate() {
            let right = right as i32;
            while st.len() > 1 && heights[*st.last().unwrap() as usize] >= h {
                let i = st.pop().unwrap() as usize;
                let left = *st.last().unwrap();
                ans = ans.max(heights[i] * (right - left - 1));
            }
            st.push(right);
        }
        ans
    }
    let n = matrix[0].len();
    let mut heights = vec![0; n + 1];
    let mut ans = 0;
    for row in matrix {
        for (j, c) in row.into_iter().enumerate() {
            if c == '0' {
                heights[j] = 0;
            } else {
                heights[j] += 1;
            }
        }
        ans = ans.max(largest_rectangle_area(&heights))
    }
    ans
}
