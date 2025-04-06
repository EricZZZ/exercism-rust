pub fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello());
}

fn main() {
    println!("hello")
}
