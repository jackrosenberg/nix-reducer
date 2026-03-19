use std::{
    env,
    fmt::{Debug, Display},
    fs,
    sync::Arc,
};

pub mod parser;
pub mod types;

use crate::parser::{applicative, biased_choice, choice, fmap, greedy, greedy1, greedy_choice, greedy_until, many, option, Parser};
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
    let tokens = lex_tokens(&contents[..]);
    println!("{:?} ", tokens);


}

fn lex_tokens(input: &[String]) -> Vec<Token> {

    let identifier = {
        // https://nix.dev/manual/nix/2.28/language/identifiers.html
        // not going to check that it's not a keyword yet, will do
        // that later if it's needed
        fn is_ident_start(string: &String) -> bool { 
            let c = string.chars().next().expect("is_ident_start failed"); 
            matches!(c, 
                'a'..='z' |
                'A'..='Z' |
                '0'..='9' |
                '_'
            )
        }
        fn is_ident(string: &String) -> bool {
            let c = string.chars().next().expect("is_ident failed"); 
            matches!(c, 
                'a'..='z' |
                'A'..='Z' |
                '0'..='9' |
                '_' | 
                '\'' |
                '-' 
            )
        }
        let identifier = |ident_start: String| {
            move |rest: Vec<String>| {
                let res = format!("{}{}", ident_start.clone(), (rest.into_iter().collect::<String>()));
                Token::TypePrimitive(TypePrimitive::Identifier(res))
            }
        };
        let identifier = fmap(identifier, Parser::satisfy(is_ident_start));
        applicative(identifier, greedy(Parser::satisfy(is_ident)))
    };

    let string_literal = {
        // todo interpolation elems
        // https://nix.dev/manual/nix/2.28/language/string-literals.html
        fn is_str_char(string: &String) -> bool {
            // not allowed to match these chars
            if let Some(c) = string.chars().next() {
                // return !matches!(c, '\"' | '\\' | '$')
                return true
            }
            unreachable!();
        }

        fn is_indented_str_char(string: &String) -> bool {
            // voodoo
            if let Some(c) = string.chars().next() {
                // return !matches!(c, '\'' | '\\' | '$')
                return true
            }
            unreachable!();
        }
        let string_lit = move |open_quotes: Vec<String>| {
            let open_quotes = open_quotes.clone();
            move |rest_including_close_quotes: Vec<Vec<String>>| {
                Token::TypePrimitive(TypePrimitive::String(
                    format!("{}{}", 
                        open_quotes.clone().into_iter().collect::<String>(),
                        rest_including_close_quotes.into_iter().flatten().collect::<String>())
                ))
            }
        };

        let double_quotes = Parser::token(vec![String::from("\"")]);
        let two_single_quotes = Parser::token(vec![String::from("\'"); 2]);

        fn lex_till_end_str_lit<'a>(repeater: Parser<'a, String, Vec<String>>, terminator: Parser<'a, String, Vec<String>>) -> Parser<'a, String, Vec<Vec<String>>> {
            greedy_until(terminator, repeater)
        }

        let string_literal = {
            let string_literal = fmap(string_lit, double_quotes.clone());
            applicative(string_literal, lex_till_end_str_lit(double_quotes.clone(), fmap(|r| vec![r], Parser::satisfy(is_str_char))))
        };

        let string_literal_indented = {
            let string_literal_indented = fmap(string_lit, two_single_quotes.clone());
            applicative(string_literal_indented, lex_till_end_str_lit(two_single_quotes.clone(), fmap(|r| vec![r], Parser::satisfy(is_indented_str_char))))
        };

        choice(string_literal, string_literal_indented)
    };

    let interpolation_elem = {
        let f = move |t: Vec<String>|
            move |cs: Vec<String>| {
               let t = t.clone();
               Token::TypePrimitive(TypePrimitive::InterpolationElement(
                   [t, cs].concat().into_iter().collect::<String>()
               ))
        };
        let parser = fmap(f, Parser::token(vec![String::from("$"), String::from("{")]));
        applicative(parser, Parser::greedy_stack_symbol(String::from("}")))
    };

    let path_literal = {
        fn is_path_char(string: &String) -> bool {
            let c = string.chars().next().expect("is_ident_start failed"); 
            matches!(c, 
                'a'..='z' |
                'A'..='Z' |
                '0'..='9' |
                '.' |
                '_' |
                '-' |
                '+'
            )
        }
        let f = move |path_chars: Vec<String>| {
            let path_chars = path_chars.clone();
            move |slash_path_chars: Vec<Vec<String>>| {
                let path_chars = path_chars.clone();
                let slash_path_chars = slash_path_chars.clone();
                move |opt_slash: String| {
                    let path_chars = path_chars.clone();
                    let slash_path_chars = slash_path_chars.clone();
                    let opt_slash = opt_slash.clone();
                    Token::TypePrimitive(TypePrimitive::Path(
                        [path_chars, slash_path_chars.into_iter().flatten().collect::<Vec<String>>(), vec![opt_slash]].concat().into_iter().collect::<String>()
                    ))
                }
            }
        };

        let g = move |t: String|
            move |cs: Vec<String>| {
                let t = t.clone();
                let cs = cs.clone();
                [vec![t], cs].concat()
        };

        let path_chars = Parser::satisfy(is_path_char);

        let sub_parser = fmap(g, Parser::symbol(String::from("/")));
        let sub_parser = applicative(sub_parser.clone(), greedy1(path_chars.clone()));

        let parser = fmap(f, greedy(path_chars.clone()));
        let parser = applicative(parser, greedy1(sub_parser.clone()));
        applicative(parser, option(Parser::symbol(String::from("/")), String::from("")))
    };

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

    let operator_pairs: [(&str, Token); 13] = [
        ("==", Token::Operator(Operator::LEq)),
        ("!=", Token::Operator(Operator::LNEq)),
        ("!", Token::Operator(Operator::LNeg)),
        ("&&", Token::Operator(Operator::LAnd)),
        ("||", Token::Operator(Operator::LOr)),
        ("->", Token::Operator(Operator::LImpl)),
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
            parser: Arc::new(move |input: &[String]| {
                let (word, token) = key_pair.clone();
                // ex ["r","e","c"];
                if let Some(chars) = input.get(..word.len()) &&
                    chars == word.chars().map(|c| c.to_string()).collect::<Vec<String>>()
                {
                    vec![(token, &input[word.len()..])]
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
    lexers.push(string_literal.clone());
    lexers.push(interpolation_elem.clone());
    lexers.push(path_literal.clone());

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
    
    /// Returns True for any Unicode space character, and the control characters
    /// , "\f", "\v" are not supported by rust, lets hope this doesn't break!
    fn is_space(string: &String) -> bool {
        ["\t", "\n", "\r", " "].contains(&&string[..])
    }
    // can be nothing
    let lex_whitespace = greedy(Parser::satisfy(is_space));

    let not_newline = greedy(Parser::satisfy(|c| c != "\n"));

    let lex_comment = {
        let f = move |t: Vec<String>| move |cs: Vec<String>| {
            let t = t.clone();
            let cs = cs.clone();
            move |_| {
                let t = t.clone(); let cs = cs.clone();
                [t.clone(), cs].concat()
            }
        };

        let lex_comment = fmap(f, Parser::token(vec![String::from("#")]));
        let lex_comment = applicative(lex_comment, not_newline);
        applicative(lex_comment, lex_whitespace.clone())
    };

    let l_comments = greedy(lex_comment.clone());

    let final_parser = {
        let l_tokens = fmap(ignore_whitespace, lex_token.clone());
        let l_tokens = applicative(l_tokens, lex_whitespace.clone());

        let parser = fmap(ignore_comments, l_comments.clone());
        let parser = applicative(parser, l_tokens.clone());

        let final_parser = fmap(ignore_all_sans_tokens, lex_whitespace.clone());
        let final_parser = applicative(final_parser, greedy(parser.clone()));
        let final_parser = applicative(final_parser, l_comments.clone());
        applicative(final_parser, Parser::eof())
    };
    // let final_parser = applicative(final_parser, Parser::succeed(()));


    // let tmp = lex_whitespace.clone().run(input);
    // let tmp = parser.clone().run(tmp[0].1);
    
    // println!("res {:?}", tmp[0].0);
    // println!("left {:?}", &tmp[0].1[..15]);
    // println!("num parses {:?}", tmp.len());

    final_parser.run(input)[0].0.clone()
}
