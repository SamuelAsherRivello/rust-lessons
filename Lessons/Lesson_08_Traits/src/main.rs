// Demo purpose: give different types shared behavior.

fn main() {
    let player = Player {
        name: String::from("Ferris"),
    };
    let chest = Chest { coins: 25 };

    print_description(&player);
    print_description(&chest);

    let items: Vec<Box<dyn Describable>> = vec![Box::new(player), Box::new(chest)];
    for item in items {
        println!("trait object: {}", item.describe());
    }
}

// -------------------------------------------------------

trait Describable {
    fn describe(&self) -> String;
}

struct Player {
    name: String,
}

struct Chest {
    coins: u32,
}

impl Describable for Player {
    fn describe(&self) -> String {
        format!("player named {}", self.name)
    }
}

impl Describable for Chest {
    fn describe(&self) -> String {
        format!("chest with {} coins", self.coins)
    }
}

fn print_description<T: Describable>(item: &T) {
    println!("generic trait: {}", item.describe());
}
