// Primitive str =  immutable fixed length somewhare in the memory
// String =  growable ,  heap allocated data structure -  used when  we want modify 

pub fn run() {

    // String object demo
    let mut hello =  String::from("Hello");

    // get letngth
    println!("Length : {}", hello.len());

    // push char 
    hello.push(' ');
    hello.push('W');

    // push more than one char , string
    hello.push_str("orld!");
    println!("{}" , hello);

    // Capacity  in bytes
    println!("Capacity: {}", hello.capacity());

    // check for empty
    println!(" Is empy {}", hello.is_empty());

    // check  to see contains other word substring
    println!(" Contains word World {}", hello.contains("World"));

    // replace 
    println!("Replace {}", hello.replace("World", "There"));

    // split string 
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create a string with certian capacity

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion test

    assert_eq!(2, s.len());
    //assert_eq!(3, s.len());

    assert_eq!(10, s.capacity());
    //assert_eq!(11, s.capacity());


    println!("{}", s);

    // str demo
    let  world = "Hello";
    println!("world  is {}", world);

    // can't perform push or push_str of str type
    //world.push('G');

}