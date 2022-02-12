
pub fn run() {

    let age: u8= 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // if/Else
    if age >= 21 && check_id || knows_person_of_age  {
        println!("Bartenders: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartenders: Sorry, you have to leave");
    } else  {
        println!("Bartenders: I'll need to see your ID");
    }

    // Shorthand if 
    let is_of_age = if age>= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}