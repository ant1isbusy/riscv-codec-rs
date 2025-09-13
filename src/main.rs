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
                Ok(instr) => match instr {
                    // todo added nice prints for each type
                    encoder::Instruction::RType(r) => println!("ENC: {:?} (0x{:08x})", instr, r.0),
                    encoder::Instruction::IType(i) => println!("ENC: {:?} (0x{:08x})", instr, i.0),
                    encoder::Instruction::SType(s) => println!("ENC: {:?} (0x{:08x})", instr, s.0),
                    encoder::Instruction::BType(b) => println!("ENC: {:?} (0x{:08x})", instr, b.0),
                    encoder::Instruction::UType(u) => println!("ENC: {:?} (0x{:08x})", instr, u.0),
                    encoder::Instruction::JType(j) => println!("ENC: {:?} (0x{:08x})", instr, j.0),
                },
                Err(e) => println!("Error encoding instruction: {:?}", e),
            }
        }
    }
}
