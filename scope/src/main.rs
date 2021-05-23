// *******RAII(Resource Acquisition Is Initialization)********
fn create_box() {
    let _box1 = Box::new(3i32);
    // `_box1` is destroyed here, and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Dropped!");
    }
}
// *******move********
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

// Rustでは 借用(borrowing) という仕組みを用います。値そのもの(T)を受け渡すのではなく、そのリファレンス(&T)を渡すのです。
fn not_destroy_box(c: &Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn eat_i32(i: i32) {
    println!("Destroying i32 that contains {}", i);
}
// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    println!("Hello, scope!");

    // *******RAII(Resource Acquisition Is Initialization)********
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
        // `_box3` is destroyed here, and memory gets freed
    }

    for _ in 0u32..1_000 {
        // println!("{}", i);
        create_box();
    }
    // `_box2` is destroyed here, and memory gets freed

    let _x = ToDrop;
    println!("Make a drop");

    // *******move********
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y); // OK

    let a = Box::new(5i32);
    println!("a contains: {}", a);
    let b = a;
    // println!("a contains: {}", a); //  Error! because this pointer belongs to b

    destroy_box(b);

    let a2 = &Box::new(20i32);
    not_destroy_box(a2);
    // println!("b contains: {}", b); //  Error! because this pointer belongs to b

    // *******mut********
    // データのミュータビリティは所有権を移譲した際に変更できます。
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    // *******borrow********
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let _ref_to_i32: &i32 = &boxed_i32;
        let _ref_to_stack_i32: &i32 = &boxed_i32;
        // destroy_box(boxed_i32);
        eat_i32(stacked_i32); // It's OK
        borrow_i32(_ref_to_i32);
        borrow_i32(_ref_to_stack_i32);
    }
    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
    eat_i32(stacked_i32);
}
