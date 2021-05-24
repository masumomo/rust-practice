// A struct with annotation of lifetimes.
#[derive(Debug)]
 struct Borrowed<'a> {
     x: &'a i32,
 }

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}
#[derive(Debug)]
 struct Borrowed2 {
     x: i32,
 }

// Annotate lifetimes to impl.
impl Default for Borrowed2{
    fn default() -> Self {
        let value = 100;
        Self {
            x: value,
        }
    }
}
fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
    let b2: Borrowed2 = Default::default();
    println!("b2 is {:?}", b2);
}
