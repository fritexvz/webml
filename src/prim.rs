use crate::util::PP;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String, pub u64);

impl Symbol {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Symbol(s.into(), 0)
    }
}

impl PP for Symbol {
    fn pp<W: io::Write>(&self, w: &mut W, _indent: usize) -> io::Result<()> {
        write!(w, "{}@{}", self.0, self.1)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Real(f64),
    Char(u32),
}

impl PP for Literal {
    fn pp<W: io::Write>(&self, w: &mut W, _indent: usize) -> io::Result<()> {
        use self::Literal::*;
        match self {
            Int(v) => {
                write!(w, "{}", v)?;
            }
            Real(v) => {
                write!(w, "{}", v)?;
            }
            Char(c) => {
                write!(w, r##"#"{}""##, c)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BIF {
    Add,
    Sub,
    Mul,
    Div,
    Divf,
    Mod,
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
}

impl PP for BIF {
    fn pp<W: io::Write>(&self, w: &mut W, _indent: usize) -> io::Result<()> {
        use self::BIF::*;
        match self {
            Add => {
                write!(w, "add")?;
            }
            Sub => {
                write!(w, "sub")?;
            }
            Mul => {
                write!(w, "mul")?;
            }
            Div => {
                write!(w, "div")?;
            }
            Divf => {
                write!(w, "divf")?;
            }
            Mod => {
                write!(w, "mod")?;
            }
            Eq => {
                write!(w, "eq")?;
            }
            Neq => {
                write!(w, "neq")?;
            }
            Gt => {
                write!(w, "gt")?;
            }
            Ge => {
                write!(w, "ge")?;
            }
            Lt => {
                write!(w, "lt")?;
            }
            Le => {
                write!(w, "le")?;
            }
        }
        Ok(())
    }
}
