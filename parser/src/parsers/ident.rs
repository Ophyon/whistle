use crate::eat_type;
use crate::parse_expr;
use crate::parse_ident_type;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentImport;
use whistle_ast::IdentTyped;
use whistle_ast::IdentVal;
use whistle_ast::Primary;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_ident(parser: &mut Parser) -> Result<String, ParserError> {
  eat_type!(parser, Token::Ident)
}

pub fn parse_ident_typed(parser: &mut Parser) -> Result<IdentTyped, ParserError> {
  let ident = parse_ident(parser)?;
  let mut type_ident = None;
  if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    type_ident = Some(parse_ident_type(parser)?);
  }
  Ok(IdentTyped { ident, type_ident })
}

pub fn parse_ident_import(parser: &mut Parser) -> Result<IdentImport, ParserError> {
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Keyword(Keyword::As)).is_ok() {
    let as_ident = parse_ident(parser)?;
    return Ok(IdentImport {
      ident,
      as_ident: Some(as_ident),
    });
  }
  Ok(IdentImport {
    ident,
    as_ident: None,
  })
}

pub fn parse_ident_val(parser: &mut Parser, ident: String) -> Result<Primary, ParserError> {
  parse_ident(parser)?;
  let mut prim = Vec::new();
  while parser.within() {
    prim.push(match parser.peek()? {
      Token::Punc(Punc::Dot) => parse_selector(parser)?,
      Token::Punc(Punc::LeftParen) => parse_arguments(parser)?,
      _ => break,
    })
  }
  Ok(Primary::IdentVal { ident, prim })
}

pub fn parse_selector(parser: &mut Parser) -> Result<IdentVal, ParserError> {
  parser.eat_tok(Token::Punc(Punc::Dot))?;
  let ident = parse_ident(parser)?;
  Ok(IdentVal::Selector(ident))
}

pub fn parse_arguments(parser: &mut Parser) -> Result<IdentVal, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let mut args = Vec::new();
  if let Some(first) = parser.maybe(parse_expr) {
    args.push(first);
    args.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_expr(parser)
    }));
  }
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  Ok(IdentVal::Arguments(args))
}
