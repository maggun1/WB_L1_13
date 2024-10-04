use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut last_line = String::new();

    loop {
        print!("Input: ");
        io::stdout().flush().expect("Failed to flush");

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");

        let line = line.trim();
        if line != last_line {
            println!("Output: {}", line);
            last_line = line.to_string();
        }
        else {
            println!("Repeat!");
        }
    }
}
