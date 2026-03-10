#[derive(Clone, Copy, Debug)]
pub enum Keyword {
    If,
    Then,
    Else,
    Assert,
    With,
    Let,
    In,
    Rec,
    Inherit,
    Or,
    Ellipsis, // ...
}

#[derive(Clone, Copy, Debug)]
pub enum Punctuation {
    POpen, PClose,   // () parenthesis
    SOpen, SClose,   // [] square
    COpen, CClose,   // {} curly
    Comma, Semicolon // , ; (... duhh)
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Eq,
    Neq,
    Leq,
    Geq,
    Land,
    Lor,
    Impl,
    Update,
    Concat,
    PipeFrom, //REQUIREEXPERIMENTALFEATURE
    PipeInto, //REQUIREEXPERIMENTALFEATURE
}

// #[derive(Clone, Copy, Debug)]
pub enum TypePrimitive {
    Integer(usize),
    Float(f64),
    Boolean(bool),
    String(String),
    Path(String),
    Null(NULL),
    Attrset(Box<AttrSet>),
    List(Vec<TypePrimitive>),
    Function(fn(Vec<TypePrimitive>) -> TypePrimitive),
    External(NULL), // i think these are custom values, defined by users? TODO!!
}

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Operator(Operator),
    // TypePrimitive(TypePrimitive),
    TypePrimitive,
    Keyword(Keyword),
    Punctuation(Punctuation)
}

#[derive(Clone, Copy, Debug)]
pub struct NULL;

// #[derive(Clone, Copy, Debug)]
pub struct AttrSet {
    pub name: String,
    pub value: TypePrimitive,
}
