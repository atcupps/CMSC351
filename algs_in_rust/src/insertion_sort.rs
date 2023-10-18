/// Given a `Vec<i32>`, sort it using insertion sort and return the sorted list.
/// ```rust
/// use algs_in_rust::insertion_sort::insertion_sort;
/// 
/// let unsorted = vec![5, 4, 3, 2, 1];
/// let sorted = insertion_sort(unsorted.clone());
/// 
/// assert_eq!(unsorted, vec![5, 4, 3, 2, 1]);
/// assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
/// ```
pub fn insertion_sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        return vec;
    }

    for i in 1..(vec.len()) {
        let mut cur_index = i;
        while cur_index > 0 && vec[cur_index] < vec[cur_index - 1] {
            vec.swap(cur_index - 1, cur_index);
            cur_index -= 1;
        }
    }

    vec
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_insertion_sort() {
        assert_eq!(insertion_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            insertion_sort(vec![1, 5, 2, 9, 8, 7, 0, 3, 6, -5]),
            vec![-5, 0, 1, 2, 3, 5, 6, 7, 8, 9]
        );
        assert_eq!(insertion_sort(vec![3, 2]), vec![2, 3]);
        assert_eq!(insertion_sort(vec![0]), vec![0]);
        assert_eq!(insertion_sort(vec![]), vec![]);
    }
}
