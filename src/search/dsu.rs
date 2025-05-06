pub struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_parent = self.find(x);
        let y_parent = self.find(y);
        if x_parent != y_parent {
            self.parent[y_parent] = x_parent;
        }
    }
}
