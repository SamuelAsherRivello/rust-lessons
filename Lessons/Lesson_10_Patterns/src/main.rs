// Demo purpose: build reusable problem-solving examples.

fn main() {
    observer_demo();
    strategy_demo();
    builder_demo();
}

// -------------------------------------------------------

struct Event {
    name: String,
}

trait Subscriber {
    fn notify(&self, event: &Event);
}

struct ConsoleSubscriber {
    name: String,
}

impl Subscriber for ConsoleSubscriber {
    fn notify(&self, event: &Event) {
        println!("observer: {} received {}", self.name, event.name);
    }
}

struct EventBus {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl EventBus {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    fn publish(&self, event: Event) {
        for subscriber in &self.subscribers {
            subscriber.notify(&event);
        }
    }
}

fn observer_demo() {
    let mut bus = EventBus::new();
    bus.subscribe(Box::new(ConsoleSubscriber {
        name: String::from("scoreboard"),
    }));
    bus.subscribe(Box::new(ConsoleSubscriber {
        name: String::from("logger"),
    }));

    bus.publish(Event {
        name: String::from("player_scored"),
    });
}

// -------------------------------------------------------

trait ScoreStrategy {
    fn score(&self, base: u32) -> u32;
}

struct NormalScore;
struct DoubleScore;

impl ScoreStrategy for NormalScore {
    fn score(&self, base: u32) -> u32 {
        base
    }
}

impl ScoreStrategy for DoubleScore {
    fn score(&self, base: u32) -> u32 {
        base * 2
    }
}

fn strategy_demo() {
    print_score("normal", &NormalScore, 10);
    print_score("double", &DoubleScore, 10);
}

fn print_score(name: &str, strategy: &dyn ScoreStrategy, base: u32) {
    println!("strategy {name}: {}", strategy.score(base));
}

// -------------------------------------------------------

#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    level: u32,
}

struct CharacterBuilder {
    name: String,
    health: u32,
    level: u32,
}

impl CharacterBuilder {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            health: 100,
            level: 1,
        }
    }

    fn health(mut self, health: u32) -> Self {
        self.health = health;
        self
    }

    fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    fn build(self) -> Character {
        Character {
            name: self.name,
            health: self.health,
            level: self.level,
        }
    }
}

fn builder_demo() {
    let character = CharacterBuilder::new("Ferris").health(80).level(5).build();

    println!(
        "builder: {} has {} health at level {}",
        character.name, character.health, character.level
    );
}
