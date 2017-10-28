fn main() {
    println!("Hello, World!");
}

/*
 fn     <- function definiton keyword
 main   <- new function name
 ()     <- parameters bracket
 {}     <- function body bracket
 
 println!()         <- rust macro (how rust do metaprogramming)
 println()          <- just function call (not macro)
 {some function}'!' <- call macro not function
 
 * compile
 rustc {src name}.rs <- rust compile source
 
 * Cargo <- rust's build system (like Gradle?)
 cargo --version    <- check version. if output is "command not found",
                                      that means Cargo is not installed.
 
 * create project whith Cargo
 $ cargo new hello_cargo --bin  <- create project (--bin : executable)
 $ cd hello_cargo               <- output
 
 {output}
 .git           <- git
 .gitignore     <- gitignore file
 src            <- src root
 src/main.rs    <- default src file
 Cargo.toml     <- Cargo project configuration file
 
 * Next on hello_cargo/src/main.rs
 */