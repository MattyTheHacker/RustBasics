fn main() {
    // basic variable declaration, immutable
    let x = 5;

    // basic variable declaration, mutable
    let mut y = 6;

    // variable declaration with type annotation
    let x: u32 = 5;

    // change value of variable without being mutable
    let a = 5;
    println!("a = {}", a);
    let a = 6;
    println!("a = {}", a);

    let b = 6;
    println!("b = {}", b);
    let b = b + 1;
    println!("b = {}", b); // b will be 7

    // string formatting
    println!("value of x is: {}", x);

    // shadowing, curly braces create a new scope
    {
        let c = 2;
        println!("c = {}", c);
    }
}
