use reed_solomon_erasure::*;
pub fn encode(data: Vec<Vec<u8>>, shard_len: usize) {
    // let data_matrix =
    // 23 shards
    // sqrt of 23 = 4.79
    // round up 4.79 ~ 5
    // 5^2 =25
    // 25-23= 2
    // 5x5
    // construct zero 2d array of 5x5
    // sequentially fill matrix with data
    // once data is over fill digest
    let data_array_length = data.len();
    let sq_root = (data_array_length as f64).sqrt();
    let sq_root_rounded = sq_root.ceil() as usize;
    let mut data_matrix = vec![vec![vec![0; shard_len]; sq_root_rounded]; sq_root_rounded];
    for i in 0..data_array_length {
        let row = i / sq_root_rounded;
        let col = i % sq_root_rounded;
        data_matrix[row][col] = data[i].clone();
    }
    let data_matrix = data_matrix
        .as_slice()
        .iter()
        .map(|a| a.as_slice())
        .collect::<&[&[Vec<u8>]]>();

    // horizontally encode each row
    for mut row in data_matrix {
        let r = ReedSolomon::new(sq_root_rounded, sq_root_rounded).unwrap(); // assuming n:k is 1:1
        r.encode(&mut row.as_mut_slice()).unwrap();
    }
}

pub fn decode() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        // make a 2d vec of 23 numbers from 1 to 23
        let mut data = vec![];
        for i in 1..24 {
            data.push(vec![i; 3]);
        }
        encode(data, 3);
    }
}
