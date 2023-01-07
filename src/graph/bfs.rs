// 幅優先（ex: 連結判定)
// node0~5まで、隣接リストは既知

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  let n = 5;
  let list = vec![
    vec![1, 3],
    vec![0, 2 ,4],
    vec![1],
    vec![0, 4],
    vec![1, 3]
  ];

  let mut vis = vec![false; n];
  let mut queue = VecDeque::new();

  vis[0] = true;
  queue.push_back(0);

  while let Some(x) = queue.pop_front() {
    for &nd in list[x].iter() {
      if !vis[nd] {
        vis[nd] = true;
        queue.push_back(nd);
      }
    }
  }
  println!("{:?}", vis);

}
