fn main() {
    println!("Hello, world!");

    another_function();
    function_with_argument(10);
    function_with_two_argument(10, 20);
    scope();

    let x = five();
    println!("The value of x is: {}", x);
    println!("The value of x + 1 is: {}", plus_one(x));
}

fn another_function() {
    println!("Another function!");
}

fn function_with_argument(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_two_argument(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn scope() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // if we put a ; here it turns into a statement
              // and doesn't return a value and will fail to compile
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 { 
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}