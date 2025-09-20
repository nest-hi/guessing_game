use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    println!("Pick a number: ");
    println!("please input your guess");
    let mut input = String::new();
    let secret_number: i32 = rand::thread_rng().gen_range(1..100);
    // println!("The secret number is: {secret_number}");

    while input.trim() != secret_number.to_string().trim() {

        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("tf did you even type brah?");

        let input = input
            .trim()
            .parse::<i32>()
            .expect("ERROR YOU DUMM FUCK!!");

        match input.cmp(&secret_number)
            {
            Ordering::Greater => println!("Smaller!"),
            Ordering::Equal => println!("YOU ARE CORRECT!!"),
            Ordering::Less => println!("Bigger!!"),
        }
        println!("You guessed: {input}");

    }

    println!("The secret number is: {secret_number}");
    
}


