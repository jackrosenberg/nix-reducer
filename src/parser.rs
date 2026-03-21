use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

pub struct Parser<'a, Sym, Res> {
    pub parser: Arc<dyn Fn(&'a [Sym]) -> Vec<(Res, &'a [Sym])> + 'a>,
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
    pub fn run(&self, input: &'a [Sym]) -> Vec<(Res, &'a [Sym])> {
        (self.parser)(input)
    }
    pub fn succeed(a: Res) -> Self {
        Self {
            parser: Arc::new(move |input: &[Sym]| vec![(a.clone(), input)]),
        }
    }
    pub fn empty() -> Self {
        Self {
            parser: Arc::new(move |_: &[Sym]| vec![]),
        }
    }
}

impl<'a, Sym> Parser<'a, Sym, ()>
where
    Sym: Clone + Debug,
{
    /// succeed only when find end of file
    pub fn eof() -> Self {
        Self {
            parser: Arc::new(move |input: &[Sym]| {
                println!("{:?}", &input);
                if input.is_empty() {
                    (Parser::succeed(()).parser)(input)
                } else {
                    (Parser::empty().parser)(input)
                }
            }),
        }
    }
}

impl<'a, Sym> Parser<'a, Sym, Sym>
where
    Sym: PartialEq + Clone + Debug + 'a,
{
    /// INPUT :: Symbol
    /// OUTPUT:: Symbol -> [(Result, [Symbol])]
    /// ex.:
    /// Parser::symbol("A").run(vec!["A","B","C","D"]) -> [("A", ["B", "C", "D"])]
    /// Parser::symbol("A").run([hello world]) -> []
    pub fn symbol(a: Sym) -> Self {
        Self {
            parser: Arc::new(move |input: &[Sym]| {
                if input.is_empty() || a != input[0] {
                    vec![]
                } else {
                    vec![(input[0].clone(), &input[1..])]
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
            parser: Arc::new(move |input: &[Sym]| {
                if input.is_empty() {
                    vec![]
                } else {
                    vec![(input[0].clone(), &input[1..])]
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
            parser: Arc::new(move |input: &[Sym]| {
                if input.is_empty() || !(c)(&input[0].clone()) {
                    vec![]
                } else {
                    vec![(input[0].clone(), &input[1..])]
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
    Sym: PartialEq + Clone + Display + 'a,
{
    pub fn token(word: Vec<Sym>) -> Self {
        Self {
            parser: Arc::new(move |input: &[Sym]| {
                let k = word.len();
                if let Some(chars) = input.get(..k) &&
                 chars == word {
                    vec![(input[..k].to_vec(), &input[k..])]
                } else {
                    vec![]
                }
            }),
        }
    }
    /// take untill the corresponding occurence of the requested symbol
    pub fn greedy_stack_symbol(a: Sym) -> Self {
        // grosshack, but i can't be asked
        fn opposite<Sym: Display>(symb: Sym) -> &'static str {
            match &symb.to_string()[..] {
                "{" => "}",
                "}" => "{",
                _ => "",
            }
        }
        Self {
            parser: Arc::new(move |input: &[Sym]| {
                let mut depth = 0;
                for (i, s) in input.iter().enumerate() {
                    if s == &a {
                        if depth == 0 {
                            return vec![(input[..=i].to_vec(), &input[i + 1..])];
                        }
                        depth -= 1;
                    } else if s.to_string() == opposite(a.to_string()) {
                        depth += 1;
                    }
                }
                // fail
                Parser::empty().run(input)
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
    B: Clone + 'a,
    Sym: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &'a [Sym]| {
            p.run(xs)
                .into_iter()
                .map(|(y, ys): (A, &'a [Sym])| ((f)(y), ys))
                .collect::<Vec<(B, &'a [Sym])>>()
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
        parser: Arc::new(move |xs: &'a [Sym]| {
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
        parser: Arc::new(move |xs: &[Sym]| {
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
        parser: Arc::new(move |xs: &[Sym]| {
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
        parser: Arc::new(move |input: &[Sym]| {
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
        parser: Arc::new(move |input: &[Sym]| {
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

pub fn greedy1<'a, Sym, Res>(p: Parser<'a, Sym, Res>) -> Parser<'a, Sym, Vec<Res>>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    first(many1(p))
}

pub fn greedy_choice<'a, Sym, Res>(ps: Vec<Parser<'a, Sym, Res>>) -> Parser<'a, Sym, Res>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    // check if needs to be swapped, make sure that the empty parser does not
    // go first, since the whole thing will fail
    ps.into_iter()
        // .fold(Parser::empty(), |acc, elem| biased_choice(acc, elem)) // too greedy
        .fold(Parser::empty(), |acc, elem| choice(acc, elem))
}

/// biased_choice to take as many p, until one q is found
pub fn greedy_until<'a, Sym, Res>(
    p: Parser<'a, Sym, Res>,
    q: Parser<'a, Sym, Res>,
) -> Parser<'a, Sym, Vec<Res>>
where
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &[Sym]| {
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

            let r = q.run(xs);
            if !r.is_empty() {
                fmap(|e| vec![e], q.clone())
            } else {
                applicative(fmap(f, p.clone()), greedy_until(p.clone(), q.clone()))
            }
            .run(xs)
        }),
    }
}
