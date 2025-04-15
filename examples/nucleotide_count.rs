fn main() {}

use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if dna.contains('X') || nucleotide == 'X' {
        return Err('X');
    }
    let count = dna.chars().filter(|ch| *ch == nucleotide).count();
    Ok(count)
}

static VALID_NUCLEOTIDES: &str = "ACGT";
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // let mut map: HashMap<char, usize> = HashMap::with_capacity(4);
    // map.insert('A', 0);
    // map.insert('T', 0);
    // map.insert('C', 0);
    // map.insert('G', 0);

    let mut map: HashMap<char, usize> = VALID_NUCLEOTIDES.chars().map(|c| (c, 0)).collect();

    for key in dna.chars() {
        match key {
            'A' | 'T' | 'C' | 'G' => {
                *map.get_mut(&key).unwrap() += 1;
            }
            _ => return Err('X'),
        }
    }

    Ok(map)
}

#[test]
fn count_returns_result() {
    assert!(count('A', "").is_ok());
}

#[test]
//#[ignore]
fn test_count_empty() {
    assert_eq!(count('A', ""), Ok(0));
}

#[test]
//#[ignore]
fn count_invalid_nucleotide() {
    assert_eq!(count('X', "A"), Err('X'));
}

#[test]
//#[ignore]
fn count_invalid_dna() {
    assert_eq!(count('A', "AX"), Err('X'));
}

#[test]
//#[ignore]
fn test_count_repetitive_cytosine() {
    assert_eq!(count('C', "CCCCC"), Ok(5));
}

#[test]
//#[ignore]
fn test_count_only_thymine() {
    assert_eq!(count('T', "GGGGGTAACCCGG"), Ok(1));
}

#[test]
//#[ignore]
fn counts_returns_result() {
    assert!(nucleotide_counts("ACGT").is_ok());
}

#[test]
//#[ignore]
fn test_nucleotide_count_empty() {
    check_dna("", &[('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
}

#[test]
//#[ignore]
fn test_nucleotide_count_only_guanine() {
    check_dna("GGGGGGGG", &[('A', 0), ('T', 0), ('C', 0), ('G', 8)]);
}

#[test]
//#[ignore]
fn test_nucleotide_count_counts_all() {
    check_dna(
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAA\
        GAGTGTCTGATAGCAGC",
        &[('A', 20), ('T', 21), ('C', 12), ('G', 17)],
    );
}

#[test]
//#[ignore]
fn counts_invalid_nucleotide_results_in_err() {
    assert_eq!(nucleotide_counts("GGXXX"), Err('X'));
}
