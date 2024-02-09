fn main(){
    let name: &str = "Alan";
    let mut old: u8 = 25;
    let number: u8 = 28;

    print!("Hello, world! My names is {} and I'm {} years old and my number is {}.\n\n", name, old, number);

    old = 30; // Reasigment for mutable variables
    let number = 25; // Shadowing 
	print!("Hello, world! My names is {name} and I'm {old} years old and my number is {number}.\n");
}
