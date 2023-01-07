// 深さ優先探索(再帰)
#![allow(unused_imports)]
use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{cmp::*, vec};
use std::f64::consts::PI;

fn dfs(nd: usize, list: &Vec<Vec<usize>>, vis: &mut Vec<bool>) {
  vis[nd] = true;
  for &v in list[nd].iter() {
    if !vis[v] {
      dfs(v, list, vis);
    }
  }
}

fn main() {
  let n = 5;
  let list = vec![
    vec![3],
    vec![0, 2 ,4],
    vec![1],
    vec![0, 4],
    vec![3]
  ];
  let mut vis = vec![false; n];
  dfs(0, &list, &mut vis);
  println!("{:?}", vis);
}


// ref: https://atcoder.jp/contests/abc277/submissions/37784742