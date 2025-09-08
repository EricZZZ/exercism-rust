pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    let sqrt = (upper_bound as f64).sqrt() as usize;
    for i in 2..=sqrt {
        if is_prime[i] {
            for j in (i * i..=upper_bound as usize).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    let mut primes = Vec::new();
    is_prime
        .iter()
        .enumerate()
        .take(upper_bound as usize + 1)
        .skip(2)
        .for_each(|(i, &is_prime)| {
            if is_prime {
                primes.push(i as u64);
            }
        });
    primes
}

#[test]
fn limit_lower_than_the_first_prime() {
    assert_eq!(primes_up_to(1), []);
}

#[test]
//#[ignore]
fn limit_is_the_first_prime() {
    assert_eq!(primes_up_to(2), [2]);
}

#[test]
//#[ignore]
fn primes_up_to_10() {
    assert_eq!(primes_up_to(10), [2, 3, 5, 7]);
}

#[test]
//#[ignore]
fn limit_is_prime() {
    assert_eq!(primes_up_to(13), [2, 3, 5, 7, 11, 13]);
}

#[test]
//#[ignore]
fn limit_of_1000() {
    let expected = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];
    assert_eq!(primes_up_to(1000), expected);
}

fn main() {}
