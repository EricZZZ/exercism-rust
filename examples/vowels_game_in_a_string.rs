pub fn does_alice_win(s: String) -> bool {
    // s.chars().filter(|&c| "aeiou".contains(c)).count() > 0
    s.bytes().any(|c| "aeiou".contains(c as char))
}

fn main() {}
