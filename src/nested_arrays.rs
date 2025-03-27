pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result_matrix : [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..=2 {
        for j in  0..=2 {
            result_matrix[i][j] = matrix[j][i]
        }
    }
    result_matrix
}
