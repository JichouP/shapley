# シャープレイ値の計算アルゴリズム

シャープレイ値の計算アルゴリズムは、以下のように行う。

## 方針

1. 全てのプレイヤーの順列を生成する。
2. 各順列について、提携値を計算する。
3. 各順列について、シャープレイ値を計算する。
4. シャープレイ値を平均する。

## 計算アルゴリズムの詳細

特性関数 $v$ は、以下のような型を持つ。

```rust
type V = impl Fn(&[&str]) -> i64;
```

$v$ の戻り値は、必ずしも正である必要はない。
例えば、かかった金額ベースで考える場合など、戻り値が負になることもある。
また、その人が負の貢献をした場合、その人の貢献値が負になることもある。

### 1. 全てのプレイヤーの順列 $R$ を作成する

$N$ の全てのプレイヤーの順列を作成する。
$|N|!$ 通りの順列 $R$ が作成される。
[`itertools::Itertools::permutations`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.permutations) を使うと作成できる。

```rust
use itertools::Itertools;

let permutations = n.iter().permutations(n.len());
```

### 2. プレイヤー $i$ が参加する事による貢献値 $c_i(P_i^R)$ を計算する

$P_i^R$ を $R$ でプレイヤー $i$ の前に並んでいるプレイヤーの集合とする。
プレイヤー $i$ が順列 $R$ に参加する事による貢献値 $c_i(R)$ は次のように表せる。

$$
c_i(R) = v(P_i^R \cup \{i\}) - v(P_i^R)
$$

実装例

```rust
let contributions = permutations.map(|r| {
  // i の直前までを取り出す
  let p = r.iter().take_while(|&x| *x != i).collect::<Vec<_>>();
  // i を含めた集合を作る
  let s = p.clone().push(i);

  v(s) - v(p)
});
```

### 3. シャープレイ値を計算する

プレイヤー $i$ のシャープレイ値は、プレイヤー $i$ が順列に参加する事による貢献値の平均値である。

$$
\phi_i(v) = \frac{1}{|N|!} \sum_{R}\left[v(P_i^R \cup \{i\}) - v(P_i^R)\right]
$$

これを全てのプレイヤーについて行い、シャープレイ値 $\phi_i(v)$ を全て計算する。

```rust
let shapley_values = contributions.sum::<i64>() / (n.len() as i64);
```
