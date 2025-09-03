use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::rng();
    rng.random_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

pub fn modular_exponentiation(base: u64, ex: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut ex = ex;
    let mut b = base;
    while ex > 0 {
        if (ex & 1) == 1 {
            result = (result * b) % modulus;
        }
        ex >>= 1;
        b = (b * b) % modulus;
    }
    result
}

#[test]
fn test_private_key_in_range_key() {
    let primes: Vec<u64> = vec![
        5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
    ];
    let private_keys: Vec<u64> = primes.iter().map(|x| private_key(*x)).collect();

    for i in 0..primes.len() {
        assert!(1 < private_keys[i] && private_keys[i] < primes[i]);
    }
}

#[test]
//#[ignore]
fn test_public_key_correct() {
    let p: u64 = 23;
    let g: u64 = 5;

    let private_key: u64 = 6;
    let expected: u64 = 8;

    assert_eq!(public_key(p, g, private_key), expected);
}

#[test]
//#[ignore]
fn test_secret_key_correct() {
    let p: u64 = 11;

    let private_key_a = 7;
    let public_key_b = 8;
    let secret = secret(p, public_key_b, private_key_a);
    let expected = 2;

    assert_eq!(secret, expected);
}

#[test]
//#[ignore]
fn test_public_key_correct_big_numbers() {
    let p: u64 = 4_294_967_299;

    let g: u64 = 8;

    let private_key: u64 = 4_294_967_296;

    let expected: u64 = 4096;

    assert_eq!(public_key(p, g, private_key), expected);
}

#[test]
//#[ignore]
fn test_secret_key_correct_big_numbers() {
    let p: u64 = 4_294_967_927;

    let private_key_a = 4_294_967_300;

    let public_key_b = 843;

    let secret = secret(p, public_key_b, private_key_a);

    let expected = 1_389_354_282;

    assert_eq!(secret, expected);
}

#[test]
//#[ignore]
fn test_changed_secret_key() {
    let p: u64 = 13;
    let g: u64 = 11;

    let private_key_a = private_key(p);
    let private_key_b = private_key(p);

    let public_key_a = public_key(p, g, private_key_a);
    let public_key_b = public_key(p, g, private_key_b);

    // Key exchange
    let secret_a = secret(p, public_key_b, private_key_a);
    let secret_b = secret(p, public_key_a, private_key_b);

    assert_eq!(secret_a, secret_b);
}

fn main() {}
