use std::{collections::HashMap, fmt};

/// 整数
/// ```text
/// Number ::= 整数
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Number(i32);

impl std::ops::Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl std::ops::Sub for Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl std::ops::Mul for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.0).into()
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Number> for i32 {
    fn eq(&self, other: &Number) -> bool {
        *self == other.0
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number(n)
    }
}

/// 真偽値
/// ```text
/// Truth ::= "true" | "false"
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Truth(bool);

impl std::ops::Not for Truth {
    type Output = Self;
    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}

impl PartialEq<bool> for Truth {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Truth> for bool {
    fn eq(&self, other: &Truth) -> bool {
        *self == other.0
    }
}

impl From<bool> for Truth {
    fn from(b: bool) -> Self {
        Truth(b)
    }
}

impl From<Truth> for bool {
    fn from(value: Truth) -> Self {
        value.0
    }
}

/// 変数
/// ```text
/// VarName ::= 変数（X,Y,Z,...）
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarName(String);

impl From<&str> for VarName {
    fn from(name: &str) -> Self {
        VarName(name.to_string())
    }
}

impl From<String> for VarName {
    fn from(name: String) -> Self {
        VarName(name)
    }
}

impl fmt::Display for VarName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 状態
#[derive(Debug, Clone, PartialEq)]
pub struct State(HashMap<VarName, Option<Number>>);

impl State {
    /// 初期状態を生成します。
    pub fn init() -> State {
        State(HashMap::new())
    }

    /// 変数名と値の組のスライスから状態を生成します。
    pub fn from(defs: &[(&str, Number)]) -> State {
        let mut vars = HashMap::new();
        for def in defs {
            vars.insert(VarName::from(def.0), Some(def.1));
        }
        State(vars)
    }

    /// この状態での変数 `var` の値を返します。
    fn get(&self, var: &VarName) -> &Option<Number> {
        self.0.get(var).unwrap_or(&None)
    }

    /// 変数 var の値を value に置き換えた状態を生成します。
    fn update_variable(mut self, var: &VarName, value: Number) -> Self {
        let vars = &mut self.0;
        if let Some(v) = vars.get_mut(&var) {
            *v = Some(value);
        } else {
            vars.insert(var.to_owned(), Some(value));
        }
        self
    }
}

pub trait Evaluate<T> {
    /// 与えられた状態のもとで自身を評価します。
    /// 評価結果と評価後の状態の組を返します。
    fn evaluate(&self, state: State) -> (T, State);
}

pub trait Execute {
    /// 与えられた状態のもとで自身を実行します。
    /// 未実行のコマンドと実行後の状態の組を返します。
    fn execute(&self, state: State) -> (Option<Self>, State)
    where
        Self: Sized;
}

pub mod imp;
