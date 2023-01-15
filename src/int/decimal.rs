// n進法変換
// ref: https://atcoder.jp/contests/abc220/editorial/2681

// n進法 => 10進法
// k: n進法, num: n進法の数値 -> 10進法の数値
fn to_decimal(k: usize, mut num: usize) -> usize {
  let mut ans = 0;
  let mut d = 1;
  while num > 0 {
    ans += num % 10 * d;
    d *= k;
    num /= 10;
  }
  ans
}

// 10進数 => 2進数
// => format!("{:b}", 3)で変換できる -> String