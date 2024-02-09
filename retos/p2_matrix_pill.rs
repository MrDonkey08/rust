use std::process::Command;
use std::io::{self, Write};

fn sys_clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

fn sys_pause() {
    println!("Press Enter to continue...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    loop {
        println!("This is your last chance.
                After this there's not coming back.
                You take the blue pill: The story ends, you wake up in the bed and believe wathever you want to believe.
                You take the red pill, you stay in Wonderland and I show you how deep the rabbit hole goes.
                Remember, all I'm offering is the true, nothing more.\n");

        println!("Choose a pill: blue or red (b/r)");

        let mut pill : String = String::new();
        std::io::stdin()
            .read_line(&mut pill)
            .expect("Failed to read line");
        pill = pill.trim().to_string();

        if pill == "r" || pill == "red" {
            println!("Follow me...");
            break;
        }
        else if pill == "b" || pill == "blue" {
            println!("Motherf*cker!");
            break;
        }
        else {
            println!("Invalid option: Try again");
        }

        sys_pause();
        sys_clear();
    }
}
