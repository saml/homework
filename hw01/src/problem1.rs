/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for x in slice {
        result += *x;
    }
    result
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut v = vec![];

    for x in vs {
        if !v.contains(x) {
            v.push(*x);
        }
    }

    v
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut v = vec![];

    for x in vs {
        if pred(*x) {
            v.push(*x);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    // Problem 1
    //

    // Part 1

    #[test]
    fn test_sum_small() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(sum(&array), 15);
    }

    // Part 2

    #[test]
    fn test_dedup_small() {
        let vs = vec![1, 2, 2, 3, 4, 1];
        assert_eq!(dedup(&vs), vec![1, 2, 3, 4]);
    }

    // Part 3

    fn even_predicate(x: i32) -> bool {
        (x % 2) == 0
    }

    #[test]
    fn test_filter_small() {
        let vs = vec![1, 2, 3, 4, 5];
        assert_eq!(filter(&vs, &even_predicate), vec![2, 4]);
    }

}
