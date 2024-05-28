use crate::Shuffle;

impl<T> Shuffle<T> for Vec<T>
where
    T: Clone,
{
    fn shuffle(&mut self) {
        fastrand::shuffle(self.as_mut_slice());
    }
}
