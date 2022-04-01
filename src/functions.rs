pub fn run() {
    greeting("Hello", "Anjan");

    // Bind function values to variables 
    let results = add(34, 56);
    println!(" Results {}",  results);

    // Closure 
    let add_nums = | n1: i32 , n2: i32| n1 + n2;
    println!(" Closure  add_nums: {}", add_nums(3, 4));

    // Closure with outside variable 
    let n3: i32 = 10;
    let add_nums_more =  | n1: i32, n2: i32 | n1 + n2 + n3;

    println!(" Closure add_nums_more: {}", add_nums_more(3, 4));

}

// function taking two str arguments and not returning anything 
fn greeting( greet: &str, name: &str) {
    println!( "{} {}, nice to meet you!", greet, name);
}

// function  taking  two i32  arguments and returning i32
// note : there is no explicit return , ommiting ; colon in last line of the function 
// indicates retun  value

fn add( n1: i32, n2: i32) -> i32 {
    n1 + n2
}