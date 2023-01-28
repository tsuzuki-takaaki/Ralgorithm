// ランレングス圧縮

fn run_length(s: &Vec<char>) -> Vec<(char, usize)>{
  let mut v = vec![];
  let ll = s.len();
  let mut cnt = 1;
  for i in 0..ll {
    if i == ll-1 {
      v.push((s[i], cnt));
      break;
    }
    if s[i] != s[i+1] {
      v.push((s[i], cnt));
      cnt = 1;
    } else {
      cnt += 1;
    }
  }
  v
}

fn main() {
  let s = "aabbbaabbbbb";
  // std::str::Charsとproconio::marker::Charsは異なる
  let ss = s.chars().collect::<Vec<char>>();
  println!("{:?}", run_length(&ss));
  println!("{:?}", ss);
}

// fn main() {
//   input! {
//       s: Chars, => Vec<char>
//       t: Chars,
//   }
//   // std::str::Charsとproconio::marker::Charsは異なる
//   println!("{:?}", run_length(&s));
// }
