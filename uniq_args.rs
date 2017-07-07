use std::collections::HashSet;
use std::env;

fn main() {
    let mut uniq_args = HashSet::new();
    for arg in env::args().skip(1) {
        uniq_args.insert(arg);
    }
    // Will print out the set in curly braces, but we want one item per line
    // println!("{:?}", uniq_args);
    for arg in uniq_args {
        println!("{}", arg);
    }
}
