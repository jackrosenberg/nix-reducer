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
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // let path = &args[1];
    // println!("Searching path '{}'", path);
    //
    // let contents = fs::read_to_string(path).expect("Unable to read path ");
    //
    // println!("File contains \n '{}'", contents);

    let p = Parser::symbol("A").run(vec!["A", "B", "C", "D"]);
    let q = Parser::any_symbol().run(vec!["B", "C", "D"]);
    println!("{:?} {:?}", p, q);
}

// parsing
struct Parser<Sym, Res>
{
    parser: Box<dyn Fn(Vec<Sym>) -> Vec<(Res, Vec<Sym>)>>,
}

/// consumes a list of tokens and returns a
/// list of (partial) parses
/// INPUT :: [Symbol]
/// OUTPUT:: [(Result, [Symbol])]
impl<Sym, Res> Parser<Sym, Res>
{
    fn run(self, input: Vec<Sym>) -> Vec<(Res, Vec<Sym>)> {
        if input.is_empty() {
            return vec![];
        }
        (self.parser)(input)
    }
}
impl<Sym> Parser<Sym, Sym>
where Sym: PartialEq + Clone + Copy + 'static
{
    /// INPUT :: Symbol
    /// OUTPUT:: Symbol -> [(Result, [Symbol])]
    /// ex.:
    /// Parser::symbol("A").run(vec!["A","B","C","D"]) -> [("A", ["B", "C", "D"])]
    /// Parser::symbol("A").run([hello world]) -> []
    fn symbol(a: Sym) -> Self {
        Self {
            parser: Box::new(move |input: Vec<Sym>| {
                if input.is_empty() || a != input[0] {
                    vec![]
                } else {
                    vec![(input[0], input[1..].to_vec())]
                }
            }),
        }
    }
    /// INPUT :: ()
    /// OUTPUT:: Symbol -> [(Result, [Symbol])]
    /// ex.:
    /// Parser::any_symbol().run(vec!["B","C","D"]) -> [("B", ["C", "D"])]
    /// symbol('e').parser.run([hello world]) -> []
    fn any_symbol() -> Self {
        Self {
            parser: Box::new(move |input: Vec<Sym>| {
                if input.is_empty() {
                    vec![]
                } else {
                    vec![(input[0], input[1..].to_vec())]
                }
            }),
        }
    }
}

