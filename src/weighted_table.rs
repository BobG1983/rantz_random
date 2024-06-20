use crate::random_traits::RandomWeightedContainer;
use std::{marker::PhantomData, slice::IterMut};

/// WeightedTable
///
/// This is a simple implementation of a weighted table. It is designed to be used with a random number generator.
/// It can be used as follows:
///
/// ```rust
/// use rantz_random::{WeightedTable, RandomContainer};
///
/// let mut table = WeightedTable::new();
///  
/// table.insert("Bob", 10); // Bob has a weight of 10
/// table.insert("Alice", 20); // Alice has a weight of 20
/// table.remove(&"Bob"); // Bob is removed
///
/// table.random(); // Returns a random element from the table based on the weights
/// ```
/// The table is iterable and returns owned tuples of (value, weight)
/// For references, use the [iter](WeightedTable::iter) method
/// For mutable references, use the [iter_mut](WeightedTable::iter_mut) method
///
/// ```rust
/// use rantz_random::WeightedTable;
///
/// let mut table = WeightedTable::new();
///
/// table.insert("Bob".to_string(), 10); // Bob has a weight of 10
/// table.insert("Alice".to_string(), 20); // Alice has a weight of 20
///
///
/// for (value, weight) in table.iter() {
///   println!("Reference {} has a weight of {}", value, weight);
/// }
///
/// for (value, weight) in table.iter_mut() {
///   *value += " Test";
///   *weight += 10;
///   println!("Reference {} has a weight of {}", value, weight);
/// }
///
/// for value in table { // Must be done last as consumes the table
///   println!("Owned {}", value);
/// }
/// ```
///
/// Adding an element that already exists will update the weight of the existing element.
#[derive(Clone, Debug)]
pub struct WeightedTable<T>
where
    T: PartialEq + Clone,
{
    pub(crate) weights: Vec<u32>,
    pub(crate) total_weight: u32,
    pub(crate) values: Vec<T>,
}

pub type WeightedItem<T> = (T, u32);
pub type WeightedItemRef<'a, T> = (&'a T, &'a u32);
pub type WeightedItemRefMut<'a, T> = (&'a mut T, &'a mut u32);

