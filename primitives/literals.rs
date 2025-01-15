
// https://doc.rust-lang.org/rust-by-example/primitives/literals.html


// This is similar to waht it is in C-like languages

fn main(){

    // unsigned 32 bit
    println!("Testing Addition: 1u32 + 2");
    println!("1 + 2 = {}", 1u32 + 2);

    println!("Testing Subtraction: 1i32 - 2");
    println!("1 - 2  = {}", 1i32 - 2);

    // checking scientific notation
    println!("Checking scientific notations");
    println!("1e4 is {}", 1e4);

    // boolean logic
    println!("Checking boolean logic");
    println!("true AND false = {}", true && false);
    println!("true OR false = {}", true || false);
    println!("Not true =  {}", !true);

    // underscores
    println!("Underscores improve the readability of the code, 1_000 is the same as 1000");
    println!("One million is {}", 1_000_000u32);


}