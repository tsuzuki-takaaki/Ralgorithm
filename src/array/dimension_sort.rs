// n次元sort

// 要素0を基準に昇順sortした後、要素1を基準に降順sort
fn main() {
  let mut vec = vec![[28, 8], [1, 8], [1, 1],  [1000, 99], [1, 9]];
  vec.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
  println!("{:?}", vec);
}

// 要素0を基準に昇順sort
// fn main() {
//   let mut vec = vec![[28, 8], [1, 8], [1, 1],  [1000, 99], [1, 9]];
//   vec.sort_by(|a, b| a[0].cmp(&b[0]));
//   println!("{:?}", vec); 
// }
