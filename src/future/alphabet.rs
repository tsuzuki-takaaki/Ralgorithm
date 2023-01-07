// alphabet周回
// ref: https://atcoder.jp/contests/abc232/tasks/abc232_b

#![allow(unused_imports)]
use proconio::{input,marker::*};
use itertools::{Itertools, max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::*;
use std::f64::consts::PI;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let diff = (t[0] as i32- s[0] as i32 + 26) % 26;
    for i in 0..s.len() {
        let pre_t = (s[i] as i32 - 'a' as i32 + diff) % 26 + 'a' as i32;
        if pre_t != t[i] as i32 {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
