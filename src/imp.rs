//! プログラミング言語 IMP
//!
//! ```text
//! N    ::= 整数
//! T    ::= "true" | "false"
//! Loc  ::= プログラム変数（X,Y,Z,...）
//!
//! Aexp ::= N | Loc | Aexp "+" Aexp | Aexp "-" Aexp | Aexp "*" Aexp
//! Bexp ::= T | Aexp "=" Aexp | Aexp "<=" Aexp | "not" Bexp | Bexp "and" Bexp | Bexp "or" Bexp
//! Com  ::= "skip" | Loc ":=" Aexp | Com ";" Com | "if" Bexp "then" Com "else" Com | "while" Bexp "do" Com
//! ```
use crate::{Evaluate, Number, State, Truth, VarName};

/// プログラミング言語 IMP の構文解析木
#[derive(Debug, PartialEq)]
pub struct IMP(Com);

/// 算術式
#[derive(Debug, PartialEq)]
pub enum Aexp {
    /// 整数 n
    N(Number),
    /// プログラム変数 `X`
    Loc(VarName),
    /// 加算 `a_0 + a_1`
    Add(Box<Aexp>, Box<Aexp>),
    /// 減算 `a_0 - a_1`
    Sub(Box<Aexp>, Box<Aexp>),
    /// 乗算 `a_0 * a_1`
    Mul(Box<Aexp>, Box<Aexp>),
}

