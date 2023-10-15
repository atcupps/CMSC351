/// Given a `Vec<i32>`, sort it using bubble sort, and return the sorted list.
pub fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut max_index = vec.len();

    while max_index > 1 {
        for i in 1..max_index {
            if vec[i - 1] > vec[i] {
                let temp = vec[i - 1];
                vec[i - 1] = vec[i];
                vec[i] = temp;
            }
        }
        max_index -= 1;
    }

    vec
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_bubble_sort() {
        assert_eq!(bubble_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            bubble_sort(vec![1, 3, 2, 9, 8, 3, 5, 10, 2, 5]),
            vec![1, 2, 2, 3, 3, 5, 5, 8, 9, 10]
        );
        assert_eq!(bubble_sort(vec![]), vec![]);
        assert_eq!(bubble_sort(vec![1, 1, 2, 1]), vec![1, 1, 1, 2]);
    }
}
