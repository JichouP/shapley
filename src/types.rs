/// 特性関数の型
pub type CharacteristicFunction = fn(&[&str]) -> Result<i64, String>;

/// シャープレイ値の結果を表す構造体
#[derive(Debug, Clone)]
pub struct ShapleyResult {
    /// プレイヤー名
    pub player: String,
    /// シャープレイ値
    pub value: f64,
}

/// シャープレイ値の計算エラー
#[derive(Debug, thiserror::Error)]
pub enum ShapleyError {
    #[error("プレイヤーリストが空です")]
    EmptyPlayers,
    #[error("特性関数の計算中にエラーが発生しました: {0}")]
    CharacteristicFunctionError(String),
}
