// 正方形のベクター
// indexはusizeだけで、isizeは許されない => castする
// ref: https://users.rust-lang.org/t/why-indexing-by-isize-is-disallowed/69775, https://atcoder.jp/contests/abc275/tasks/abc275_c

// 与えられた2点と正方形をなす２点を求める

fn main() {
    input! {
      x1: isize,
      y1: isize,
      x2: isize,
      y2: isize,
    }
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    let x3 = x2 - dy;
    let y3 = y2 + dx;
  
    let x4 = x3 - dx;
    let y4 = y3 - dy;
  
    println!("{}, {}, {}, {}", x3, y3, x4, y4);
  }
  