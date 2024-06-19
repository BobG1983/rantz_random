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
    fn random(&self) -> Option<Self::Item> {
        self.random_element()
    }

    fn random_with_index(&self) -> Option<(usize, Self::Item)> {
        if let Some(idx) = self.random_index() {
            let item = self.clone().into_iter().nth(idx).unwrap();
            return Some((idx, item));
        }
        None
    }

    fn random_index(&self) -> Option<usize> {
        let size = self.clone().into_iter().count();
        if size == 0 {
            return None;
        } else {
            return Some(fastrand::usize(1..=size) - 1);
        }
    }
    fn random_element(&self) -> Option<Self::Item> {
        return self.random_with_index().map(|(_, item)| item);
    }
}

pub trait RandomWeightedContainer<T>: Clone + IntoIterator<Item = T>
where
    T: Clone,
{
    fn max_weight(&self) -> u32;
    fn weights(&self) -> &Vec<u32>;
    fn values(&self) -> &Vec<T>;
    fn random_weight(&self) -> Option<u32> {
        if self.max_weight() == 0 {
            return None;
        }
        Some(fastrand::u32(0..=self.max_weight()))
    }
    fn weighted_random_with_weight(&self, weight: u32) -> Option<Self::Item> {
        if weight > self.max_weight()
            || self.max_weight() == 0
            || self.weights().is_empty()
            || self.values().is_empty()
        {
            return None;
        }

        let mut n = weight;
        for (i, weight) in self.weights().iter().enumerate() {
            if &n <= weight {
                return self.values().get(i).cloned();
            } else {
                n -= weight;
            }
        }

        None
    }
    fn weighted_random(&self) -> Option<Self::Item> {
        if let Some(weight) = self.random_weight() {
            return self.weighted_random_with_weight(weight);
        }

        None
    }
}
