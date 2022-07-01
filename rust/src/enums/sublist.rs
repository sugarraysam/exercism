use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Equal => sublist_equal_len(_first_list, _second_list),
        Ordering::Less => sublist_unequal_len(_first_list, _second_list),
        // invert result
        Ordering::Greater => match sublist_unequal_len(_second_list, _first_list) {
            Comparison::Sublist => Comparison::Superlist,
            _ => Comparison::Unequal,
        },
    }
}

fn sublist_equal_len<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    for i in 0.._first_list.len() {
        if _first_list[i] != _second_list[i] {
            return Comparison::Unequal;
        }
    }
    Comparison::Equal
}

// _first_list.len() < _second_list.len()
fn sublist_unequal_len<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut start = 0;

    let len1 = _first_list.len();
    let len2 = _second_list.len();

    let n_to_match = len1;
    let mut counter = n_to_match;

    while p1 < len1 && p2 < len2 && len2 - start >= n_to_match {
        if _first_list[p1] == _second_list[p2] {
            p1 += 1;
            p2 += 1;
            counter -= 1;
            if counter == 0 {
                return Comparison::Sublist;
            }
        } else {
            counter = n_to_match;
            start += 1;

            p1 = 0;
            p2 = start;
        }
    }
    if counter == 0 {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert_eq!(Comparison::Equal, sublist(&[0, 1, 2], &[0, 1, 2]))
    }

    #[test]
    fn test_sublist() {
        assert_eq!(
            Comparison::Sublist,
            sublist(&[0, 1, 2], &[0, 1, 0, 1, 2, 3])
        )
    }

    #[test]
    fn test_unequal() {
        assert_eq!(Comparison::Unequal, sublist(&['c', 'd'], &['a', 'b']))
    }

    #[test]
    fn test_partially_matching_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
    }
}
