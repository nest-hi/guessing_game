use std::cmp::Ordering;//for match statement
use std::io;//for user input 
use rand::Rng;//for random number 

fn main() {

//We will prompt the user to play and let them guess a number.
    println!("A number was picked from 1 to 100. What would that number be?");
    println!("input your guess");


    let mut input = String::new();
//we generate a random number for the user to guess
    let secret_number: i32 = rand::thread_rng().gen_range(1..100);

    let mut trys: i32 = 0;//we create a counter of how many tries did the user took;


//this while loop is created so the program would prompt the user for an answer until the got it right. :3
    while input.trim() != secret_number.to_string().trim()// Nessie: I've converted the input:String and secret_number:i32 to string slices. Don't ask me why.
    {
        input = String::new();
        io::stdin()//here is where the user would input their inputs :D
            .read_line(&mut input)//then we read that input
            .expect("tf did you even type brah?");//or this will print if we can't.

        trys+=1;// iteration: this will add 1 for each guesses/tries the user does.

// Converts the data type of input:String into an i32 integer and contain it in a variable with the same name
        let input = input
            .trim()//NOTE: .trim() converts a String into a string slice,
            .parse::<i32>()//as directly converting a String into i32 will throw a ParseIntError / InvalidDigits because of whitespaces.
            .expect("ERROR ON COVERTING THE String INTO i32.");//The program will stop the program and will made us aware of the error and where it occurred

// match blocks compare the value of two variable and executes a block of code corresponding on the matched pattern like Ordering::[Greater, Equal, Less].
        match input.cmp(&secret_number)//we compare the value of secret_number into the input. match acts like a shorter if-statement.
        {
            Ordering::Greater => println!("Smaller!"),// If the guess of the player is greater than the secret number, We would ask them to guess a little lower.
            Ordering::Equal => println!("YOU ARE CORRECT!!"),//executes when the have guessed the secret number.
            Ordering::Less => println!("Bigger!!"),//then, if they game a  lower number than the secret number we would ask for a higher one.
        }

        println!("You guessed: {input}");//prints the user's guess. Why not?

    }

//prints the secret number along with the number of tries that the user took to guess it.
    println!("The secret number is: {secret_number} and you guessed it in {trys} tries!");
    
}


