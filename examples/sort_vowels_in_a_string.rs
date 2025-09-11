pub fn sort_vowels(s: String) -> String {
    let mut vowels = s
        .bytes()
        .filter(|&c| "aeiouAEIOU".contains(c as char))
        .collect::<Vec<_>>();

    vowels.sort_unstable();
    let mut vowel_index = 0;

    let mut s = s.into_bytes();
    for ch in s.iter_mut() {
        if "aeiouAEIOU".contains(*ch as char) {
            *ch = vowels[vowel_index];
            vowel_index += 1;
        }
    }

    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {}
