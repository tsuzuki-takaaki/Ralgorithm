// 隣接リスト ref: https://atcoder.jp/contests/abc262/tasks/abc262_b
// 固定長の配列をnodeごとに持たせてbool値で管理する

// n頂点m辺の単純無向グラフ

fn main() {
  input! {
    n: usize,
    m: usize,
  }
  let mut g = vec![vec![false; n]; n];
  for i in 0..m {
    input!{
      u: usize,
      v: usize,
    }
    g[u-1][v-1] = true;
    g[v-1][u-1] = true;
  }
  println!("{:?}", g);
}