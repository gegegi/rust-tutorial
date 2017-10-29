use std::io // load io libarary on std library

fn main() {
    println!("Guessing the number!");
    println!("Please input your guess.");

    /*
     let <- new variable (immutable)
     mut <- make it mutable
     String::new() <- make new 'String' (UTF-8 base)
     {Type?}::{function} <- function that is an associated on {Type?}
    */
    let mut guess = String::new();

    /*
     io::stdin() <- same as std::io::stdin(). line 1 makes it io::stdin()
                    it returns {instance : std::io::Stdin}
     &mut guess  <- give it's reference.
                    reference is immutable (default) so we neet &mut for mutable
     read_line   <- it returns std::io::Result {enum type}
     expect      <- std::Result.expect : unwrap Result. yeild until Ok
                    "Failed to read line" : panic message that using when Result is Err.
    */
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    /*
     {}   <- placeholder : will replaced guess
     EX : println!("{} {} {}", 1, 2, 3) == "1 2 3"
    */
    println!("You guessed: {}", guess);
    
    // First part end
}
