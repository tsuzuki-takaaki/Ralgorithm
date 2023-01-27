// tally(ref: https://docs.ruby-lang.org/ja/latest/method/Enumerable/i/tally.html)

fn tally(arr: Vec<usize>) -> HashMap<usize, usize> {
  let mut map = HashMap::new();
  for i in 0..arr.len() {
    let count = map.entry(arr[i]).or_insert(0);
    *count += 1;
  }
  map
}