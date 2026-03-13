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
    POpen,
    PClose, // () parenthesis
    SOpen,
    SClose, // [] square
    COpen,
    CClose, // {} curly
    Comma,
    Semicolon,
    Colon,
    Period,
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    LEq,  // LogicalEquals
    LNEq, // LogicalNotEquals
    LAnd, // logical and
    LOr,  // logical or
    LImpl, // logical implies
    LessEq,  // LessThanEquals
    GrEq,   // GreatherThanEquals
    Assign,  // GreatherThanEquals
    Update,
    Concat,
    PipeFrom, //REQUIREEXPERIMENTALFEATURE
    PipeInto, //REQUIREEXPERIMENTALFEATURE
}

#[derive(Clone, Debug)]
pub enum TypePrimitive {
    Integer(usize),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String),
    Path(String),
    Null(NULL),
    // Attrset(Box<AttrSet>),
    List(Vec<TypePrimitive>),
    Function(fn(Vec<TypePrimitive>) -> TypePrimitive),
    External(NULL), // i think these are custom values, defined by users? TODO!!
}

#[derive(Clone, Debug)]
pub enum Token {
    Operator(Operator),
    TypePrimitive(TypePrimitive),
    Keyword(Keyword),
    Punctuation(Punctuation),
}

#[derive(Clone, Copy, Debug)]
pub struct NULL;

// #[derive(Clone, Copy, Debug)]
pub struct AttrSet {
    pub name: String,
    pub value: TypePrimitive,
}
