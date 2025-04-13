fn main() {}

fn ascii(ch: char) -> u8 {
    ch as u8
}

fn get_transpose(ch: char) -> char {
    if ch.is_ascii_digit() {
        ch
    } else {
        (ascii('z') - ascii(ch) + ascii('a')) as char
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|&ch| ch.is_ascii())
        .filter(|&ch| ch.is_alphanumeric())
        .map(get_transpose)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|slice| slice.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    // cipher.trim().chars().map(get_transpose).collect::<String>()
    cipher
        .split_whitespace()
        .collect::<String>()
        .chars()
        .map(get_transpose)
        .collect::<String>()
}

#[test]
fn test_encode_yes() {
    assert_eq!("bvh", encode("yes"));
}

#[test]
//#[ignore]
fn test_encode_no() {
    assert_eq!("ml", encode("no"));
}

#[test]
//#[ignore]
fn test_encode_omg() {
    assert_eq!("lnt", encode("OMG"));
}

#[test]
//#[ignore]
fn test_encode_spaces() {
    assert_eq!("lnt", encode("O M G"));
}

#[test]
//#[ignore]
fn test_encode_mindblowingly() {
    assert_eq!("nrmwy oldrm tob", encode("mindblowingly"));
}

#[test]
//#[ignore]
fn test_encode_numbers() {
    assert_eq!("gvhgr mt123 gvhgr mt", encode("Testing,1 2 3, testing."));
}

#[test]
//#[ignore]
fn test_encode_deep_thought() {
    assert_eq!("gifgs rhurx grlm", encode("Truth is fiction."));
}

#[test]
//#[ignore]
fn test_encode_all_the_letters() {
    assert_eq!(
        "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt",
        encode("The quick brown fox jumps over the lazy dog.")
    );
}

#[test]
//#[ignore]
fn test_encode_ignores_non_ascii() {
    assert_eq!("mlmzh xrrrt mlivw", encode("non ascii éignored"));
}

#[test]
//#[ignore]
fn test_decode_exercism() {
    assert_eq!("exercism", decode("vcvix rhn"));
}

#[test]
//#[ignore]
fn test_decode_a_sentence() {
    assert_eq!(
        "anobstacleisoftenasteppingstone",
        decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v")
    );
}

#[test]
//#[ignore]
fn test_decode_numbers() {
    assert_eq!("testing123testing", decode("gvhgr mt123 gvhgr mt"));
}

#[test]
//#[ignore]
fn test_decode_all_the_letters() {
    assert_eq!(
        "thequickbrownfoxjumpsoverthelazydog",
        decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt")
    );
}
