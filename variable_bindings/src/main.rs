fn main() {
    println!("Hello, Variable Binding!");

    // ********Basic********
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", an_integer);
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // ********Mutability********
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // Get error!
    let _immutable_binding = 1;
    // immutable_binding += 1; should comment out 


    // ********Scope and Shadowing********

    let long_lived_bindings = 1;
    {
        let short_lived_bindings = 2;
        println!("Short bindings Inner block: {}", short_lived_bindings);

        println!("Short bindings Inner block: {}", long_lived_bindings);

        let long_lived_bindings = 3;
        println!("Short bindings Inner block after shadowing: {}", long_lived_bindings);

    }
    // Get Error!
    // println!("Short bindings Outer block: {}", short_lived_bindings);

    // Get value that I assigned outer block
    println!("Short bindings Outer block: {}", long_lived_bindings);


    // ********declare**********
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);

}
