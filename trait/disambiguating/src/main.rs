trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait FirstnameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> &str;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    first_name: &'static str,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl FirstnameWidget for Form {
    fn get(&self) -> &str {
        self.first_name
    }
}


impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form{
        username: "rustacean".to_owned(), // Creates owned data from borrowed data, usually by cloning.
        first_name : "String::from(miki)",
        age: 28,
    };

    // If you uncomment this line, you'll get an error saying 
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    let first_name = <Form as FirstnameWidget>::get(&form);
    println!("{}",first_name);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
