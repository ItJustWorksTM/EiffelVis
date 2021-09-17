struct Node {
    links: Vec<u16>, // small vec maybe?
}

struct List {
    inner: Vec<Node>,
}

impl List {
    fn new() -> Self {
        List {
            inner: Vec::with_capacity(u16::MAX as usize),
        }
    }
    fn push(&mut self, links: Vec<u16>) {
        assert!(links.iter().all(|v| (*v as usize) < self.inner.len()));
        self.inner.push(Node { links });
    }
}
