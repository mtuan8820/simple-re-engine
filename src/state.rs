use std::collections::HashMap;

use crate::{parser::ParseContext, token::Token};

#[derive(Default)] 
pub struct State{
    pos: usize,
    start: bool,
    terminate: bool,
    transitions: HashMap<u8, Vec<usize>>,
}

const EPSILON: u8 =0;

#[derive(Default)] 
pub struct NFA{
    states: Vec<State>
}

impl NFA{

    fn token_to_nfa(token: &Token) -> (State, State){
        todo!();
    }

    fn to_nfa(&mut self, ctx: &ParseContext) -> usize{
        let (start_state, mut end_state) = Self::token_to_nfa(&ctx.tokens[0]);
        
        let start_index = self.states.len();
        let start = State{
            pos: start_index,
            start: true,
            transitions: HashMap::from([(EPSILON, vec![start_state.pos])]),
            ..Default::default()
        };
        self.states.push(start);

        for i in 1..ctx.tokens.len(){
            let (start_next, end_next) = Self::token_to_nfa(&ctx.tokens[i]);

            //connect to NFA by create a transition
            //from last node of prev NFA with the first node of next NFA
            end_state.transitions.entry(EPSILON).or_default().push(start_next.pos);

            //update pointer
            end_state = end_next;
        }

        let end = State{
            pos: self.states.len(),
            terminate: true,
            ..Default::default()
        };

        end_state.transitions.entry(EPSILON).or_default().push(end.pos);
        self.states.push(end);

        start_index
    }
}
