use std::io::{self, Write};

mod decoder;
mod encoder;
mod error;
mod util;

fn main() {
    let mut input = String::new();

    loop {
        print!("Instruction: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit")
            || input.eq_ignore_ascii_case("q")
            || input.eq_ignore_ascii_case("quit")
        {
            break;
        }

        if util::is_hex(input) {
            match u32::from_str_radix(&input[2..], 16) {
                Ok(hex) => match decoder::decode(hex) {
                    Ok(instr) => println!("DEC: {:?}", instr),
                    Err(e) => println!("Error decoding instruction: {:?}", e),
                },
                Err(e) => println!("Error parsing hex input: {}", e),
            }
        } else {
            match encoder::encode(input) {
                Ok(instr) => println!("ENC: {:?}", instr),
                Err(e) => println!("Error encoding instruction: {:?}", e),
            }
        }
    }
}
