use std::error::Error;
#[derive(Debug)]
enum DecodeError {
    InvalidOpcode(u32),
    UnknownInstruction(u32),
}

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
                    eprintln!("Error: {:?}", e);
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

fn decode(hex: u32) -> Result<String, DecodeError> {
    let opcode = hex & 0x7f;
    let funct3 = (hex >> 12) & 0x7;
    let funct7 = (hex >> 25) & 0x7f;
    let rd = (hex >> 7) & 0x1f;
    let rs1 = (hex >> 15) & 0x1f;
    let rs2 = (hex >> 20) & 0x1f;

    Ok("WIP".to_string())
}

fn try_parse_hexword(s: &str) -> Result<u32, Box<dyn Error>> {
    let s = s.trim();
    let s = if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    };
    if s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(u32::from_str_radix(s, 16)?)
    } else {
        Err("Not a valid hexword".into())
    }
}
