//! # Shapley
//! シャープレイ値の計算を行うライブラリ。
//!
//! ## 使用例
//!
//! <https://github.com/JichouP/shapley/blob/main/src/lib.rs> のテスト参照。

mod shapley;
mod types;

pub use shapley::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_shapley_calculation() {
        // プレイヤーを定義
        let players = ["A", "B", "C"];

        // 特性関数を定義
        let characteristic_function: CharacteristicFunction = |coalition| {
            // ソートする
            let mut coalition = coalition.to_vec();
            coalition.sort();
            let coalition = &coalition[..];

            match coalition {
                [] => Ok(0),
                ["A"] => Ok(10),
                ["B"] => Ok(20),
                ["C"] => Ok(30),
                ["A", "B"] => Ok(40),
                ["A", "C"] => Ok(50),
                ["B", "C"] => Ok(60),
                ["A", "B", "C"] => Ok(100),
                _ => Ok(0),
            }
        };

        // シャープレイ値を計算
        let results = calculate_shapley_values(&players, characteristic_function).unwrap();

        // 結果を検証
        assert_eq!(results.len(), 3);

        // 各プレイヤーのシャープレイ値を検証
        for result in results {
            match result.player.as_str() {
                "A" => assert!((result.value - 140.0 / 6.0).abs() < 1e-10),
                "B" => assert!((result.value - 200.0 / 6.0).abs() < 1e-10),
                "C" => assert!((result.value - 260.0 / 6.0).abs() < 1e-10),
                _ => panic!("予期しないプレイヤー: {}", result.player),
            }
        }
    }

    #[test]
    fn test_empty_players() {
        let players: &[&str] = &[];
        let characteristic_function: CharacteristicFunction = |_| Ok(0);

        let result = calculate_shapley_values(players, characteristic_function);
        assert!(matches!(result, Err(ShapleyError::EmptyPlayers)));
    }

    #[test]
    fn test_characteristic_function_error() {
        let players = ["A", "B"];
        let characteristic_function: CharacteristicFunction = |_| Err("テストエラー".to_string());

        let result = calculate_shapley_values(&players, characteristic_function);
        assert!(matches!(
            result,
            Err(ShapleyError::CharacteristicFunctionError(_))
        ));
    }
}
