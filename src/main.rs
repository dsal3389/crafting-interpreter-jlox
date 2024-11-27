use anyhow::{bail, Result};
use ast::AcceptVisitor;
use std::env;
use std::fs;
use std::path::PathBuf;

mod ast;
mod error;
mod scanner;

use ast::{ASTPrint, Expr};
use scanner::{Scanner, Token, TokenKind};

fn main() -> Result<()> {
    let expr = Expr::Binary {
        left: &Expr::Unary {
            prefix: Token::new(TokenKind::Minus, "-".into(), "".into(), 1),
            expression: &Expr::LiteralNumber(55.5),
        },
        operator: Token::new(TokenKind::Star, "*".into(), "".into(), 1),
        right: &Expr::Grouping {
            expression: &Expr::LiteralNumber(77.0),
        },
    };

    println!("{}", expr.accept::<ASTPrint>());

    Ok(())
    // match env::args().nth(1) {
    //     Some(p) => {
    //         let path = PathBuf::from(p);
    //         if !path.exists() {
    //             bail!(format!("given path `{:?}` does not exists", path));
    //         }

    //         let scanner = Scanner::new(fs::read(path).unwrap());

    //         for token in scanner {
    //             if let Err(e) = token {
    //                 bail!(format!("{}", e));
    //             }

    //             let token = token.unwrap();
    //             match token.kind() {
    //                 TokenKind::WhiteSpace | TokenKind::NewLine | TokenKind::Comment => {}
    //                 _ => {
    //                     println!("{}", token);
    //                 }
    //             }
    //         }
    //         Ok(())
    //     }
    //     None => Ok(()),
    // }
}
