use crate::scanner::Token;

pub trait Visitor<T: ?Sized> {
    type Return;
    fn visit(value: &T) -> Self::Return;
}

pub trait AcceptVisitor {
    fn accept<V: Visitor<Self>>(&self) -> V::Return {
        V::visit(self)
    }
}

pub struct ASTPrint;
impl Visitor<Expr<'_>> for ASTPrint {
    type Return = String;

    fn visit(value: &Expr<'_>) -> Self::Return {
        match value {
            Expr::LiteralString(s) => format!("literal {}", s),
            Expr::LiteralNumber(n) => format!("literal {}", n),
            Expr::LiteralTrue => "literal true".to_string(),
            Expr::LiteralFalse => "literal false".to_string(),
            Expr::LiteralNil => "literal nil".to_string(),
            Expr::Grouping { expression } => format!("grouping ( {} )", Self::visit(expression)),
            Expr::Unary { prefix, expression } => {
                format!("unary {} {}", prefix, Self::visit(expression))
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "binary {} {} {}",
                Self::visit(left),
                operator,
                Self::visit(right)
            ),
        }
    }
}

pub enum Expr<'a> {
    LiteralString(String),
    LiteralNumber(f64),
    LiteralTrue,
    LiteralFalse,
    LiteralNil,
    Grouping {
        expression: &'a Expr<'a>,
    },
    Unary {
        prefix: Token,
        expression: &'a Expr<'a>,
    },
    Binary {
        left: &'a Expr<'a>,
        operator: Token,
        right: &'a Expr<'a>,
    },
}

impl AcceptVisitor for Expr<'_> {}
