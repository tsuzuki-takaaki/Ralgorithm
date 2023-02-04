// 二部グラフ判定

fn bi_partite(nd: usize, list: &Vec<Vec<usize>>, col: &mut Vec<isize>) -> bool {
  for &nex in list[nd].iter() {
    if col[nex] == -1 {
      col[nex] = 1 - col[nd];
      if !bi_partite(nex, list, col) {
        return false;
      }
    } else if col[nex] == col[nd] {
      return false;
    }
  }
  true
}

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  let mut list = vec![vec![]; n];
  let mut col: Vec<isize> = vec![-1; n];

  for _ in 0..m {
    input! {
      a: usize,
      b: usize,
    }
    list[a-1].push(b-1);
    list[b-1].push(a-1);
  }

  for i in 0..n {
    if col[i] == -1 {
      col[i] = 0;
      if !bi_partite(i, &list, &mut col) {
        println!("No");
        return;
      }
    }
  }
  println!("Yes");
}
