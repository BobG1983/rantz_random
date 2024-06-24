use crate::RandomContainer;

pub trait Shuffle<T>: RandomContainer<T> + FromIterator<T>
where
    T: Clone,
{
    fn shuffle(&mut self) {
        let mut vec = self.clone().into_iter().collect::<Vec<_>>();
        vec.shuffle();
        *self = Self::from_iter(vec);
    }
    fn shuffled(&self) -> Self {
        let mut vec = self.clone().into_iter().collect::<Vec<_>>();
        vec.shuffle();
        Self::from_iter(vec)
    }
}
