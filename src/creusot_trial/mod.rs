use creusot_contracts::*;

// 関数の事後条件を"ensures"アトリビュートで記述する
// 返り値はresultで表される
// ここでは、関数の返り値が1になるという仕様を記述している。
#[ensures(result == 1i32)]
// ^記号は参照の終了時の値を表す。ここでは、関数から戻る時点でaが0であるという仕様を記述している
#[ensures(^a == 0i32)]
// aの場合と同様、関数から戻る辞典でbが0であるという仕様を記述している
#[ensures(^b == 1i32)]
pub fn hello(a : &mut i32, b : &mut i32) -> i32 {
    *a = 0;
    *b = 1;
    *a + *b
}