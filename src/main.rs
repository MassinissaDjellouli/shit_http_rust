use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number from 0 to 100");
    let random_value = rand::thread_rng().gen_range(0..100);
    let parsed_guess = loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unable to Read");
        let guess = guess.trim();
        println!("guess: {guess}");

        let parsed_guess_result = str::parse::<i32>(guess);
        match parsed_guess_result {
            Ok(ok) => {
                if ok <= 100 && ok >= 0 {
                    break ok;
                }                
                println!("0 to 100????");
            }
            Err(err) => {
                println!("{}", err);
            }
        };
    };
    if random_value == parsed_guess {
        println!("Yer right!");
    } else {
        println!("Loser");
        println!("The value is {random_value}");
    }
}
