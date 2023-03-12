fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess={guess}");

    // Scalar types (integer, float, bool, char)
    // integer
    // i32 - integer can be negative or positve
    // u32 - integer can only be positive

    //Float
    // f32, f64 (default)
    let x = 2.03;
    println!("X = {x}");
    let y: f32 = 3.03;
    println!("Y = {y}");

    //Numeric Operations
    //addition
    let sum = 3 + 2;
    println!("Sum 3 + 2 = {sum}");

    //Subtraction
    let difference = 92.4 - 92.0;
    println!("Difference of 92.4 and 92.0 = {difference}");

    //Multiplication
    let prod = 4 * 30;
    println!("4 * 30 = {prod}");

    //Division
    let quotient = 10.23 / 32.2;
    println!("10.23/32.2 = {quotient}");

    let floored = 2 / 3; //return 0
    println!("2/3 = {floored}");

    // Boolean
    let t = true;
    let f: bool = false;

    println!("Booleans");
    println!("t={t}");
    println!("f={f}");

    //Char - the most primitive alphabetic type
    println!("Char:");
    let c = 'z';
    println!("c = {c}");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("z = {z}");
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {heart_eyed_cat}");

    // Compound types (tuples, arrays)
    // Tuple
    // Mixed types but fixes length
    println!("Tuple");
    let first_tuple: (u32, f64, &str) = (500, 6.4, "hello world");
    println!("first_tuple = {:#?}", first_tuple); //Way to print arrays and tuples (with # will add indentation)
    let (x, y, z) = first_tuple;
    println!("tuple[0] = {x}");
    println!("tuple[1] = {y}");
    println!("tuple[2] = {z}");

    let first_value = first_tuple.0;
    let second_value = first_tuple.1;
    let third_value = first_tuple.2;

    println!("First value={first_value}, second={second_value}, third={third_value}");

    // Arrays
    // always same type and fixed length
    println!("Arrays");
    let first_array = [1, 2, 3, 4, 5]; //array is ['type'; 'length']
    println!("first_array = {:#?}", first_array);

    let first_el = first_array[0];
    println!("first_array[0] = {:?}", first_el);
    println!("first_array[1] = {:?}", first_array[1]);
    println!("first_array[2] = {:?}", first_array[2]);
    println!("first_array[-1] = {:?}", first_array.last().unwrap());
    
}
