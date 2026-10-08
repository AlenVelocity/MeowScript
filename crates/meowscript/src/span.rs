use std::fmt;

/// Half-open byte range `[start, end)` into the source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to(self, other: Span) -> Span {
        Span::new(self.start, other.end.max(self.start))
    }

    pub fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

/// 1-based line and column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Columns count characters, not bytes, so they match what an editor shows.
pub fn position(source: &str, offset: usize) -> Position {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.matches('\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    let column = before[line_start..].chars().count() + 1;
    Position { line, column }
}

pub fn line_text(source: &str, offset: usize) -> &str {
    let offset = offset.min(source.len());
    let start = source[..offset].rfind('\n').map_or(0, |i| i + 1);
    let end = source[offset..]
        .find('\n')
        .map_or(source.len(), |i| offset + i);
    source[start..end].trim_end_matches('\r')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_one_based() {
        let src = "ab\ncd";
        assert_eq!(position(src, 0), Position { line: 1, column: 1 });
        assert_eq!(position(src, 1), Position { line: 1, column: 2 });
        assert_eq!(position(src, 3), Position { line: 2, column: 1 });
        assert_eq!(position(src, 4), Position { line: 2, column: 2 });
    }

    #[test]
    fn columns_count_chars() {
        let src = "🐱x";
        assert_eq!(position(src, 4).column, 2);
    }

    #[test]
    fn line_text_strips_newline() {
        assert_eq!(line_text("one\ntwo\r\nthree", 5), "two");
    }
}
