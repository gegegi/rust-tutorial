fn main() {
    println!("Hello, World!");
}

/*
 ! READ FIRST chapter1/hello_world/main.rs !

 {output}
 .git           <- git
 .gitignore     <- gitignore file
 src/           <- src root
 src/main.rs    <- default src file
 Cargo.toml     <- Cargo project configuration file
 
 * change vcs
 cargo new {project name} --vcs {vcs}
 
 * Cargo.toml <- Cargo project's configuration (Tom’s Obvious, Minimal Language)
 [package]
 name = "hello_cargo"                           <- default : project dir name
 version = "0.1.0"
 authors = ["SeokHui Lee <tjzl1995@gmail.com>"] <- default : vcs(git) user name

 [dependencies] <- project's dependencies
 {dependencies}
 
 * build project
 $ cargo build
 
 * build output
 target/
 Cargo.lock     <-  Cargo.lock to keep track of dependencies in your application (controlled by Cargo, not user)
 
 * run
 $ cargo run
 
 * release
 $ cargo build --release
 
 */