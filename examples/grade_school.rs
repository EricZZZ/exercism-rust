use std::collections::HashMap;

fn main() {}
#[derive(Default)]
pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.grades.entry(grade).or_default();
        entry.push(student.to_string());
        entry.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut s = self.grades.keys().cloned().collect::<Vec<u32>>();
        s.sort();
        s
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|v| v.to_vec())
    }
}

fn some_strings(v: &[&str]) -> Option<Vec<String>> {
    Some(v.iter().map(|s| s.to_string()).collect())
}

#[test]
fn test_grades_for_empty_school() {
    let s = School::new();
    assert_eq!(s.grades(), vec![]);
}

#[test]
//#[ignore]
fn test_grades_for_one_student() {
    let mut s = School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grades(), vec![2]);
}

#[test]
//#[ignore]
fn test_grades_for_several_students_are_sorted() {
    let mut s = School::new();
    s.add(2, "Aimee");
    s.add(7, "Logan");
    s.add(4, "Blair");
    assert_eq!(s.grades(), vec![2, 4, 7]);
}

#[test]
//#[ignore]
fn test_grades_when_several_students_have_the_same_grade() {
    let mut s = School::new();
    s.add(2, "Aimee");
    s.add(2, "Logan");
    s.add(2, "Blair");
    assert_eq!(s.grades(), vec![2]);
}

#[test]
//#[ignore]
fn test_grade_for_empty_school() {
    let s = School::new();
    assert_eq!(s.grade(1), None);
}

#[test]
//#[ignore]
fn test_grade_when_no_students_have_that_grade() {
    let mut s = School::new();
    s.add(7, "Logan");
    assert_eq!(s.grade(1), None);
}

#[test]
//#[ignore]
fn test_grade_for_one_student() {
    let mut s = School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grade(2), some_strings(&["Aimee"]));
}

#[test]
//#[ignore]
fn test_grade_returns_students_sorted_by_name() {
    let mut s = School::new();
    s.add(2, "James");
    s.add(2, "Blair");
    s.add(2, "Paul");
    assert_eq!(s.grade(2), some_strings(&["Blair", "James", "Paul"]));
}

#[test]
//#[ignore]
fn test_add_students_to_different_grades() {
    let mut s = School::new();
    s.add(3, "Chelsea");
    s.add(7, "Logan");
    assert_eq!(s.grades(), vec![3, 7]);
    assert_eq!(s.grade(3), some_strings(&["Chelsea"]));
    assert_eq!(s.grade(7), some_strings(&["Logan"]));
}
