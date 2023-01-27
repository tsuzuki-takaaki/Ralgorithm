// 尺取法
// ref: https://atcoder.jp/contests/abc130/tasks/abc130_d, https://onlinejudge.u-aizu.ac.jp/problems/DSL_3_C

// 長さnの数列aで、al + al+1 + ... + ar-1 + ar <= tを満たす整数(l, r)の組み合わせ
fn main() {
  input! {
    n: usize,
    t: usize,
    a: [usize; n],
  }

  let mut sum = 0;
  let mut r = 0;
  let mut cnt = 0;

  for l in 0..n {
    while r < n && sum + a[r] <= t {
      sum += a[r];
      r += 1;
    }
    cnt += r - l;
    if r == l { r += 1 } else { sum -= a[l] }
  }
  println!("{}", cnt);
}
