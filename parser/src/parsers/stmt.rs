use crate::parse_expr;
use crate::parse_ident_typed;
use crate::parser::Parser;

use whistle_ast::Expr;
use whistle_ast::Stmt;
use whistle_lexer::Keyword;
use whistle_lexer::Punc;
use whistle_lexer::Tip;
use whistle_lexer::Token;

pub fn parse_stmt(parser: &mut Parser) -> Option<Stmt> {
  match parser.peek() {
    Some(Token::Keyword(Keyword::If)) => parse_if_stmt(parser),
    Some(Token::Keyword(Keyword::While)) => parse_while_stmt(parser),
    Some(Token::Keyword(Keyword::Continue)) => parse_continue_stmt(parser),
    Some(Token::Keyword(Keyword::Break)) => parse_break_stmt(parser),
    Some(Token::Keyword(Keyword::Return)) => parse_return_stmt(parser),
    Some(Token::Keyword(Keyword::Var)) => parse_var_decl(parser),
    Some(Token::Keyword(Keyword::Val)) => parse_val_decl(parser),
    Some(Token::Tip(_)) => parse_tip(parser),
    Some(Token::Punc(Punc::LeftBrace)) => parse_block_stmt(parser),
    _ => parse_expr_stmt(parser),
  }
}

pub fn parse_if_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
    if let Some(cond) = parse_expr(parser) {
      if let Some(then_stmt) = parse_stmt(parser) {
        let cond = Box::new(cond);
        let then_stmt = Box::new(then_stmt);

        return Some(Stmt::If {
          cond,
          then_stmt,
          else_stmt: parse_else_stmt(parser),
        });
      }
    }
  }

  None
}

pub fn parse_else_stmt(parser: &mut Parser) -> Option<Box<Stmt>> {
  if parser.eat_tok(Token::Keyword(Keyword::Else)).is_some() {
    if let Some(else_stmt) = parse_stmt(parser) {
      return Some(Box::new(else_stmt));
    }
  }

  None
}

pub fn parse_while_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::While)).is_some() {
    let cond = if let Some(cond) = parser.maybe(parse_expr) {
      Some(Box::new(cond))
    } else {
      None
    };
    if let Some(do_stmt) = parse_stmt(parser) {
      let do_stmt = Box::new(do_stmt);

      return Some(Stmt::While { cond, do_stmt });
    }
  }

  None
}

pub fn parse_continue_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Continue)).is_some() {
    return Some(Stmt::Continue);
  }

  None
}

pub fn parse_break_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Break)).is_some() {
    return Some(Stmt::Break);
  }

  None
}

pub fn parse_return_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Return)).is_some() {
    if let Some(expr) = parser.maybe(parse_expr) {
      return Some(Stmt::Return(Some(Box::new(expr))));
    } else {
      return Some(Stmt::Return(None));
    }
  }

  None
}

pub fn parse_assign(parser: &mut Parser) -> Option<Expr> {
  if let Some(Token::Operator(op)) = parser.peek() {
    if op.is_assign() {
      parser.step();
      return parse_expr(parser);
    }
  }

  None
}

pub fn parse_var_decl(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Var)).is_some() {
    if let Some(ident_typed) = parse_ident_typed(parser) {
      if let Some(value) = parse_assign(parser) {
        let val = Box::new(value);

        return Some(Stmt::VarDecl { ident_typed, val });
      }
    }
  }

  None
}

pub fn parse_val_decl(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Val)).is_some() {
    if let Some(ident_typed) = parse_ident_typed(parser) {
      if let Some(value) = parse_assign(parser) {
        let val = Box::new(value);

        return Some(Stmt::ValDecl { ident_typed, val });
      }
    }
  }

  None
}

pub fn parse_tip(parser: &mut Parser) -> Option<Stmt> {
  if let Some(Token::Tip(tip)) = parser.eat_type(Token::Tip(Tip {
    ident: String::new(),
    value: String::new(),
  })) {
    Some(Stmt::Tip(tip.to_owned()))
  } else {
    None
  }
}

pub fn parse_block_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Punc(Punc::LeftBrace)).is_some() {
    let stmts = parser.repeating(parse_stmt);
    if parser.eat_tok(Token::Punc(Punc::RightBrace)).is_some() {
      return Some(Stmt::Block(stmts));
    }
  }

  None
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Option<Stmt> {
  if let Some(expr) = parse_expr(parser) {
    return Some(Stmt::Expr(expr));
  }

  None
}