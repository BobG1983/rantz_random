pub trait Random {
    fn random() -> Self;
}

pub trait RandomRange {
    fn random_range(low: Self, high: Self) -> Self;
}

pub trait RandomContainer<T>: Clone + IntoIterator
where
    T: Clone,
{
    fn random(&self) -> Self::Item;
}
