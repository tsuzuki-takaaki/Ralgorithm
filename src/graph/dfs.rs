// 深さ優先探索(ex: 連結判定)
#![allow(unused_imports)]
use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::*;
use std::f64::consts::PI;

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
  let mut stack = vec![];

  vis[0] = true;
  stack.push(0);

  while let Some(x) = stack.pop() {
    for &nd in list[x].iter() {
      if !vis[nd] {
        vis[nd] = true;
        stack.push(nd);
      }
    }
  }
  
  println!("{:?}", vis);

}