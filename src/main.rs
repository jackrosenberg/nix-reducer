use std::{
    env,
    fmt::{Debug, Display},
    fs,
    sync::Arc,
};

pub mod parser;
pub mod types;

use crate::parser::{applicative, fmap, choice, greedy, greedy_choice, option, many, greedy1, Parser};
use crate::types::{Keyword, Operator, Punctuation, Token, TypePrimitive};

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
    // println!("{:?} ", tokens);


}

fn lex_tokens(input: &Vec<String>) -> Vec<Token> {
    // lex out the whitespaces and comments

    /// Returns True for any Unicode space character, and the control characters
    /// , "\f", "\v" are not supported by rust, lets hope this doesn't break!
    fn is_space(string: &String) -> bool {
        ["\t", "\n", "\r", " "].contains(&&string[..])
    }
    // can be nothing
    let lex_whitespace = greedy(Parser::satisfy(is_space));

    let not_newline = greedy(Parser::satisfy(|c| c != "\n"));

    let f = move |t: Vec<String>| move |cs: Vec<String>| {
        let t = t.clone();
        let cs = cs.clone();
        move |sp| {
            let t = t.clone(); let cs = cs.clone();
            [t.clone(), cs].concat()
        }
    };
    let comment_ident = vec![String::from("#")];
    let lex_comment = fmap(f, Parser::token(&comment_ident));
    let lex_comment = applicative(lex_comment, not_newline);
    let lex_comment = applicative(lex_comment, lex_whitespace.clone());



    
    // https://nix.dev/manual/nix/2.28/language/identifiers.html
    // not going to check that it's not a keyword yet, will do
    // that later if it's needed
    fn is_ident(string: &String) -> bool {
        let c = string.chars().nth(0).expect("is_ident failed"); 
        matches!(c, 
            'a'..='z' |
            'A'..='Z' |
            '0'..='9' |
            '_' | 
            '\'' |
            '-' 
        )
    }
    let ident = |ident: Vec<String>| {
        // println!("ident {:?}", &ident);
        Token::TypePrimitive(TypePrimitive::Identifier(ident.into_iter().collect::<String>()))
    } ;
    let identifier = fmap(ident, greedy1(Parser::satisfy(is_ident)));

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

    let punctuation_pairs: [(&str, Token); 10] = [
        ("(", Token::Punctuation(Punctuation::POpen)),
        (")", Token::Punctuation(Punctuation::PClose)),
        ("[", Token::Punctuation(Punctuation::SOpen)),
        ("]", Token::Punctuation(Punctuation::SOpen)),
        ("{", Token::Punctuation(Punctuation::COpen)),
        ("}", Token::Punctuation(Punctuation::CClose)),
        (",", Token::Punctuation(Punctuation::Comma)),
        (";", Token::Punctuation(Punctuation::Semicolon)),
        (":", Token::Punctuation(Punctuation::Colon)),
        (".", Token::Punctuation(Punctuation::Period)),
    ];

    let operator_pairs: [(&str, Token); 12] = [
        ("==", Token::Operator(Operator::LEq)),
        ("!=", Token::Operator(Operator::LNEq)),
        ("&&", Token::Operator(Operator::LAnd)),
        ("||", Token::Operator(Operator::LOr)),
        ("=>", Token::Operator(Operator::LImpl)),
        ("<=", Token::Operator(Operator::LessEq)),
        (">=", Token::Operator(Operator::GrEq)),
        ("=" , Token::Operator(Operator::Assign)),
        ("//", Token::Operator(Operator::Update)),
        ("++", Token::Operator(Operator::Concat)),
        ("|>", Token::Operator(Operator::PipeFrom)),
        ("<|", Token::Operator(Operator::PipeInto)),
    ];

    fn gen_key_parser(key_pair: (&'_ str, Token)) -> Parser<'_, String, Token> {
        Parser {
            parser: Arc::new(move |input: &Vec<String>| {
                let (word, token) = key_pair.clone();
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
        greedy_choice(pairs.iter().map(|c| gen_key_parser(c.clone())).collect::<Vec<_>>())
    }

    let mut lexers = 
        [
            keyword_pairs.to_vec(),
            punctuation_pairs.to_vec(),
            operator_pairs.to_vec(),
        ]
        .into_iter()
        .map(|p| lex_keyword(p))
        .collect::<Vec<_>>()
    ;
    lexers.push(identifier);

    let lex_token = greedy_choice(lexers);


    fn ignore_whitespace<F1, F2>(f: F1) -> F1
    where
        F1: Fn(Token) -> F2,
        F2: Fn(Vec<String>) -> Token
    {
        f
    }
    let ignore_whitespace = ignore_whitespace(
        |tk| {
            let tk = tk.clone();
            move |cms| {
                tk.clone()
            }
        }
    );

    fn ignore_comments<F1, F2>(f: F1) -> F1
    where
        F1: Fn(Vec<Vec<String>>) -> F2,
        F2: Fn(Token)            -> Token
    { 
        f
    }
    let ignore_comments = ignore_comments(
        |cms| {
            |tk| {
                tk
            }
        }
    );

    fn ignore_all_sans_tokens<F1, F2, F3, F4>(f: F1) -> F1
    where 
        F1: Fn(Vec<String>) -> F2,
        F2: Fn(Vec<Token>) -> F3,
        F3: Fn(Vec<Vec<String>>) -> F4,
        F4: Fn(()) -> Vec<Token>
    { 
        f
    }
    let ignore_all_sans_tokens = ignore_all_sans_tokens(
        |wh| {
            |tks| {
                move |cms| {
                    let tks = tks.clone();
                    move |eof| {
                        tks.clone()
                    }
                }
            }
        }
    );

    // parse and ignore all comments
    let l_comments = greedy(lex_comment.clone());

    // parse and ignore all comments and spaces
    // let l_cns = greedy(choice(lex_comment.clone(), lex_whitespace.clone()));

    let l_tokens = fmap(ignore_whitespace, lex_token.clone());
    let l_tokens = applicative(l_tokens, lex_whitespace.clone());

    let parser = fmap(ignore_comments, l_comments.clone());
    let parser = applicative(parser, l_tokens.clone());
    let parser = greedy(parser);

    // let final_parser = fmap(ignore_all_sans_tokens, lex_whitespace.clone());
    // let final_parser = applicative(final_parser, parser.clone());
    // let final_parser = applicative(final_parser, l_comments.clone());
    // // let final_parser = applicative(final_parser, Parser::eof());
    // let final_parser = applicative(final_parser, Parser::succeed(()));

    // println!("fin {:?}", final_parser.run(input)[0].0);

    let tmp = lex_whitespace.clone().run(input);
    // let tmp = l_comments.clone().run(&tmp[0].1);
    // let tmp = lex_comment.clone().run(&tmp[0].1);
    let tmp = parser.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    // let tmp = l_tokens.clone().run(&tmp[0].1);
    println!("tmp {:?}", tmp[0].0);


    // final_parser.run(input)[0].0.clone()
    todo!()
}
