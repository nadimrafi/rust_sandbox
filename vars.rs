pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("my name is {} and i am {}", name, age);
    age = 38;
    
    println!("my name is {} and i am {}", name, age);
    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID); 

    //Asssign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);

}