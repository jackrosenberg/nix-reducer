use std::{
    env,
    fmt::{Debug, Display},
    fs,
    sync::Arc,
};

pub mod parser;
pub mod types;

use crate::parser::{applicative, fmap, greedy, greedy_choice, many, many1, Parser};
use crate::types::{Keyword, Operator, Punctuation, Token};

fn main() {
    // take in command line args
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let path = "./example.nix";
    // println!("path '{}'", path);

    let contents = fs::read_to_string(path)
        .expect("Unable to read path ")
        .chars()
        .map(|c| char::to_string(&c))
        .collect::<Vec<_>>();

    // println!("{:?} ", parser.run(&contents));

    // println!("{:?} ", contents);
    let tokens = lex_tokens(&contents);
}

fn lex_tokens(input: &Vec<String>) -> Vec<types::Token> {
    // lex out the whitespaces and comments

    /// Returns True for any Unicode space character, and the control characters
    /// , "\f", "\v" are not supported by rust, lets hope this doesn't break!
    fn is_space(string: &String) -> bool {
        ["\t", "\n", "\r", " "].contains(&&string[..])
    }
    let lex_whitespace = greedy(Parser::satisfy(is_space));

    let not_newline = greedy(Parser::satisfy(|c| c != "\n"));
    let f = move |t: Vec<String>| move |cs: Vec<String>| [t.clone(), cs].concat();
    let comment_ident = vec![String::from("#")];
    let lex_comments = applicative(fmap(f, Parser::token(&comment_ident)), not_newline);

    // god this is verbose and ugly
    let keyword_pairs: [(&str, Token); 11] = [
        ("if", Token::Keyword(Keyword::If)),
        ("then", Token::Keyword(Keyword::Then)),
        ("else", Token::Keyword(Keyword::Else)),
        ("assert", Token::Keyword(Keyword::Assert)),
        ("with", Token::Keyword(Keyword::With)),
        ("let", Token::Keyword(Keyword::Let)),
        ("in", Token::Keyword(Keyword::In)),
        ("rec", Token::Keyword(Keyword::Rec)),
        ("inherit", Token::Keyword(Keyword::Inherit)),
        ("or", Token::Keyword(Keyword::Or)),
        ("...", Token::Keyword(Keyword::Ellipsis)),
    ];

    let punctuation_pairs: [(&str, Token); 8] = [
        ("(", Token::Punctuation(Punctuation::POpen)),
        (")", Token::Punctuation(Punctuation::PClose)),
        ("[", Token::Punctuation(Punctuation::SOpen)),
        ("]", Token::Punctuation(Punctuation::SOpen)),
        ("{", Token::Punctuation(Punctuation::COpen)),
        ("}", Token::Punctuation(Punctuation::CClose)),
        (",", Token::Punctuation(Punctuation::Comma)),
        (";", Token::Punctuation(Punctuation::Semicolon)),
    ];

    let operator_pairs: [(&str, Token); 11] = [
        ("==", Token::Operator(Operator::Eq)),
        ("!=", Token::Operator(Operator::Neq)),
        ("<=", Token::Operator(Operator::Leq)),
        (">=", Token::Operator(Operator::Geq)),
        ("&&", Token::Operator(Operator::Land)),
        ("||", Token::Operator(Operator::Lor)),
        ("=>", Token::Operator(Operator::Impl)),
        ("//", Token::Operator(Operator::Update)),
        ("++", Token::Operator(Operator::Concat)),
        ("|>", Token::Operator(Operator::PipeFrom)),
        ("<|", Token::Operator(Operator::PipeInto)),
    ];

    fn gen_key_parser(key_pair: (&'_ str, Token)) -> Parser<'_, String, Token> {
        Parser {
            parser: Arc::new(move |input: &Vec<String>| {
                let (word, token) = key_pair;
                // ex ["r","e","c"];
                if input[..word.len()]
                    == word.chars().map(|c| c.to_string()).collect::<Vec<String>>()
                {
                    vec![(token, input[word.len()..].to_vec())]
                } else {
                    Parser::empty().run(input)
                }
            }),
        }
    }

    fn lex_keyword(pairs: Vec<(&'_ str, Token)>) -> Parser<'_, String, Token> {
        greedy_choice(pairs.iter().map(|c| gen_key_parser(*c)).collect::<Vec<_>>())
    }

    let lex_token = greedy_choice(
        // list of constructors for tokens
        [
            keyword_pairs.to_vec(),
            punctuation_pairs.to_vec(),
            punctuation_pairs.to_vec(),
            operator_pairs.to_vec(),
        ]
        .into_iter()
        .map(|p| lex_keyword(p))
        .collect(),
    );

    fn func<F1, F2, F3, F4, F5>(f: F1) -> F1
    where
        F1: Fn(Vec<String>) -> F2,
        F2: Fn(Vec<String>) -> F3,
        F3: Fn(Token)       -> F4,
        F4: Fn(Vec<String>) -> F5,
        F5: Fn(Token)       -> Vec<Token>,
    {
        f
    }

    let f = func(|a: Vec<String>| {
        // let a = a.clone();
        // println!("a: {:?}", a);
        |b: Vec<String>| {
            // println!("b: {:?}", b);
            |tkns: Token| {
                // println!("tk1: {:?}", tkns);
                move |c: Vec<String>| {
                    // println!("c: {:?}", c);
                    let tkns = tkns.clone();
                    move |tkns2: Token| {
                        let tkns2 = tkns2.clone();
                        // println!("rks2: {:?}", tkns2.clone());
                        vec![tkns, tkns2]
                    }
                }
            }
        }
    });

    let parser = fmap(f, lex_comments);
    let parser = applicative(parser, lex_whitespace.clone());
    let parser = applicative(parser, lex_token.clone());
    let parser = applicative(parser, lex_whitespace.clone());
    let parser = applicative(parser, lex_token.clone());

    println!("{:?}", parser.run(input)[0].0);

    todo!()
}
