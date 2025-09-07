mod error;
mod util;
use crate::error::{Error, Result}; // <-- Import Error
use util::decode;

fn main() -> () {
    // read args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <hexword>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    // try and parse the instruction either 0x.. or "addi ..."
    match try_parse_hexword(input) {
        Ok(hex) => {
            // decode instruction
            match decode(hex) {
                Ok(instr) => println!("{}", instr),
                Err(e) => {
                    match e {
                        Error::InvalidOpcode(op) => eprintln!("Invalid opcode: 0x{:02x}", op),
                        Error::UnknownInstruction(instr) => {
                            eprintln!("Unknown instruction: 0x{:08x}", instr)
                        }
                        Error::Other => eprintln!("Other error"),
                    }
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            // not a hex, try to parse as string instruction
            eprintln!("TODO: parse string instruction: {}", input);
        }
    }
}

fn try_parse_hexword(s: &str) -> Result<u32> {
    let s = s.trim();
    let s = if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    };
    if s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
        u32::from_str_radix(s, 16).map_err(|_| Error::Other)
    } else {
        Err(Error::Other)
    }
}
