// エラトステネスの篩 O(NloglogN) ほぼ O(N)
// ref: https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cy

// nまでの素数
fn main() {
  input! {
    n: usize,
  }
  // 素数の場合はtrue
  let mut arr = vec![true; n+1];
  arr[0] = false;
  arr[1] = false;

  let mut i = 2;

  while i * i <= n {
    if arr[i] {
      let mut c = 2;
      while i * c <= n {
        arr[i*c] = false;
        c += 1;
      }
    }
    i += 1;
  }
  
  for j in 0..n+1 {
    if arr[j] {
      println!("{}", j);
    }
  }
}
