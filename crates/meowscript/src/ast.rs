use crate::span::Span;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: Rc<str>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Let {
        name: Ident,
        value: Expr,
    },
    /// `target` is an `Ident`, `Index`, or `Member` expression; the parser checks.
    Assign {
        target: Expr,
        value: Expr,
    },
    Return {
        value: Option<Expr>,
    },
    Import {
        path: String,
        span: Span,
    },
    Break,
    Continue,
    FnDecl {
        name: Ident,
        func: Rc<FuncDef>,
    },
    Loop {
        cond: Option<Expr>,
        body: Block,
    },
    ForIn {
        var: Ident,
        iter: Expr,
        body: Block,
    },
    Expr(Expr),
}

/// Shared through `Rc` so closures don't clone the body on every evaluation of a `pawction` expression.
#[derive(Clone, Debug, PartialEq)]
pub struct FuncDef {
    pub name: Option<Rc<str>>,
    pub params: Vec<Ident>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Null,
    Bool(bool),
    Number(f64),
    Str(Rc<str>),
    Ident(Rc<str>),
    Array(Vec<Expr>),
    Object(Vec<(Expr, Expr)>),
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Block,
        otherwise: Option<Box<Else>>,
    },
    Function(Rc<FuncDef>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Member {
        object: Box<Expr>,
        name: Ident,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Else {
    If(Expr),
    Block(Block),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    Typeof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    And,
    Or,
    In,
}

impl BinaryOp {
    /// Left and right binding powers. Right is one higher so every operator is left-associative.
    pub fn binding_power(self) -> (u8, u8) {
        use BinaryOp::*;
        let base = match self {
            Or => 1,
            And => 3,
            Eq | NotEq => 5,
            Lt | Gt | LtEq | GtEq | In => 7,
            BitOr => 9,
            BitXor => 11,
            BitAnd => 13,
            Shl | Shr => 15,
            Add | Sub => 17,
            Mul | Div | Rem => 19,
        };
        (base, base + 1)
    }

    pub fn symbol(self) -> &'static str {
        use BinaryOp::*;
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Rem => "%",
            Eq => "==",
            NotEq => "!=",
            Lt => "<",
            Gt => ">",
            LtEq => "<=",
            GtEq => ">=",
            BitAnd => "&",
            BitOr => "|",
            BitXor => "^",
            Shl => "<<",
            Shr => ">>",
            And => "&&",
            Or => "||",
            In => "~",
        }
    }
}

impl UnaryOp {
    pub fn symbol(self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::Typeof => "furreal",
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

impl Expr {
    pub fn ends_with_block(&self) -> bool {
        matches!(self.kind, ExprKind::If { .. } | ExprKind::Function(_))
    }
}
