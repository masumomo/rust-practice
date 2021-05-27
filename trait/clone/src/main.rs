// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Nil;

// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct TestInt(i32);

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Nil`
    let nil = Nil;
    // Copy `Nil`, there are no resources to move
    let copied_nil = nil;

    // Both `Nil`s can be used independently
    // いずれの`Nil`も独立に使用できる。
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Copy `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    // println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line
    
    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);

    // Instantiate `Nil`
    let test_int = TestInt(2);
    // Copy `Nil`, there are no resources to move
    let copied_test_int = test_int;

    // Both `Nil`s can be used independently
    // いずれの`Nil`も独立に使用できる。
    dbg!(test_int);
    dbg!(copied_test_int);
}
