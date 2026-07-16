use std::any::Any;
#[derive(Debug)]
pub enum TokenType {
    Group,
    Bracket,
    Or,
    Repeat,
    Literal,
    GroupUncaptured,
}

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub value: Box<dyn Any>,
}

pub struct RepeatPayload{
    pub min: i8,
    pub max: i8,
    pub token: Token,
}