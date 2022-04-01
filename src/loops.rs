pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // while loop demo using FizzBuzz  game

    // print  FizzBuzz if divisable by  3 and 5
    // print Fizz if divisable by 3
    // print Buzz if divisable by 5

    while count <= 100 {
        if count % (3 * 5) == 0 {
            println!(" {} : fizbuzz" , count);
        } else if count % 3 == 0  {
            println!(" {} :fiz" , count);
        } else if count % 5 == 0 {
            println!(" {}: buzz", count);
        } else {
            println!(" {} " , count );
        }
        count += 1;
    }

    // for  range loop
    
    for x in 1..100 {
        if x % (3 * 5) == 0 {
            println!(" {} : fizbuzz" , x);
        } else if x % 3 == 0  {
            println!(" {} :fiz" , x);
        } else if x % 5 == 0 {
            println!(" {}: buzz", x);
        } else {
            println!(" {} " , x );
        }
    }

    
}