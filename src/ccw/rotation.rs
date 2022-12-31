// 原点を中心として(a, b)を反時計回りにd度回転させた時の座標
use std::f64::consts::PI;

fn main() {
  input!{
    a: f64,
    b: f64,
    d: f64,
  }
  let s = a * (PI * d / 180.0).cos() - b * (PI * d / 180.0).sin();
  let t = a * (PI * d / 180.0).sin() + b * (PI * d / 180.0).cos();
  
  println!("{} {}", s, t);
}
