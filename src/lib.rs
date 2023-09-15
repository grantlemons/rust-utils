use std::collections::HashMap;
use std::hash::Hash;

pub fn frequency<T: Sized + Copy + Hash + Eq>(input: &[T]) -> HashMap<T, u64> {
    let mut map = HashMap::new();

    for value in input {
        if let Some(freq) = map.get_mut(value) {
            *freq += 1;
        } else {
            map.insert(*value, 1);
        }
    }

    map
}

pub fn count_occurrences<T: Sized + Copy + PartialEq>(array: &[T], value: T) -> usize {
    array.iter().filter(|v| **v == value).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_test() {
        let arr = [
            1, 534, 312, 543, 22, 43, 22, 1, 15, 312, 312, 534, 876, 15, 22, 1, 534, 43, 14,
        ];
        let map = frequency(&arr);

        assert_eq!(*map.get(&15).unwrap(), 2);
        assert_eq!(*map.get(&312).unwrap(), 3);
        assert_eq!(*map.get(&543).unwrap(), 1);
    }

    #[test]
    fn count_occurrences_test() {
        let arr = [
            1, 534, 312, 543, 22, 43, 22, 1, 15, 312, 312, 534, 876, 15, 22, 1, 534, 43, 14,
        ];

        assert_eq!(count_occurrences(&arr, 15), 2);
        assert_eq!(count_occurrences(&arr, 312), 3);
        assert_eq!(count_occurrences(&arr, 543), 1);
    }
}
