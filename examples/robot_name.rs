use rand::Rng;

pub struct Robot {
    name: String,
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_name() -> String {
    let mut rng = rand::rng();

    let ramdom_num = rng.random_range(0..1000);
    let random_letter = rng.random_range(0..26);
    let random_letter2 = rng.random_range(0..26);
    let random_letter = (random_letter as u8 + b'A') as char;
    let random_letter2 = (random_letter2 as u8 + b'A') as char;
    format!("{}{}{:03}", random_letter, random_letter2, ramdom_num)
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }
}

#[allow(dead_code)]
fn assert_name_matches_pattern(n: &str) {
    println!("name:{}", n);
    assert!(n.len() == 5, "name is exactly 5 characters long");
    assert!(
        n[0..2].chars().all(|c: char| c.is_ascii_uppercase()),
        "name starts with 2 uppercase letters"
    );
    assert!(
        n[2..].chars().all(|c: char| c.is_ascii_digit()),
        "name ends with 3 numbers"
    );
}
#[allow(dead_code)]
fn assert_name_is_persistent(r: &Robot) {
    // The type system already proves this, but why not.
    let n1 = r.name();
    let n2 = r.name();
    let n3 = r.name();
    assert_eq!(n1, n2);
    assert_eq!(n2, n3);
}

#[test]
fn test_name_should_match_expected_pattern() {
    let r = Robot::new();
    assert_name_matches_pattern(r.name());
}

#[test]
//#[ignore]
fn test_name_is_persistent() {
    assert_name_is_persistent(&Robot::new());
}

#[test]
//#[ignore]
fn test_different_robots_have_different_names() {
    let r1 = Robot::new();
    let r2 = Robot::new();
    assert_ne!(r1.name(), r2.name(), "Robot names should be different");
}

#[test]
//#[ignore]
fn test_new_name_should_match_expected_pattern() {
    let mut r = Robot::new();
    assert_name_matches_pattern(r.name());
    r.reset_name();
    assert_name_matches_pattern(r.name());
}

#[test]
//#[ignore]
fn test_new_name_is_persistent() {
    let mut r = Robot::new();
    r.reset_name();
    assert_name_is_persistent(&r);
}

#[test]
//#[ignore]
fn test_new_name_is_different_from_old_name() {
    let mut r = Robot::new();
    let n1 = r.name().to_string();
    r.reset_name();
    let n2 = r.name().to_string();
    assert_ne!(n1, n2, "Robot name should change when reset");
}

fn main() {}
