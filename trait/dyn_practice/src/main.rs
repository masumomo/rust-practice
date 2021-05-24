struct Sheep<'a> {name:&'a str}
struct Cow {}
// Prefer &str as a function parameter or if you want a read-only view of a string; String when you want to own and mutate a string.
trait Animal {
    // Instance method signature
    fn noise(&self) -> &str;
}

// Implement the `Animal` trait for `Sheep`.
impl<'a> Animal for Sheep<'a> {
    fn noise<'b>(&'b self) -> &'b str {
        self.name
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        let name = "I'm sheep";
        Box::new(Sheep {name:name})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

