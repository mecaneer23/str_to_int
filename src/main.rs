use std::io::{self, Write};

fn int(s: &str) -> i32 {
    let mut num: i32 = 0;
    let len: u32 = s.len() as u32;
    let mut multiplier = 1;
    for (i, byte) in s.bytes().enumerate() {
        if byte == 45 {
            multiplier = -1;
        } else if byte > 47 && byte < 58 {
            num += (byte - 48) as i32 * i32::pow(10, len - (i as u32) - 1);
        }
    }
    num * multiplier
}

fn input(query_string: &str) -> String {
    print!("{}", query_string);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("{}", int(&input("Enter a number: ")));
}
