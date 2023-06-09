//! プログラミング言語 IMP
//!
//! ```text
//! Aexp ::= Number | VarName | Aexp "+" Aexp | Aexp "-" Aexp | Aexp "*" Aexp
//! Bexp ::= Truth | Aexp "=" Aexp | Aexp "<=" Aexp | "not" Bexp | Bexp "and" Bexp | Bexp "or" Bexp
//! Com  ::= "skip" | VarName ":=" Aexp | Com ";" Com | "if" Bexp "then" Com "else" Com | "while" Bexp "do" Com
//! ```

use crate::{Evaluate, Execute, Number, State, Truth, VarName};

/// プログラミング言語 IMP の抽象構文木 (Abstract Syntax Tree)
#[derive(Debug, PartialEq)]
pub struct AST(Com);

/// 算術式
#[derive(Debug, Clone, PartialEq)]
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
    fn evaluate(&self, state: State) -> (Number, State) {
        match &self {
            Aexp::N(n) => (n.to_owned(), state),
            Aexp::Loc(var) => (
                state
                    .get(var)
                    .as_ref()
                    .expect(format!("variable {} is undefined", var).as_str())
                    .to_owned(),
                state,
            ),
            Aexp::Add(left, right) => {
                let (left, state) = left.evaluate(state);
                let (right, state) = right.evaluate(state);
                (left + right, state)
            }
            Aexp::Sub(left, right) => {
                let (left, state) = left.evaluate(state);
                let (right, state) = right.evaluate(state);
                (left - right, state)
            }
            Aexp::Mul(left, right) => {
                let (left, state) = left.evaluate(state);
                let (right, state) = right.evaluate(state);
                (left * right, state)
            }
        }
    }
}

/// ブール式
#[derive(Debug, Clone, PartialEq)]
pub struct Bexp {
    bexp: BexpImpl,
}

impl Bexp {
    /// 真偽値 `true`, `false`
    #[inline]
    pub fn truth(b: bool) -> Bexp {
        Bexp {
            bexp: BexpImpl::T(b.into()),
        }
    }

    /// 等値比較 `a_0 = a_1`
    #[inline]
    pub fn eq(left: Aexp, right: Aexp) -> Bexp {
        Bexp {
            bexp: BexpImpl::Eq(left, right),
        }
    }

    /// より小さいか等しい `a_0 <= a_1`
    #[inline]
    pub fn le(left: Aexp, right: Aexp) -> Bexp {
        Bexp {
            bexp: BexpImpl::Le(left, right),
        }
    }

    /// 否定 `not b`
    #[inline]
    pub fn not(expr: Bexp) -> Bexp {
        Bexp {
            bexp: BexpImpl::Not(Box::new(expr.bexp)),
        }
    }

    /// 論理積 `b_0 and b_1`
    #[inline]
    pub fn and(left: Bexp, right: Bexp) -> Bexp {
        Bexp {
            bexp: BexpImpl::And(Box::new(left.bexp), Box::new(right.bexp)),
        }
    }

    /// 論理和 `b_0 or b_1`
    #[inline]
    pub fn or(left: Bexp, right: Bexp) -> Bexp {
        Bexp {
            bexp: BexpImpl::Or(Box::new(left.bexp), Box::new(right.bexp)),
        }
    }
}

impl Evaluate<Truth> for Bexp {
    fn evaluate(&self, state: State) -> (Truth, State) {
        self.bexp.evaluate(state)
    }
}

/// ブール式
#[derive(Debug, Clone, PartialEq)]
enum BexpImpl {
    /// 真偽値 `true`, `false`
    T(Truth),
    /// 等値比較 `a_0 = a_1`
    Eq(Aexp, Aexp),
    /// より小さいか等しい `a_0 <= a_1`
    Le(Aexp, Aexp),
    /// 否定 `not b`
    Not(Box<BexpImpl>),
    /// 論理積 `b_0 and b_1`
    And(Box<BexpImpl>, Box<BexpImpl>),
    /// 論理和 `b_0 or b_1`
    Or(Box<BexpImpl>, Box<BexpImpl>),

    /// 短絡評価のテスト用（evaluate すると panic する）
    #[allow(dead_code)]
    Dummy,
}

