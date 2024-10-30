// 1. Variables and Mutability

// -- immutable --
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
/*
Output :-

cannot assign twice to immutable variable `x`
*/


// -- mutable --
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
/*
Output :-

The value of x is: 5
The value of x is: 6
*/



// 2. Constants

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;



// 3. Shadowing

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}")
}
/*
Output :-

The value of x in the inner scope is: 12
The value of x is: 6
*/