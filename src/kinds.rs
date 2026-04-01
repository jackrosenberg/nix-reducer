use crate::SyntaxKind;

// Nodes
pub const FN: SyntaxKind = SyntaxKind(1);
pub const NAME: SyntaxKind = SyntaxKind(2);
pub const PARAM_LIST: SyntaxKind = SyntaxKind(3);
pub const BIN_EXPR: SyntaxKind = SyntaxKind(4);

// Trivia
pub const WHITESPACE: SyntaxKind = SyntaxKind(999);
// Tokens
pub const IDENT: SyntaxKind = SyntaxKind(1001);
pub const FN_KW: SyntaxKind = SyntaxKind(1002);
pub const INT: SyntaxKind = SyntaxKind(1003);
pub const PLUS: SyntaxKind = SyntaxKind(1004);
pub const STAR: SyntaxKind = SyntaxKind(1005);
