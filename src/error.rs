pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOpcode(u32),
    UnknownInstruction(u32),
    Other,
}
