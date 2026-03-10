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
use crate::types::{Token, Punctuation, Keyword};

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
    let lex_keyword: Parser<String, Token> = greedy_choice(
        keyword_pairs.iter().map(|c| gen_key_parser(*c)).collect::<Vec<_>>()
    );

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

    // let lex_token = greedy_choice(
    //     // list of constructors for tokens
    //     vec![
    //         lex_keyword
    //         fmap(keyword, Parser::token(""))
    //     ]
    // )

    let f = |cmts: Vec<String> | |spcs: Vec<String>| move |tkns: Vec<Token>| {
        // println!("{:?} ", cmts.clone());
        println!("{:?} ", spcs.clone());
        println!("{:?} ", tkns.clone());
    };

    let parser = fmap(f, lex_comments.clone());
    let parser = applicative(parser, lex_whitespace.clone());
    let parser = applicative(parser, greedy(lex_keyword));
    println!("{:?}", parser.run(input)[0].0);

    todo!()

}
