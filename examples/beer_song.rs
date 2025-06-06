fn main() {}

pub fn verse(n: i32) -> String {
    match n {
        0 =>  "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        n if n >2 && n <= 99 => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {y} bottles of beer on the wall.\n",n =n,y = n-1),
        _ => panic!()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    // let mut res: String = String::new();
    // let mut current = start;
    // while current >= end {
    //     res.push_str(verse(current).as_str());

    //     if current != end {
    //         res.push_str("\n");
    //     }
    //     current -= 1;
    // }
    // return res;
    (end..start + 1)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_verse_0() {
    assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_verse_1() {
    assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_verse_2() {
    assert_eq!(verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_verse_8() {
    assert_eq!(verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_song_8_6() {
    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_song_3_0() {
    assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
