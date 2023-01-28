// めぐる式二分探索でlower_bound
// ref: https://qiita.com/drken/items/97e37dd6143e33a64c8c
// example: https://atcoder.jp/contests/abc130/tasks/abc130_d

#![allow(unused_imports)]
use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{cmp::*, usize};
use std::f64::consts::PI;
use superslice::Ext;

fn lower_bound(arr: &Vec<usize>, key: usize) -> usize {
  let mut left: isize = -1;
  let mut right = arr.len();

  while right as isize - left > 1 {
    let mid = (left + (right as isize - left) / 2) as usize;
    if arr[mid] >= key {
        right = mid;
    } else {
        left = mid as isize;
    }
  }
  right
}

fn main() {
  let a = vec![1, 2, 3, 3, 4, 4, 4, 4, 4, 6, 6];
  println!("{}", lower_bound(&a, 3));
}
