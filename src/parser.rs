use crate::token::{
    TokenType,
    Token
};

#[derive(Default)]
pub struct ParseContext{
    pub pos: usize,
    pub tokens: Vec<Token>,
}

trait Parser{
    fn parse(&self, regex: &str, ctx: &mut ParseContext);
}

struct Process;
impl Process{
    fn process<T: Parser>(g: T, regex: &str, ctx: &mut ParseContext){
        g.parse(regex, ctx);
    }
}

struct Group;
impl Parser for Group{
    fn parse(&self, regex: &str, ctx: &mut ParseContext){
        print!("parse group");

        ctx.pos += 1;

        while regex.as_bytes()[ctx.pos] != b')' {
            process_general(regex, ctx);
            ctx.pos += 1;
        }
    }
}

struct Bracket;
impl Parser for Bracket{
    fn parse(&self, regex: &str, ctx: &mut ParseContext){
        ctx.pos+=1;
        let mut literals: Vec<String> = Vec::new();
        while regex.as_bytes()[ctx.pos] != b']'{
            let ch = regex.as_bytes()[ctx.pos];
            if ch == b'-'{
                let next = regex.as_bytes()[ctx.pos+1];
                let prev = literals[literals.len()-1].as_bytes()[0];
                let str = format!("{}{}", prev, next);
                literals.insert(literals.len(),str);
                ctx.pos+=1;
            }
            else{
                literals.insert(literals.len(), ch.to_string());
            }
            ctx.pos+=1;
        }
        let new_token = Token{
            token_type: TokenType::Bracket,
            value: Box::new(1),
        };
        ctx.tokens.push(new_token);
        print!("{:#?}", literals)
    }
}


pub fn process_general(regex: &str, ctx: &mut ParseContext){
    let ch = regex.chars().nth(ctx.pos);
    match ch{
        Some('(') => {
            let mut group_ctx = ParseContext{pos: ctx.pos, ..Default::default()};
            
            Process::process(Group, regex, &mut group_ctx);

            let new_token = Token{
                token_type: TokenType::Group,
                value: Box::new(group_ctx.tokens),
            };

            ctx.tokens.push(new_token);
        },
        Some('[') => {
            Process::process(Bracket, regex, ctx);

        },
        Some('|') => {
                    ctx.pos+=1;

        },
        Some('*'| '?'|'+') => {
                    ctx.pos+=1;

        },
        Some('{') => {
                    ctx.pos+=1;

        },
        _=>{
        ctx.pos+=1;

        },
    };
}