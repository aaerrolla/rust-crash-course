// primitive  or reference 
// immutable by default
// block scoped

pub fn run() {
    let name = "Anjan";
    let mut age = 47;
    println!("My name is {} I'm {}", name, age);
    age = 48;
    println!("My name is {} I'm {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign muliple variables 
    let ( my_name, my_age ) = ("Anjan", 47);
    println!(" My name is {} , and I'm {} old", my_name, my_age);
}