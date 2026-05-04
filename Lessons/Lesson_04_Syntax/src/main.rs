// Demo purpose: practice common Rust control flow.

fn main() {
    show_if_else(7);
    show_match("north");
    show_loop();
    show_while();
    show_for();
}

// -------------------------------------------------------

fn show_if_else(score: i32) {
    if score >= 10 {
        println!("if/else: high score");
    } else if score > 0 {
        println!("if/else: positive score");
    } else {
        println!("if/else: no score");
    }
}

// -------------------------------------------------------

fn show_match(direction: &str) {
    match direction {
        "north" => println!("match: moving up"),
        "south" => println!("match: moving down"),
        _ => println!("match: standing still"),
    }
}

// -------------------------------------------------------

fn show_loop() {
    let mut count = 0;
    loop {
        count += 1;
        println!("loop: {count}");

        if count == 2 {
            break;
        }
    }
}

// -------------------------------------------------------

fn show_while() {
    let mut fuel = 3;
    while fuel > 0 {
        println!("while: fuel {fuel}");
        fuel -= 1;
    }
}

// -------------------------------------------------------

fn show_for() {
    for index in 1..=3 {
        println!("for: item {index}");
    }
}
