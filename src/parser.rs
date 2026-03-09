use std::sync::Arc;

pub struct Parser<'a, Sym, Res>
{
    pub parser: Arc<dyn Fn(&Vec<Sym>) -> Vec<(Res, Vec<Sym>)> + 'a>,
}

impl<'a, Sym, Res> Clone for Parser<'a, Sym, Res> {
    fn clone(&self) -> Self {
        Self { parser: self.parser.clone() }
    }
}

impl<'a ,Sym, Res> Parser<'a, Sym, Res>
where 
    Sym: Clone,
    Res: Clone + 'a
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
            parser: Arc::new(move |input: &Vec<Sym>| {
                 vec![(a.clone(), input.clone())]
            }),
        }
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
    pub fn satisfy(
        c: impl Fn(Sym) -> bool +'a,
    ) -> Self
    {
        Self {
            parser: Arc::new(move |input: &Vec<Sym>| {
                if input.is_empty() || !(c)(input[0].clone()) {
                    vec![]
                } else {
                    vec![(input[0].clone(), input[1..].to_vec())]
                }
            })
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
pub fn fmap<'a, A, B, Sym>(
    f: impl Fn(A) -> B + 'a,
    p: Parser<'a, Sym,A>
) -> Parser<'a, Sym, B>
where 
    A: Clone + 'a,
    B: Clone,
    Sym: Clone + 'a
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            p.run(&xs)
                .into_iter()
                .map(|(y, ys)| ((f)(y), ys))
                .collect::<Vec<_>>()
        }),
    }
}

/// Takes two parsers, one of type (A->B) 
/// and applies the function within the
/// parser datastructure
/// INPUT :: [((B -> A), [Symbol])] -> [(A, [Symbol])]
/// OUTPUT:: Symbol -> [(A, [Symbol])]
/// ex.:
/// fmap(concat(), applicative(symbol a, symbol b)).run("ab") -> [("ab"), []]
pub fn applicative<'a, A, B, Sym>(
    p: Parser<'a, Sym,impl Clone + Fn(B) -> A + 'a>,
    q: Parser<'a, Sym,B>
) -> Parser<'a, Sym, A>
where 
    A: Clone,
    B: Clone + 'a,
    Sym: Clone + 'a
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            p.run(&xs)
                .into_iter()
                .flat_map(|(f, ys)| 
                    q.run(&ys)
                        .into_iter()
                        .map(move |(r, zs)|
                            ((f)(r), zs)
                        )
                )
                .collect::<Vec<_>>()
        }),
    }
}

/// Takes two parsers, and 'choses'
/// the one that succeeds
/// INPUT :: [(A, [Symbol])] -> [(A, [Symbol])]
/// OUTPUT:: Symbol -> [(A, [Symbol])]
/// ex.: 
/// choice(symbol c, symbol a)).run("a") -> [("a"), []]
/// choice(symbol c, symbol a)).run("c") -> [("c"), []]
pub fn choice<'a, Sym, Res>(
    p: Parser<'a, Sym,Res>,
    q: Parser<'a, Sym,Res>
) -> Parser<'a, Sym, Res>
where 
    Sym: Clone + 'a,
    Res: Clone + 'a,
{
    Parser {
        parser: Arc::new(move |xs: &Vec<Sym>| {
            [p.run(&xs),q.run(&xs)].concat() // expensive, maybe refactor
        }),
    }
}
pub fn many<'a, Sym, Res>(
    p: Parser<'a, Sym,Res>
) -> Parser<'a, Sym, Vec<Res>>
where
    Res: Clone + 'a,
    Sym: Clone + 'a
{
    // prepends x to xs
    fn f<Sym>(x: &Sym, xs: Vec<Sym>) -> Vec<Sym>
    where
        Sym: Clone
    {
        let mut res = xs.to_vec();
        res.insert(0, x.clone());
        res
    }
    let f = move |x: Res| move |xs: Vec<Res>| f(&x,xs);
    choice(applicative(fmap(f, p.clone()), many(p.clone())), Parser::succeed(vec![]))
}
