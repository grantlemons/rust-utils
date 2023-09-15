use std::borrow::Cow;
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

pub fn ordinal_suffix<'a, T: Into<i128> + PartialOrd>(value: T) -> &'a str {
    match value.into().abs() {
        1 => "st",
        2 => "nd",
        3 => "rd",
        v if v >= 20 => ordinal_suffix(v % 10),
        _ => "th",
    }
}

pub fn ordinal<T: Into<i128> + PartialOrd + Copy>(value: T) -> String {
    format!("{}{}", value.into(), ordinal_suffix(value))
}

pub fn pluralize_int<'a, T: Into<i128> + PartialOrd>(word: &'a str, count: T) -> Cow<'a, str> {
    if count.into().abs() == 1 {
        word.into()
    } else {
        format!("{}s", word).into()
    }
}

pub fn pluralize_iter<'a, T: Iterator>(word: &'a str, collection: T) -> Cow<'a, str> {
    if collection.count() == 1 {
        word.into()
    } else {
        format!("{}s", word).into()
    }
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

    #[test]
    fn ordinal_suffix_test() {
        assert_eq!(ordinal_suffix(0), "th");
        assert_eq!(ordinal_suffix(1), "st");
        assert_eq!(ordinal_suffix(2), "nd");
        assert_eq!(ordinal_suffix(3), "rd");
        assert_eq!(ordinal_suffix(-1), "st");
        assert_eq!(ordinal_suffix(-2), "nd");
        assert_eq!(ordinal_suffix(-3), "rd");
        assert_eq!(ordinal_suffix(8), "th");
        assert_eq!(ordinal_suffix(156), "th");
        assert_eq!(ordinal_suffix(890345823), "rd");
    }

    #[test]
    fn ordinal_test() {
        assert_eq!(ordinal(0), "0th");
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(2), "2nd");
        assert_eq!(ordinal(3), "3rd");
        assert_eq!(ordinal(-1), "-1st");
        assert_eq!(ordinal(-2), "-2nd");
        assert_eq!(ordinal(-3), "-3rd");
        assert_eq!(ordinal(8), "8th");
        assert_eq!(ordinal(156), "156th");
        assert_eq!(ordinal(890345823), "890345823rd");
    }

    #[test]
    fn pluralize_int_test() {
        assert_eq!(pluralize_int("word", 0).to_owned(), "words");
        assert_eq!(pluralize_int("word", 1).to_owned(), "word");
        assert_eq!(pluralize_int("word", 2).to_owned(), "words");
        assert_eq!(pluralize_int("word", -1).to_owned(), "word");
        assert_eq!(pluralize_int("word", -2).to_owned(), "words");
    }

    #[test]
    fn pluralize_iter_test() {
        assert_eq!(pluralize_iter("word", 1..3).to_owned(), "words");
        assert_eq!(pluralize_iter("word", 1..1).to_owned(), "words");
        assert_eq!(pluralize_iter("word", 1..=1).to_owned(), "word");
    }
}