impl<T> WeightedTable<T>
where
    T: PartialEq + Clone,
{
    pub fn new() -> Self {
        Self {
            weights: Vec::<u32>::new(),
            total_weight: 0,
            values: Vec::<T>::new(),
        }
    }

    pub fn from_vec(vec: Vec<(T, u32)>) -> Self {
        let mut table = Self::new();
        for (value, weight) in vec {
            table.insert(value, weight);
        }
        table
    }

    pub fn insert(&mut self, value: T, weight: u32) {
        if let Some(index) = self.values.iter().position(|v| v == &value) {
            let old_weight = self.weights[index];
            self.weights[index] = weight;
            self.total_weight += weight;
            self.total_weight -= old_weight;
            return;
        }

        self.weights.push(weight);
        self.total_weight += weight;
        self.values.push(value);
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(index) = self.values.iter().position(|v| v == value) {
            self.total_weight -= self.weights[index];
            self.weights.remove(index);
            self.values.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.weights.clear();
        self.total_weight = 0;
        self.values.clear();
    }

    pub fn get_entry(&self, index: usize) -> Option<WeightedItem<T>> {
        if index < self.values.len() {
            Some((self.values[index].clone(), self.weights[index].clone()))
        } else {
            None
        }
    }

    pub fn get_entry_ref(&self, index: usize) -> Option<WeightedItemRef<T>> {
        if index < self.values.len() {
            Some((&self.values[index], &self.weights[index]))
        } else {
            None
        }
    }

    pub fn get_entry_mut(&mut self, index: usize) -> Option<WeightedItemRefMut<T>> {
        if index < self.values.len() {
            Some((&mut self.values[index], &mut self.weights[index]))
        } else {
            None
        }
    }

    pub fn get_weight(&self, value: &T) -> Option<u32> {
        self.values
            .iter()
            .position(|v| v == value)
            .map(|i| self.weights[i])
    }

    pub fn get_weight_mut(&mut self, value: &T) -> Option<&mut u32> {
        if let Some(index) = self.values.iter().position(|v| v == value) {
            Some(&mut self.weights[index])
        } else {
            None
        }
    }

    pub fn random_with(&self, n: u32) -> WeightedItem<T> {
        let mut n = n;
        for (i, weight) in self.weights.iter().enumerate() {
            if &n <= weight {
                return self.get_entry(i).unwrap();
            } else {
                n -= weight;
            }
        }
        unreachable!();
    }

    pub fn iter(&self) -> impl Iterator<Item = WeightedItemRef<T>> {
        WeightedTableIter {
            table: self,
            index: 0,
            size: self.values.len(),
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = WeightedItemRefMut<T>> {
        WeightedTableIterMut {
            value_iter: self.values.iter_mut(),
            weight_iter: self.weights.iter_mut(),
            marker: PhantomData,
        }
    }

    pub fn combine(&mut self, other: Self) {
        self.total_weight += other.total_weight;
        for (v, w) in other.iter() {
            if let Some(index) = self.values.iter().position(|x| x == v) {
                self.weights[index] += w;
            } else {
                self.weights.push(*w);
                self.values.push(v.clone());
            }
        }
    }
}

impl<T> IntoIterator for WeightedTable<T>
where
    T: PartialEq + Clone,
{
    type Item = T;
    type IntoIter = WeightedTableTupleIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let size = self.values.len();
        WeightedTableTupleIntoIter {
            table: self,
            index: 0,
            size,
        }
    }
}

pub struct WeightedTableTupleIntoIter<T>
where
    T: PartialEq + Clone,
{
    table: WeightedTable<T>,
    index: usize,
    size: usize,
}

impl<T> Iterator for WeightedTableTupleIntoIter<T>
where
    T: PartialEq + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let value = self.table.values[self.index].clone();
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub struct WeightedTableIter<'a, T>
where
    T: PartialEq + Clone,
{
    table: &'a WeightedTable<T>,
    index: usize,
    size: usize,
}

impl<'a, T> Iterator for WeightedTableIter<'a, T>
where
    T: PartialEq + Clone,
{
    type Item = WeightedItemRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let value = &self.table.values[self.index];
            let weight = &self.table.weights[self.index];
            self.index += 1;
            Some((value, weight))
        } else {
            None
        }
    }
}

pub struct WeightedTableIterMut<'a, T>
where
    T: PartialEq + Clone,
{
    value_iter: IterMut<'a, T>,
    weight_iter: IterMut<'a, u32>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for WeightedTableIterMut<'a, T>
where
    T: PartialEq + Clone,
{
    type Item = WeightedItemRefMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(value), Some(weight)) = (self.value_iter.next(), self.weight_iter.next()) {
            Some((value, weight))
        } else {
            None
        }
    }
}

impl<T> FromIterator<(T, u32)> for WeightedTable<T>
where
    T: PartialEq + Clone,
{
    fn from_iter<I: IntoIterator<Item = (T, u32)>>(iter: I) -> Self {
        let mut table = WeightedTable::new();
        for (value, weight) in iter {
            table.insert(value, weight);
        }
        table
    }
}

impl<'a, T> FromIterator<(T, &'a u32)> for WeightedTable<T>
where
    T: PartialEq + Clone,
{
    fn from_iter<I: IntoIterator<Item = (T, &'a u32)>>(iter: I) -> Self {
        let mut table = WeightedTable::new();
        for (value, weight) in iter {
            table.insert(value, weight.clone());
        }
        table
    }
}

impl<T> RandomWeightedContainer<T> for WeightedTable<T>
where
    T: Clone + PartialEq,
{
    fn max_weight(&self) -> u32 {
        self.total_weight
    }

    fn weights(&self) -> &Vec<u32> {
        &self.weights
    }

    fn values(&self) -> &Vec<T> {
        &self.values
    }
}
