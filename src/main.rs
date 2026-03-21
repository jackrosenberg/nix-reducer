use std::{
    env,
    fmt::{Debug, Display},
    fs,
    sync::Arc,
};

pub mod parser;
pub mod types;

use crate::parser::{
    Parser, applicative, biased_choice, choice, fmap, greedy, greedy_choice, greedy_until, greedy1,
    many, option,
};
use crate::types::{Keyword, Operator, Punctuation, Token, TypePrimitive};

fn main() {
    // take in command line args
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // let path = "./example.nix";
    // println!("path '{}'", path);

    let contents = fs::read_to_string(path)
        .expect("Unable to read path ")
        .chars()
        .collect::<Vec<_>>();

    // println!("{:?} ", parser.run(&contents));

    // println!("{:?} ", contents);
    let tokens = lex_tokens(&contents[..]);
    // println!("{:?} ", tokens);
}

fn lex_tokens(input: &[char]) -> Vec<Token> {
    let identifier = {
        // https://nix.dev/manual/nix/2.28/language/identifiers.html
        // not going to check that it's not a keyword yet, will do
        // that later if it's needed
        fn is_ident_start(c: char) -> bool {
            matches!(c,
                'a'..='z' |
                'A'..='Z' |
                '_'
            )
        }
        fn is_ident(c: char) -> bool {
            matches!(c,
                'a'..='z' |
                'A'..='Z' |
                '0'..='9' |
                '_' |
                '\'' |
                '-'
            )
        }
        let identifier = |ident_start: char| {
            move |rest: Vec<char>| {
                let res = format!(
                    "{}{}",
                    ident_start,
                    (rest.into_iter().collect::<String>())
                );
                Token::TypePrimitive(TypePrimitive::Identifier(res))
            }
        };
        let identifier = fmap(identifier, Parser::satisfy(|c| is_ident_start(*c)));
        applicative(identifier, greedy(Parser::satisfy(|c| is_ident(*c))))
    };

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    let integer = {
        let f = |digits: Vec<char>| {
            Token::TypePrimitive(TypePrimitive::Integer(
                digits
                    .into_iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("integer conversion failed"),
            ))
        };
        fmap(f, greedy1(Parser::satisfy(|c| is_digit(*c))))
    };
    let float = {
        // either (\d)+\.(\d)*, or (\d)*\.(\d)+
        let f = |pre_opt_digits: Vec<char>| {
            move |dot: char| {
                let pre_opt_digits = pre_opt_digits.clone();
                move |post_opt_digits: Vec<char>| {
                    let pre_opt_digits = pre_opt_digits.clone();
                    Token::TypePrimitive(TypePrimitive::Float(
                        [pre_opt_digits, vec![dot], post_opt_digits ]
                            .concat()
                            .into_iter()
                            .collect::<String>()
                            .parse::<f64>()
                            .expect("float conversion failed"),
                    ))
                }
            }
        };
        let before_dot = fmap(f, greedy1(Parser::satisfy(|c| is_digit(*c))));
        let before_dot = applicative(before_dot, Parser::symbol('.'));
        let before_dot = applicative(before_dot, greedy(Parser::satisfy(|c| is_digit(*c))));

        let after_dot = fmap(f, greedy(Parser::satisfy(|c| is_digit(*c))));
        let after_dot = applicative(after_dot, Parser::symbol('.'));
        let after_dot = applicative(after_dot, greedy1(Parser::satisfy(|c| is_digit(*c))));
        choice(before_dot, after_dot)
    };

    let bool = {
        fmap(
            |bool: Vec<char>| Token::TypePrimitive(TypePrimitive::Bool(bool
                .into_iter()
                .collect::<String>()
                .parse::<bool>()
                .expect("bool conversion failed")
            )),
            choice(
                Parser::token("true".chars().collect::<>()),
                Parser::token("false".chars().collect::<>()),
            ))
    };

    let string_literal = {
        // todo interpolation elems
        // https://nix.dev/manual/nix/2.28/language/string-literals.html
        fn is_str_char(c: char) -> bool {
            // not allowed to match these chars
            // return !matches!(c, '\"' | '\\' | '$')
            true
        }

        fn is_indented_str_char(c: char) -> bool {
            // voodoo
            // return !matches!(c, '\'' | '\\' | '$')
            true
        }
        let string_lit = move |open_quotes: Vec<char>| {
            move |rest_including_close_quotes: Vec<Vec<char>>| {
                Token::TypePrimitive(TypePrimitive::String(format!(
                    "{}{}",
                    open_quotes.clone().into_iter().collect::<String>(),
                    rest_including_close_quotes
                        .into_iter()
                        .flatten()
                        .collect::<String>()
                )))
            }
        };

        let double_quotes = Parser::token(vec!['\"']);
        let two_single_quotes = Parser::token(vec!['\''; 2]);

        fn lex_till_end_str_lit<'a>(
            repeater: Parser<'a, char, Vec<char>>,
            terminator: Parser<'a, char, Vec<char>>,
        ) -> Parser<'a, char, Vec<Vec<char>>> {
            greedy_until(terminator, repeater)
        }

        let string_literal = {
            let string_literal = fmap(string_lit, double_quotes.clone());
            applicative(
                string_literal,
                lex_till_end_str_lit(
                    double_quotes,
                    fmap(|r| vec![r], Parser::satisfy(|c| is_str_char(*c))),
                ),
            )
        };

        let string_literal_indented = {
            let string_literal_indented = fmap(string_lit, two_single_quotes.clone());
            applicative(
                string_literal_indented,
                lex_till_end_str_lit(
                    two_single_quotes,
                    fmap(|r| vec![r], Parser::satisfy(|c| is_indented_str_char(*c))),
                ),
            )
        };

        choice(string_literal, string_literal_indented)
    };

    let interpolation_elem = {
        let f = move |t: Vec<char>| {
            move |cs: Vec<char>| {
                Token::TypePrimitive(TypePrimitive::InterpolationElement(
                    [t.clone(), cs].concat().into_iter().collect::<String>(),
                ))
            }
        };
        let parser = fmap(f, Parser::token("${".chars().collect::<>()));
        applicative(parser, Parser::greedy_stack_symbol('}'))
    };

    let path_literal = {
        fn is_path_char(c: char) -> bool {
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
        let f = move |path_chars: Vec<char>| {
            move |slash_path_chars: Vec<Vec<char>>| {
                let path_chars = path_chars.clone();
                move |opt_slash: char| {
                    let path_chars = path_chars.clone();
                    let slash_path_chars = slash_path_chars.clone();
                    Token::TypePrimitive(TypePrimitive::Path(
                        [
                            path_chars,
                            slash_path_chars
                                .into_iter()
                                .flatten()
                                .collect::<Vec<char>>(),
                            vec![opt_slash],
                        ]
                        .concat()
                        .into_iter()
                        .collect::<String>(),
                    ))
                }
            }
        };

        let g = move |t: char| {
            move |cs: Vec<char>| {
                [vec![t], cs].concat()
            }
        };

        let path_chars = Parser::satisfy(|c| is_path_char(*c));

        let sub_parser = fmap(g, Parser::symbol('/'));
        let sub_parser = applicative(sub_parser, greedy1(path_chars.clone()));

        let parser = fmap(f, greedy(path_chars));
        let parser = applicative(parser, greedy1(sub_parser));
        applicative(
            parser,
            option(Parser::symbol('/'), ' '), // todo, check if this causes issues
        )
    };

    // god this is verbose and ugly
    let keyword_pairs: [(&str, Token); 11] = [
        ("if", Token::Keyword(Keyword::If)),
        ("then", Token::Keyword(Keyword::Then)),
        ("else", Token::Keyword(Keyword::Else)),
        ("assert", Token::Keyword(Keyword::Assert)),
        ("with", Token::Keyword(Keyword::With)),
        ("let", Token::Keyword(Keyword::Let)),
        ("inherit", Token::Keyword(Keyword::Inherit)), // inherit must be before in, since otherwise
        ("in", Token::Keyword(Keyword::In)),           // the lexer will find inherit
        ("rec", Token::Keyword(Keyword::Rec)),
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
        ("=", Token::Operator(Operator::Assign)),
        ("//", Token::Operator(Operator::Update)),
        ("++", Token::Operator(Operator::Concat)),
        ("|>", Token::Operator(Operator::PipeFrom)),
        ("<|", Token::Operator(Operator::PipeInto)),
    ];

    fn gen_key_parser(key_pair: (&'_ str, Token)) -> Parser<'_, char, Token> {
        Parser {
            parser: Arc::new(move |input: &[char]| {
                let (word, token) = key_pair.clone();
                // ex ["r","e","c"];
                if let Some(chars) = input.get(..word.len())
                    && chars == word.chars().collect::<Vec<char>>()
                {
                    vec![(token, &input[word.len()..])]
                } else {
                    Parser::empty().run(input)
                }
            }),
        }
    }

    fn lex_keyword(pairs: Vec<(&'_ str, Token)>) -> Parser<'_, char, Token> {
        greedy_choice(
            pairs
                .iter()
                .map(|c| gen_key_parser(c.clone()))
                .collect::<Vec<_>>(),
        )
    }

    let mut lexers = [
        keyword_pairs.to_vec(),
        punctuation_pairs.to_vec(),
        operator_pairs.to_vec(),
    ]
    .into_iter()
    .map(|p| lex_keyword(p))
    .collect::<Vec<_>>();
    lexers.push(bool);
    lexers.push(identifier);
    lexers.insert(0, float.clone()); // must be done before punctuation, otherwise will be read as 2x integer
    lexers.push(integer.clone());
    lexers.push(string_literal.clone());
    lexers.push(interpolation_elem.clone());
    lexers.push(path_literal.clone());

    let lex_token = greedy_choice(lexers);

    fn ignore_whitespace<F1, F2>(f: F1) -> F1
    where
        F1: Fn(Token) -> F2,
        F2: Fn(Vec<char>) -> Token,
    {
        f
    }
    let ignore_whitespace = ignore_whitespace(|tk| {
        move |cms| tk.clone()
    });

    fn ignore_comments<F1, F2>(f: F1) -> F1
    where
        F1: Fn(Vec<Vec<char>>) -> F2,
        F2: Fn(Token) -> Token,
    {
        f
    }
    let ignore_comments = ignore_comments(|cms| |tk| tk);

    fn ignore_all_sans_tokens<F1, F2, F3, F4>(f: F1) -> F1
    where
        F1: Fn(Vec<char>) -> F2,
        F2: Fn(Vec<Token>) -> F3,
        F3: Fn(Vec<Vec<char>>) -> F4,
        F4: Fn(()) -> Vec<Token>,
    {
        f
    }
    let ignore_all_sans_tokens = ignore_all_sans_tokens(|wh| {
        |tks| {
            move |cms| {
                let tks = tks.clone();
                move |eof| tks.clone()
            }
        }
    });

    // parse and ignore all comments

    /// Returns True for any Unicode space character, and the control characters
    /// , "\f", "\v" are not supported by rust, lets hope this doesn't break!
    fn is_space(c: char) -> bool {
        ['\t', '\n', '\r', ' '].contains(&c)
    }
    // can be nothing
    let lex_whitespace = greedy(Parser::satisfy(|c| is_space(*c)));

    let not_newline = greedy(Parser::satisfy(|c| *c != '\n'));

    let lex_comment = {
        let f = move |t: Vec<char>| {
            move |cs: Vec<char>| {
                let t = t.clone();
                move |_| {
                    [t.clone(), cs.clone()].concat()
                }
            }
        };

        let lex_comment = fmap(f, Parser::token(vec!['#']));
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

    // let tmp = lex_whitespace.clone().run(input);
    // let tmp = greedy1(Parser::satisfy(is_digit)));
    // let before_dot = applicative(before_dot, Parser::symbol(String::from(".")));
    // let before_dot = applicative(before_dot, greedy(Parser::satisfy(is_digit)));
    // let tmp = before_dot.clone().run(tmp[0].1);

    // println!("res {:?}", tmp[0].0);
    // println!("left {:?}", &tmp[0].1[..15]);
    // println!("num parses {:?}", tmp.len());

    println!("{:#?}", final_parser.run(input));
    final_parser.run(input)[0].0.clone()
}
