mod ast;
mod lexer;
mod span;
mod token;

use lexer::Lexer;
use miette::Error;
use token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source: &'a str,
    syntax_errors: Vec<Error>,
    current_token: Token,
    end_of_previous_token: u32,
    state: ParserState<'a>,
    context: Context,
    ast_builder: AstBuilder<'a>,
}

struct ParserState<'a>;
struct Context;
struct AstBuilder<'a>;

#[cfg(test)]
mod test {
    use oxc_allocator::Allocator;

    #[test]
    pub fn test() {
        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, r#"<?php echo 1;"#);
    }
}
