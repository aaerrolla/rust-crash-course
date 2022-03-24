/*
Primitive types --
    integers: u8, i8, u16 , i16 , u32 , i32, u64 , i64 , u128, i128
    floats: f32, f64
    boolean: bool
    charecter: char
    Tuples
    Arrays : fixed length
*/

// Rust is statically typed ,  but compiler can infer the variable type by the value assigned 
//

fn print_type_of<T>(_: &T)  -> String {
     return std::any::type_name::<T>().to_string();
}

pub fn run() {
    // Example compiler infer type by value 

    let x = 1;

    let y = 2.6;
    println!("Type of x  is  {} ,  y is {} ",print_type_of(&x) , print_type_of(&y));

    println!(" x is {} , y  is {}", x,y);

    // Explicit type
    let z: i64 =  34343434343;

    // find the max size 
    println!("Max i32 is {}", std::i32::MAX);
    println!("Max i64 is {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // Boolean from expression , again here type is infered

    let is_true = 7 > 9;
    println!("{:?}", (x, y, z, is_true));

    

}