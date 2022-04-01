// control the flow

pub fn run() {
    let age: u8 = 32;
    let check_id: bool = false;
    let knows_persons_age = true;

    // If / Else
    if age >= 21 && check_id || knows_persons_age {
        println!(" Bartender: What would you like to drik?");
    } else if age < 21 && check_id{
        println!(" Bartender: Sorry, you have to leave");
    } else {
        println!(" Bartender: I'll need to see your ID");
    }

    // shorthand if

    let is_of_age = if age >= 21 {true } else {false};
    println!(" Is of Age: {}" , is_of_age);
}