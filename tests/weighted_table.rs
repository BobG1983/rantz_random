#[cfg(test)]
mod tests {
    use rantz_random::{RandomContainer, RandomWeightedContainer, WeightedTable};

    #[test]
    fn inserting_adds_to_values_and_weights() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);

        assert_eq!(table.weights(), &vec![10]);
        assert_eq!(table.max_weight(), 10);
        assert_eq!(table.values(), &vec![1]);
    }

    #[test]
    fn inserting_multiple_adds_to_values_and_weights() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);
        table.insert(2, 20);
        table.insert(3, 30);

        assert_eq!(table.weights(), &vec![10, 20, 30]);
        assert_eq!(table.max_weight(), 60);
        assert_eq!(table.values(), &vec![1, 2, 3]);
    }

    #[test]
    fn removing_removes_from_values_and_weights() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);
        table.insert(2, 20);

        table.remove(&1);

        assert_eq!(table.weights(), &vec![20]);
        assert_eq!(table.max_weight(), 20);
        assert_eq!(table.values(), &vec![2]);
    }

    #[test]
    fn removing_multiple_removes_from_values_and_weights() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);
        table.insert(2, 20);
        table.insert(3, 30);

        table.remove(&1);

        assert_eq!(table.weights(), &vec![20, 30]);
        assert_eq!(table.max_weight(), 50);
        assert_eq!(table.values(), &vec![2, 3]);
    }

    #[test]
    fn removing_all_removes_from_values_and_weights() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);
        table.insert(2, 20);

        table.remove(&1);
        table.remove(&2);

        assert_eq!(table.weights(), &Vec::<u32>::new());
        assert_eq!(table.max_weight(), 0);
        assert_eq!(table.values(), &Vec::<u32>::new());
    }

    #[test]
    fn clear_removes_from_values_and_weights() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);
        table.insert(2, 20);
        table.insert(3, 30);

        table.clear();

        assert_eq!(table.weights(), &Vec::<u32>::new());
        assert_eq!(table.max_weight(), 0);
        assert_eq!(table.values(), &Vec::<u32>::new());
    }

    #[test]
    fn inserting_the_same_value_updates_that_values_weight() {
        let mut table = WeightedTable::new();

        table.insert(1, 10);
        table.insert(1, 20);

        assert_eq!(table.weights(), &vec![20]);
        assert_eq!(table.max_weight(), 20);
        assert_eq!(table.values(), &vec![1]);
    }

    #[test]
    fn implements_get_entry() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.get_entry(0), Some((1, 10)));
    }

    #[test]
    fn implements_get_entry_ref() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.get_entry_ref(0), Some((&1, &10)));
    }

    #[test]
    fn implements_get_entry_mut() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.get_entry_mut(0), Some((&mut 1, &mut 10)));
    }

    #[test]
    fn implements_get_weight() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.get_weight(&1), Some(10));
    }

    #[test]
    fn implements_get_weight_mut() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.get_weight_mut(&1), Some(&mut 10));
    }

    #[test]
    fn changing_weight_with_get_weight_mut_updates_weight() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        *table.get_weight_mut(&1).unwrap() = 20;

        assert_eq!(table.get_weight_mut(&1), Some(&mut 20));
    }

    #[test]
    fn changing_entry_with_get_entry_mut_updates_entry() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        let entry = table.get_entry_mut(0).unwrap();
        *entry.0 = 2;
        *entry.1 = 40;

        assert_eq!(table.get_entry(0), Some((2, 40)));
    }

    #[test]
    fn implements_iter() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        table.insert(2, 20);

        let mut iter = table.iter();

        assert_eq!(iter.next(), Some((&1, &10)));
        assert_eq!(iter.next(), Some((&2, &20)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_iter() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        for (v, w) in table.iter() {
            assert!(v == &1);
            assert!(w == &10);
        }
    }

    #[test]
    fn implements_iter_mut() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        table.insert(2, 20);

        let mut iter = table.iter_mut();

        assert_eq!(iter.next(), Some((&mut 1, &mut 10)));
        assert_eq!(iter.next(), Some((&mut 2, &mut 20)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_iter_mut() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        for (v, w) in table.iter_mut() {
            assert!(v == &mut 1);
            assert!(w == &mut 10);
        }
    }

    #[test]
    fn iter_mut_changes_underlying_table() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        for (v, w) in table.iter_mut() {
            *v = 2;
            *w = 20;
        }

        assert_eq!(table.get_entry(0), Some((2, 20)));
    }

    #[test]
    fn can_iter_without_call() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        for v in table {
            assert!(v == 1);
        }
    }

    #[test]
    fn implements_random() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.random(), Some(1));
    }

    #[test]
    fn empty_table_returns_none() {
        let table = WeightedTable::<u32>::new();

        assert_eq!(table.random(), None);
        assert_eq!(table.weighted_random(), None);
        assert_eq!(table.weighted_random_with_weight(1), None);
    }

    #[test]
    fn implements_weighted_random() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);

        assert_eq!(table.weighted_random(), Some(1));
    }

    #[test]
    fn weighted_random_returns_correct_value() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        table.insert(2, 10);

        assert_eq!(table.weighted_random_with_weight(20), Some(2));
    }

    #[test]
    fn implements_combine() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        let mut table2 = WeightedTable::new();
        table2.insert(1, 10);
        table2.insert(2, 20);

        table.combine(table2);

        assert_eq!(table.get_entry(0), Some((1, 20)));
        assert_eq!(table.get_entry(1), Some((2, 20)));
    }

    #[test]
    fn handles_exact_random_value() {
        let mut table = WeightedTable::new();
        table.insert(1, 10);
        table.insert(2, 10);

        assert_eq!(table.random_with(10), (1, 10));
        assert_eq!(table.random_with(20), (2, 10));
    }
}
