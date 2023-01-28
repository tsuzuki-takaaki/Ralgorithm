// 二分探索

// lower_boundではない(tが最初に見つかったindexを返す)
fn bsearch(arr: &Vec<isize>, t: isize) -> isize {
  let mut l = 0;
  let mut r = arr.len() - 1;
  while l <= r {
      let mid = (l + r) / 2;
      println!("{}", mid);
      if arr[mid] == t {
          return mid as isize;
      } else if arr[mid] < t {
          l = mid + 1;
      } else {
          r = mid - 1;
      }
  }
  return -1;
}

fn main() {
  let mut a = vec![1, 2, 3, 4, 4, 4, 4, 4,  6, 6];
  println!("{}", bsearch(&a, 4));
}

// lower_bound, upper_bound => superslice
// use superslice::Ext;

// fn main() {
//   let a = vec![1, 2, 3, 4, 4, 4, 4, 4, 6, 6];
//   println!("{}", a.lower_bound(&4));
// }
