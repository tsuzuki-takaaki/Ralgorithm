// Union Find(ref: https://ja.wikipedia.org/wiki/%E7%B4%A0%E9%9B%86%E5%90%88%E3%83%87%E3%83%BC%E3%82%BF%E6%A7%8B%E9%80%A0)

#[derive(Debug)]
struct Node {
    parent: usize,
    rank: usize,
}
#[derive(Debug)]
struct UnionFind {
  node: Vec<Node>,
}

impl UnionFind {
  // @params n: 最終node(nodeの'数'ではない)
  fn new(n: usize) -> UnionFind {
    UnionFind {
      node: (0..=n).map(|x| Node {parent: x, rank: 0}).collect::<Vec<Node>>()
    }
  }

  // どの素集合に属するか(再帰的に調べる)
  fn find(&mut self, x: usize) -> usize {
    if self.node[x].parent == x {
      x 
    } else {
      self.node[x].parent = self.find(self.node[x].parent);
      self.node[x].parent
    }
  }

  fn unite(&mut self, a: usize, b: usize) {
    let par_a = self.find(a);
    let par_b = self.find(b);
    if par_a == par_b {
      return;
    } else if self.node[par_a].rank < self.node[par_b].rank {
      self.node[par_a].parent = par_b;
    } else {
      self.node[par_b].parent = par_a;
      // rankが同じだった場合は高さが1大きくなる
      if self.node[par_a].rank == self.node[par_b].rank { self.node[par_a].rank += 1 }
    }
  }

  // 同じ素集合に属するか判定
  fn same(&mut self, a: usize, b: usize) -> bool {
    self.find(a) == self.find(b)
  }

  // 各々のnodeの素集合を返す
  fn parents(&mut self) -> Vec<usize> {
    let l = self.node.len();
    for i in 0..l {
      self.find(i);
    }
    self.node.iter().map(|x| x.parent).collect()
  }
}

fn main() {
  let mut uf = UnionFind::new(n-1);
}
