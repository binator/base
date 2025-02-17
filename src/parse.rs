use core::fmt::Debug;

use binator_core::Parse;

/// Take a parser and return a Parser that will call parse on it.
pub fn parse<Stream, Context, Parser>(
  mut parser: Parser,
) -> impl Parse<Stream, Context, Token = Parser::Token>
where
  Parser: Parse<Stream, Context>,
  Parser::Token: Debug,
{
  move |stream: Stream| parser.parse(stream)
}
