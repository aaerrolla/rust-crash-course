// Arrays -  Fied list where elements are the same data types 


pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // in above initilization you can't initialize less or more elements than the declared size 5
    // i.e  = [ 1, 2, 3, 4]  is wrong  also = [ 1, 2, 4, 5, 5, 6]  is also wrong
    println!("{:?}", numbers);

    // indexing ,  rust also follows start with 0 index
    println!("Single Value : {}", numbers[0]);

    // updating array values
    // note : can't update unless numbers declared as mut 
    numbers[1] = 10;
    println!("{:?}", numbers);

    // Arrays are stack allocated
    println!(" numbers arrays occupied {} bytes", std::mem::size_of_val(&numbers));

    let primes: [i32; 6] = [2, 5, 7, 11, 13, 17];
    println!(" primes arrays occupied {} bytes", std::mem::size_of_val(&primes));

    // Slice of an array , in this example all  of numbers

    let numbers1: &[i32] = &numbers;
    println!(" numbers1 arrays content {:?} ", numbers1);

    // Slice  of an array , elements at index 0 and 1 
    let numbers2: &[i32] = &numbers[0..2];
    println!(" numbers2 arrays content {:?} ", numbers2);

    // Slice  of an array , elements at index 2  to   4 
    let numbers3: &[i32] = &primes[2..5];
    println!(" numbers2 arrays content {:?} ", numbers3);

}