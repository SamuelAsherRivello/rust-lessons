// Demo purpose: add metadata and code shortcuts.

fn main() {
    show_derived_attributes();
    print_platform();
}

// -------------------------------------------------------

#[derive(Debug, Clone)]
struct Badge {
    name: String,
    level: u32,
}

macro_rules! log_lesson {
    ($label:expr, $value:expr) => {
        println!("{}: {:?}", $label, $value);
    };
}

fn show_derived_attributes() {
    let badge = Badge {
        name: String::from("macro learner"),
        level: 9,
    };
    let copied_badge = badge.clone();

    log_lesson!("badge", badge);
    log_lesson!("copied badge", copied_badge);
    println!(
        "badge details: {} level {}",
        copied_badge.name, copied_badge.level
    );
}

// -------------------------------------------------------

#[cfg(target_os = "windows")]
fn print_platform() {
    println!("cfg: running on Windows");
}

#[cfg(not(target_os = "windows"))]
fn print_platform() {
    println!("cfg: running outside Windows");
}
