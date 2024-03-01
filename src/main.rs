use hashit::{hash_line_sha256, hash_line_sha3, parse_args, Args};
use std::io;
use std::io::Error;

fn process_input(input: &str, args: &Args) -> String {
    let mut line_buf = String::new();
    if args.verbose {
        line_buf.push_str(input);
        line_buf.push(' ');
    }
    let digest = if args.sha256 {
        hash_line_sha256(input)
    } else {
        hash_line_sha3(input)
    };
    line_buf.push_str(&digest);
    line_buf
}

fn main() -> Result<(), Error> {
    let args = parse_args();
    loop {
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        match result {
            Ok(0) => return Ok(()),
            Ok(_) => println!("{}", process_input(&input, &args)),
            Err(err) => return Err(err),
        }
    }
}
