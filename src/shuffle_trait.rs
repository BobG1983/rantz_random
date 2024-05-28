pub trait Shuffle<T>: Clone + IntoIterator<Item = T> + FromIterator<T>
where
    T: Clone,
{
    fn shuffle(&mut self) {
        let mut vec = self.clone().into_iter().collect::<Vec<_>>();
        vec.shuffle();
        *self = vec.into_iter().collect();
    }
    fn shuffled(&self) -> Self {
        let mut vec = self.clone().into_iter().collect::<Vec<_>>();
        vec.shuffle();
        vec.into_iter().collect()
    }
    fn random_index(&self) -> usize {
        let size = self.clone().into_iter().count();
        fastrand::usize(0..size - 1)
    }
    fn random_element(&self) -> T {
        self.clone().into_iter().nth(self.random_index()).unwrap()
    }
}
