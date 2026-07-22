use crate::state::NFA;

mod parser;
mod token;
mod state;

fn parse(mut regex: &str ) -> parser::ParseContext{
    let mut ctx = parser::ParseContext{
        ..Default::default()
    };
    loop  {
        if ctx.pos >= regex.len() {break;}
        parser::process_general(&mut regex, &mut ctx);
        println!("{} {:?}", ctx.pos, regex.chars().nth(ctx.pos));
        ctx.pos+=1;
    }
    ctx
}   

fn main() {
    let ctx = parse(&"[a-zA-Z][a-zA-Z0-9_.]+@[a-zA-Z  0-9]+.[a-zA-Z]{2,}".to_string());
    let mut nfa = NFA{..Default::default()};
    nfa.context_to_nfa(&ctx);
}
