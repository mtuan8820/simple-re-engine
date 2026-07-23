use std::{collections::HashMap, f32::consts::E};

use crate::{parser::ParseContext, token::{RepeatPayload, Token, TokenType}};

#[derive(Default)] 
pub struct State{
    start: bool,
    terminate: bool,
    transitions: HashMap<u8, Vec<usize>>,
}

const EPSILON: u8 =0;

#[derive(Default)] 
pub struct NFA{
    pub states: Vec<State>
}

trait StateMachine{
   fn to_nfa(token: &Token, states: &mut Vec<State>, start_pos: usize, end_pos: usize) -> (usize, usize);
}

fn token_to_nfa( token: &Token, states: &mut Vec<State>) -> (usize, usize){
    let start_pos = init_state(states);
    let end_pos = init_state(states);

    match token.token_type{
        TokenType::Or => Or::to_nfa(token, states, start_pos, end_pos),
        TokenType::Repeat => Repeat::to_nfa(token, states, start_pos, end_pos),
        TokenType::Literal => Literal::to_nfa(token, states, start_pos, end_pos),
        TokenType::Bracket => Bracket::to_nfa(token, states, start_pos, end_pos),
        TokenType::Group | TokenType::GroupUncaptured => Group::to_nfa(token, states, start_pos, end_pos),
    }
}

fn init_state(states: &mut Vec<State>) -> usize{
    let state = State{..Default::default()};
    let pos = states.len();
    states.push(state);
    pos
} 

struct Literal;
impl StateMachine for Literal{
   fn to_nfa(token: &Token, states: &mut Vec<State>, start_pos: usize, end_pos: usize) -> (usize, usize){
    let ch = token.value
        .downcast_ref::<char>()
        .unwrap();
    let start = &mut states[start_pos];
    start.transitions.insert(*ch as u8, vec![end_pos]);
    (start_pos, end_pos)
   }
}

struct Or;
impl StateMachine for Or{
   fn to_nfa(token: &Token, states: &mut Vec<State>, start_pos: usize, end_pos: usize) -> (usize, usize){

    let values = token.value.downcast_ref::<Vec<Token>>().unwrap();
    let (left, right) = (&values[0], &values[1]);
    let (left_start_pos, left_end_pos) = token_to_nfa(left, states);
    let (right_start_pos, right_end_pos) = token_to_nfa(right, states);

    states[start_pos].transitions.insert(EPSILON, vec![left_start_pos, right_start_pos]);
    states[left_end_pos].transitions.insert(EPSILON, vec![end_pos]);
    states[right_end_pos].transitions.insert(EPSILON,vec![end_pos]);

    (start_pos, end_pos)
   }
}

struct Bracket;
impl StateMachine for Bracket{
   fn to_nfa(token: &Token, states: &mut Vec<State>, start_pos: usize, end_pos: usize) -> (usize, usize){

    let literals = token.value.downcast_ref::<HashMap<u8, bool>>().unwrap();

    for (k, _v) in literals {
        let _key = *k as char;
        states[start_pos].transitions.insert(*k, vec![end_pos]);
        // let _a = match states[start_pos].transitions.get(k) {
        //     Some(v) => {v},
        //     None => panic!("error"),
        // };
        // print!("{:#?}", _a);
    }

    (start_pos, end_pos)
   }
}

struct Group;
impl StateMachine for Group{
    fn to_nfa(token: &Token, states: &mut Vec<State>, start_pos: usize, end_pos: usize) -> (usize, usize){
        let tokens = token.value.downcast_ref::<Vec<Token>>().unwrap();

        let (start_pos, mut end_pos) = token_to_nfa(&tokens[0], states);

        for i in 1..tokens.len(){
            let (temp_start_pos, temp_end_pos) = token_to_nfa(&tokens[i], states);
            states[end_pos].transitions.entry(EPSILON).or_default().push(temp_start_pos);
            end_pos = temp_end_pos;
        }
        
        (start_pos, end_pos)
    }
}

struct Repeat;
impl StateMachine for Repeat{
    fn to_nfa(token: &Token, states: &mut Vec<State>, start_pos: usize, end_pos: usize) -> (usize, usize){
        let payload = token.value.downcast_ref::<RepeatPayload>().unwrap();

        if payload.min == 0{
            states[start_pos].transitions.entry(EPSILON).or_insert(vec![end_pos]);
        }

        let copy_count: i8;
        if payload.max == -1{
            if payload.min == 0{
                copy_count = 1;
            } else {
                copy_count = payload.min;
            }
        } else {
            copy_count = payload.max;
        }

        let (mut from, mut to) = token_to_nfa(&payload.token, states);
        states[start_pos].transitions.entry(EPSILON).or_default().push(from);

        for i in 2..copy_count+1{
            let (s, e) = token_to_nfa(&payload.token, states);
            states[to].transitions.entry(EPSILON).or_default().push(s);

            to = e;
            from = s;

            if i > payload.min{
                states[s].transitions.entry(EPSILON).or_default().push(to);
            }
        }

        states[to].transitions.entry(EPSILON).or_default().push(end_pos);

        if payload.max == -1{
            states[end_pos].transitions.entry(EPSILON).or_default().push(from);
        }

        (start_pos, end_pos)
    }
}


impl NFA{
    pub fn context_to_nfa(&mut self, ctx: &ParseContext) -> usize{
        let (start_pos, mut end_pos) = token_to_nfa(&ctx.tokens[0],&mut self.states);
        //todo, make start state 's start = true, end state 's temniate = false
        self.states[start_pos].start = true;

        for i in 1..ctx.tokens.len(){
            let (start_next, end_next) = token_to_nfa(&ctx.tokens[i], &mut self.states);

            //connect to NFA by create a transition
            //from last node of prev NFA with the first node of next NFA
            self.states[end_pos].transitions.entry(EPSILON).or_default().push(start_next);

            //update pointer
            end_pos = end_next;
        }

        self.states[end_pos].terminate = true;

        start_pos
    }

    /// Run `input` against this NFA. Returns true if it is accepted.
    pub fn matches(&self, input: &str) -> bool {
        let start_pos = match self.states.iter().position(|s| s.start) {
            Some(pos) => pos,
            None => return false,
        };
        self.states[start_pos].check(input, 0, &self.states)
    }
}

const START_OF_TEXT: u8 = 1;
const END_OF_TEXT: u8 = 2;

trait Check{
    fn check(&self, input: &str, pos: usize, states: &Vec<State>) -> bool;
}

fn get_char(input: &str, pos: usize) -> u8{
    if pos >= input.len(){
        return END_OF_TEXT
    } 
    if pos < 0 {
        return START_OF_TEXT
    }
    match input.chars().nth(pos){
        Some(c) => c as u8,
        None => panic!("empty string")
    }
}

impl Check for State {
        fn check(&self, input: &str, pos: usize, states: &Vec<State>) -> bool{
            let ch = get_char(input, pos);
            if ch == END_OF_TEXT && self.terminate{
                return true
            }

            if let Some(next_states) = self.transitions.get(&ch){
                for next_state_pos in next_states{
                    let next_state = &states[*next_state_pos];
                    if next_state.check(input, pos+1, &states){
                        return true
                    }
                }
            }

            if let Some(epsilon_states) = self.transitions.get(&EPSILON){
                for i in epsilon_states{
                    let state = &states[*i];
                    if state.check(input, pos, &states){
                        return true
                    }

                    if ch == START_OF_TEXT && state.check(input, pos+1, &states){
                        return true
                    }
                }
            }

            false
        }

}
