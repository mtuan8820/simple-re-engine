mod parser;
mod token;

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
    parse(&"[a-zA-Z][a-zA-Z0-9_.]+@[a-zA-Z0-9]+.[a-zA-Z]{2,}".to_string());
}

