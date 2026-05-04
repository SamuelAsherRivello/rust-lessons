// Demo purpose: check small functions with tests.
// Run tests: D:\Documents\Projects\VC\Rust\rust-lessons\Scripts\Other\RunTests.ps1

fn main() {
    let total = add_points(10, 5);
    let label = score_label(total);

    println!("points: {total}");
    println!("label: {label}");
}

// -------------------------------------------------------

fn add_points(current: u32, bonus: u32) -> u32 {
    current + bonus
}

// -------------------------------------------------------

fn score_label(score: u32) -> &'static str {
    if score >= 10 { "winner" } else { "learning" }
}

// -------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_points_returns_sum() {
        assert_eq!(add_points(2, 3), 5);
    }

    #[test]
    fn score_label_marks_winner() {
        assert_eq!(score_label(10), "winner");
    }

    #[test]
    fn score_label_marks_learning() {
        assert_eq!(score_label(9), "learning");
    }
}
