// ANCHOR: transpose
pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // ANCHOR_END: transpose
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // println!("i: {}, j: {}", i, j);
            // println!("{:?}", matrix[i][j]);
            result[j][i] = matrix[i][j];
        }
    }
    // println!("{result:?}");

    return result;
}

// ANCHOR: pretty_print
pub fn pretty_print(matrix: &[[i32; 3]; 3]) {
    // ANCHOR_END: pretty_print
    for row in matrix {
        println!("{row:?}");
    }
}

// ANCHOR: tests
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
// ANCHOR_END: tests
