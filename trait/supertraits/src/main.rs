trait Person {
    fn name(&self) -> String;
}

// Student is a supertrait of Person.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}
impl dyn Student {
}

trait Programmer {
    fn fav_language(&self) -> String;
}

impl dyn Programmer {
}


// CompSciStudent (computer science student) is a supertrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both subtraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct StanfordCompSciStudent {
    name : String,
    git_username : String,
    fav_language : String
}

impl StanfordCompSciStudent {
    fn greeting(&self) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {} and Git username is {}",
            self.name(),
            self.university(),
            self.fav_language(),
            self.git_username()
        )
    }
}

impl Person for StanfordCompSciStudent { 
    fn name(&self) -> String {
        let name = &self.name;
        name.to_string()
    }
}

impl Student for StanfordCompSciStudent { 
    fn university(&self) -> String {
        String::from("stanford")
    }
}

impl Programmer for StanfordCompSciStudent { 
    fn fav_language(&self) -> String {
        let fav_language = &self.fav_language;
        fav_language.to_string()
    }
}

impl CompSciStudent for StanfordCompSciStudent { 
    fn git_username(&self) -> String {
        let result = &self.git_username;
        result.to_string()
    }
}


fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

fn main() {
    let me = StanfordCompSciStudent{
        name : String::from("miki"),
        git_username :String::from("masumomo"),
        fav_language :String::from("rust!")
    };
    println!("{:?}",me.greeting());

}
