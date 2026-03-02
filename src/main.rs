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

    // let p = Parser::symbol("A").run(vec!["A", "B", "C", "D"]);
    // let abc = vec!["A", "B", "C", "D"];
    let abc = "ABCD".chars().map(|c| char::to_string(&c)).collect::<Vec<_>>();
    let p = Parser::symbol(String::from("A"));
    let q = Parser::symbol(String::from("B"));
    fn c(s: &String) -> bool {
        s == "C"
    }
    let s = Parser::satisfy(c);
    fn f(a: &String, b: &String, c: &String) -> String {
    // fn f(a: &str,b: &str) -> str {
        let mut res = String::from(a);
        res.push_str(&b);
        res.push_str(&c);
        res
    }
    // TODO, make macro?
    let parser = fmap(move |a| move |b| move |c| f(&a,&b,&c), &p);
    let parser = applicative(&parser, &q);
    let parser = applicative(&parser, &s);
    println!("{:?} ", parser.run(abc));
}

// parsing
struct Parser<'a, Sym, Res>
{
    parser: Box<dyn Fn(Vec<Sym>) -> Vec<(Res, Vec<Sym>)> + 'a>,
}

/// consumes a list of tokens and returns a
/// list of (partial) parses
/// INPUT :: [Symbol]
/// OUTPUT:: [(Result, [Symbol])]
impl<'a ,Sym, Res> Parser<'a, Sym, Res>
{
    fn run(&self, input: Vec<Sym>) -> Vec<(Res, Vec<Sym>)> {
        if input.is_empty() {
            return vec![];
        }
        (self.parser)(input)
    }

}
impl<'a, Sym> Parser<'a, Sym, Sym>
where Sym: PartialEq + Clone + 'a
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
                    vec![(input[0].clone(), input[1..].to_vec())]
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
                    vec![(input[0].clone(), input[1..].to_vec())]
                }
            }),
        }
    }
    /// INPUT :: (Symbol -> bool)
    /// OUTPUT:: Symbol -> [(Result, [Symbol])]
    /// ex.:
    /// Parser::satisfy().run(vec!["B","C","D"]) -> [("B", ["C", "D"])]
    fn satisfy(
        c: impl Fn(&Sym) -> bool +'a,
    ) -> Self
    {
        Self {
            parser: Box::new(move |input: Vec<Sym>| {
                if input.is_empty() || !(c)(&input[0].clone()) {
                    vec![]
                } else {
                    vec![(input[0].clone(), input[1..].to_vec())]
                }
            })
        }
    }
}

// parser combinators
fn fmap<'a, A, B, Sym>(
    f: impl Fn(A) -> B +'a,
    p: &'a Parser<Sym,A>
) -> Parser<'a, Sym, B>
{
    Parser {
        parser: Box::new(move |xs: Vec<Sym>| {
            p.run(xs)
                .into_iter()
                .map(|(y, ys)| ((f)(y), ys))
                .collect::<Vec<_>>()
        }),
    }
}

fn applicative<'a, A, B, Sym>(
    p: &'a Parser<Sym,impl Fn(B) -> A + 'a>,
    q: &'a Parser<Sym,B>
) -> Parser<'a, Sym, A>
{
    Parser {
        parser: Box::new(move |xs: Vec<Sym>| {
            p.run(xs)
                .into_iter()
                .flat_map(|(f, ys)| 
                    q.run(ys)
                        .into_iter()
                        .map(move |(r, zs)|
                            ((f)(r), zs)
                        )
                )
                .collect::<Vec<_>>()
        }),
    }
}
