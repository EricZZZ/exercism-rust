fn main() {
    println!(
        "{:?}",
        successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
    )
    // println!("{}", get_index(&vec![1, 2, 3, 4, 5], 5, 7));
}
pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut ans = vec![0; spells.len()];
    potions.sort_unstable();
    for (index, spell) in spells.iter().enumerate() {
        ans[index] = get_index(&potions, *spell, success);
    }
    ans
}

fn get_index(potions: &Vec<i32>, spell: i32, success: i64) -> i32 {
    let mut left = 0;
    let mut right = potions.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if (potions[mid] as i64 * spell as i64) < success {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    (potions.len() - left) as i32
}
