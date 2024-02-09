fn main() {
    println!("Please, enter the following data ");

    println!("Name: ");

    let mut name: String = String::new();
    std::io::stdin()
        .read_line(&mut name) // Console Input
        .expect("Failed to read line");
    name = name.trim().to_string();


    println!("Age: ");
    let mut age: String = String::new();
    std::io::stdin()
        .read_line(&mut age) // Console Input
        .unwrap();

    // Convert age to number
    let age_int : u8 = age
        .trim()
        .parse()
        .expect("Failed to read line");

    println!("Hola, bienvenido {} de {}.", name, age_int);
}
