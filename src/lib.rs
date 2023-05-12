use std::{collections::HashMap, fmt};

pub mod imp;

/// 整数
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(i32);

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
#[derive(Debug, PartialEq)]
pub struct Truth(bool);

impl From<bool> for Truth {
    fn from(b: bool) -> Self {
        Truth(b)
    }
}

/// プログラム変数
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VarName(String);

impl From<&str> for VarName {
    fn from(name: &str) -> Self {
        VarName(name.to_string())
    }
}

impl fmt::Display for VarName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 状態
pub struct State(HashMap<VarName, Option<Number>>);

impl State {
    pub fn init() -> State {
        State(HashMap::new())
    }

    pub fn from(defs: &[(&str, Number)]) -> State {
        let mut vars = HashMap::new();
        for def in defs {
            vars.insert(VarName::from(def.0), Some(def.1));
        }
        State(vars)
    }

    fn get(&self, var: &VarName) -> &Option<Number> {
        self.0.get(var).unwrap_or(&None)
    }
}

pub trait Evaluate<T> {
    /// 与えられた状態のもとで自身を評価する
    fn evaluate(&self, state: &State) -> T;
}
