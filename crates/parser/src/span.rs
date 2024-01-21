use miette::{SourceOffset, SourceSpan};

pub const SPAN: Span = Span::new(0, 0);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> u32 {
        debug_assert!(self.start <= self.end);

        self.end - self.start
    }

    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        Self::new(self.start.min(other.start), self.end.max(other.end))
    }

    pub fn source_text<'a>(&self, source_text: &'a str) -> &'a str {
        &source_text[self.start as usize..self.end as usize]
    }
}

impl From<Span> for SourceSpan {
    fn from(value: Span) -> Self {
        Self::new(
            SourceOffset::from(value.start as usize),
            SourceOffset::from(value.size() as usize),
        )
    }
}

pub trait GetSpan {
    fn span(&self) -> Span;
}
