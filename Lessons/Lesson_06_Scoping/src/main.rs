// Demo purpose: share values without losing control.

fn main() {
    show_scope();
    show_move();
    show_clone();
    show_borrow();
    show_mutable_borrow();
}

// -------------------------------------------------------

fn show_scope() {
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("scope: {outer} and {inner}");
    }
    println!("scope: {outer} remains");
}

// -------------------------------------------------------

fn show_move() {
    let message = String::from("moved value");
    take_ownership(message);
}

// -------------------------------------------------------

fn take_ownership(value: String) {
    println!("move: {value}");
}

// -------------------------------------------------------

fn show_clone() {
    let original = String::from("clone me");
    let copied = original.clone();

    println!("clone: original={original}, copied={copied}");
}

// -------------------------------------------------------

fn show_borrow() {
    let name = String::from("Ferris");
    print_borrowed(&name);
    println!("borrow: {name} is still usable");
}

// -------------------------------------------------------

fn print_borrowed(value: &str) {
    println!("borrow: {value}");
}

// -------------------------------------------------------

fn show_mutable_borrow() {
    let mut count = 1;
    add_one(&mut count);
    println!("mutable borrow: {count}");
}

// -------------------------------------------------------

fn add_one(value: &mut i32) {
    *value += 1;
}
