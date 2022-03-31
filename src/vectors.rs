pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numbers1: [i32; 5] = [ 1, 2, 3, 4, 5];
    let numbers2: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];


    println!("numbers contents {:?}" , numbers);
    println!("numbers1 contents {:?}" , numbers1);

    // size of various type vectors vs arrays 
    // stack allocated vs heap allocated 
    println!("numbers size : {}", std::mem::size_of_val(&numbers));
    println!("numbers1 size : {}", std::mem::size_of_val(&numbers1));
    println!("numbers2 size : {}", std::mem::size_of_val(&numbers2));

    // Vector length
    println!("numbers length : {}", numbers.len());
    println!("numbers2 length : {}", numbers2.len());

    // slicing vector  , same as arrays 

    let slice_numbers: &[i64] = &numbers2[0..5];
    println!("slice_numbers length : {}", slice_numbers.len());
    println!("slice_numbers content : {:?}", slice_numbers);

    // Add to vector , push only works if numbers declared as mut
    numbers.push(6);
    numbers.push(7);

    println!("numbers contents {:?}" , numbers);
    println!("numbers size : {}", std::mem::size_of_val(&numbers));

    // itration
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // mutate 
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("numbers contents {:?}" , numbers);

    

}