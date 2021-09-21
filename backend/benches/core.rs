use criterion::{BatchSize, black_box, criterion_group, criterion_main, Criterion};
use eiffelvis_core::graph::Graph;
use eiffelvis_core::graph_impls::basic_hashmap::BasicHashmapGraph;
use eiffelvis_core::graph_impls::basic_btreemap::BasicBTreemapGraph;
use uuid::Uuid;

#[derive(Default)]
struct Data {
    _no: u32,
}

#[derive(Copy, Clone)]
enum LinkType {
    None,
    Foo,
    Bar,
}

impl Default for LinkType {
    fn default() -> Self { LinkType::None }
}

fn criterion_benchmark<G: Graph<Data, LinkType> + Default>(c: &mut Criterion) {
    let mut g: G = Default::default();
    c.bench_function(&*format!("{} noop", G::type_name()), |b| b.iter(|| black_box(0)));
    c.bench_function(&*format!("{} push", G::type_name()), |b| {
        b.iter_batched(Uuid::new_v4, |v| g.push(&v, vec![], Default::default()), BatchSize::SmallInput);
    });
    c.bench_function(&*format!("{} lookups", G::type_name()), |b| {
        b.iter_batched(Uuid::new_v4, |v| g.get(&v), BatchSize::SmallInput);
    });
}

criterion_group!(benches,
    criterion_benchmark<BasicHashmapGraph<Data, LinkType>>,
    criterion_benchmark<BasicBTreemapGraph<Data, LinkType>>
);
criterion_main!(benches);
