extern crate regex;


use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;

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

    println!("{:#?}", tm);
}

#[derive(Debug)]
enum TMCreationError {
    StartStateNotSpecified,
    StateDoesntExist,

    WrongLiteral,
    LetterDoesntExist,
}

#[derive(Debug)]
struct TM {
    start: State,
    states: Vec<State>,
    alphabet: Vec<char>,
    transitions: Vec<Transition>,
    config: Config,
}

impl TM {
    fn from_bufreader(reader: BufReader<File>) -> Result<TM, TMCreationError> {
        let mut states = Vec::new();
        let mut alphabet = Vec::new();
        let mut transitions = Vec::new();

        let mut start = State { name: "".into() };

        let start_r = Regex::new(r"\[e\]:(.*)$").unwrap();
        let states_r = Regex::new(r"\[s\]:(.*)$").unwrap();
        let alphabet_r = Regex::new(r"\[a\]:(.*)$").unwrap();
        let trans_start_r = Regex::new(r"\[t\|([^\]]*)\]:(.*)$").unwrap();
        let trans_end_r = Regex::new(r"([^-]*)->\(([^,]*),([^,]*),([^\)]*)\)").unwrap();

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
            return Err(TMCreationError::StateDoesntExist);
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
                    return Err(TMCreationError::StateDoesntExist);
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
                            return Err(TMCreationError::StateDoesntExist);
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

        let config = Config { max_steps: 100 };

        Ok(TM { start: start, states: states, alphabet: alphabet, 
             transitions: transitions, config: config })
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

