/*
* We are going to populate the matrix layer by layer,
* progressively moving inwards, like peeling an onion.
*
* Some observations:
* 1. - A 2x2 matrix has 1 layer.
*    - A 3x3 matrix has 1 more layer on top of a 2x2 matrix,
     - so, 2 layers.
*    - A 4x4 matrix has 1 more layer on top of a 3x3 matrix,
     - so, 3 layers.
*    - A 5x5 matrix has 1 more layer on top of a 4x4 matrix,
     - so, 4 layers.
*    - Generalizing, a nxn matrix has ((n + 1) / 2) layers.
*
* 2. - 1st layer has n elements in each row.
*    - 2nd layer has (n - 2) elements in each row.
*    - 3rd layer has (n - 4) elements in each row.
*    - Generalizing, each layer has row_len = n - 2 * layer_number
*    elements in each row.
*
* 3. Total number of elements in a layer is given by:
     2 * row_len <-- top and bottom rows
     + 2 * (row_len - 2) <-- 1 element each for the left and right columns
*
* 4. There are row_len - 1 consecutive numbers between corner elements.
*
* For each layer, we will populate 4 elements at each iteration, and
* proceed in a spiral manner, as if rotating the layer by 1 element
* at each iteration.
*
* Example: For a 3x3 matrix:
*
* Layer 1, end of iteration 1:
*   1 _ 3
*   _ _ _
*   7 _ 5
*
* Layer 1, end of iteration 2:
*   1 2 3
*   8 _ 4
*   7 6 5
*
* Layer 2, end of iteration 1:
*   1 2 3
*   8 9 4
*   7 6 5
*/
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Observation 1
    let num_layers: usize = ((size + 1) / 2) as usize;
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut start: u32 = 1;

    for layer in 0_usize..num_layers {
        // Observation 2
        let row_len: u32 = size - (2 * layer) as u32;
        let inc: u32 = size - 1 - (2 * layer) as u32;

        if row_len == 1 {
            matrix[layer][layer] = start;
        } else {
            // Observation 4
            for i in 0..(row_len - 1) as usize {
                let j = i as u32;
                let k = size as usize;
                // top left, same row, column moves to the right
                matrix[layer][layer + i] = start + j;
                // top right, same column, row moves downwards
                matrix[layer + i][k - layer - 1] = start + inc + j;
                // bottom right, same row, column moves to the left
                matrix[k - layer - 1][k - layer - 1 - i] = start + j + 2 * inc;
                // bottom left, same column, row moves upwards
                matrix[k - layer - 1 - i][layer] = start + j + 3 * inc;
            }

            // Observation 3
            start += 2 * row_len + 2 * (row_len - 2);
        }
    }
    matrix
}
