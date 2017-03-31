extern crate regex;

mod input;
mod tm;

use input::get_input;
use std::io::BufReader;
use std::io::prelude::*;

use tm::{TM, Tape};

fn main() {
    let arg = match get_input(std::env::args()) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut reader = BufReader::new(arg);
    let mut input = String::new();

    match reader.read_to_string(&mut input) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut tm = match TM::from_string(input.clone()) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let tapes = match Tape::from_string(input) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let mut counter = 1;
    for t in tapes {
        let tape = tm.execute(t).unwrap();
        println!("Tape {}: {}", counter, tape);

        counter += 1;
    }
}
