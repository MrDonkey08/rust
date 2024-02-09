fn main() {
    let num_1 = 123;  
    let num_2 = 123;

    let sum = num_1 + num_2;

    loop {
        println!("Please, digit the result of the following operation:\n");
        println!("{num_1} + {num_2}: ");

        let mut sum_user = String::new();
        std::io::stdin()
            .read_line(&mut sum_user)
            .unwrap();
        let sum_user_int: i32 = sum_user.trim().parse().unwrap();

        if sum_user_int == sum {
            println!("{sum} is the correct answer");
            break;
        } else {
            println!("{sum_user_int} isn't the correct answer. Try again\n");
        }
    }
}
