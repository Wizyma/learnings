use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng()
        .gen_range(1..100);

    turples_training();
    loop {
        println!("Please input your guess!");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
            println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn turples_training() {
    let tup: (i32, f64, u8) = (200, 2.9, 12);
    let (a, _b, _c) = tup;
    let b = tup.1;
    println!("The value of a is: {} and b is {}", a, b);
}

