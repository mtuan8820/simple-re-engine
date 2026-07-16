use core::panic;
use std::collections::HashMap;
use crate::token::{
    TokenType,
    Token,
    RepeatPayload
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
                let _i = ctx.pos+1;
                let next = regex.as_bytes()[ctx.pos+1] as char;                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
                let _len = literals.len()-1;
                let prev = literals[literals.len()-1].clone() ;
                let str = format!("{}{}", prev, next);
                literals.pop();
                literals.insert(literals.len(), str);
                ctx.pos+=1;
            }
            else{
                literals.insert(literals.len(), (ch as char).to_string());
            }
            ctx.pos+=1;
        }

        let mut literal_set: HashMap<u8, bool> = HashMap::new();

        for l in literals.iter(){
            for i in l.chars() {
                literal_set.insert(i as u8, true);
            } 
        }
        println!("literal_set: {:#?}", literal_set);

        let new_token = Token{
            token_type: TokenType::Bracket,
            value: Box::new(literal_set),
        };

        ctx.tokens.push(new_token);

    }
}

struct Or;
impl Parser for Or{
    fn parse(&self, regex: &str, ctx: &mut ParseContext){
        let mut rhs_context = ParseContext{
            pos: ctx.pos,
            ..Default::default()
        };

        rhs_context.pos+=1;
        while rhs_context.pos < regex.len() && regex.as_bytes()[rhs_context.pos] != b')'{
            process_general(regex, ctx);
            rhs_context.pos+=1;
        }

        let left = Token{
            token_type: TokenType::GroupUncaptured,
            value: Box::new(std::mem::take(&mut ctx.tokens))
        };

        let right: Token = Token{
            token_type: TokenType::GroupUncaptured,
            value: Box::new(rhs_context.tokens)
        };

        ctx.pos = rhs_context.pos;

        let new_token = Token{
            token_type: TokenType::Or,
            value: Box::new(vec![left, right])
        };

        let tokens: Vec<Token> = vec![new_token];

        ctx.tokens = tokens;
    }
}

struct Repeat;
impl Parser for Repeat{
    fn parse(&self, regex: &str, ctx: &mut ParseContext){
        let ch = regex.as_bytes()[ctx.pos] as char;
        let min: i8; 
        let max: i8;

        if ch == '*'{
            min = 0;
            max = -1;
        } else if ch == '?'{
            min = 0;
            max = 1;
        } else { //case '+'
            min = 1;
            max = -1;
        }

        let last_token = ctx.tokens.pop();
        
        let playload = RepeatPayload{
            max: max, 
            min: min,
            token: last_token.unwrap(),
        };

        let new_token = Token{
            token_type: TokenType::Repeat,
            value: Box::new(playload)
        };

        ctx.tokens.push(new_token);


    }
}

struct RepeatSpecified;
impl Parser for RepeatSpecified{
    fn parse(&self, regex: &str, ctx: &mut ParseContext) {
        let start = ctx.pos+1;
        while(regex.as_bytes()[ctx.pos] != b'}'){
            ctx.pos+=1;
        }

        let boundary_str = &regex[start..ctx.pos];
        let pieces:Vec<&str> = boundary_str.split(',').collect();
        let min: i8;
        let max: i8;
        if pieces.len() == 1{
            let value = match pieces[0].parse::<i8>(){
                Ok(n) => n,
                Err(e) => panic!("error parsing string to int {}", e),
            };

            min = value;
            max = value;
        } else if pieces.len() == 2{
            let value = match pieces[0].parse::<i8>(){
                Ok(n) => n,
                Err(e) => panic!("error parsing string to int {}", e),
            };
            min = value;

            if pieces[1] == ""{
                max = -1
            } else {
                max = match pieces[1].parse::<i8>(){
                    Ok(n) => n,
                    Err(e) => panic!("error parsing string to int {}", e),
                };
            }
        } else {
            panic!("There must be either 1 or 2 values specified for the quantifier: provided {}", boundary_str);
        }

        let last_token = ctx.tokens.pop().unwrap();

        let payload = RepeatPayload{
            min: min,
            max: max,
            token: last_token
        };

        let new_token = Token{
            token_type: TokenType::Repeat,
            value: Box::new(payload),
        };

        ctx.tokens.push(new_token);
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
           Process::process(Or, regex, ctx);
        },
        Some('*'| '?'|'+') => {
            Process::process(Repeat, regex, ctx);
        },
        Some('{') => {
            Process::process(RepeatSpecified, regex, ctx);
        },
        _=>{
            let new_token = Token{
                token_type: TokenType::Literal,
                value: Box::new(ch)
            };

            ctx.tokens.push(new_token);
        },
    };
}
