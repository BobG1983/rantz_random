mod random_impl;
mod random_range_impl;
mod random_traits;
mod shuffle_impl;
mod shuffle_trait;
mod weighted_table;
pub use random_traits::Random;
pub use random_traits::RandomContainer;
pub use random_traits::RandomRange;
pub use shuffle_trait::Shuffle;
pub use weighted_table::WeightedTable;

pub fn seed(seed: u64) {
    fastrand::seed(seed);
}
