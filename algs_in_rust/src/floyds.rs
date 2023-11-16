/// Runs Floyd's algorithm on an adjacency matrix for a graph
/// and returns a predacessor matrix. Assumes the matrix is square.
pub fn floyds(mut adj: Vec<Vec<Option<u32>>>) -> Vec<Vec<Option<u32>>> {
    let length = adj.len();

    // Initializing the predacessor matrix
    let mut p = vec![vec![None; adj[0].len()]; adj.len()];
    for i in 0..length {
        for j in 0..length {
            if i == j {
                p[i][j] = Some(i as u32);
            }
            else if adj[i][j].is_some() {
                p[i][j] = Some(i as u32);
            }
        }
    }
    
    // Performing algorithm
    for k in 0..length {
        for i in 0..length {
            for j in 0..length {
                if let Some(ik) = adj[i][k] {
                    if let Some(kj) = adj[k][j] {
                        if let Some(ij) = adj[i][j] {
                            if ik + kj < ij {
                                adj[i][j] = Some(ik + kj);
                                p[i][j] = p[k][j];
                            }
                        } else {
                            adj[i][j] = Some(ik + kj);
                            p[i][j] = p[k][j];
                        }
                    }
                }
            }
        }
    }
    p
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_foyd() {
        let d = 
            vec![
                vec![Some(0), Some(20), None],
                vec![None, Some(0), Some(10)],
                vec![Some(40), Some(80), Some(0)]
            ];
        let expected = 
            vec![
                vec![Some(0), Some(0), Some(1)],
                vec![Some(2), Some(1), Some(1)],
                vec![Some(2), Some(0), Some(2)]
            ];
        assert_eq!(expected, floyds(d));
    }

}