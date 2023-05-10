pub mod imp;

/// 整数
#[derive(Debug, PartialEq)]
pub struct Number(i32);

/// 真偽値
#[derive(Debug, PartialEq)]
pub struct Truth(bool);

/// プログラム変数
#[derive(Debug, PartialEq)]
pub struct VarName(String);
