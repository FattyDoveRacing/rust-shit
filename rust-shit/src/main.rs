use rand::Rng; // importing random number generator from the rand package added in Cargo.toml
use std::cmp::Ordering;
use std::io; //std(STANDARD LIBRARY).io imported. if this was not imported at the start could do"std::io::stdin".
fn main() {
    println!("Guess the number (edge of the seat exitment)!");
    println!("input guess:");

    let mut guess = String::new(); //new is a funtion of string, creating a new string type, akin to "String guess = new String();" in java

    let secret_number = rand::thread_rng() // thread_rng spesifies to use random generator on current thread.
        .gen_range(1..=100); // calling the .gen_range method to specify the.. well.. gen range
                             //gen range is inclusive on both bounds when specifying lower..=upper
    println!("{}", secret_number);
    io::stdin() // stdin is a function which handles terminal input
        .read_line(&mut guess) //akin to scannerName.nextLine(); - assighning input to mutable variable guess NOTE this appends to existing string contents
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number"); //u32 means 32 bit unsighned int, we are trimming and parsing a string for int values
                                                                          //it forces you to have an axpect statemtn so it doesnt crash when you try to be a smartass and type chars into the prompt
                                                                          //ooh also we are "shadowing" the old guess var with a new one, its basically overriting ands using the same name

    println!("You guessed: {guess}");
    //match expression decides what to do based on what variant of ordering was returned
    //match matches the thing on the left to the ARM (=>) it is getting ponted at on the left (fancy fancy very brittish if/elseif statment for krumpet munchers)
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You won!"),
    }
}
