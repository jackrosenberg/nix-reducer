use std::{
    fmt::{Debug, Display},
    sync::Arc,
    env,
    fs
};

pub mod parser;
pub mod types;

use crate::parser::{
    applicative,
    fmap,
    many, many1,
    greedy,
    greedy_choice,
    Parser
};
use crate::types::{Token, Punctuation, Keyword, Operator};

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
        ["\t", "\n", "\r"].contains(&&string[..])
    }
    let lex_whitespace = greedy(Parser::satisfy(is_space));

    let not_whitespace = greedy(Parser::satisfy(|c| c != "\n"));
    let f = move |t: Vec<String> | move |cs: Vec<String>| [t.clone(),cs].concat();
    let comment_ident = vec![String::from("#")];
    let lex_comments = applicative(fmap(f, Parser::token(&comment_ident)), not_whitespace);

    // god this is verbose and ugly
    let keyword_pairs: [(&str, Token); 11] = [
        ("if",      Token::Keyword(Keyword::If)),
        ("then",    Token::Keyword(Keyword::Then)),
        ("else",    Token::Keyword(Keyword::Else)),
        ("assert",  Token::Keyword(Keyword::Assert)),
        ("with",    Token::Keyword(Keyword::With)),
        ("let",     Token::Keyword(Keyword::Let)),
        ("in",      Token::Keyword(Keyword::In)),
        ("rec",     Token::Keyword(Keyword::Rec)),
        ("inherit", Token::Keyword(Keyword::Inherit)),
        ("or",      Token::Keyword(Keyword::Or)),
        ("...",     Token::Keyword(Keyword::Ellipsis)),

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
                let (word,token) = key_pair;
                    // ex ["r","e","c"];
                if input[..word.len()] == word.chars().map(|c| c.to_string()).collect::<Vec<String>>() {
                    vec![(token, input[word.len()..].to_vec())]
                }
                else {
                    Parser::empty().run(input)
                }
               
            })
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
        .into_iter().map(|p| lex_keyword(p)).collect::<>()
    );                   
                         
    let f = |_: Vec<String> | |_: Vec<String>| |tkns: Vec<Token>| { move |cms2: Vec<String>| move |tkns2: Vec<Token>| {
        // println!("{:?} ", cmts.clone());
        // println!("{:?} ", spcs.clone());
        // println!("{:?} ", tkns.clone());
        let a = tkns.clone();  
        [ a, tkns2.clone()].concat()
    }};

    let parser = fmap(f, lex_comments);
    let parser = applicative(parser, lex_whitespace.clone());
    let parser = applicative(parser, greedy(lex_token.clone()));
    let parser = applicative(parser, lex_whitespace.clone());
    let parser = applicative(parser, greedy(lex_token.clone()));

    println!("{:?}", greedy(parser).run(input)[0].0);

    todo!()

}


