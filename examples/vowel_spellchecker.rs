use std::collections::{HashMap, HashSet};

pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    fn replace_vowels(s: String) -> String {
        s.clone()
            .chars()
            .map(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '?',
                _ => c,
            })
            .collect::<String>()
    }

    let mut orign = HashSet::with_capacity(wordlist.len());
    let mut lower_map = HashMap::with_capacity(wordlist.len());
    let mut vowel_map = HashMap::with_capacity(wordlist.len());

    for s in wordlist.into_iter().rev() {
        orign.insert(s.clone());
        let lower_s = s.to_lowercase();
        lower_map.insert(lower_s.clone(), s.clone());
        vowel_map.insert(replace_vowels(lower_s), s.clone());
    }

    queries
        .into_iter()
        .map(|q| {
            if orign.contains(&q) {
                return q;
            }
            let lower = q.to_lowercase();
            if let Some(s) = lower_map.get(&lower) {
                s.clone()
            } else if let Some(s) = vowel_map.get(&replace_vowels(lower)) {
                s.clone()
            } else {
                "".to_string()
            }
        })
        .collect()
}

fn main() {}
