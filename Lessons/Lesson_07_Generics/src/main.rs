// Demo purpose: reuse code across many types.

use std::fmt::{Debug, Display};

fn main() {
    print_value("number", 42);
    print_value("word", "rust");

    let numbers = Pair::new(3, 7);
    let words = Pair::new("left", "right");

    print_pair("numbers", &numbers);
    print_pair("words", &words);
    print_display_pair("words display", &words);
    print_larger(10, 20);
}

// -------------------------------------------------------

fn print_value<T: Display>(label: &str, value: T) {
    println!("{label}: {value}");
}

// -------------------------------------------------------

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

fn print_pair<T: Debug>(label: &str, pair: &Pair<T>) {
    println!("{label}: {:?}", pair);
}

// -------------------------------------------------------

impl<T: Display> Pair<T> {
    fn describe(&self) -> String {
        format!("{} and {}", self.first, self.second)
    }
}

fn print_display_pair<T: Display>(label: &str, pair: &Pair<T>) {
    println!("{label}: {}", pair.describe());
}

// -------------------------------------------------------

fn print_larger<T>(left: T, right: T)
where
    T: PartialOrd + Display,
{
    if left >= right {
        println!("larger: {left}");
    } else {
        println!("larger: {right}");
    }
}
