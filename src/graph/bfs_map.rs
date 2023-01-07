// 隣接リストをmapで持つときのbfs
// ref: https://atcoder.jp/contests/abc277/tasks/abc277_c

use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
      n: usize,
  }

  let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

  for _ in 0..n {
      input! {
          a: usize,
          b: usize,
      }
      let cnd_a = map.entry(a).or_insert(vec![]);
      cnd_a.push(b);
      let cnd_b = map.entry(b).or_insert(vec![]);
      cnd_b.push(a);
  }

  let mut vis: HashSet<usize> = HashSet::new();
  let mut queue = VecDeque::new();

  vis.insert(1);
  queue.push_back(1);

  while let Some(x) = queue.pop_front() {
      if let Some(y) = map.get(&x) {
          for &v in y.iter() {
              if !vis.contains(&v) {
                  vis.insert(v);
                  queue.push_back(v);
              }
          }
      }
  }

  println!("{:?}", vis.iter().max().unwrap());
}