impl Evaluate<Number> for Aexp {
    fn evaluate(&self, state: &State) -> Number {
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

/// ブール式
#[derive(Debug, PartialEq)]
pub enum Bexp {
    /// 真偽値 `true`, `false`
    T(Truth),
    /// 等値比較 `a_0 = a_1`
    Eq(Aexp, Aexp),
    /// より小さいか等しい `a_0 <= a_1`
    Le(Aexp, Aexp),
    /// 否定 `not b`
    Not(Box<Bexp>),
    /// 論理積 `b_0 and b_1`
    And(Box<Bexp>, Box<Bexp>),
    /// 論理和 `b_0 or b_1`
    Or(Box<Bexp>, Box<Bexp>),
}

impl Evaluate<Truth> for Bexp {
    fn evaluate(&self, state: &State) -> Truth {
        match &self {
            Bexp::T(Truth(true)) => Truth(true),
            Bexp::T(Truth(false)) => Truth(false),
            Bexp::Eq(left, right) if left.evaluate(&state) == right.evaluate(&state) => Truth(true),
            Bexp::Eq(_, _) => Truth(false),
            Bexp::Le(left, right) if left.evaluate(&state) <= right.evaluate(&state) => Truth(true),
            Bexp::Le(_, _) => Truth(false),
            Bexp::Not(b) if b.evaluate(&state).0 => Truth(false),
            Bexp::Not(_) => Truth(true),
            Bexp::And(left, _) if !left.evaluate(&state).0 => Truth(false),
            Bexp::And(_, right) => right.evaluate(&state),
            Bexp::Or(left, _) if left.evaluate(&state).0 => Truth(true),
            Bexp::Or(_, right) => right.evaluate(&state),
        }
    }
}

/// コマンド
#[derive(Debug, PartialEq)]
pub enum Com {
    /// 基礎コマンド
    Skip,
    /// 代入 `X := a`
    Subst(VarName, Aexp),
    /// 逐次実行 `c_0 ; c_1`
    Seq(Box<Com>, Box<Com>),
    /// 条件分岐 `if b then c_0 else c_1`
    If(Bexp, Box<Com>, Box<Com>),
    /// whileループ `while b do c`
    While(Bexp, Box<Com>),
}

#[cfg(test)]
mod tests {
    use crate::{
        imp::{Aexp, Bexp},
        Evaluate, State,
    };

    #[test]
    fn equals() {
        // 3 ≡ 3
        assert_eq!(Aexp::N(3.into()), Aexp::N(3.into()));

        // ¬( 8 ≡ 3 + 5 )
        assert_ne!(
            Aexp::N(8.into()),
            Aexp::Add(Box::new(Aexp::N(3.into())), Box::new(Aexp::N(5.into())))
        );

        // ¬( 5 + 3 ≡ 3 + 5 )
        assert_ne!(
            Aexp::Add(Box::new(Aexp::N(5.into())), Box::new(Aexp::N(3.into()))),
            Aexp::Add(Box::new(Aexp::N(3.into())), Box::new(Aexp::N(5.into())))
        );
    }

    #[test]
    fn evaluate_number() {
        let state = State::init();

        // 〈2, σ〉 → 2
        assert_eq!(2, Aexp::N(2.into()).evaluate(&state));

        // 〈5, σ〉 → 5
        assert_eq!(5, Aexp::N(5.into()).evaluate(&state));
    }

    #[test]
    fn evaluate_variable() {
        // 〈Init, σ_0〉 → 0
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(0, Aexp::Loc("Init".into()).evaluate(&state));
    }

    #[test]
    fn evaluate_addition() {
        // 〈7 + 9, σ〉 → 16
        let state = State::init();
        assert_eq!(
            16,
            Aexp::Add(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(&state),
        );

        // 〈Init + 5, σ_0〉 → 5
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            5,
            Aexp::Add(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(&state),
        );

        // 〈(Init + 5) + (7 + 9), σ_0〉 → 21
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            21,
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
            -2,
            Aexp::Sub(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(&state),
        );

        // 〈Init - 5, σ_0〉 → -5
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            -5,
            Aexp::Sub(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(&state),
        );

        // 〈(Init - 5) - (7 - 9), σ_0〉 → -3
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            -3,
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
            63,
            Aexp::Mul(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(&state),
        );

        // 〈Init * 5, σ_0〉 → 0
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            0,
            Aexp::Mul(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(&state),
        );

        // 〈(Init * 5) * (7 * 9), σ_0〉 → 0
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            0,
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

    #[test]
    fn evaluate_truth() {
        let state = State::init();

        // 〈true, σ〉 → true
        assert_eq!(true, Bexp::T(true.into()).evaluate(&state),);

        // 〈false, σ〉 → false
        assert_eq!(false, Bexp::T(false.into()).evaluate(&state),);
    }

    #[test]
    fn evaluate_equality() {
        let state = State::init();

        // 〈0 = 0, σ〉 → true
        assert_eq!(
            true,
            Bexp::Eq(Aexp::N(0.into()), Aexp::N(0.into())).evaluate(&state),
        );

        // 〈0 = 1, σ〉 → false
        assert_eq!(
            false,
            Bexp::Eq(Aexp::N(0.into()), Aexp::N(1.into())).evaluate(&state),
        )
    }

    #[test]
    fn evaluate_less_than_or_equal() {
        let state = State::init();

        // 〈0 <= 0, σ〉 → true
        assert_eq!(
            true,
            Bexp::Le(Aexp::N(0.into()), Aexp::N(0.into())).evaluate(&state),
        );

        // 〈0 <= 1, σ〉 → true
        assert_eq!(
            true,
            Bexp::Le(Aexp::N(0.into()), Aexp::N(1.into())).evaluate(&state),
        );

        // 〈1 <= 0, σ〉 → false
        assert_eq!(
            false,
            Bexp::Le(Aexp::N(1.into()), Aexp::N(0.into())).evaluate(&state),
        );
    }

    #[test]
    fn evaluate_not() {
        let state = State::init();

        // 〈not true, σ〉 → false
        assert_eq!(
            false,
            Bexp::Not(Box::new(Bexp::T(true.into()))).evaluate(&state),
        );

        // 〈not false, σ〉 → true
        assert_eq!(
            true,
            Bexp::Not(Box::new(Bexp::T(false.into()))).evaluate(&state),
        );
    }

    #[test]
    fn evaluate_and() {
        let state = State::init();

        // 〈false and b_1, σ〉 → false
        assert_eq!(
            false,
            Bexp::And(
                Box::new(Bexp::T(false.into())),
                Box::new(Bexp::T(false.into()))
            )
            .evaluate(&state),
        );
        assert_eq!(
            false,
            Bexp::And(
                Box::new(Bexp::T(false.into())),
                Box::new(Bexp::T(true.into()))
            )
            .evaluate(&state),
        );
        // TODO: 右辺の Bexp が evaluate されないことをテストしたい

        // 〈true and false, σ〉 → false
        assert_eq!(
            false,
            Bexp::And(
                Box::new(Bexp::T(true.into())),
                Box::new(Bexp::T(false.into()))
            )
            .evaluate(&state),
        );

        // 〈true and true, σ〉 → true
        assert_eq!(
            true,
            Bexp::And(
                Box::new(Bexp::T(true.into())),
                Box::new(Bexp::T(true.into()))
            )
            .evaluate(&state),
        );
    }

    #[test]
    fn evaluate_or() {
        let state = State::init();

        // 〈true or b_1, σ〉 → true
        assert_eq!(
            true,
            Bexp::Or(
                Box::new(Bexp::T(true.into())),
                Box::new(Bexp::T(false.into()))
            )
            .evaluate(&state),
        );
        assert_eq!(
            true,
            Bexp::Or(
                Box::new(Bexp::T(true.into())),
                Box::new(Bexp::T(true.into()))
            )
            .evaluate(&state),
        );
        // TODO: 右辺の Bexp が evaluate されないことをテストしたい

        // 〈false or true, σ〉 → true
        assert_eq!(
            true,
            Bexp::Or(
                Box::new(Bexp::T(false.into())),
                Box::new(Bexp::T(true.into()))
            )
            .evaluate(&state),
        );

        // 〈false or false, σ〉 → false
        assert_eq!(
            false,
            Bexp::Or(
                Box::new(Bexp::T(false.into())),
                Box::new(Bexp::T(false.into()))
            )
            .evaluate(&state),
        );
    }
}
