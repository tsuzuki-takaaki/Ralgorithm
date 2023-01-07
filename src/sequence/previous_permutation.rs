// previous_permutation(ライブラリを使わないver)
// ref: https://atcoder.jp/contests/abc276/tasks/abc276_c

#![allow(unused_imports)]
use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{cmp::*, usize};
use std::f64::consts::PI;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    }
    let mut ind: usize = 0;
    for i in 0..n-1 {
        if p[i] > p[i+1] { ind = i; }
    }
    let num = p[ind];
    let mut cnd = 0;
    let mut pos = 0;
    for j in ind+1..n {
        if p[j] < num && cnd < p[j] {
            cnd = p[j];
            pos = j;
        }
    }
    p.swap(ind, pos);
    p[ind+1..].sort();
    p[ind+1..].reverse();
    println!("{}", p.iter().join(" "));
}

// ライブラリを使うver
// ref: https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update#superslice
// #![allow(unused_imports)]
// use proconio::{input,marker::*};
// use itertools::{Itertools, max, min};
// use std::collections::{HashMap, HashSet, VecDeque};
// use std::{cmp::*, usize};
// use std::f64::consts::PI;
// use superslice::Ext;

// fn main() {
//     input! {
//         n: usize,
//         mut p: [usize; n],
//     }
//     p.prev_permutation();
//     println!("{}", p.iter().join(" "));
// }
