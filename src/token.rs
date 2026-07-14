use std::any::Any;

pub enum TokenType {
    Group,
    Bracket,
    Or,
    Repeat,
    Literal,
    GroupUncaptured,
}

pub struct Token{
    pub token_type: TokenType,
    pub value: Box<dyn Any>,
}