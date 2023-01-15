// bit全探索
// ref: https://www.ktpc.tokyo/le/k_c_pej_p_gm/bitSearch/bitSearch.html, https://drken1215.hatenablog.com/entry/2019/12/14/171657

// 32bit超えると死ぬ場合は1_u64 << nとかにする必要あり

#![allow(unused_imports)]
use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
use std::{cmp::*, usize, isize, vec};
use std::f64::consts::PI;
use superslice::Ext;

// n桁のbit全探索(bがbool値で列挙したもの)
fn main() {
    input! {
        n: usize,
    }
    for i in 0..(1 << n) {
        let mut b = vec![false; n];
        for j in 0..n {
            if (i >> j) & 1 == 1 { b[j] = true; }
        }
        println!("{}", b.iter().join(" "))
    }
}

// ref: https://atcoder.jp/contests/abc128/tasks/abc128_c
fn main() {
  input! {
      n: usize,
      m: usize,
  }
  let mut list = vec![];
  for _ in 0..m {
      input! {
          k: usize,
          s: [usize; k],
      }
      list.push(s);
  }
  input! {
      p: [usize; m],
  }
  let mut ans = 0;

  // bit全探索(n桁)
  for i in 0..(1 << n) {
      // bitが立っている場合true
      let mut b = vec![false; n];

      for j in 0..n {
          if (i >> j) & 1 == 1 { b[j] = true; }
          // ref: https://atcoder.jp/contests/abc128/submissions/5638020
      }

      let mut cnt = 0;
      for k in 0..m {
          let mut cnt_i = 0;
          for ll in list[k].iter() {
              if b[ll-1] { cnt_i += 1 };
          }
          if cnt_i % 2 == p[k] { cnt += 1 };
      }
      if cnt == m { ans += 1;};
  }
  println!("{}", ans);
}
