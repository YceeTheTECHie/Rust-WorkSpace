#![allow(non_snake_case)]

//declaring a struct
struct Student {name: String, isAbsent: bool, attendance:u8}
struct Teacher(String,f32,char,bool);
// enum GenderCategory {
//     Male,Female
// }

// struct Person{name:String, gender:GenderCategory }
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
// working with struct and enums
// let p1 = Person{name:String::from("Person"), gender:GenderCategory::Male};
// let p2 = Person{name:String::from("Bolu"), gender:GenderCategory::Female};
//    println!("{:?}",p1);
//    println!("{:?}",p2)


// Declare array, initialize all values, compiler infers length = 7
let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  
// Declare array, first value = "0", length = 5
let bytes = [0;5];

println!("{}, {}", days[0],bytes[1]);

let time = vec![0,1,3,5];

println!("{}", time[0]);

// Create empty vector, declare vector mutable so it can grow and shrink
let mut fruit = Vec::new();
// Push values onto end of vector, type changes from generic `T` to String
fruit.push("Apple");
fruit.push("Banana");
fruit.push("Cherry");
println!("Fruits: {:?}", fruit);


use std::collections::HashMap;

let mut reviews : HashMap<String, String> = HashMap::new();

reviews.insert("hello".to_string(), "there".to_string());
reviews.insert("hi".to_string(), "there".to_string());

let firstGreeting : &str = "hello";
println!("{:?}", reviews.get(firstGreeting));


let mut marks = HashMap::new();

// adding a value

marks.insert("Mathematics",90);
marks.insert("English Language",80);
marks.insert("Biology",70);
marks.insert("History",50);
marks.insert("ICT",100);

// finding a value in a hashmap
println!("{:?}", marks);


// println!("{:?}", marks.get(number));

// removing values in a hashmap
marks.remove("Mathematics");
println!("{:?}", marks);

// finding the total value of key-values in a hashmap

println!("{:?}", marks.len());

// looping through an hashmap

for(subject,mark) in &marks{
    println!("{} is {}", subject,mark);
}
// using the contains key and value method

println!("did we study French {}", marks.contains_key("French"));
// println!("did we study French {}", marks.contains_value("French"));
let mut counter = 1;
// stop_loop is set when loop stops
let stop_loop = loop {
    counter *= 2;
    if counter > 100 {
        // Stop loop, return counter value
        break counter;
    }
};
// Loop should break when counter = 128
println!("Break the loop at counter = {}.", stop_loop);

let big_birds = ["ostrich", "peacock", "stork"];
for bird in big_birds.iter() {
    println!("The {} is a big bird.", bird);
}


// let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
// for index in [0, 2, 99].iter() {
//     match fruits.get(index) {
//         Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
//         None => println!("There is no fruit! :("),
//     }
// }


// closures

let diff = |n1:i32, n2:i32| n1 - n2;
println!("diff is {}", diff(8,3));
// let arr = vec![1,2,3,5];
// println!("{}", arr[10]);

// handling errors
// use std::fs::File;

// let f = File::open("./hey.txt");


// let f = match f {
//     Ok(file) => file,
//     Err(error) => panic!("cant open file {:?}", error),
// };

// println!("executing f {:?}", f);

// 


}
