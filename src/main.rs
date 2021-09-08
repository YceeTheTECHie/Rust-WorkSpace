#![allow(non_snake_case)]
//declaring a struct
struct Student {name: String, isAbsent: bool, attendance:u8}
struct Teacher(String,f32,char,bool);
enum Example {
    // An enum variant can be like a unit struct without fields or data types
        firstExample,
        // An enum variant can be like a unit struct with  data types like tuple structs
        secondExample(String,char),
        // An enum variant can be like a unit struct with fields and data types
        thirdExample{name:String,page:u8}
}
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
println!("{}", greeting);   // Prints "Good day to you."
// instantiating
let student = Student{name:String::from("Boluwatife"),isAbsent:false,attendance:5};
println!("Name is {} has attendance {} and his absence is {}",student.name,student.attendance,student.isAbsent);
let teacher = Teacher(String::from("Boluwatife"),10.4,'b',true);
println!("{} is the name with height {}, first name begins with {}, loves coding is {}
", teacher.0,teacher.1,teacher.2,teacher.3 );  


}
