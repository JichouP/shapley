# シャープレイ値

Wikipedia: [English](https://en.wikipedia.org/wiki/Shapley_value), [Japanese](https://ja.wikipedia.org/wiki/シャープレイ値)

## 定義

### 特性関数

特性関数 $v$ を以下のような性質をもつ関数とする。
ここで、$S, T \subseteq N$ かつ $S \cap T = \emptyset$ である。

1. $v(\emptyset) = 0$
2. $v(S \cup T) \geq v(S) + v(T)$

### シャープレイ値

プレイヤーの集合を $N = \{1, 2, \ldots, n\}$ とし、提携 (coalition) $S \subseteq N$ に対して、特性関数 $v: 2^N \to \mathbb{R}$ が定義されているとする。このとき、プレイヤー $i \in N$ のシャープレイ値 $\phi_i(v)$ は以下のように定義される。

$$
\phi_i(v) = \sum_{S \subseteq N \setminus \{i\}} \frac{|S|!(|N|-|S|-1)!}{|N|!} (v(S \cup \{i\}) - v(S))
$$

式の意味は以下の通り。

- $S \subseteq N \setminus \{i\}$: プレイヤー $i$ を含まない全ての提携
- $|S|$: 提携 $S$ に含まれるプレイヤーの数
- $\displaystyle\frac{|S|!(|N|-|S|-1)!}{|N|!}$: 重み付け係数。プレイヤーの全ての可能な順列における提携 $S$ の出現確率
- $v(S \cup \{i\}) - v(S)$: プレイヤー $i$ が提携 $S$ に参加することによる限界貢献 (marginal contribution)

### シャープレイベクトル

シャープレイベクトル (Shapley vector) は、全プレイヤーのシャープレイ値を要素とするベクトルである。

$$
\phi(v) = (\phi_1(v), \phi_2(v), \ldots, \phi_n(v))
$$

## 性質・公理

シャープレイ値は以下の性質を満たす。

1. 個人合理性: 全てのプレイヤーに1人で提携を作る時の利得 $v(\{i\})$ 以上の利得を与える。すなわち 全てのプレイヤー $i \in N$ に対して、以下が成り立つ。

$$
\phi_i(v) \geq v(\{i\})
$$

2. 全体合理性: $N$ 全体の提携値は各プレイヤーの利得の総和に等しい。

$$
\sum_{i \in N} \phi_i(v) = v(N)
$$

3. 対称性: プレイヤー $i$ と $j$ 同様の意味を持つ、つまり、$v(S \cup \{i\}) = v(S \cup \{j\})$ が $N$ の $i$ も $j$ も含まない全ての部分集合 $S$ について成り立つ場合、以下が成り立つ。

$$
\phi_i(v) = \phi_j(v)
$$

4. 加法性: 提携値 $v$ が2つの特性関数 $v_1$ と $v_2$ の和で表せる場合、以下が成り立つ。

$$
\phi_i(v) = \phi_i(v_1 + v_2) = \phi_i(v_1) + \phi_i(v_2)
$$

5. ナルプレイヤーに関する性質: プレイヤー $i$ がナルプレイヤーである場合、つまり、$v(S \cup \{i\}) = v(S)$ が $S \subseteq N \setminus \{i\}$ の全ての部分集合 $S$ について成り立つ場合、以下が成り立つ。

$$
\phi_i(v) = 0
$$

シャープレイベクトルは、上記 2, 3, 4, 5 の性質を満たす唯一のベクトルである。
