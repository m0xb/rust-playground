use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let address = "127.0.0.1";
    let port = 5000;
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();

    // Nice suggestion from a compiler warning:
    //   warning: denote infinite loops with loop { ... }
    // (instead of "while true { ... }")
    loop {
        println!("Listening to {}:{} for connections...", address, port);
        match listener.accept() {
            // Need to use "mut" here since read_to_string modifies self
            Ok((mut sock, addr)) => {
                println!("--> Connection from {}", addr);
                let mut buffer = String::new();
                // Not using "?" here because it's only applicable in functions that return a Result (this returns "()"?)
                sock.read_to_string(&mut buffer);
                println!("{}", buffer);
            }
            Err(e) => {
                println!("There was an error");
            }
        }
    }

}

/*
Notes:

https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact


error[E0277]: the trait bound `(): std::ops::Carrier` is not satisfied
https://github.com/rust-lang/rust/issues/35946

*/

