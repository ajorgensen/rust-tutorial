#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // default impl
    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

/*
pub fn notify<T>(item: T)
    where T: Summary
{

}
*/

/*
// Implement a trait for any type that implements another trait
impl <T: Display> ToString for T {

}
*/

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
            hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!(
        "The largest number in {:?} is {}",
        number_list,
        largest(&number_list)
    );

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!(
        "The largest number in {:?} is {}",
        number_list,
        largest(&number_list)
    );

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!(
        "The largest char in {:?} is {}",
        char_list,
        largest(&char_list)
    );

    println!("Point: {:?}", Point { x: 10, y: 10.5 });
    println!("Point: {:?}", Point { x: 10, y: 10.5 }.x());

    let tweet = Tweet {
        username: String::from("testing"),
        content: String::from("hello world"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
