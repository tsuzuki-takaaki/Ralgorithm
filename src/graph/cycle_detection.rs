// 有向グラフのサイクル検知
// ref: https://algo-method.com/tasks/971

fn main() {
  input! {
    n: usize,
    m: usize,
  }
  // トポロジカルソートの逆順を表す
  let mut order = vec![];

  // 出次数をもつ配列(出次数が0のnodeをシンクという)
  let mut deg = vec![0; n];

  let mut list = vec![vec![]; n];
  for _ in 0..m {
    input! {
      a: usize,
      b: usize,
    }
    list[b].push(a); // 終点nodeの隣接リストに始点nodeを追加
    deg[a] += 1; // 出次数をインクリメント
  }
  for i in 0..n { list[i].sort() };

  let mut queue = VecDeque::new();
  for j in 0..n {
    if deg[j] == 0 { queue.push_back(j) };
  }

  while let Some(v) = queue.pop_front() {
    order.push(v);
    for &nd in list[v].iter() {
      deg[nd] -= 1;
      if deg[nd] == 0 {
        queue.push_back(nd)
      }
    }
  }
  println!("{}", if order.len() == n { "No" } else { "Yes" });
}
