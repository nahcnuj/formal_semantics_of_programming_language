use std::{collections::HashMap, fmt};

pub mod imp;

/// 整数
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(i32);

/// 真偽値
#[derive(Debug, PartialEq)]
pub struct Truth(bool);

/// プログラム変数
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VarName(String);

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
            vars.insert(VarName(def.0.to_string()), Some(def.1));
        }
        State(vars)
    }

    fn get(&self, var: &VarName) -> &Option<Number> {
        self.0.get(var).unwrap_or(&None)
    }
}
