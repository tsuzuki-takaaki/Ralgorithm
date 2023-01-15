// Counter Clock Wise(反時計回り判定)
// ref: https://atcoder.jp/contests/abc266/tasks/abc266_c

// a -> b -> cが反時計回りに並んでいるかの判定
fn ccw(ax: isize, ay: isize, bx: isize, by: isize, cx: isize, cy: isize) -> isize {
  let dx1 = ax - bx;
  let dy1 = ay - by;
  let dx2 = cx - bx;
  let dy2 = cy - by;
  let cross_product = dx1 * dy2 - dx2 * dy1;
  // 反時計回りの場合は1, 同一直線上の場合は0, 時計回りの場合は-1
  if cross_product < 0 {
    return 1;
  } else if cross_product == 0 {
    return 0;
  } else {
    return -1;
  }
}

// ref: https://atcoder.jp/contests/abc266/tasks/abc266_c
fn main() {
  input! {
    a: [isize; 2],
    b: [isize; 2],
    c: [isize; 2],
    d: [isize; 2]
  }
  let mut ans = 0;

  ans += ccw(a[0], a[1], b[0], b[1], c[0], c[1]);
  ans += ccw(b[0], b[1], c[0], c[1], d[0], d[1]);
  ans += ccw(c[0], c[1], d[0], d[1], a[0], a[1]);
  ans += ccw(d[0], d[1], a[0], a[1], b[0], b[1]);

  println!("{}", if ans == 4 {"Yes"} else {"No"});
}
