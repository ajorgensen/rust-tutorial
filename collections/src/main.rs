use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    println!("v: {:?}", v);
    println!("v2: {:?}", v2);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v: {:?}", v);

    let v3 = vec![1, 2, 3, 4, 5,];
    let third: &i32 = &v3[2];
    println!("v3: {:?}", v3);
    println!("third: {}", third);

    let v_idx = 6;
    match v.get(v_idx){
        Some(_) => { println!("Reachable element at index: {}", v_idx); },
        None => { println!("Unreachable element at index: {}", v_idx); },
    }

    /*
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    v.push(6);
    // This will error since we have an immutable borrow from &v[0];
    */

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![100, 32, 57];
    println!("v5: {:?}", v5);
    for i in &mut v5 {
        *i += 50;
    }
    println!("v5: {:?}", v5);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;

    println!("{}", tic_tac_toe);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    println!("{}", format!("{}-{}-{}", tic, tac, toe));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    {
        let score = scores.get(&String::from("Blue"));
        println!("score: {:?}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(25);
    println!("scores: {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("words: {:?}", map);
} // When v goes out of scope the values are dropped as well
