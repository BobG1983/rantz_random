pub trait Random {
    fn random() -> Self;
}

pub trait RandomRange {
    fn random_range(low: Self, high: Self) -> Self;
}

pub trait RandomContainer<T>: Clone + IntoIterator<Item = T>
where
    T: Clone,
{
    fn random(&self) -> Self::Item {
        self.random_element()
    }

    fn random_with_index(&self) -> (usize, Self::Item) {
        let idx = self.random_index();
        let item = self.clone().into_iter().nth(idx).unwrap();
        (idx, item)
    }

    fn random_index(&self) -> usize {
        let size = self.clone().into_iter().count();
        fastrand::usize(0..size - 1)
    }
    fn random_element(&self) -> Self::Item {
        self.clone().into_iter().nth(self.random_index()).unwrap()
    }
}
