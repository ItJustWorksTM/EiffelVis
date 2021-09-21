use crate::graph::*;
use uuid::Uuid;
use std::collections::BTreeMap;
use IntoIterator;

struct Node<T, E: Copy> {
    links: Vec<Link<E>>,
    data: T,
}

#[derive(Default)]
pub struct BasicBTreemapGraph<T, E: Copy> {
    underlying: BTreeMap<Uuid, Node<T, E>>,
}

pub struct Iter<'a, T, E: Copy> {
    underlying: <&'a BTreeMap<Uuid, Node<T, E>> as IntoIterator>::IntoIter,
}

impl<'a, T, E: Copy> Iterator for Iter<'a, T, E> {
    type Item = (&'a [Link<E>], &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|(_, node)| (node.links.as_slice(), &node.data))
    }
}

impl<T, E: Copy> Graph<T, E> for BasicBTreemapGraph<T, E> {
    fn push(&mut self, key: &Uuid, links: impl Into<Vec<Link<E>>>, data: T) {
        self.underlying.insert(*key, Node { links: links.into(), data });
    }

    fn get(&self, key: &Uuid) -> Option<(&[Link<E>], &T)> {
        self.underlying.get(key).map(|e| (e.links.as_slice(), &e.data))
    }

    fn iter<'a>(&'a self) -> Self::Iter<'a> { Iter { underlying: self.underlying.iter() } }

    type Iter<'a> where T: 'a, E: 'a = Iter<'a, T, E>;

    fn type_name() -> &'static str {
        "Basic BTreeMap"
    }
}
