pub fn max_freq_sum(s: String) -> i32 {
    let mut vowel_count = 0;
    let mut consonant_count = 0;
    let mut v = [0; 26];
    for c in s.bytes() {
        v[(c - 97) as usize] += 1;
    }
    for (i, _) in v.iter().enumerate() {
        if i == 0 || i == 4 || i == 8 || i == 14 || i == 20 {
            vowel_count = vowel_count.max(v[i]);
        } else {
            consonant_count = consonant_count.max(v[i]);
        }
    }
    vowel_count + consonant_count
}

fn main() {}
