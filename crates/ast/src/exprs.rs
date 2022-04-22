#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use crate::{Node, Span};
use lunar_macros::PropertyGetter;

mod op;
pub use op::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Paren(Box<Expr>),
    Suffixed(Suffixed),
    TypeAssertion(TypeAssertion),
    Unary(Unary),
}

impl Node for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Binary(node) => node.span(),
            Expr::Literal(node) => node.span(),
            Expr::Paren(node) => node.span(),
            Expr::Suffixed(node) => node.span(),
            Expr::TypeAssertion(node) => node.span(),
            Expr::Unary(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct TypeAssertion {
    base: Box<Expr>,
    cast: TypeInfo,
}

impl Node for TypeAssertion {
    fn span(&self) -> Span {
        Span::from_two_spans(self.base.span(), self.cast.span())
    }
}

pub type ExprList = Vec<Expr>;

/// Due to the limitations of implementing traits with a type
/// that is not belonged to a crate.
///
/// This function will help, but it will return as `Span(0,0)` if
/// the vector is empty.
pub fn vector_span<N: Node>(vec: &[N]) -> Span {
    let first = vec.first().map(|v| v.span().start()).unwrap_or(0);
    let last = vec.last().map(|v| v.span().start()).unwrap_or(0);
    Span::new(first, last)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Args {
    ExprList(ExprList),
    Table(TableCtor),
    Str(Token),
}

impl Node for Args {
    fn span(&self) -> Span {
        match self {
            Args::ExprList(node) => vector_span(node),
            Args::Table(node) => node.span(),
            Args::Str(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(Token),
    Function(FunctionExpr),
    Name(Token),
    Number(Token),
    Nil(Token),
    Str(Token),
    Table(TableCtor),
    Varargs(Token),
}

impl Node for Literal {
    fn span(&self) -> Span {
        match self {
            Literal::Bool(node) => node.span(),
            Literal::Function(node) => node.span(),
            Literal::Name(node) => node.span(),
            Literal::Number(node) => node.span(),
            Literal::Nil(node) => node.span(),
            Literal::Str(node) => node.span(),
            Literal::Table(node) => node.span(),
            Literal::Varargs(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SuffixKind {
    Call(Args),
    Computed(Box<Expr>),
    Method(Token),
    Name(Token),
}

impl Node for SuffixKind {
    fn span(&self) -> Span {
        match self {
            SuffixKind::Call(node) => node.span(),
            SuffixKind::Computed(node) => node.span(),
            SuffixKind::Method(node) => node.span(),
            SuffixKind::Name(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct Suffixed {
    base: Box<Expr>,
    suffix_span: Span,
    suffix: SuffixKind,
}

impl Node for Suffixed {
    fn span(&self) -> Span {
        Span::from_two_spans(self.base.span(), self.suffix_span)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TableField {
    Array(Box<Expr>),
    Expr {
        span: Span,
        index: Box<Expr>,
        value: Box<Expr>,
    },
}

impl Node for TableField {
    fn span(&self) -> Span {
        match self {
            TableField::Array(exp) => exp.span(),
            TableField::Expr { span, .. } => *span,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct TableCtor {
    span: Span,
    fields: Vec<TableField>,
}

impl Node for TableCtor {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct Binary {
    left: Box<Expr>,
    op: Binop,
    right: Box<Expr>,
}

impl Node for Binary {
    fn span(&self) -> Span {
        Span::from_two_spans(self.left.span(), self.right.span())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct Unary {
    op: Unop,
    expr: Box<Expr>,
}

impl Node for Unary {
    fn span(&self) -> Span {
        Span::from_two_spans(self.op.token.span(), self.expr.span())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Name(Token),
    Varargs(Token),
}

impl Node for Param {
    fn span(&self) -> Span {
        match self {
            Param::Name(_) => todo!(),
            Param::Varargs(_) => todo!(),
        }
    }
}

pub type ParamList = Vec<Param>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct FunctionBody {
    span: Span,
    params: ParamList,
    block: Block,
}

impl Node for FunctionBody {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct FunctionExpr {
    span: Span,
    body: FunctionBody,
}

impl Node for FunctionExpr {
    fn span(&self) -> Span {
        self.span
    }
}
