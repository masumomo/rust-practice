use std::convert::{From,Into, TryFrom,TryInto};
// ********From and Into**********
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<i16> for Number {
    fn from(item: i16) -> Self {
        Number { value: i32::from(item) }
    }
}
    
// ********TryFrom and TryInto**********
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
       if value % 2 == 0 {
           Ok(EvenNumber(value))
       } else {
           Err(())
       }
    }
}

fn main() {
    println!("Hello, Conversion!");
    // ********From and Into**********
    // String is a growable, heap-allocated data structure whereas 
    // str is an immutable fixed-length string somewhere in memory 1
    let my_str = "Hello";
    let my_string = String::from(my_str);
    dbg!(my_str);
    dbg!(my_string);

    let mut num = Number::from(30);
    println!("My number is {:?}", num);

    num = Number::from(20i16);
    println!("My number is {:?}", num);



    let int =5;
    // let num2 = int.into(); ERROR!
    let num2: Number = int.into();
    println!("My number is {:?}", num2);

    // ********TryFrom and TryInto**********
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));


    // ********To and from Strings**********
}
