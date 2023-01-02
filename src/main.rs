use std::{env};

use mini_ls::run;

fn main() {
    let (path, file_names) = run(&mut env::args().collect());
    println!("Showing files in '{path}':");
    
    for file_name in file_names {
       println!("-- {}", file_name);
    }
}