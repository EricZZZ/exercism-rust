fn main() {
    println!(
        "{}",
        count_permutations(vec![
            3470, 9997, 31628, 55082, 43915, 14129, 49183, 99427, 71495, 31577, 74287, 96625,
            55548, 56522, 45502, 20407, 24812, 48500, 48200, 57692, 20660, 50351, 29619, 88947,
            65929, 3471, 37527, 17931, 62499, 69198, 45737, 38605, 94241, 68261, 61653, 97116,
            97217, 79694, 58943, 54248, 24192, 39712, 34398, 84847, 89541, 90309, 17607, 67739,
            84663, 35600
        ])
    )
}

pub fn count_permutations(complexity: Vec<i32>) -> i32 {
    fn factorial_fold(n: u64) -> u64 {
        (1..=n).fold(1, |acc, x| acc * x % 1_000_000_007)
    }

    for i in 1..complexity.len() {
        if complexity[0] >= complexity[i] {
            return 0;
        }
    }
    factorial_fold(complexity.len() as u64 - 1) as i32
}
