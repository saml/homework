/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let cols1 = mat1[0].len();
    let rows2 = mat2.len();
    assert_eq!(cols1, rows2);

    let rows = mat1.len();
    let cols = mat2[0].len();
    let m = vec![vec![]];

    for x in 0..cols1 {
        for y in 0..rows2 {
            mat1[x][]
        }

    }


}

#[cfg(test)]
mod tests {
    use super::*;
    // Problem 2
    //

    #[test]
    fn test_mat_mult_identity() {
        let mut mat1 = vec![vec![0.;3]; 3];
        for i in 0..mat1.len() {
            mat1[i][i] = 1.;
        }
        let mat2 = vec![vec![5.;3]; 3];
        let result = mat_mult(&mat1, &mat2);
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                assert_eq!(result[i][j], mat2[i][j]);
            }
        }
    }

}
