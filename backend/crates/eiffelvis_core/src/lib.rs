#![feature(generic_associated_types)]
pub mod graph;
pub mod graph_impls;

#[cfg(test)]
mod tests {
    use crate::graph_impls::basic_hashmap::BasicHashmapGraph;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    struct Data {}

    #[derive(Copy, Clone)]
    enum LinkType {}
    #[test]
    fn basic_hashmap() {
        let g : BasicHashmapGraph<Data, LinkType>;
    }
}
