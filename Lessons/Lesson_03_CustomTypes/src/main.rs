// Demo purpose: create your own simple types.

fn main() {
    let player = Player {
        name: String::from("Ferris"),
        health: 100,
    };
    let score = Score(900);
    let ready = Ready;
    let states = [
        GameState::Loading,
        GameState::Playing { level: 3 },
        GameState::Finished,
    ];

    println!("player: {}", player.describe());
    println!("score: {}", score.0);
    println!("ready: {:?}", ready);

    for state in states {
        print_state(state);
    }
}

// -------------------------------------------------------

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
}

impl Player {
    fn describe(&self) -> String {
        format!("{} has {} health", self.name, self.health)
    }
}

// -------------------------------------------------------

#[derive(Debug)]
struct Score(u32);

// -------------------------------------------------------

#[derive(Debug)]
struct Ready;

// -------------------------------------------------------

#[derive(Debug)]
enum GameState {
    Loading,
    Playing { level: u32 },
    Finished,
}

fn print_state(state: GameState) {
    match state {
        GameState::Loading => println!("state: loading"),
        GameState::Playing { level } => println!("state: playing level {level}"),
        GameState::Finished => println!("state: finished"),
    }
}
