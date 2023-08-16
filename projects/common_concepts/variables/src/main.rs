// By default, variables are immutable.
// However, you still have the option to make your variables mutable.

fn main() {
    
    /*
        let x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The vlaue of x is: {x}");
    */

    // can make mutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants are ALWAYS immutable (you can't use mut)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}