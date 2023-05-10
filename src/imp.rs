use crate::{Number, Truth, VarName};

/// プログラミング言語 IMP の構文解析木
///
/// ```text
/// IMP  ::= N | T | Loc | Aexp | Bexp | Com
/// N    ::= 整数
/// T    ::= "true" | "false"
/// Loc  ::= プログラム変数（X,Y,Z,...）
///
/// Aexp ::= N | Loc | Aexp "+" Aexp | Aexp "-" Aexp | Aexp "*" Aexp
/// Bexp ::= T | Aexp "=" Aexp | Aexp "<=" Aexp | "not" Bexp | Bexp "and" Bexp | Bexp "or" Bexp
/// Com  ::= "skip" | Loc ":=" Aexp | Com ";" Com | "if" Bexp "then" Com "else" Com | "while" Bexp "do" Com
/// ```
#[derive(Debug, PartialEq)]
pub enum IMP {
    Aexp(Aexp),
    Bexp(Bexp),
    Com(Com),
}

#[derive(Debug, PartialEq)]
pub enum Aexp {
    N(Number),
    Loc(VarName),
    Add(Box<Aexp>, Box<Aexp>),
    Sub(Box<Aexp>, Box<Aexp>),
    Mul(Box<Aexp>, Box<Aexp>),
}

#[derive(Debug, PartialEq)]
pub enum Bexp {
    T(Truth),
    Eq(Aexp, Aexp),
    Le(Aexp, Aexp),
    Not(Box<Bexp>),
    And(Box<Bexp>, Box<Bexp>),
    Or(Box<Bexp>, Box<Bexp>),
}

#[derive(Debug, PartialEq)]
pub enum Com {
    Skip,
    Subst(VarName, Aexp),
    Seq(Box<Com>, Box<Com>),
    If(Bexp, Box<Com>, Box<Com>),
    While(Bexp, Box<Com>),
}

#[cfg(test)]
mod tests {
    use crate::imp::*;

    #[test]
    fn equals() {
        // 3 ≡ 3
        assert_eq!(IMP::Aexp(Aexp::N(Number(3))), IMP::Aexp(Aexp::N(Number(3))));

        // ¬( 8 ≡ 3 + 5 )
        assert_ne!(
            IMP::Aexp(Aexp::N(Number(8))),
            IMP::Aexp(Aexp::Add(
                Box::new(Aexp::N(Number(3))),
                Box::new(Aexp::N(Number(5)))
            ))
        );

        // ¬( 5 + 3 ≡ 3 + 5 )
        assert_ne!(
            IMP::Aexp(Aexp::Add(
                Box::new(Aexp::N(Number(5))),
                Box::new(Aexp::N(Number(3)))
            )),
            IMP::Aexp(Aexp::Add(
                Box::new(Aexp::N(Number(3))),
                Box::new(Aexp::N(Number(5)))
            ))
        );
    }
}
