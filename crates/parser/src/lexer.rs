use miette::Error;
use oxc_allocator::Allocator;
use std::{collections::VecDeque, str::Chars};

#[derive(Debug, Clone)]
struct LexerCheckpoint<'a> {
    chars: Chars<'a>,
    token: Token,
    error_pos: usize,
}

pub struct Lexer<'a> {
    allocator: &'a Allocator,
    source: &'a str,
    current: LexerCheckpoint<'a>,
    pub(crate) errors: Vec<Error>,
    lookahead: VecDeque<LexerCheckpoint<'a>>,
}
