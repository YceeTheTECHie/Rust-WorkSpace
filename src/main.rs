#![allow(non_snake_case)]

fn main()  {
    // let mut about; 
    // about = "tool";
    //  about = "letter";
    // println!("Cli tool {}.",about);
    // let num = 5;
    // let num = num * 2;
    // let num = num + 3;
    // println!("here is the result, {}", num);
    // let number: f32 = 5.0;
    let text: &str = "hello there"; 
    // Addition, Subtraction, and Multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, -8i32 - 5, 15 * 3);

// Integer and Floating point division
println!("9 / 2 = {} but 9.0 / 2.0 = {} answer is {}", 9u32 / 2, 9.0 / 2.0, text);

let sample = (2i32,true,'C');
println!("The last item of the element is {}",sample.2);
println!("The first item of the element is {}",sample.0);
println!("The second item of the element is {}",sample.1);

let formal: bool = false;
let greeting = if formal { // "if" keyword used here as an expression
    "Good day to you."     // Returns the string "Good day to you."
} else {
    "Hey!"                 // Returns the string "Hey!" 
};
println!("{}", greeting)   // Prints "Good day to you."


}
