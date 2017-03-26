extern crate regex;


use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use std::collections::VecDeque;

use std::env::Args;

use regex::Regex;

fn main() {
    let input = match get_input(std::env::args()) {
        Some(x) => x,
        None => {
            println!("Specify input");
            return;
        }
    };

    let reader = BufReader::new(input);

    let tm = TM::from_bufreader(reader).unwrap();

    let tapes = tm.execute().unwrap();
    println!("{:?}", tapes);
}

#[derive(Debug)]
enum TMCreationError {
    StartStateNotSpecified,
    EndStateNotSpecified,
    StateDoesntExist(String),

    WrongLiteral,
    LetterDoesntExist,
    StartIndexNotSpecified,
    TransitionNotSpecified(String, char),
}

#[derive(Debug)]
struct TM {
    start: State,
    end: State,
    states: Vec<State>,
    alphabet: Vec<char>,
    transitions: Vec<Transition>,
    tapes: Vec<Tape>,
    config: Config,
}

#[derive(Debug, Clone)]
struct Tape {
    default: char,
    start_pos: usize,
    band: VecDeque<char>,
}

impl TM {
    fn from_bufreader(reader: BufReader<File>) -> Result<TM, TMCreationError> {
        let mut states = Vec::new();
        let mut alphabet = Vec::new();
        let mut transitions = Vec::new();

        let mut start = State { name: "".into() };
        let mut end = State { name: "".into() };

        let start_r = Regex::new(r"\[e\]:(.*)$").unwrap();
        let end_r = Regex::new(r"\[x\]:(.*)$").unwrap();
        let states_r = Regex::new(r"\[s\]:(.*)$").unwrap();
        let alphabet_r = Regex::new(r"\[a\]:(.*)$").unwrap();
        let trans_start_r = Regex::new(r"\[t\|([^\]]*)\]:(.*)$").unwrap();
        let trans_end_r = Regex::new(r"([^-]*)->\(([^,]*),([^,]*),([^\)]*)\)").unwrap();
        let band_r = Regex::new(r"\[b\|([^\]]*)\]:(.*)$").unwrap();

        let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().unwrap();

        // Parse all states
        for l in lines.iter() {
            if states_r.is_match(&l) {
                let ss: String = states_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
                for s in ss.split(',') {
                    states.push(State { name: s.into() });
                }
            }
        }

        // Parse start state
        let mut found = false;
        for l in lines.iter() {
            if start_r.is_match(&l) {
                start.name = start_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
                found = true;
            }
        }

        if !found {
            return Err(TMCreationError::StartStateNotSpecified);
        }

        if !state_exists(&states, &start.name) {
            return Err(TMCreationError::StateDoesntExist(start.name.clone()));
        }

        // Parse terminating state
        let mut found = false;
        for l in lines.iter() {
            if end_r.is_match(&l) {
                end.name = end_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
                found = true;
            }
        }

        if !found {
            return Err(TMCreationError::EndStateNotSpecified);
        }

        if !state_exists(&states, &end.name) {
            return Err(TMCreationError::StateDoesntExist(end.name.clone()));
        }

        // Parse the alphabet
        for l in lines.iter() {
            if alphabet_r.is_match(&l) {
                let chars: String = alphabet_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
                for a in chars.split(',') {
                    alphabet.push(a.chars().next().unwrap());
                }
            }
        }

        // Parse all transitions
        for l in lines.iter() {
            if trans_start_r.is_match(&l) {
                let cap = trans_start_r.captures(&l).unwrap();
                let start: String = cap[1].into();

                if !state_exists(&states, &start) {
                    return Err(TMCreationError::StateDoesntExist(start.clone()));
                }

                let ends: String = cap[2].into();

                for e in ends.split('|') {
                    if trans_end_r.is_match(&e) {
                        let end_cap = trans_end_r.captures(&e).unwrap();

                        let start_letter = end_cap[1].chars().next().unwrap();
                        let end_letter = end_cap[3].chars().next().unwrap();

                        let mov = match &end_cap[4] {
                            "<" => Move::Left,
                            ">" => Move::Right,
                            _ => return Err(TMCreationError::WrongLiteral),
                        };

                        if !state_exists(&states, &end_cap[2].into()) {
                            return Err(TMCreationError::StateDoesntExist(end_cap[2].into()));
                        }

                        if !letter_exists(&alphabet, &start_letter) || !letter_exists(&alphabet, &end_letter) {
                            return Err(TMCreationError::LetterDoesntExist);
                        }

                        let trans = Transition::new(State::new(start.clone()), start_letter, 
                                                    State::new(end_cap[2].into()), end_letter, mov);

                        transitions.push(trans);
                    }
                }

            }
        }

        let mut tapes = Vec::new();

        // Parse all bands
        for l in lines.iter() {
            if band_r.is_match(&l) {
                let cap = band_r.captures(&l).unwrap();

                let default: char = cap[1].chars().next().unwrap();

                let mut band_chars = VecDeque::new();

                let mut start_index = -1;
                for (k, i) in cap[2].chars().enumerate() {
                    if i == '[' {
                        start_index = k as i32;
                        continue;
                    }
                    if i == ']' {
                        continue;
                    }

                    band_chars.push_back(i);
                }
                if start_index == -1 {
                    return Err(TMCreationError::StartIndexNotSpecified);
                }
                let t = Tape { default: default, start_pos: start_index as usize, band: band_chars };
                tapes.push(t);
            }
        }

        let config = Config { max_steps: 1000000 };

        Ok(TM { start: start, end: end, states: states, alphabet: alphabet, 
             transitions: transitions, tapes: tapes, config: config })
    }

