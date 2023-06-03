use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100); 
    let mut won = false;
    loop {
        if won {
            println!("Would you like to continue? (y/n)")
        }else{
            println!("Please input your guess.");
        }
        let mut guess = String::new();
        // Read user input into a variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // condition to handle the choice after winning game
        // quit the game loop
        if ( guess.trim().eq_ignore_ascii_case("quit")) || (won && guess.trim().eq_ignore_ascii_case("n")){
            println!("Thank you for playing the game! ciao!");
            break;
        }else if won {
            won = false;
            println!("\n\n");
            continue;
        }
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
         };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\n\n");
                won = true;
            }
        };
    }
}
