pub struct Parser<'a, Sym, Res>
{
    pub parser: Box<dyn Fn(Vec<Sym>) -> Vec<(Res, Vec<Sym>)> + 'a>,
}

/// consumes a list of tokens and returns a
/// list of (partial) parses
/// INPUT :: [Symbol]
/// OUTPUT:: [(Result, [Symbol])]
impl<'a ,Sym, Res> Parser<'a, Sym, Res>
{
    pub fn run(&self, input: Vec<Sym>) -> Vec<(Res, Vec<Sym>)> {
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
    pub fn symbol(a: Sym) -> Self {
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
    pub fn any_symbol() -> Self {
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
    pub fn satisfy(
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
pub fn fmap<'a, A, B, Sym>(
    f: &'a impl Fn(A) -> B ,
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

pub fn applicative<'a, A, B, Sym>(
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

