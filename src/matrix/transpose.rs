// 転置行列(h, wの入れ替え) ref: https://atcoder.jp/contests/abc279/tasks/abc279_c
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    let mut tr_s = vec![];
    let mut tr_t = vec![];

    for j in 0..w {
        let mut v_s = vec![];
        let mut v_t = vec![];
        for i in 0..h {
            v_s.push(s[i][j]);
            v_t.push(t[i][j]);
        }
        tr_s.push(v_s);
        tr_t.push(v_t);
    }
    tr_s.sort();
    tr_t.sort();
    println!("{}", if tr_s == tr_t {"Yes"} else {"No"});
}

// 
fn main() {
  input! {
      h: usize,
      w: usize,
      s: [Chars; h],
      t: [Chars; h],
  }
                                        //[char; h]で宣言するとエラー
  let mut tr_s: Vec<Vec<char>> = vec![vec!['.'; h]; w];
  let mut tr_t: Vec<Vec<char>> = vec![vec!['.'; h]; w];

  for i in 0..h {
      for j in 0..w {
          tr_s[j][i] = s[i][j];
          tr_t[j][i] = t[i][j];
      }
  }
  println!("{:?}", tr_s);
  println!("{:?}", tr_t);
}

