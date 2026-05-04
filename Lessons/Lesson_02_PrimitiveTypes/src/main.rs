// Demo purpose: log Rust's everyday value types.

fn main() {
    log_bool();
    log_char();
    log_i32();
    log_u32();
    log_f64();
    log_string();
    log_str_slice();
    log_tuple();
    log_array();
    log_vector();
}

// -------------------------------------------------------

fn log_bool() {
    let value: bool = true;
    println!("bool: {value}");
}

// -------------------------------------------------------

fn log_char() {
    let value: char = 'R';
    println!("char: {value}");
}

// -------------------------------------------------------

fn log_i32() {
    let value: i32 = -42;
    println!("i32: {value}");
}

// -------------------------------------------------------

fn log_u32() {
    let value: u32 = 42;
    println!("u32: {value}");
}

// -------------------------------------------------------

fn log_f64() {
    let value: f64 = 3.14;
    println!("f64: {value}");
}

// -------------------------------------------------------

fn log_string() {
    let value: String = String::from("owned string");
    println!("String: {value}");
}

// -------------------------------------------------------

fn log_str_slice() {
    let value: &str = "borrowed string slice";
    println!("&str: {value}");
}

// -------------------------------------------------------

fn log_tuple() {
    let value: (&str, i32) = ("level", 2);
    println!("tuple: {:?}", value);
}

// -------------------------------------------------------

fn log_array() {
    let value: [i32; 3] = [1, 2, 3];
    println!("array: {:?}", value);
}

// -------------------------------------------------------

fn log_vector() {
    let value: Vec<i32> = vec![1, 2, 3, 4];
    println!("Vec: {:?}", value);
}
