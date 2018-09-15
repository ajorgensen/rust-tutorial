#[derive(Debug)]
struct User {
    username: String, // Using String over &str is deliberate. This ensures the struct owns the data
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other. width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let mut user = User {
        email: String::from("foo@bar.com"),
        username: String::from("foo"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {:?}", user);

    user.email = String::from("baz@bar.com");
    println!("User: {:?}", user);

    let other_user = build_user(String::from("other@bar.com"), String::from("other"));
    println!("Other User: {:?}", other_user);

    let user2 = User {
        email: String::from("foobar@bar.com"),
        username: String::from("foobar"),
        ..user // Copies all remaining fields from user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: {:?}", black);
    println!("Origin: {:?}", origin);

    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    let area = area(&rectangle);
    println!("Area of {:#?} is {}", rectangle, rectangle.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rec1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rec1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("Square: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    // If the field is the same name as the variable we
    // can omit the redundant key
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
