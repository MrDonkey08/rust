fn main() {
    // Get userage
    println!("Please, enter the following data");

    println!("age:");
    let mut age : String = String::new();
    std::io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    age = name.trim().to_string();

    // Convert age to num
    let edad_int : u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 && edad_int != 30 {
        println!("You can enter the disco.");
    } 
    else if edad_int == 30 {
        println!("We don't allow 30 years old people.");
    } 
    else {
        println!("You're a minor.");
    }
}
