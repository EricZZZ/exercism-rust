fn main() {
    let count = DNA::new("C").unwrap().to_rna();
    println!("{:?}", count)
}

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut out = String::new();
        for (index, ch) in dna.chars().enumerate() {
            if !(ch == 'G' || ch == 'C' || ch == 'T' || ch == 'A') {
                return Err(index);
            } else {
                out.push(ch);
            }
        }
        Ok(DNA(out))
    }

    pub fn to_rna(self) -> RNA {
        let chang: String = self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                _ => 'U',
            })
            .collect();
        RNA(chang)
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut out = String::new();
        for (index, ch) in rna.chars().enumerate() {
            if !(ch == 'G' || ch == 'C' || ch == 'U' || ch == 'A') {
                return Err(index);
            } else {
                out.push(ch);
            }
        }
        Ok(RNA(out))
    }
}

#[test]
fn test_valid_dna_input() {
    assert!(DNA::new("GCTA").is_ok());
}

#[test]
//#[ignore]
fn test_valid_rna_input() {
    assert!(RNA::new("CGAU").is_ok());
}

#[test]
//#[ignore]
fn test_invalid_dna_input() {
    // Invalid character
    assert_eq!(DNA::new("X").err(), Some(0));
    // Valid nucleotide, but invalid in context
    assert_eq!(DNA::new("U").err(), Some(0));
    // Longer string with contained errors
    assert_eq!(DNA::new("ACGTUXXCTTAA").err(), Some(4));
}

#[test]
//#[ignore]
fn test_invalid_rna_input() {
    // Invalid character
    assert!(RNA::new("X").is_err());
    // Valid nucleotide, but invalid in context
    assert!(RNA::new("T").is_err());
    // Longer string with contained errors
    assert!(RNA::new("ACGUTTXCUUAA").is_err());
}

#[test]
//#[ignore]
fn test_acid_equals_acid() {
    assert_eq!(DNA::new("CGA").unwrap(), DNA::new("CGA").unwrap());
    assert_ne!(DNA::new("CGA").unwrap(), DNA::new("AGC").unwrap());
    assert_eq!(RNA::new("CGA").unwrap(), RNA::new("CGA").unwrap());
    assert_ne!(RNA::new("CGA").unwrap(), RNA::new("AGC").unwrap());
}

#[test]
//#[ignore]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(RNA::new("G").unwrap(), DNA::new("C").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(RNA::new("C").unwrap(), DNA::new("G").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_adenine_uracil() {
    assert_eq!(RNA::new("U").unwrap(), DNA::new("A").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(RNA::new("A").unwrap(), DNA::new("T").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(
        RNA::new("UGCACCAGAAUU").unwrap(),
        DNA::new("ACGTGGTCTTAA").unwrap().to_rna()
    )
}
