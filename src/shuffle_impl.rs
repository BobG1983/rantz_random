use crate::Shuffle;

impl<T> Shuffle<T> for Vec<T>
where
    T: Clone,
{
    fn shuffle(&mut self) {
        fastrand::shuffle(self.as_mut_slice());
    }
}

#[cfg(feature = "bevy")]
mod bevy {
    use crate::Shuffle;
    use bevy::utils::HashSet;
    use std::hash::Hash;

    impl<T> Shuffle<T> for HashSet<T> where T: Clone + Eq + Hash {}
}
