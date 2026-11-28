#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TokenType {
    Plus,      // +
    Minus,     // -
    Asterisk,  // *
    Slash,     // /
    PlusMinus, // +- | ±

    Numbers,
    Identifier,

    Eof,
    NewLine, // \n

    Nai,
}