    fn execute(self) -> Result<Vec<Tape>, TMCreationError> {
        let mut tapes = self.tapes.clone();

'out:   for tape in tapes.iter_mut() {
            let mut state = self.start.clone();
            let mut pos = tape.start_pos;

            let mut counter = 0;
            while counter < self.config.max_steps {
                println!("{:?}", tape);
                let symbol = tape.band[pos];
                let (new_state, new_symbol, new_pos) = match self.get_transition(&state, symbol, pos) {
                    Some((x, y, z)) => (x,y,z),
                    None => return Err(TMCreationError::TransitionNotSpecified(state.name.clone(), symbol)),
                };

                state = new_state;
                tape.band[pos] = new_symbol;

                if state == self.end {
                    continue 'out;
                }

                if new_pos < 0 {
                    tape.band.insert(0, tape.default);
                } else {
                    pos = new_pos as usize;
                }

                if pos > tape.band.len()-1 {
                    let def = tape.default;
                    let len = tape.band.len();
                    tape.band.insert(len, def);
                }


                counter += 1;
            }
        }

        Ok(tapes)
    }

    fn get_transition(&self, state: &State, symbol: char, pos: usize) -> Option<(State, char, isize)> {
        for trans in self.transitions.iter() {
            if *state == trans.start.0 && trans.start.1 == symbol {
                let mut new_pos = pos as isize;
                if trans.end.2 == Move::Left {
                    new_pos -= 1;
                } else {
                    new_pos += 1;
                }
                return Some((trans.end.0.clone(), trans.end.1, new_pos));
            }
        }
        None
    }
}

fn state_exists(states: &Vec<State>, a: &String) -> bool {
    let mut exists = false;
    for s in states.iter() {
        if *s.name == *a {
            exists = true;
        }
    }

    exists
}

fn letter_exists(alphabet: &Vec<char>, a: &char) -> bool {
    let mut exists = false;
    for s in alphabet.iter() {
        if *s == *a {
            exists = true;
        }
    }

    exists
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    name: String,
}

impl State {
    fn new(name: String) -> State {
        State { name: name }
    }
}

#[derive(Debug, Clone)]
struct Transition {
    start: (State, char),
    end: (State, char, Move),
}

impl Transition {
    fn new(start: State, input: char, end: State, output: char, mov: Move) -> Transition {
        Transition { start: (start, input), end: (end, output, mov) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Config {
    max_steps: u32,
}

fn get_input(args: Args) -> Option<File> {
    if let Some(x) = args.skip(1).next() {
        match File::open(x) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    } else {
        None
    }
}

