use std::{env, fs};

pub mod parser;
pub mod types;

use crate::parser::{
    applicative,
    applicative_l,
    fmap,
    many, many1,
    greedy,
    Parser
};
use crate::types::TOKEN;

fn main() {
    // take in command line args
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let path = "../nixpkgs/nixos/modules/services/web-servers/traefik.nix";
    // println!("path '{}'", path);

    let contents = fs::read_to_string(path)
        .expect("Unable to read path ")
        .chars()
        .map(|c| char::to_string(&c))
        .collect::<Vec<_>>();

    // println!("{:?} ", parser.run(&contents));

    // println!("{:?} ", contents);
    // let tokens = lex_tokens(contents);
}

fn lex_tokens(input: Vec<String>) -> Vec<types::TOKEN> {
    let not_whitespace = greedy(Parser::satisfy(|c| &c != "\n"));
    let lex_comments = applicative_l(Parser::token(&vec![String::from("#")]), not_whitespace);
    todo!()
}
