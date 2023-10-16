/// Sorts a `Vec<i32>` using selection sort, and returns the sorted list.
pub fn selection_sort(mut vec: Vec<i32>) -> Vec<i32> {
    for i in 0..(vec.len()) {
        let mut min_index = i;
        for j in i..(vec.len()) {
            if vec[j] < vec[min_index] {
                min_index = j;
            }
        }
        let temp = vec[i];
        vec[i] = vec[min_index];
        vec[min_index] = temp;
    }

    vec
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_selection_sort() {
        assert_eq!(selection_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(selection_sort(vec![]), vec![]);
        assert_eq!(selection_sort(vec![1]), vec![1]);
        assert_eq!(
            selection_sort(vec![3, 2, 9, 58, 10, 3]),
            vec![2, 3, 3, 9, 10, 58]
        );
    }
}
