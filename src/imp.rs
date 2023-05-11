use crate::{Number, State, Truth, VarName};

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

impl Aexp {
    pub fn evaluate(&self, state: &State) -> Number {
        match &self {
            Aexp::N(n) => n.0.into(),
            Aexp::Loc(var) => state
                .get(var)
                .as_ref()
                .expect(format!("variable {} is undefined", var).as_str())
                .to_owned(),
            Aexp::Add(left, right) => (left.evaluate(&state).0 + right.evaluate(&state).0).into(),
            Aexp::Sub(left, right) => (left.evaluate(&state).0 - right.evaluate(&state).0).into(),
            Aexp::Mul(left, right) => (left.evaluate(&state).0 * right.evaluate(&state).0).into(),
        }
    }
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
    use crate::{
        imp::{Aexp, IMP},
        Number, State,
    };

    #[test]
    fn equals() {
        // 3 ≡ 3
        assert_eq!(IMP::Aexp(Aexp::N(3.into())), IMP::Aexp(Aexp::N(3.into())));

        // ¬( 8 ≡ 3 + 5 )
        assert_ne!(
            IMP::Aexp(Aexp::N(8.into())),
            IMP::Aexp(Aexp::Add(
                Box::new(Aexp::N(3.into())),
                Box::new(Aexp::N(5.into()))
            ))
        );

        // ¬( 5 + 3 ≡ 3 + 5 )
        assert_ne!(
            IMP::Aexp(Aexp::Add(
                Box::new(Aexp::N(5.into())),
                Box::new(Aexp::N(3.into()))
            )),
            IMP::Aexp(Aexp::Add(
                Box::new(Aexp::N(3.into())),
                Box::new(Aexp::N(5.into()))
            ))
        );
    }

    #[test]
    fn evaluate_number() {
        let state = State::init();

        // 〈2, σ〉 → 2
        assert_eq!(Number(2), Aexp::N(2.into()).evaluate(&state));

        // 〈5, σ〉 → 5
        assert_eq!(Number(5), Aexp::N(5.into()).evaluate(&state));
    }

    #[test]
    fn evaluate_variable() {
        // 〈Init, σ_0〉 → 0
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(Number(0), Aexp::Loc("Init".into()).evaluate(&state));
    }

    #[test]
    fn evaluate_addition() {
        // 〈7 + 9, σ〉 → 16
        let state = State::init();
        assert_eq!(
            Number(16),
            Aexp::Add(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(&state),
        );

        // 〈Init + 5, σ_0〉 → 5
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            Number(5),
            Aexp::Add(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(&state),
        );

        // 〈(Init + 5) + (7 + 9), σ_0〉 → 21
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            Number(21),
            Aexp::Add(
                Box::new(Aexp::Add(
                    Box::new(Aexp::Loc("Init".into())),
                    Box::new(Aexp::N(5.into()))
                )),
                Box::new(Aexp::Add(
                    Box::new(Aexp::N(7.into())),
                    Box::new(Aexp::N(9.into()))
                )),
            )
            .evaluate(&state),
        );
    }

    #[test]
    fn evaluate_subtraction() {
        // 〈7 - 9, σ〉 → -2
        let state = State::init();
        assert_eq!(
            Number(-2),
            Aexp::Sub(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(&state),
        );

        // 〈Init - 5, σ_0〉 → -5
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            Number(-5),
            Aexp::Sub(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(&state),
        );

        // 〈(Init - 5) - (7 - 9), σ_0〉 → -3
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            Number(-3),
            Aexp::Sub(
                Box::new(Aexp::Sub(
                    Box::new(Aexp::Loc("Init".into())),
                    Box::new(Aexp::N(5.into()))
                )),
                Box::new(Aexp::Sub(
                    Box::new(Aexp::N(7.into())),
                    Box::new(Aexp::N(9.into()))
                )),
            )
            .evaluate(&state),
        );
    }

    #[test]
    fn evaluate_multiplication() {
        // 〈7 * 9, σ〉 → 63
        let state = State::init();
        assert_eq!(
            Number(63),
            Aexp::Mul(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(&state),
        );

        // 〈Init * 5, σ_0〉 → 0
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            Number(0),
            Aexp::Mul(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(&state),
        );

        // 〈(Init * 5) * (7 * 9), σ_0〉 → 0
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            Number(0),
            Aexp::Mul(
                Box::new(Aexp::Mul(
                    Box::new(Aexp::Loc("Init".into())),
                    Box::new(Aexp::N(5.into()))
                )),
                Box::new(Aexp::Mul(
                    Box::new(Aexp::N(7.into())),
                    Box::new(Aexp::N(9.into()))
                )),
            )
            .evaluate(&state),
        );
    }
}