impl Evaluate<Truth> for BexpImpl {
    fn evaluate(&self, state: State) -> (Truth, State) {
        match &self {
            BexpImpl::T(Truth(true)) => (Truth(true), state),
            BexpImpl::T(Truth(false)) => (Truth(false), state),
            BexpImpl::Eq(left, right) => {
                let (left, state) = left.evaluate(state); // TODO: state が変わらないことは Aexp::evaluate の事後条件
                let (right, state) = right.evaluate(state); // TODO: state が変わらないことは Aexp::evaluate の事後条件
                (Truth(left == right), state)
            }
            BexpImpl::Le(left, right) => {
                let (left, state) = left.evaluate(state); // TODO: state が変わらないことは Aexp::evaluate の事後条件
                let (right, state) = right.evaluate(state); // TODO: state が変わらないことは Aexp::evaluate の事後条件
                (Truth(left <= right), state)
            }
            BexpImpl::Not(b) => {
                let (b, state) = b.evaluate(state);
                (!b, state)
            }
            BexpImpl::And(left, right) => {
                let (left, state) = left.evaluate(state);
                if !<Truth as Into<bool>>::into(left) {
                    (Truth(false), state)
                } else {
                    right.evaluate(state)
                }
            }
            BexpImpl::Or(left, right) => {
                let (left, state) = left.evaluate(state);
                if <Truth as Into<bool>>::into(left) {
                    (Truth(true), state)
                } else {
                    right.evaluate(state)
                }
            }
            _ => panic!(), // 短絡評価のテスト用
        }
    }
}

/// コマンド
#[derive(Debug, Clone, PartialEq)]
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

impl Execute for Com {
    fn execute(&self, state: State) -> (Option<Self>, State) {
        let boxed_self = Box::new(self.clone());

        let mut cmd = self.clone();
        let mut state = state;
        loop {
            let (rest, new_state) = match &cmd {
                Com::Skip => (None, state),
                Com::Subst(var, a) => {
                    let (a, state) = a.evaluate(state);
                    (None, state.update_variable(&var, a))
                }
                Com::Seq(c_0, c_1) => {
                    let (None, state) = c_0.execute(state) else { panic!() };
                    (Some(c_1), state)
                }
                Com::If(b, c_0, c_1) => {
                    let (b, state) = b.evaluate(state);
                    let c = if b.into() { c_0 } else { c_1 };
                    (Some(c), state)
                }
                Com::While(b, c) => {
                    // ⟨b, σ⟩ → ⟨t, σ⟩
                    let (Truth(t), state) = b.evaluate(state);

                    if t {
                        // ⟨b, σ⟩ → ⟨true, σ⟩  ⟨c, σ⟩ → ⟨(), σ''⟩  ⟨while b do c, σ''⟩ → ⟨(), σ'⟩
                        // ----------------------------------------------------------------------
                        //                      ⟨while b do c, σ⟩ → ⟨(), σ'⟩

                        let (None, state) = c.execute(state) else { panic!() };
                        (Some(&boxed_self), state)
                    } else {
                        //     ⟨b, σ⟩ → ⟨false, σ⟩
                        // ---------------------------
                        // ⟨while b do c, σ⟩ → ⟨(), σ⟩

                        (None, state)
                    }
                }
            };
            state = new_state;

            if let Some(rest) = rest {
                cmd = rest.as_ref().clone();
            } else {
                break;
            }
        }
        (None, state)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        imp::{Aexp, Bexp, BexpImpl, Com},
        Evaluate, Execute, Number, State, Truth,
    };

    #[test]
    fn equals() {
        // 3 ≡ 3
        assert_eq!(Aexp::N(3.into()), Aexp::N(3.into()));

        // ¬( 8 ≡ 3 + 5 )
        assert_ne!(
            Aexp::N(8.into()),
            Aexp::Add(Box::new(Aexp::N(3.into())), Box::new(Aexp::N(5.into()))),
        );

        // ¬( 5 + 3 ≡ 3 + 5 )
        assert_ne!(
            Aexp::Add(Box::new(Aexp::N(5.into())), Box::new(Aexp::N(3.into()))),
            Aexp::Add(Box::new(Aexp::N(3.into())), Box::new(Aexp::N(5.into()))),
        );
    }

    #[test]
    fn evaluate_number() {
        // ⟨2, σ₀⟩ → ⟨2, σ₀⟩
        let state = State::init();
        assert_eq!((Number(2), state.clone()), Aexp::N(2.into()).evaluate(state),);

        // ⟨5, σ₀⟩ → ⟨5, σ₀⟩
        let state = State::init();
        assert_eq!((Number(5), state.clone()), Aexp::N(5.into()).evaluate(state),);
    }

