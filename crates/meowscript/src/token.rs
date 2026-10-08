use crate::span::Span;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Number(f64),
    Str(String),

    /// `scratch`: declare
    Scratch,
    /// `amew`: assign
    Amew,
    /// `pawction`: function
    Pawction,
    /// `purrhaps`: if
    Purrhaps,
    /// `meowtually`: else
    Meowtually,
    /// `tail`: return
    Tail,
    /// `pawckage`: import
    Pawckage,
    /// `purrfect`: true
    Purrfect,
    /// `clawful`: false
    Clawful,
    /// `mew`: null
    Mew,
    /// `furreal`: typeof
    Furreal,
    /// `furrever`: loop
    Furrever,
    /// `fur`: for-each
    Fur,
    /// `hiss`: break
    Hiss,
    Continue,

    Assign,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Amp,
    Pipe,
    Caret,
    Shl,
    Shr,
    AndAnd,
    OrOr,
    /// `~`: membership
    Tilde,
    /// `'s`: property access
    Possessive,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semicolon,

    Eof,
}

impl TokenKind {
    pub fn keyword(ident: &str) -> Option<TokenKind> {
        Some(match ident {
            "scratch" => TokenKind::Scratch,
            "amew" => TokenKind::Amew,
            "pawction" => TokenKind::Pawction,
            "purrhaps" => TokenKind::Purrhaps,
            "meowtually" => TokenKind::Meowtually,
            "tail" => TokenKind::Tail,
            "pawckage" => TokenKind::Pawckage,
            "purrfect" => TokenKind::Purrfect,
            "clawful" => TokenKind::Clawful,
            "mew" => TokenKind::Mew,
            "furreal" => TokenKind::Furreal,
            "furrever" => TokenKind::Furrever,
            "fur" => TokenKind::Fur,
            "hiss" => TokenKind::Hiss,
            "continue" => TokenKind::Continue,
            _ => return None,
        })
    }

    pub const KEYWORDS: &'static [&'static str] = &[
        "scratch",
        "amew",
        "pawction",
        "purrhaps",
        "meowtually",
        "tail",
        "pawckage",
        "purrfect",
        "clawful",
        "mew",
        "furreal",
        "furrever",
        "fur",
        "hiss",
        "continue",
    ];

    pub fn describe(&self) -> String {
        match self {
            TokenKind::Ident(name) => format!("identifier `{name}`"),
            TokenKind::Number(n) => format!("number `{n}`"),
            TokenKind::Str(_) => "a string".to_string(),
            TokenKind::Eof => "end of input".to_string(),
            other => format!("`{}`", other.symbol()),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            TokenKind::Ident(_) => "identifier",
            TokenKind::Number(_) => "number",
            TokenKind::Str(_) => "string",
            TokenKind::Scratch => "scratch",
            TokenKind::Amew => "amew",
            TokenKind::Pawction => "pawction",
            TokenKind::Purrhaps => "purrhaps",
            TokenKind::Meowtually => "meowtually",
            TokenKind::Tail => "tail",
            TokenKind::Pawckage => "pawckage",
            TokenKind::Purrfect => "purrfect",
            TokenKind::Clawful => "clawful",
            TokenKind::Mew => "mew",
            TokenKind::Furreal => "furreal",
            TokenKind::Furrever => "furrever",
            TokenKind::Fur => "fur",
            TokenKind::Hiss => "hiss",
            TokenKind::Continue => "continue",
            TokenKind::Assign => "=",
            TokenKind::Eq => "==",
            TokenKind::NotEq => "!=",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::LtEq => "<=",
            TokenKind::GtEq => ">=",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Bang => "!",
            TokenKind::Amp => "&",
            TokenKind::Pipe => "|",
            TokenKind::Caret => "^",
            TokenKind::Shl => "<<",
            TokenKind::Shr => ">>",
            TokenKind::AndAnd => "&&",
            TokenKind::OrOr => "||",
            TokenKind::Tilde => "~",
            TokenKind::Possessive => "'s",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::Eof => "end of input",
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.describe())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
