本题需要找规律优化算法。

首先我们知道末尾有 0 是因为这个数是 10 的倍数(50 是 10 的倍数，500 是 100 的倍数，100 可以分解为 `10 * 10`)。也就是说在阶乘中有数相乘的乘积是 10，那么每多一个 10 结尾就会多一个 0。

另外可以发现在 `9!` 中唯一能产生 10 的乘积的就是 `2 * 5` 。而在 `9!` 中许多数都有 2 这个因子，也就是说在一段阶乘区间内，10 的乘积个数取决于能分解出 5 的个数。比如 `15!` 中数字 5、10、15 这 3 个数都都可以分解成 `n * 5` ，也就是说对于每一个数都能找到一个 2 与之相乘，并且乘积结果为 `n * 5 * 2` 。因此 `15!` 阶乘末尾 0 的个数为 3 个。

需要注意的是某些数可以产生多个 5 ，比如 25 有 2 个 5 ，50 也有 2 个 5 。下面我们通过表格来观察一下规律。

| 5    | 10   | 15   | 20   | 25   |
| ---- | ---- | ---- | ---- | ---- |
| 30   | 35   | 40   | 45   | 50   |
| 55   | 60   | 65   | 70   | 75   |
| 80   | 85   | 90   | 95   | 100  |
| 105  | 110  | 115  | 120  | 125  |
| 130  | 135  | 140  | 145  | 150  |

可以观察到对于前 4 列来说，都只能分解出 1 个 5，但是第 5 列却能分解出多个 5。

| 25   | 5 * 5 * 1 |
| ---- | --------- |
| 50   | 5 * 5 * 2 |
| 75   | 5 * 5 * 3 |
| 100  | 5 * 5 * 4 |
| 125  | 5 * 5 * 5 |
| 150  | 5 * 5 * 6 |

也就是说 `20!` 中值为 5 的倍数的个数为 `20 / 5` ，但是 `25!` 中值为 5 的倍数的个数为 `25 / 5` 再加上 25 多的那个 5。本题规律的关键在于第 5 列，对于 n 的阶乘来说有几个数能列入上面表格中只需要 `n / 5` 即可。但是第 5 列是多了 n 个 5 的。可以发现 25、50、75、100、125、150 这些数被计数过 1 次以后，就可以将这个数中的 5 去了。那么结果将变为如下表格

| 25   | 5 * 5 * 1 | 5 * 1 |
| ---- | --------- | ----- |
| 50   | 5 * 5 * 2 | 5 * 2 |
| 75   | 5 * 5 * 3 | 5 * 3 |
| 100  | 5 * 5 * 4 | 5 * 4 |
| 125  | 5 * 5 * 5 | 5 * 5 |
| 150  | 5 * 5 * 6 | 5 * 6 |

可以发现这刚好又变成了最初表格的 5、10、15、20、25、30。我们只要一直计算 `n / 5` 、`n / 5 / 5` 、`n / 5 / 5 / 5` 直到其结果小于 5，并将结果累加即可。