    #[test]
    fn evaluate_variable() {
        // σ := { (Init, 0) }
        // ⟨Init, σ⟩ → ⟨0, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(0), state.clone()),
            Aexp::Loc("Init".into()).evaluate(state),
        );
    }

    #[test]
    fn evaluate_addition() {
        // ⟨7 + 9, σ₀⟩ → ⟨16, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Number(16), state.clone()),
            Aexp::Add(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(state),
        );

        // σ := { (Init, 0) }
        // ⟨Init + 5, σ⟩ → ⟨5, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(5), state.clone()),
            Aexp::Add(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(state),
        );

        // σ := { (Init, 0) }
        // ⟨(Init + 5) + (7 + 9), σ⟩ → ⟨21, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(21), state.clone()),
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
            .evaluate(state),
        );
    }

    #[test]
    fn evaluate_subtraction() {
        // ⟨7 - 9, σ₀⟩ → ⟨-2, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Number(-2), state.clone()),
            Aexp::Sub(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(state),
        );

        // σ := { (Init, 0) }
        // ⟨Init - 5, σ⟩ → ⟨-5, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(-5), state.clone()),
            Aexp::Sub(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(state),
        );

        // σ := { (Init, 0) }
        // ⟨(Init - 5) - (7 - 9), σ⟩ → ⟨-3, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(-3), state.clone()),
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
            .evaluate(state),
        );
    }

    #[test]
    fn evaluate_multiplication() {
        // ⟨7 * 9, σ₀⟩ → ⟨63, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Number(63), state.clone()),
            Aexp::Mul(Box::new(Aexp::N(7.into())), Box::new(Aexp::N(9.into()))).evaluate(state),
        );

        // σ := { (Init, 0) }
        // ⟨Init * 5, σ⟩ → ⟨0, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(0), state.clone()),
            Aexp::Mul(
                Box::new(Aexp::Loc("Init".into())),
                Box::new(Aexp::N(5.into()))
            )
            .evaluate(state),
        );

        // σ := { (Init, 0) }
        // ⟨(Init * 5) * (7 * 9), σ⟩ → ⟨0, σ⟩
        let state = State::from(&[("Init", 0.into())]);
        assert_eq!(
            (Number(0), state.clone()),
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
            .evaluate(state),
        );
    }

    #[test]
    fn evaluate_truth() {
        // ⟨true, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::T(true.into()).evaluate(state),
        );

        // ⟨false, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::T(false.into()).evaluate(state),
        );
    }

    #[test]
    fn evaluate_equality() {
        // ⟨0 = 0, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::Eq(Aexp::N(0.into()), Aexp::N(0.into())).evaluate(state),
        );

        // ⟨0 = 1, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::Eq(Aexp::N(0.into()), Aexp::N(1.into())).evaluate(state),
        )
    }

    #[test]
    fn evaluate_less_than_or_equal() {
        // ⟨0 <= 0, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::Le(Aexp::N(0.into()), Aexp::N(0.into())).evaluate(state),
        );

        // ⟨0 <= 1, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::Le(Aexp::N(0.into()), Aexp::N(1.into())).evaluate(state),
        );

        // ⟨1 <= 0, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::Le(Aexp::N(1.into()), Aexp::N(0.into())).evaluate(state),
        );
    }

    #[test]
    fn evaluate_not() {
        // ⟨not true, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::Not(Box::new(BexpImpl::T(true.into()))).evaluate(state),
        );

        // ⟨not false, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::Not(Box::new(BexpImpl::T(false.into()))).evaluate(state),
        );
    }

    #[test]
    fn evaluate_and() {
        // ⟨false and b₁, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::And(
                Box::new(BexpImpl::T(false.into())),
                Box::new(BexpImpl::Dummy),
            )
            .evaluate(state),
        );

        // ⟨true and false, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::And(
                Box::new(BexpImpl::T(true.into())),
                Box::new(BexpImpl::T(false.into()))
            )
            .evaluate(state),
        );

        // ⟨true and true, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::And(
                Box::new(BexpImpl::T(true.into())),
                Box::new(BexpImpl::T(true.into()))
            )
            .evaluate(state),
        );
    }

    #[test]
    fn evaluate_or() {
        // ⟨true or b₁, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::Or(
                Box::new(BexpImpl::T(true.into())),
                Box::new(BexpImpl::Dummy),
            )
            .evaluate(state),
        );

        // ⟨false or true, σ₀⟩ → ⟨true, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(true), state.clone()),
            BexpImpl::Or(
                Box::new(BexpImpl::T(false.into())),
                Box::new(BexpImpl::T(true.into()))
            )
            .evaluate(state),
        );

        // ⟨false or false, σ₀⟩ → ⟨false, σ₀⟩
        let state = State::init();
        assert_eq!(
            (Truth(false), state.clone()),
            BexpImpl::Or(
                Box::new(BexpImpl::T(false.into())),
                Box::new(BexpImpl::T(false.into()))
            )
            .evaluate(state),
        );
    }

    #[test]
    fn execute_skip() {
        // ⟨skip, σ₀⟩ → ⟨(), σ₀⟩
        let before = State::init();
        let (None, after) = Com::Skip.execute(before.clone()) else { panic!() };
        assert_eq!(before, after);
    }

    #[test]
    fn execute_substitution() {
        // ⟨X := 5, σ₀⟩ →* ⟨(), σ₀[5/X]⟩
        let (None, state) = Com::Subst("X".into(), Aexp::N(5.into())).execute(State::init()) else { panic!() };
        assert_eq!(&Some(Number(5)), state.get(&"X".into()));
    }

    #[test]
    fn execute_sequence() {
        // ⟨X := 5 ; Y := 3, σ₀⟩ →* ⟨(), σ₀[5/X][3/Y]⟩
        let (None, state) = Com::Seq(
            Box::new(Com::Subst("X".into(), Aexp::N(5.into()))),
            Box::new(Com::Subst("Y".into(), Aexp::N(3.into()))),
        )
        .execute(State::init()) else { panic!() };
        assert_eq!(&Some(Number(5)), state.get(&"X".into()));
        assert_eq!(&Some(Number(3)), state.get(&"Y".into()));
    }

    #[test]
    fn execute_if_command() {
        // ⟨if true then X := 5 else X := 3, σ₀⟩ →* ⟨(), σ₀[5/X]⟩
        let (None, state) = Com::If(
            Bexp::truth(true),
            Box::new(Com::Subst("X".into(), Aexp::N(5.into()))),
            Box::new(Com::Subst("X".into(), Aexp::N(3.into()))),
        )
        .execute(State::init()) else { panic!() };
        assert_eq!(&Some(Number(5)), state.get(&"X".into()));

        // ⟨if false then X := 5 else X := 3, σ₀⟩ →* ⟨(), σ₀[3/X]⟩
        let (None, state) = Com::If(
            Bexp::truth(false),
            Box::new(Com::Subst("X".into(), Aexp::N(5.into()))),
            Box::new(Com::Subst("X".into(), Aexp::N(3.into()))),
        )
        .execute(State::init()) else { panic!() };
        assert_eq!(&Some(Number(3)), state.get(&"X".into()));
    }

    #[test]
    fn execute_while_loop() {
        // ⟨while false do X := 5, σ₀⟩ →* ⟨(), σ₀⟩
        let (None, state) = Com::While(
            Bexp::truth(false),
            Box::new(Com::Subst("X".into(), Aexp::N(5.into()))),
        )
        .execute(State::init()) else { panic!() };
        assert_eq!(&None, state.get(&"X".into()));

        // σ := { (X, 0) }
        // ⟨while X <= 3 do X := X + 1, σ⟩ →* ⟨(), σ[4/X]⟩
        let (None, state) = Com::While(
            Bexp::le(Aexp::Loc("X".into()), Aexp::N(3.into())),
            Box::new(Com::Subst(
                "X".into(),
                Aexp::Add(Box::new(Aexp::Loc("X".into())), Box::new(Aexp::N(1.into()))),
            )),
        )
        .execute(State::from(&[("X", 0.into())])) else { panic!() };
        assert_eq!(&Some(Number(4)), state.get(&"X".into()));
    }

    #[test]
    #[ignore = "it takes long time"]
    fn execute_long_while_loop() {
        // σ := { (X, 0) }
        // ⟨while X <= 999_999 do X := X + 1, σ⟩ →* ⟨(), σ[1_000_000/X]⟩
        let (None, state) = Com::While(
            Bexp::le(Aexp::Loc("X".into()), Aexp::N(999_999.into())),
            Box::new(Com::Subst(
                "X".into(),
                Aexp::Add(Box::new(Aexp::Loc("X".into())), Box::new(Aexp::N(1.into()))),
            )),
        )
        .execute(State::from(&[("X", 0.into())])) else { panic!() };
        assert_eq!(&Some(Number(1_000_000)), state.get(&"X".into()));
    }
}
