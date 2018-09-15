fn main() {
    let s = String::from("Hello world!");
    let idx = first_word(&s);
    println!("First Word Idx: {}", idx);

    string_slice();

    let first_word = first_word_slice(&s);
    println!("First Word: {}", first_word);
}

// Tedious to have to manage the index since if s is changed in
// any way then the index is invalid. s.clear() for example
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn string_slice() {
    let s = String::from("Hello world!");
    // Reference to a portion of the string
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("s: {}, hello: {}, world: {}", s, hello, world);
}