#[derive(Debug)]
pub enum TokenType {
    Plus,      // +
    Minus,     // -
    Asterisk,  // *
    Slash,     // /
    PlusMinus, // +- | ±

    Numbers,
    Identifier,

    Eof,
}
