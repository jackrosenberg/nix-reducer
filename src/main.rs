use std::{env, fs};

// the static operators and keywords
// TODO,
// -- make this a separate file
// -- change to list of tokens, for after we lex everything to tokens
// -- external values
// -- make parsers polymorphic
enum KEYWORD {
    IF,
    THEN,
    ELSE,
    ASSERT,
    WITH,
    LET,
    IN,
    REC,
    INHERIT,
    OR,
    ELLIPSIS, // ...
}

enum OPERATOR {
    EQ,
    NEQ,
    LEQ,
    GEQ,
    LAND,
    LOR,
    IMPL,
    UPDATE,
    CONCAT,
    PIPE_FROM, //REQUIREEXPERIMENTALFEATURE
    PIPE_INTO, //REQUIREEXPERIMENTALFEATURE
}

enum TYPE_PRIMITIVE {
    INTEGER(usize),
    FLOAT(f64),
    BOOLEAN(bool),
    STRING(String),
    PATH(String),
    NULL(NULL),
    ATTRSET(Box<AttrSet>),
    LIST(Vec<TYPE_PRIMITIVE>),
    FUNCTION(fn(Vec<TYPE_PRIMITIVE>) -> TYPE_PRIMITIVE),
    EXTERNAL(NULL), // i think these are custom values, defined by users? TODO!!
}

enum TOKEN {
    OPERATOR,
    TYPE_PRIMITIVE,
    KEYWORD,
}

struct NULL;

struct AttrSet {
    name: String,
    value: TYPE_PRIMITIVE,
}

fn main() {
    // take in command line args
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    // let path = &args[1];
    // println!("Searching path '{}'", path);
    //
    // let contents = fs::read_to_string(path).expect("Unable to read path ");
    //
    // println!("File contains \n '{}'", contents);

    let p = Parser::symbol("A").run(vec!["A", "B", "C", "D"]);
    println!("{:?}", p);
}

// parsing
#[derive(Clone)]
struct Parser<Sym, Res>
where
    Parser<Sym, Res>: Clone,
{
    parser: Box<dyn Fn(Vec<Sym>) -> Vec<(Res, Vec<Sym>)>>,
}

/// consumes a list of tokens and returns a
/// list of (partial) parses
/// INPUT :: [Symbol]
/// OUTPUT:: [(Result, [Symbol])]
impl<Sym, Res> Parser<Sym, Res>
where
    Parser<Sym, Res>: Clone,
{
    fn run(self, input: Vec<Sym>) -> Vec<(Res, Vec<Sym>)> {
        if input.is_empty() {
            return vec![];
        }
        (self.parser)(input)
    }
}
impl<'a> Parser<&'a str, String>
where
    Parser<&'a str, String>: Clone,
{
    /// parses a single token of type S
    /// and returns a list, [] if no
    /// parse, [(S, [S])] if parse successful
    /// INPUT :: Symbol
    /// OUTPUT:: Symbol -> [(Result, [Symbol])]
    /// ex.:
    /// Parser::symbol("A").run(vec!["A","B","C","D"]) -> [("A", ["B", "C", "D"])]
    /// symbol('e').parser.run([hello world]) -> []
    fn symbol(a: &'static str) -> Self {
        Self {
            parser: Box::new(move |input: Vec<&str>| {
                if input.is_empty() || a != input[0] {
                    vec![]
                } else {
                    vec![(input[0].to_string(), input[1..].to_vec())]
                }
            }),
        }
    }
    fn anySymbol() -> Self {
        Self {
            parser: Box::new(|input: Vec<&str>| {
                if input.is_empty() {
                    vec![]
                } else {
                    vec![(input[0].to_string(), input[1..].to_vec())]
                }
            }),
        }
    }
}

// parser combinators

/// fmap, looks inside the parser and applies function to res
/// INPUT :: f: (a -> b), s: Parser Sym A
/// OUTPUT:: Parser Sym B
/// ex.:
fn fmap<A: 'static, B: 'static, Sym: 'static>(
    f: Box<dyn Fn(A) -> B>,
    p: Parser<Sym, A>,
) -> Parser<Sym, B>
where
    Parser<Sym, A>: Clone,
    Parser<Sym, B>: Clone,
{
    Parser {
        parser: Box::new(move |xs: Vec<Sym>| {
            p.clone()
                .run(xs)
                .into_iter()
                .map(|(y, ys)| ((f)(y), ys))
                .collect::<Vec<_>>()
        }),
    }
}

fn combine<A: 'static, B: 'static, Sym: 'static>(
    p: Parser<Sym, Box<dyn Fn(A) -> B>>,
    q: Parser<Sym, A>,
) -> Parser<Sym, B>
where
    Parser<Sym, A>: Clone,
    Parser<Sym, B>: Clone,
    Parser<Sym, Box<dyn Fn(A) -> B>>: Clone,
{
    Parser {
        parser: Box::new(move |xs: Vec<Sym>| {
            p.clone().run(xs)
                .into_iter()
                .flat_map(|(f, ys)| {
                    q.clone().run(ys)
                        .into_iter()
                        .map(|(r, zs)| {
                            ((f)(r), zs)
                        }) 
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }),
    }
}
