pub trait Random {
    fn random() -> Self;
}

pub trait RandomRange {
    fn random_range(low: Self, high: Self) -> Self;
}
