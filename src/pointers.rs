use std::vec;

pub fn run() {
    let array1 = [1, 2, 3, 4];
    let array2 = array1;

    println!("{:?}", (array1, array2));

    let prime1 = vec![2, 3, 5, 6];
    let prime2 = prime1;

    // this below line where we are using prime1  is results in error
    // essentially we can't use reference type once its assigned to ot nher reference

    // println!("{:?}" , (prime1, prime2));

    println!("{:?}", prime2);

    // how to fix it ,  by  means of pointers
    let numbers1: Vec<i32> = vec![10, 20, 30, 40];
    let numbers2 = &numbers1;

    println!("{:?}", (&numbers1, numbers2));
}
