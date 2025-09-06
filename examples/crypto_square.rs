pub fn encrypt(input: &str) -> String {
    // let input = input.to_lowercase();
    // let input = input
    //     .chars()
    //     .filter(|ch| ch.is_alphabetic())
    //     .collect::<String>();
    // let len = input.len();
    // let x = (len as f64).sqrt().ceil() as usize;
    // let col;
    // let row;
    // if x * x == len {
    //     col = x;
    //     row = x;
    // } else {
    //     col = x + 1;
    //     row = x;
    // }

    // let mut res: Vec<Vec<char>> = vec![vec![' '; col]; row];
    // for i in 0..len {
    //     res[i / col][i % col] = input.chars().nth(i).unwrap();
    // }

    // let mut new_res = String::new();
    // for i in 0..col * row {
    //     if i % col == 0 && i != 0 {
    //         new_res.push(' ');
    //     }
    //     new_res.push(res[i / col][i % col]);
    // }
    // new_res
    // Normalize the input string
    // Normalize the input string
    let normalized_s: String = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if normalized_s.is_empty() {
        return String::new();
    }

    let len = normalized_s.len();
    let c = (len as f64).sqrt().ceil() as usize;
    let r = (len as f64).sqrt().floor() as usize;

    let mut rows: Vec<String> = Vec::new();

    // Create the rows
    for i in 0..c {
        let start = i * r;
        let end = (i + 1) * r;
        let mut row_str = String::new();
        if start < len {
            row_str.push_str(&normalized_s[start..std::cmp::min(end, len)]);
        }
        // Pad with spaces if the last row is shorter than r
        while row_str.len() < r {
            row_str.push(' ');
        }
        rows.push(row_str);
    }

    // Read columns to get the encoded message
    let mut encoded_string = String::new();
    for i in 0..r {
        let mut column_string = String::new();
        for row in &rows {
            column_string.push(row.chars().nth(i).unwrap());
        }
        encoded_string.push_str(&column_string);
        encoded_string.push(' ');
    }

    encoded_string.trim_end().to_string()
}

#[allow(dead_code)]
fn test(input: &str, output: &str) {
    assert_eq!(&encrypt(input), output);
}

#[test]
fn test_empty_input() {
    test("", "")
}

#[test]
//#[ignore]
fn test_encrypt_also_decrypts_square() {
    // note that you only get the exact input back if:
    // 1. no punctuation
    // 2. even spacing
    // 3. all lowercase
    // 4. square input
    let example = "lime anda coco anut";
    assert_eq!(example, &encrypt(&encrypt(example)));
}

#[test]
//#[ignore]
fn test_example() {
    test(
        "If man was meant to stay on the ground, god would have given us roots.",
        "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau ",
    )
}

#[test]
//#[ignore]
fn test_empty_last_line() {
    test("congratulate", "crl oaa ntt gue")
}

#[test]
//#[ignore]
fn test_spaces_are_reorganized() {
    test("abet", "ae bt");
    test("a bet", "ae bt");
    test("     a  b     e      t             ", "ae bt");
}

#[test]
//#[ignore]
fn test_everything_becomes_lowercase() {
    test("caSe", "cs ae");
    test("cAsE", "cs ae");
    test("CASE", "cs ae");
}

#[test]
//#[ignore]
fn test_long() {
    test(
        r#"
We choose to go to the moon.

We choose to go to the moon in this decade and do the other things,
not because they are easy, but because they are hard, because that
goal will serve to organize and measure the best of our energies and
skills, because that challenge is one that we are willing to accept,
one we are unwilling to postpone, and one which we intend to win,
and the others, too.

-- John F. Kennedy, 12 September 1962
       "#,
        &(String::from("womdbudlmecsgwdwob enooetbsenaotioihe ")
            + "cwotcbeeaeunolnnnr henhaecrsrsealeaf1 ocieucavugetciwnk9 "
            + "ohnosauerithcnhde6 sotteusteehaegitn2 eohhtseotsatptchn  "
            + "tsiehetohatwtohee  oesrethrenceopwod  gtdtyhagbdhanoety  "
            + "ooehaetaesaresih1  tgcirygnsklewtne2  ooaneaoitilweptrs  "
            + "ttdgerazoleiaoese  hoesaeleflnlrnntp  etanshwaosgleedot  "
            + "mhnoyainubeiuatoe  oedtbrldreinnnojm "),
    )
}
fn main() {}
