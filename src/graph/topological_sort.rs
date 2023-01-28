// トポロジカルソート
// トポロジカルソートができる <=> 有向サイクルを含まない(DAG: Directed Acynclic Graph)

fn main() {
  input! {
    n: usize,
    m: usize,
  }
  let mut ans = vec![];
  // 各nodeの出次数を持つ配列
  let mut deg = vec![0; n];
  // 各node、自分に向かってくるnodeを記録
  let mut list = vec![vec![]; n];

  for _ in 0..m {
    // a -> b
    input! {
      a: usize,
      b: usize,
    }
    list[b].push(a);
    deg[a] += 1;
  }
  for i in 0..n { list[i].sort() }

  let mut queue = VecDeque::new();
  for j in 0..n {
    // 自分からは一つも有向辺が張っていない場合
    if deg[j] == 0 {
      queue.push_back(j);
    }
  }
  while let Some(x) = queue.pop_front() {
    ans.push(x);
    for &nex in list[x].iter() {
      deg[nex] -= 1;
      if deg[nex] == 0 { queue.push_back(nex) }
    }
  }
  ans.reverse();
  println!("{}", ans.iter().join(" "));
}
