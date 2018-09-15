fn main() {
    let s = String::from("Hello"); // s comes into scope

    // s's value moves into the functions...
    // ... and so is no longer valid here
    take_ownership(s);

    // This would throw a compilation error because s is 
    // no longer valid
    // println!("{}", s);

    // x comes into scope
    let x = 5;

    // x would move into the function, but i32 is Copy, so it's
    // okay to sill use x afterwards
    makes_copy(x);

    let s1 = gives_ownership(); // moves return value into s1
    let s2 = String::from("Hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back which also
                                       // moves its return value into s3
    
    println!("s1: {}", s1);
    // println!("s2: {}", s2); will crash since s2 is invalid
    println!("s3: {}", s3);

    let (s4, len) = calculate_length(s1);
    println!("s4: {}, len: {}", s4, len);
    println!("s4: {}, len: {}", s4, calculate_length_reference(&s4));

    let mut some_string = String::from("Hello");
    println!("Before: {}", some_string);
    change(&mut some_string);
    println!("After: {}", some_string);

    // This will fail since we can only have 1 mutable reference
    // let r1 = &mut some_string;
    // let r2 = &mut some_string;

    // This is ok because r1 goes out of scope and there
    // is no way to access it at the time r2 is created
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    // You can have as many immutable references as you'd like
    let r3 = &s;
    let r4 = &s;
    // This will fail because its already borrowed as immutable
    // let r5 = &mut s
} // x goes out of scope. s was moved so nothing special happens
// s3 goes out of scope and is dropped
// s2 goes out of scope but was moved so nothing happens
// s1 goes out of scope and is dropped

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // here, some_string goes out of scope and `drop` is called.
    // The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("Hello"); // some_string comes into scope
    some_string //some_string returned and moes out to the calling function
}

// Takes ownership but then gives it back as a return
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}