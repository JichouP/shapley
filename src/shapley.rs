use itertools::Itertools;

use crate::{CharacteristicFunction, ShapleyError, ShapleyResult};

/// シャープレイ値を計算する
pub fn calculate_shapley_values(
    players: &[&str],
    characteristic_function: CharacteristicFunction,
) -> Result<Vec<ShapleyResult>, ShapleyError> {
    if players.is_empty() {
        return Err(ShapleyError::EmptyPlayers);
    }

    let n = players.len();
    let factorial = (1..=n).product::<usize>();
    let mut results = Vec::with_capacity(n);
    let permutations = players.iter().permutations(n);

    for &player in players.iter() {
        let mut total_contribution = 0.0;

        // 全ての順列を生成
        for p in permutations.clone() {
            // プレイヤーiの前のプレイヤーを取得
            let before_i = p
                .iter()
                .take_while(|&&x| x != &player)
                .map(|&&x| x)
                .collect::<Vec<_>>();

            // プレイヤーiを含む集合を作成
            let mut with_i = before_i.clone();
            with_i.push(player);

            // 貢献値を計算
            let contribution = match (
                characteristic_function(&with_i),
                characteristic_function(&before_i),
            ) {
                (Ok(with_i_value), Ok(before_i_value)) => (with_i_value - before_i_value) as f64,
                (Err(e), _) | (_, Err(e)) => {
                    return Err(ShapleyError::CharacteristicFunctionError(e));
                }
            };

            total_contribution += contribution;
        }

        results.push(ShapleyResult {
            player: player.to_string(),
            value: total_contribution / factorial as f64,
        });
    }

    Ok(results)
}
