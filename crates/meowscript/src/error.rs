use crate::span::{Span, line_text, position};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    Name,
    Type,
    Arity,
    Import,
    Io,
    Runtime,
    /// A `yowl` that no `curious` caught.
    Thrown,
}

impl ErrorKind {
    pub fn label(self) -> &'static str {
        match self {
            ErrorKind::Syntax => "syntax error",
            ErrorKind::Name => "name error",
            ErrorKind::Type => "type error",
            ErrorKind::Arity => "arity error",
            ErrorKind::Import => "import error",
            ErrorKind::Io => "io error",
            ErrorKind::Runtime => "runtime error",
            ErrorKind::Thrown => "uncaught yowl",
        }
    }

    /// What `caught` sees as the error's `kind`.
    pub fn word(self) -> &'static str {
        match self {
            ErrorKind::Syntax => "syntax",
            ErrorKind::Name => "name",
            ErrorKind::Type => "type",
            ErrorKind::Arity => "arity",
            ErrorKind::Import => "import",
            ErrorKind::Io => "io",
            ErrorKind::Runtime => "runtime",
            ErrorKind::Thrown => "yowl",
        }
    }

    pub fn exclamation(self) -> &'static str {
        match self {
            ErrorKind::Syntax => "Meowch!",
            ErrorKind::Name => "Meow-sterious!",
            ErrorKind::Type => "Furbidden!",
            ErrorKind::Arity => "Paws off!",
            ErrorKind::Import => "Lost kitten!",
            ErrorKind::Io => "Scratched!",
            ErrorKind::Runtime => "Hiss!",
            ErrorKind::Thrown => "Yowl!",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub span: Option<Span>,
    /// The parser ran out of input mid-construct. The REPL keeps reading lines when it sees this.
    pub incomplete: bool,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            span: None,
            incomplete: false,
        }
    }

    pub fn syntax(message: impl Into<String>, span: Span) -> Self {
        Self::new(ErrorKind::Syntax, message).at(span)
    }

    pub fn name(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Name, message)
    }

    pub fn type_(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Type, message)
    }

    pub fn arity(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Arity, message)
    }

    pub fn import(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Import, message)
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Io, message)
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Runtime, message)
    }

    pub fn at(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Only fills in a missing span, so the innermost expression keeps its position.
    pub fn or_at(mut self, span: Span) -> Self {
        self.span.get_or_insert(span);
        self
    }

    pub fn incomplete(mut self) -> Self {
        self.incomplete = true;
        self
    }

    /// Counts bytes, not characters, so a span that belongs to another file can't land inside a
    /// character and panic.
    pub fn line(&self, source: &str) -> Option<usize> {
        let before = source.as_bytes().get(..self.span?.start)?;
        Some(before.iter().filter(|&&b| b == b'\n').count() + 1)
    }

    pub fn render(&self, source: &str, filename: Option<&str>) -> String {
        let mut out = self.to_string();
        let Some(span) = self.span else {
            return out;
        };
        let pos = position(source, span.start);
        let line = line_text(source, span.start);
        let gutter = pos.line.to_string();
        let pad = " ".repeat(gutter.len());
        let name = filename.unwrap_or("<input>");
        let remaining = line.chars().count().saturating_sub(pos.column - 1).max(1);
        let width = span.len().clamp(1, remaining);
        out.push_str(&format!(
            "\n {pad}--> {name}:{pos}\n {pad} |\n {gutter} | {line}\n {pad} | {caret_pad}{carets}",
            caret_pad = " ".repeat(pos.column - 1),
            carets = "^".repeat(width),
        ));
        out
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.kind.exclamation(),
            self.kind.label(),
            self.message
        )
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_points_at_the_span() {
        let src = "scratch x = ;";
        let err = Error::syntax("expected an expression, found `;`", Span::new(12, 13));
        let text = err.render(src, Some("test.meow"));
        assert!(text.contains("test.meow:1:13"), "{text}");
        assert!(text.ends_with('^'), "{text}");
    }

    #[test]
    fn line_counts_from_one() {
        let src = "meow(1);\nmeow(nope);";
        let err = Error::name("`nope` hasn't been `scratch`ed yet").at(Span::new(14, 18));
        assert_eq!(err.line(src), Some(2));
        assert_eq!(Error::runtime("no span").line(src), None);
    }
}
