pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOpcode,
    UnknownInstruction,
    InvalidFormat,
    InvalidRegister,
    ImmediateOutOfRange,
    InvalidImmediate,
}
