use std::io::{self, Write};

mod decoder;
mod encoder;
mod error;
mod format;
mod util;

pub fn run_cli() {
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
                    Ok(instr) => {
                        format::print_encoded_instruction(&instr);
                    }
                    Err(_) => println!("Error parsing hex input:"),
                },
                Err(_) => println!("Error parsing hex input:"),
            }
        } else {
            match encoder::encode(input) {
                Ok(d) => format::print_encoded_instruction(&d),
                Err(e) => println!("Error encoding instruction: {:?}", e),
            }
        }
    }
}

fn main() {
    run_cli()
}
