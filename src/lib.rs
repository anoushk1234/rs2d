use std::vec;

use reed_solomon_erasure::*;

type Shard = Vec<u8>;
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Matrix {
    row_count: usize,
    col_count: usize,
    data: Vec<Vec<Shard>>,
    erasure_matrix: Vec<ReedSolomon>,
}
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

    let mut sq_root_rounded = sq_root.ceil() as usize; // 45
                                                       // print all thre variables above
                                                       // println!(
                                                       //     "data_array_length: {:?} sq_root: {:?} sq_root_rounded: {:?}",
                                                       //     data_array_length, sq_root, sq_root_rounded
                                                       // );
    if sq_root_rounded % 2 != 0 {
        sq_root_rounded += 1;
    }
    let mut data_matrix = vec![vec![vec![0; shard_len]; sq_root_rounded]; sq_root_rounded];
    for i in 0..data_array_length {
        // 2000
        let row = i / sq_root_rounded;
        let col = i % sq_root_rounded;
        data_matrix[row][col] = data[i].clone();
    }
    for i in 0..sq_root_rounded {
        let mut parity_prefill = vec![vec![0u8; shard_len]; sq_root_rounded];
        data_matrix[i].append(&mut parity_prefill);
    }

    // let mut m = Matrix::default();
    // m.data = data_matrix.clone();
    // m.row_count = sq_root_rounded;
    // m.col_count = sq_root_rounded;
    // println!("data_matrix: {:?}", data_matrix[data_matrix.len() - 1]);
    // let mut new_data_matrix: Vec<Vec<&mut [u8]>> = vec![];
    println!("data_matrix1: {:?}", data_matrix);
    let data_matrix = data_matrix
        .iter_mut()
        .map(|row| {
            let r = ReedSolomon::new(sq_root_rounded, sq_root_rounded).unwrap(); // assuming n:k is 1:1
                                                                                 // let mut row = row.clone(); // m.erasure_matrix.push(r.clone());

            let mut x: Vec<&mut [u8]> = row.iter_mut().map(|f| f.as_mut_slice()).collect();

            r.encode(&mut x).unwrap();
            return x;
        })
        .collect::<Vec<Vec<&mut [u8]>>>();
    println!("data_matrix2: {:?}", data_matrix);
    // for i in 0..sq_root_rounded {
    //   let next_layer = vec![vec![0u8; shard_len]; sq_root_rounded];
    // }
}

pub fn decode() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_encode() {
        // make a 2d vec of 23 numbers from 1 to 23
        let mut original = vec![];
        let data_len = 9;
        let shard_len = 10;
        for _ in 0..data_len {
            let mut elem: Vec<u8> = Vec::with_capacity(shard_len as usize);
            for _ in 0..shard_len {
                elem.push(0);
            }
            // let mut elem = elem.into_boxed_slice();
            for i in 0..shard_len {
                elem[i as usize] = rand::thread_rng().gen::<u8>()
            }
            original.push(elem.clone());
        }
        encode(original, shard_len);
    }
}
