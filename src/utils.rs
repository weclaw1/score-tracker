use std::collections::HashSet;
use std::hash::Hash;

pub fn symetric_difference_between_two_arrays<'a, T>(a: &'a [T], b: &'a [T]) -> Vec<&'a T>
where
    T: Eq + Hash,
{
    let a: HashSet<&T> = HashSet::from_iter(a.iter());
    let b: HashSet<&T> = HashSet::from_iter(b.iter());
    a.symmetric_difference(&b).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_difference_between_two_arrays() {
        let a = vec![1, 2, 3, 4];
        let b = vec![3, 4, 5, 6];
        let result = symetric_difference_between_two_arrays(&a, &b);

        let expected_values: HashSet<i32> = HashSet::from_iter(vec![1, 2, 5, 6]);

        for element in result {
            assert!(expected_values.contains(element));
        }
    }

    #[test]
    fn test_symmetric_difference_between_two_arrays_with_one_empty_array() {
        let a = [];
        let b = vec![3, 4, 5, 6];
        let result = symetric_difference_between_two_arrays(&a, &b);

        let expected_values: HashSet<i32> = HashSet::from_iter(vec![3, 4, 5, 6]);

        for element in result {
            assert!(expected_values.contains(element));
        }
    }

    #[test]
    fn test_symmetric_difference_between_two_arrays_duplicates() {
        let a = vec![1, 2, 3, 4, 4];
        let b = vec![3, 4, 4, 5, 6];
        let result = symetric_difference_between_two_arrays(&a, &b);

        let expected_values: HashSet<i32> = HashSet::from_iter(vec![1, 2, 5, 6]);

        for element in result {
            assert!(expected_values.contains(element));
        }
    }
}
