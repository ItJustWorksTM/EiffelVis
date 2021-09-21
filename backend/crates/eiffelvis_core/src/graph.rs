use uuid::Uuid;

pub struct Link<E: Copy> {
    uuid: Uuid,
    type_: E,
}

pub trait Graph<T, E: Copy> {
    fn type_name() -> &'static str;

    fn push(&mut self, key: &Uuid, links: impl Into<Vec<Link<E>>>, value: T);
    fn get(&self, key: &Uuid) -> Option<(&[Link<E>], &T)>;

    type Iter<'a> where Self: 'a;
    fn iter<'a>(&'a self) -> Self::Iter<'a>;
}
