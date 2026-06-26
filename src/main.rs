use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::ControlFlow::Break;

fn main() {
    println!("Hello, world!");

    let _p1 = Person {
        name: String::from("Landry"),
        age: 20,
    };

    // ------- Guessing Game with Let's Get Rusty!!------------------------------------------

    // Generate random secret number
    let secret_number = rand::thread_rng().gen_range(1, 10);
    print!("The secret number is = {}", secret_number);

    loop {
        println!(" Input your guess ");
        let mut guess = String::new();

        // to get input from the current process
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = guess
            .trim()
            .parse()
            .expect("Failed to parse input, please input a number");

        println!("You guessed - {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!!"); 
                break;
            }  // This breaks the loop when the correct answer is provided.
            Ordering::Greater => println!("Too big!")
            
        }
        println!("The correct answer is - {}", secret_number);
    }
}

struct Person {
    name: String,
    age: u32,
}
