extern crate regex;


use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::env::Args;

use regex::Regex;

fn main() {
    let input = get_input(std::env::args()).unwrap();

    let tm = TM::from_file(input);

    println!("{:?}", tm);
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

#[derive(Debug)]
struct TM {
    start: State,
    states: Vec<State>,
    alphabet: Vec<char>,
    transitions: Vec<Transition>,
    config: Config,
}

impl TM {
    fn from_file(file: File) -> TM {
        let buf_reader = BufReader::new(file);

        let mut states = Vec::new();
        let mut alphabet = Vec::new();
        let mut transitions = Vec::new();

        let mut start = State { name: "".into() };

        let comment_r = Regex::new(r"#").unwrap();
        let start_r = Regex::new(r"\[e\]:(.*)$").unwrap();
        let states_r = Regex::new(r"\[s\]:(.*)$").unwrap();
        let alphabet_r = Regex::new(r"\[a\]:(.*)$").unwrap();
        let trans_r = Regex::new(r"\[t\]:\(([^,]*),([^\)*])\)->\(([^,]*),([^,]*),([^\)]*)\)").unwrap();

        for line in buf_reader.lines() {
            let l = line.unwrap();

            if l.len() == 0 {
                continue;
            }
            
            if comment_r.is_match(&l) {
                continue;
            }

            if states_r.is_match(&l) {
                let ss: String = states_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
                for s in ss.split(',') {
                    states.push(State { name: s.into() });
                }
            }

            if alphabet_r.is_match(&l) {
                let chars: String = alphabet_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
                for a in chars.split(',') {
                    alphabet.push(a.chars().next().unwrap());
                }
            }

            if start_r.is_match(&l) {
                start.name = start_r.captures(&l).unwrap().get(1).unwrap().as_str().into();
            }

            if trans_r.is_match(&l) {
                let cap = trans_r.captures(&l).unwrap();
                let mov = match &cap[5] {
                    "<" => Move::Left,
                    ">" => Move::Right,
                    _ => panic!("Wrong symbol"),
                };

                let trans = Transition {start: (State { name: cap[1].into() }, cap[2].chars().next().unwrap()),
                                        end: (State { name: cap[3].into() }, cap[4].chars().next().unwrap(), mov) };  

                transitions.push(trans);
            }

        }

        let config = Config { max_steps: 100 };

        TM { start: start, states: states, alphabet: alphabet, 
             transitions: transitions, config: config }
    }
}

#[derive(Debug, Clone)]
struct State {
    name: String,
}

#[derive(Debug, Clone)]
struct Transition {
    start: (State, char),
    end: (State, char, Move),
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
