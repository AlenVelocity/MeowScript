use crate::error::{Error, Result};
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    Lexer {
        src: source,
        pos: 0,
    }
    .run()
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn run(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            self.skip_trivia()?;
            let start = self.pos;
            let Some(c) = self.peek() else {
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    span: Span::new(start, start),
                });
                return Ok(tokens);
            };
            let kind = match c {
                '"' => self.string()?,
                '\'' => self.possessive()?,
                c if c.is_ascii_digit() => self.number()?,
                c if is_ident_start(c) => self.ident(),
                _ => self.punct()?,
            };
            tokens.push(Token {
                kind,
                span: Span::new(start, self.pos),
            });
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_at(&self, n: usize) -> Option<char> {
        self.src[self.pos..].chars().nth(n)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn eat(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.pos += expected.len_utf8();
            true
        } else {
            false
        }
    }

    fn skip_trivia(&mut self) -> Result<()> {
        loop {
            match (self.peek(), self.peek_at(1)) {
                (Some(c), _) if c.is_whitespace() => {
                    self.bump();
                }
                (Some('/'), Some('/')) => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.bump();
                    }
                }
                (Some('/'), Some('*')) => {
                    let start = self.pos;
                    self.pos += 2;
                    loop {
                        match (self.peek(), self.peek_at(1)) {
                            (Some('*'), Some('/')) => {
                                self.pos += 2;
                                break;
                            }
                            (None, _) => {
                                return Err(Error::syntax(
                                    "this block comment never closes; it needs a `*/`",
                                    Span::new(start, self.pos),
                                )
                                .incomplete());
                            }
                            _ => {
                                self.bump();
                            }
                        }
                    }
                }
                _ => return Ok(()),
            }
        }
    }

    fn ident(&mut self) -> TokenKind {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if is_ident_continue(c) {
                self.bump();
            } else {
                break;
            }
        }
        let text = &self.src[start..self.pos];
        TokenKind::keyword(text).unwrap_or_else(|| TokenKind::Ident(text.to_string()))
    }

    // A fraction needs digits on both sides of the dot, so `1.` and `arr[0].x` are reported
    // rather than mis-lexed.
    fn number(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        self.digits();
        if self.peek() == Some('.') && self.peek_at(1).is_some_and(|c| c.is_ascii_digit()) {
            self.bump();
            self.digits();
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            let save = self.pos;
            self.bump();
            if matches!(self.peek(), Some('+' | '-')) {
                self.bump();
            }
            if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.digits();
            } else {
                self.pos = save;
            }
        }
        let text: String = self.src[start..self.pos]
            .chars()
            .filter(|&c| c != '_')
            .collect();
        text.parse::<f64>().map(TokenKind::Number).map_err(|_| {
            Error::syntax(
                format!("`{text}` is not a number I understand"),
                Span::new(start, self.pos),
            )
        })
    }

    fn digits(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn string(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        self.bump();
        let mut text = String::new();
        loop {
            let Some(c) = self.bump() else {
                return Err(Error::syntax(
                    "this string never closes; it needs a closing `\"`",
                    Span::new(start, self.pos),
                )
                .incomplete());
            };
            match c {
                '"' => return Ok(TokenKind::Str(text)),
                '\\' => {
                    let escape_start = self.pos - 1;
                    let Some(e) = self.bump() else {
                        return Err(Error::syntax(
                            "this string never closes; it needs a closing `\"`",
                            Span::new(start, self.pos),
                        )
                        .incomplete());
                    };
                    match e {
                        'n' => text.push('\n'),
                        't' => text.push('\t'),
                        'r' => text.push('\r'),
                        '0' => text.push('\0'),
                        '\\' => text.push('\\'),
                        '"' => text.push('"'),
                        '\'' => text.push('\''),
                        'u' => text.push(self.unicode_escape(escape_start)?),
                        other => {
                            return Err(Error::syntax(
                                format!(
                                    "`\\{other}` is not an escape I know; try \\n, \\t, \\\", \\\\ or \\u{{…}}"
                                ),
                                Span::new(escape_start, self.pos),
                            ));
                        }
                    }
                }
                c => text.push(c),
            }
        }
    }

    fn unicode_escape(&mut self, escape_start: usize) -> Result<char> {
        let bad = |end: usize| {
            Error::syntax(
                "unicode escapes look like \\u{1F431}",
                Span::new(escape_start, end),
            )
        };
        if !self.eat('{') {
            return Err(bad(self.pos));
        }
        let digits_start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
            self.bump();
        }
        let digits = &self.src[digits_start..self.pos];
        if !self.eat('}') || digits.is_empty() || digits.len() > 6 {
            return Err(bad(self.pos));
        }
        u32::from_str_radix(digits, 16)
            .ok()
            .and_then(char::from_u32)
            .ok_or_else(|| bad(self.pos))
    }

    fn possessive(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        self.bump();
        if self.peek() == Some('s') && !self.peek_at(1).is_some_and(is_ident_continue) {
            self.bump();
            return Ok(TokenKind::Possessive);
        }
        Err(Error::syntax(
            "a lone `'` doesn't mean anything; strings use double quotes and property access is `'s`",
            Span::new(start, self.pos),
        ))
    }

    fn punct(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        let c = self
            .bump()
            .expect("punct is only called with a char available");
        let kind = match c {
            '=' if self.eat('=') => TokenKind::Eq,
            '=' => TokenKind::Assign,
            '!' if self.eat('=') => TokenKind::NotEq,
            '!' => TokenKind::Bang,
            '<' if self.eat('<') => TokenKind::Shl,
            '<' if self.eat('=') => TokenKind::LtEq,
            '<' => TokenKind::Lt,
            '>' if self.eat('>') => TokenKind::Shr,
            '>' if self.eat('=') => TokenKind::GtEq,
            '>' => TokenKind::Gt,
            '&' if self.eat('&') => TokenKind::AndAnd,
            '&' => TokenKind::Amp,
            '|' if self.eat('|') => TokenKind::OrOr,
            '|' => TokenKind::Pipe,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            other => {
                return Err(Error::syntax(
                    format!("I don't know what to do with `{other}`"),
                    Span::new(start, self.pos),
                ));
            }
        };
        Ok(kind)
    }
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        tokenize(src).unwrap().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn empty_input_is_just_eof() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
        assert_eq!(kinds("   \n\t"), vec![TokenKind::Eof]);
    }

    #[test]
    fn keywords_and_identifiers() {
        assert_eq!(
            kinds("scratch cat = purrfect;"),
            vec![
                TokenKind::Scratch,
                TokenKind::Ident("cat".into()),
                TokenKind::Assign,
                TokenKind::Purrfect,
                TokenKind::Semicolon,
                TokenKind::Eof
            ]
        );
        assert_eq!(
            kinds("scratchy"),
            vec![TokenKind::Ident("scratchy".into()), TokenKind::Eof]
        );
        assert_eq!(
            kinds("cat_2"),
            vec![TokenKind::Ident("cat_2".into()), TokenKind::Eof]
        );
    }

    #[test]
    fn numbers() {
        assert_eq!(kinds("42"), vec![TokenKind::Number(42.0), TokenKind::Eof]);
        assert_eq!(kinds("3.5"), vec![TokenKind::Number(3.5), TokenKind::Eof]);
        assert_eq!(
            kinds("1_000"),
            vec![TokenKind::Number(1000.0), TokenKind::Eof]
        );
        assert_eq!(
            kinds("2e3"),
            vec![TokenKind::Number(2000.0), TokenKind::Eof]
        );
    }

    #[test]
    fn strings_with_escapes() {
        assert_eq!(
            kinds(r#""meow\n\"purr\" \u{1F431}""#),
            vec![TokenKind::Str("meow\n\"purr\" 🐱".into()), TokenKind::Eof]
        );
    }

    #[test]
    fn unterminated_string_is_incomplete() {
        let err = tokenize("\"meow").unwrap_err();
        assert!(err.incomplete);
        assert_eq!(err.span, Some(Span::new(0, 5)));
    }

    #[test]
    fn possessive() {
        assert_eq!(
            kinds("cat's name"),
            vec![
                TokenKind::Ident("cat".into()),
                TokenKind::Possessive,
                TokenKind::Ident("name".into()),
                TokenKind::Eof
            ]
        );
        assert!(tokenize("cat'sname").is_err());
        assert!(tokenize("'hello'").is_err());
    }

    #[test]
    fn operators() {
        assert_eq!(
            kinds("a <= b << c && d || !e != f"),
            vec![
                TokenKind::Ident("a".into()),
                TokenKind::LtEq,
                TokenKind::Ident("b".into()),
                TokenKind::Shl,
                TokenKind::Ident("c".into()),
                TokenKind::AndAnd,
                TokenKind::Ident("d".into()),
                TokenKind::OrOr,
                TokenKind::Bang,
                TokenKind::Ident("e".into()),
                TokenKind::NotEq,
                TokenKind::Ident("f".into()),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn comments_are_skipped() {
        assert_eq!(
            kinds("1 // one\n/* two\nlines */ 2"),
            vec![
                TokenKind::Number(1.0),
                TokenKind::Number(2.0),
                TokenKind::Eof
            ]
        );
        assert!(tokenize("/* open").unwrap_err().incomplete);
    }

    #[test]
    fn spans_are_byte_ranges() {
        let toks = tokenize("ab  \"c\"").unwrap();
        assert_eq!(toks[0].span, Span::new(0, 2));
        assert_eq!(toks[1].span, Span::new(4, 7));
    }

    #[test]
    fn unknown_character() {
        let err = tokenize("a @ b").unwrap_err();
        assert_eq!(err.span, Some(Span::new(2, 3)));
    }
}
