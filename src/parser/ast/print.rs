use std::fmt;

use crate::lexer::token::Token;
use crate::parser::ast::core::*;

impl AssignExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            AssignExpr::Bool(e) => e.fmt_with_indent(f, indent),
            AssignExpr::Assign(lhs, rhs) => {
                write!(f, "\n{}Assign({:#?})", indent_str, Token::AssignOp)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl BoolExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            BoolExpr::BitOr(e) => e.fmt_with_indent(f, indent),
            BoolExpr::Bool(lhs, op, rhs) => {
                write!(f, "\n{}Bool({:#?})", indent_str, op)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl BitOrExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            BitOrExpr::BitAnd(e) => e.fmt_with_indent(f, indent),
            BitOrExpr::BitOr(lhs, rhs) => {
                write!(f, "\n{}BitOr({:#?})", indent_str, Token::BitwiseOr)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl BitAndExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            BitAndExpr::Comp(e) => e.fmt_with_indent(f, indent),
            BitAndExpr::BitAnd(lhs, rhs) => {
                write!(f, "\n{}BitAnd({:#?})", indent_str, Token::BitwiseAnd)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl CompExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            CompExpr::Shift(e) => e.fmt_with_indent(f, indent),
            CompExpr::Comp(lhs, op, rhs) => {
                write!(f, "\n{}Comp({:#?})", indent_str, op)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl ShiftExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            ShiftExpr::Add(e) => e.fmt_with_indent(f, indent),
            ShiftExpr::Shift(lhs, op, rhs) => {
                write!(f, "\n{}Shift({:#?})", indent_str, op)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl AddExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);

        match self {
            AddExpr::Mul(e) => e.fmt_with_indent(f, indent),
            AddExpr::Add(lhs, op, rhs) => {
                write!(f, "\n{}::Add({:#?})", indent_str, op)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl MulExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);

        match self {
            MulExpr::Exp(e) => e.fmt_with_indent(f, indent),
            MulExpr::Mul(lhs, op, rhs) => {
                write!(f, "\n{}::Mul({:#?})", indent_str, op)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl ExpExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            ExpExpr::Unary(e) => e.fmt_with_indent(f, indent),
            ExpExpr::Exp(lhs, rhs) => {
                write!(f, "\n{}Exp({:#?})", indent_str, Token::ExpOp)?;
                lhs.fmt_with_indent(f, indent + 1)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl UnaryExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            UnaryExpr::Primary(e) => e.fmt_with_indent(f, indent),
            UnaryExpr::Unary(op, rhs) => {
                write!(f, "\n{}Unary({:#?})", indent_str, op)?;
                rhs.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n{})", indent_str)
            }
        }
    }
}

impl PrimaryExpr {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> Result<(), std::fmt::Error> {
        let indent_str = " ".repeat(indent * 4);
        match self {
            PrimaryExpr::IntLit(e) => write!(f, "\n{}{:?}", indent_str, e),
            PrimaryExpr::FloatLit(e) => write!(f, "\n{}{:?}", indent_str, e),
            PrimaryExpr::StringLit(e) => write!(f, "\n{}{:?}", indent_str, e),
            PrimaryExpr::Ident(e) => write!(f, "\n{}{:?}", indent_str, e),
            PrimaryExpr::Paren(e) => e.fmt_with_indent(f, indent + 1),
            PrimaryExpr::Call(fn_call) => {
                write!(f, "\n{}Call({:#?})", indent_str, fn_call)
            }
        }
    }
}

impl fmt::Debug for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for BoolExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for BitOrExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for BitAndExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for CompExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for ShiftExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for AddExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for MulExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for ExpExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}

impl fmt::Debug for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.fmt_with_indent(f, 1)
    }
}
