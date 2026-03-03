pub enum KEYWORD {
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

pub enum OPERATOR {
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

pub enum TYPE_PRIMITIVE {
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

pub enum TOKEN {
    OPERATOR,
    TYPE_PRIMITIVE,
    KEYWORD,
}

pub struct NULL;

pub struct AttrSet {
    pub name: String,
    pub value: TYPE_PRIMITIVE,
}
