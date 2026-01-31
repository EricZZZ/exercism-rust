fn main() {
    println!("{}", next_greatest_letter(vec!['c', 'f', 'j'], 'a'))
}

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut ans = letters[0];
    for l in &letters {
        if *l > target {
            ans = *l;
            break;
        }
    }
    ans
}
