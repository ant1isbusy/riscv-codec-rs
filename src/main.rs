use std::io::{self, Write};

mod decoder;
mod encoder;
mod error;

fn main() {
    let mut input = String::new();

    loop {
        print!("Enter instruction or hex (or 'exit' to quit): ");
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

        let maybe_hex = input
            .strip_prefix("0x")
            .or_else(|| input.strip_prefix("0X"));
        if let Some(hex) = maybe_hex {
            if hex.chars().all(|c| c.is_ascii_hexdigit()) && hex.len() <= 8 {
                match u32::from_str_radix(hex, 16) {
                    Ok(word) => {
                        println!("Decoded: {}", decoder::decode(word).unwrap());
                        continue;
                    }
                    Err(_) => {
                        eprintln!("Invalid hex input");
                        continue;
                    }
                }
            }
        } else {
            match encoder::encode(input) {
                Ok(word) => println!("Encoded: 0x{:08x}", word),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }
}
