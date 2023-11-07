use std::cmp;

/// Given a vector of coin values `coins`, and an integer `amount` representing a monetary value,
/// returns the minimum number of coins in `coins` required to make `amount`, or `None` if there
/// is no combination of `coins` that can make `amount`.
/// ```rust
/// use algs_in_rust::coin_change::coin_change;
///
/// let coins = vec![1, 2, 5];
/// let min_coins = coin_change(coins, 36);
/// assert_eq!(min_coins, Some(8));
/// ```
pub fn coin_change(coins: Vec<i32>, amount: i32) -> Option<i32> {
    if amount < 0 || coins.is_empty() {
        return None;
    }

    let mut table = vec![None; (amount + 1) as usize];
    table[0] = Some(0);

    for coin in coins {
        for i in 1..(table.len()) {
            let index = i as i32;
            if index - coin >= 0 {
                if let Some(prev) = table[(index - coin) as usize] {
                    if let Some(val) = table[i] {
                        table[i] = Some(cmp::min(val, prev + 1));
                    } else {
                        table[i] = Some(prev + 1);
                    }
                }
            }
        }
    }

    table[amount as usize]
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_coin_change_valid() {
        assert_eq!(coin_change(vec![1, 2, 5], 5), Some(1));
        assert_eq!(coin_change(vec![1, 2, 5], 13), Some(4));
        assert_eq!(coin_change(vec![2, 4, 6], 6), Some(1));
        assert_eq!(coin_change(vec![1, 5, 10, 25], 123), Some(9));
        assert_eq!(coin_change(vec![3, 5, 8], 9), Some(3));
        assert_eq!(coin_change(vec![1, 2, 3, 4, 5], 0), Some(0));
    }

    #[test]
    fn test_coin_change_invalid() {
        assert_eq!(coin_change(vec![], 3), None);
        assert_eq!(coin_change(vec![1, 2, 5], -3), None);
    }

    #[test]
    fn test_coin_change_impossible() {
        assert_eq!(coin_change(vec![2], 3), None);
        assert_eq!(coin_change(vec![4, 5, 6], 7), None);
    }
}
