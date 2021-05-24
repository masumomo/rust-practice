struct Sheep { naked: bool, name: &'static str }
struct Miki { naked: bool, name: &'static str }

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    // It's error because Animals don't know if they have name
    // fn name(&self) -> &'static str{
    //     self.name
    // }
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}


impl Animal for Miki {
    fn new(name: &'static str) -> Miki {
        Miki { name: "Miki", naked: false}
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "Hello!"
    }
    
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly :Sheep = Animal::new("Dolly");
    let mut me : Miki = Animal::new("ver2");

    dolly.talk();
    dolly.shear();
    dolly.shear();
    dolly.talk();


    me.talk();
}
