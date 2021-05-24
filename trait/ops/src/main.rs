use std::ops;

struct Foo{int:i32}
struct Bar{int:i32}

#[derive(Debug)]
struct FooBar{int:i32}

#[derive(Debug)]
struct BarFoo{int:i32}

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
// `std::ops::Add`トレイトは`+`の振る舞いを規定するために使用される
// ここでは`Foo`に対して`Add<Bar>`を実装する。これは加算時の右辺が`Bar`型
// の時に呼び出されるトレイト。つまり以下は`Foo + Bar = FooBar`という振る舞いを
// もたらす。
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        let result:i32 = self.int+rhs.int;
        println!("{:?}",result);
        FooBar{int:result}
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
// 型を反転することで、非可換の加算を実装できる。ここでは`Bar`に対して
// `Add<Foo>`を実装する。これは加算時の右辺が`Foo`型の時に呼び出されるメソッド。
// つまり以下は`Bar + Foo = BarFoo`という結果をもたらす。
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        let result :i32= self.int-rhs.int;
        println!("{:?}",result);
        BarFoo{int:result}
    }
}

fn main() {

    println!("Foo + Bar = {:?}", Foo{int:32} + Bar{int:10});
    println!("Bar + Foo = {:?}", Bar{int:10} + Foo{int:32});
}
