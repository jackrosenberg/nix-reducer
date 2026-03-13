use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

pub mod helpers;

pub struct Parser<'a, Sym, Res> {
    pub parser: Arc<dyn Fn(&Vec<Sym>) -> Vec<(Res, Vec<Sym>)> + 'a>,
}

impl<'a, Sym, Res> Clone for Parser<'a, Sym, Res> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
        }
    }
}

impl<'a, Sym, Res> Parser<'a, Sym, Res>
where
    Sym: Clone,
    Res: Clone + 'a,
{
    /// consumes a list of tokens and returns a
    /// list of (partial) parses
    /// INPUT :: [Symbol]
    /// OUTPUT:: [(Result, [Symbol])]
    pub fn run(&self, input: &Vec<Sym>) -> Vec<(Res, Vec<Sym>)> {
        if input.is_empty() {
            return vec![];
        }
        (self.parser)(input)
    }
    pub fn succeed(a: Res) -> Self {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| vec![(a.clone(), input.clone())]),
        }
    }
    pub fn empty() -> Self {
        Self {
            parser: Arc::new(move |_: &Vec<Sym>| vec![]),
        }
    }
}

impl<'a, Sym> Parser<'a, Sym, ()> 
where
    Sym: Clone
{
    /// succeed only when find end of file
    pub fn eof() -> Self {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| {
                if input.is_empty() {
                    Parser::succeed(())
                } else {
                    Parser::empty()
                }.run(input)
            })
        }
    }
}

impl<'a, Sym> Parser<'a, Sym, Sym>
where
    Sym: PartialEq + Clone + 'a,
{
    /// INPUT :: Symbol
    /// OUTPUT:: Symbol -> [(Result, [Symbol])]
    /// ex.:
    /// Parser::symbol("A").run(vec!["A","B","C","D"]) -> [("A", ["B", "C", "D"])]
    /// Parser::symbol("A").run([hello world]) -> []
    pub fn symbol(a: Sym) -> Self {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| {
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
    pub fn any_symbol() -> Self {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| {
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
    pub fn satisfy(c: impl Fn(&Sym) -> bool + 'a) -> Self {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| {
                if input.is_empty() || !(c)(&input[0].clone()) {
                    vec![]
                } else {
                    vec![(input[0].clone(), input[1..].to_vec())]
                }
            }),
        }
    }
}
/// helper function that parses a word
/// INPUT :: Symbol
/// OUTPUT:: Symbol -> [(Result, [Symbol])]
/// ex.: TODO
impl<'a, Sym> Parser<'a, Sym, Vec<Sym>>
where
    Sym: PartialEq + Clone + 'a,
{
    pub fn token(word: &'a Vec<Sym>) -> Self {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| {
                let k = word.len();
                if input.is_empty() || &input[..k] != word {
                    vec![]
                } else {
                    vec![(input[..k].to_vec(), input[k..].to_vec())]
                }
            }),
        }
    }
}

// parser combinators

/// Takes a function, and wraps it in the
/// parser datastructure
/// INPUT :: (A -> B) -> [(A, [Symbol])]
/// OUTPUT:: Symbol -> [(B, [Symbol])]
/// ex.:
/// fmap(concat(), applicative(symbol a, symbol b)).run("ab") -> [("ab"), []]
pub fn fmap<'a, A, B, Sym>(f: impl Fn(A) -> B + 'a, p: Parser<'a, Sym, A>) -> Parser<'a, Sym, B>
where
    A: Clone + 'a,
    B: Clone,
    Sym: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            p.run(xs)
                .into_iter()
                .map(|(y, ys)| ((f)(y), ys))
                .collect::<Vec<_>>()
        }),
    }
}
// ignore the result of the parser and return a const
pub fn fmap_l<'a, A, B, Sym>(r: B, p: Parser<'a, Sym, A>) -> Parser<'a, Sym, B>
where
    A: Clone + 'a,
    B: Clone + 'a,
    Sym: Clone + 'a,
{
    fmap(move |_| r.clone(), p)
}

/// Takes two parsers, one of type (A->B)
/// and applies the function within the
/// parser datastructure
/// INPUT :: [((B -> A), [Symbol])] -> [(A, [Symbol])]
/// OUTPUT:: Symbol -> [(A, [Symbol])]
/// ex.:
/// fmap(concat(), applicative(symbol a, symbol b)).run("ab") -> [("ab"), []]
pub fn applicative<'a, A, B, Sym>(
    p: Parser<'a, Sym, impl Clone + Fn(B) -> A + 'a>,
    q: Parser<'a, Sym, B>,
) -> Parser<'a, Sym, A>
where
    A: Clone,
    B: Clone + 'a,
    Sym: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            p.run(xs)
                .into_iter()
                .flat_map(|(f, ys)| q.run(&ys).into_iter().map(move |(r, zs)| ((f)(r), zs)))
                .collect::<Vec<_>>()
        }),
    }
}
// TODO
// rahhh this function almost got me,
// haskell and it's glorious type system.
pub fn applicative_l<'a, A, B, Sym>(
    p: Parser<'a, Sym, impl Clone + Fn(B) -> A + 'a>,
    q: Parser<'a, Sym, B>,
) -> Parser<'a, Sym, impl Clone + Fn(B) -> A>
where
    A: Clone + 'a,
    B: Clone + 'a,
    Sym: Clone + 'a,
{
    fmap(move |x| move |_| x.clone(), applicative(p, q))
}

