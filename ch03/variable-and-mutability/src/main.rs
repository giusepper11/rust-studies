fn main() {
    // Mutability
    let mut x = 5;
    println!("The value x is: {x}");
    x = 6;
    println!("The value x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours have {THREE_HOURS_IN_SECONDS} seconds!");

    // Shadowing
    let x = 5;
    let x = x + 5;

    {
        let x = x * 2;
        println!("Inner scope x: {x}");
    }
    println!("Out of the scope x: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces has {spaces} spaces =)")

    // Dont compile because it changes data type
    // let mut spaces = "   ";
    // spaces = spaces.len();

    


}
