// 長さnの配列が与えられた時の累積和

use proconio::{input,marker::*};

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut cum = vec![0; n+1];

  for i in 0..n {
    cum[i+1] = cum[i] + a[i];
  }

  println!("{:?}", cum);
}