// pub fn applicative_r<'a, A, B, Sym>(
//     p: Parser<'a, Sym, impl Clone + Fn(B) -> A + 'a>,
//     q: Parser<'a, Sym, B>,
// ) -> Parser<'a, Sym, B>
// where
//     A: Clone + 'a,
//     B: Clone + 'a,
//     Sym: Clone + 'a,
// {
//     Parser {
//         parser: Arc::new(move |xs: &Vec<Sym>| {
//             let app = applicative(p, q).run(xs);
//             app.into_iter.map()
//             fmap(move |_| move |y: Parser<'a, Sym, B>| y.clone(),
//         })
//     }
// }

/// Takes two parsers, and 'chooses'
/// the one that succeeds
/// INPUT :: (Symbol -> [(A, [Symbol])]]) -> (Symbol -> [(A, [Symbol])]]
/// OUTPUT:: Symbol -> [(A, [Symbol])]
/// ex.:
/// choice(symbol c, symbol a)).run("a") -> [("a"), []]
/// choice(symbol c, symbol a)).run("c") -> [("c"), []]
pub fn choice<'a, Sym, Res>(
    p: Parser<'a, Sym, Res>,
    q: Parser<'a, Sym, Res>,
) -> Parser<'a, Sym, Res>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            [p.run(xs), q.run(xs)].concat() // expensive, maybe refactor
        }),
    }
}

/// only if p is empty return q
pub fn biased_choice<'a, Sym, Res>(
    p: Parser<'a, Sym, Res>,
    q: Parser<'a, Sym, Res>,
) -> Parser<'a, Sym, Res>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            let r = p.run(xs);
            if r.is_empty() { q.clone() } else { p.clone() }.run(xs)
        }),
    }
}
/// Takes a parser and returns a parser
/// that chains the parser as much as it can
/// P*
/// INPUT :: Symbol -> [(A, [Symbol])]
/// OUTPUT:: Symbol -> [([A], [Symbol])]
/// ex.:
/// many(symbol a).run("aaa") -> [("aaa", [])];
/// many(symbol a).run("aaaaaaa") -> [("aaaaaaa", [])];
pub fn many<'a, Sym, Res>(p: Parser<'a, Sym, Res>) -> Parser<'a, Sym, Vec<Res>>
where
    Res: Clone + 'a,
    Sym: Clone + 'a,
{
    // prepends x to xs
    fn f<Sym>(x: &Sym, xs: Vec<Sym>) -> Vec<Sym>
    where
        Sym: Clone,
    {
        let mut res = xs.to_vec();
        res.insert(0, x.clone());
        res
    }
    let f = move |x: Res| move |xs: Vec<Res>| f(&x, xs);
    // it looks like rust is not lazy enough to
    // make the following work :(
    // choice(applicative(fmap(f, p.clone()), many(p.clone())), Parser::succeed(vec![]))
    // this might be the ugliest parser i have written
    // but it works :)
    Parser {
        parser: Arc::new(move |input: &Vec<Sym>| {
            // base case
            if p.run(input).is_empty() {
                Parser::succeed(vec![])
            } else {
                applicative(fmap(f, p.clone()), many(p.clone()))
            }
            .run(input)
        }),
    }
}
pub fn many1<'a, Sym, Res>(p: Parser<'a, Sym, Res>) -> Parser<'a, Sym, Vec<Res>>
where
    Res: Clone + 'a,
    Sym: Clone + 'a,
{
    // prepends x to xs
    fn f<Sym>(x: &Sym, xs: Vec<Sym>) -> Vec<Sym>
    where
        Sym: Clone,
    {
        let mut res = xs.to_vec();
        res.insert(0, x.clone());
        res
    }
    let f = move |x: Res| move |xs: Vec<Res>| f(&x, xs);
    applicative(fmap(f, p.clone()), many(p.clone()))
}
/// takes only the first parse result, and ignores the rest
pub fn first<'a, Sym, Res>(p: Parser<'a, Sym, Res>) -> Parser<'a, Sym, Res>
where
    Res: Clone + 'a,
    Sym: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |input: &Vec<Sym>| {
            let mut r = p.run(input);
            if let Some(fst) = r.pop() {
                vec![fst]
            } else {
                vec![]
            }
        }),
    }
}

/// takes an option p , and a default d, returns default if p fails
pub fn option<'a, Sym, Res>(p: Parser<'a, Sym, Res>, d: Res) -> Parser<'a, Sym, Res>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    choice(p, Parser::succeed(d))
}

pub fn greedy<'a, Sym, Res>(p: Parser<'a, Sym, Res>) -> Parser<'a, Sym, Vec<Res>>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    first(many(p))
}

pub fn greedy_choice<'a, Sym, Res>(ps: Vec<Parser<'a, Sym, Res>>) -> Parser<'a, Sym, Res>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    // check if needs to be swapped, make sure that the empty parser does not
    // go first, since the whole thing will fail
    ps.into_iter()
        .fold(Parser::empty(), |acc, elem| biased_choice(acc, elem))
}
