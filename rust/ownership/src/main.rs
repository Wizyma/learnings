fn main() {
    let s = String::from("hello");  // s comes into scope
    let s2 = String::from("Calculate");
    takes_ownership(s); // s's value moves into the function...
     // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

    let (s2_borrowed, lenght) = calculate_length(s2);

    println!("string = {}, length = {}", s2_borrowed, lenght);

    let mut another_string = String::from("HELL YEAH!");

    let word = first_word(&another_string);

    // another_string.clear(); Do not work anymore because of borrwoed value that is referenced by word

    println!("{}, word = {}", another_string, word);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}