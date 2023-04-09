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

    if sq_root_rounded % 2 != 0 {
        sq_root_rounded += 1; // doing this so we can split data and coding chunks equally
    }
    println!("sq_root_rounded: {}", sq_root_rounded);
    let mut data_matrix = vec![vec![vec![0; shard_len]; sq_root_rounded]; sq_root_rounded * 2];
    for i in 0..data_array_length {
        // 2000
        let row = i / sq_root_rounded;
        let col = i % sq_root_rounded;
        data_matrix[row][col] = data[i].clone();
    }
    println!(
        "data_matrix: {:?} len: {:?}",
        data_matrix,
        data_matrix.len()
    );
    for i in 0..(sq_root_rounded * 2) {
        let mut parity_prefill = vec![vec![0u8; shard_len]; sq_root_rounded];
        data_matrix[i].append(&mut parity_prefill);
    }
    println!(
        "data_matrix: {:?} len: {:?}",
        data_matrix,
        data_matrix.len()
    );
    let mut vertical_erasure_swap: Vec<Vec<Vec<u8>>> = vec![];
    let mut data_matrix = data_matrix
        .iter_mut()
        .enumerate()
        .map(|(i, row)| {
            let r = ReedSolomon::new(sq_root_rounded, sq_root_rounded).unwrap(); // assuming n:k is 1:1
                                                                                 // let mut row = row.clone(); // m.erasure_matrix.push(r.clone());

            let mut x: Vec<&mut [u8]> = row.iter_mut().map(|f| f.as_mut_slice()).collect();

            r.encode(&mut x).unwrap();

            if i < sq_root_rounded {
                let y = x.iter().map(|f| f.to_vec()).collect::<Vec<Vec<u8>>>();

                vertical_erasure_swap.push(y[sq_root_rounded..y.len()].to_vec().clone());
            }
            return x;
        })
        .collect::<Vec<Vec<&mut [u8]>>>();
    // println!(
    //     "data_matrix: {:?} vs: {:?}",
    //     data_matrix, vertical_erasure_swap[0][0]
    // );
    let data_matrix = data_matrix
        .into_iter()
        .map(|f| {
            let x = f.iter().map(|f| f.to_vec()).collect::<Vec<Vec<u8>>>();
            return x;
        })
        .collect::<Vec<Vec<Vec<u8>>>>();
    let mut data_matrix = data_matrix
        .into_iter()
        .enumerate()
        .map(|(i, mut row)| {
            if i < sq_root_rounded {
                return row;
            } else {
                for item_index in 0..row.len() {
                    if item_index < sq_root_rounded {
                        row[item_index] =
                            vertical_erasure_swap[i - sq_root_rounded][item_index].clone();
                    }
                }
                return row;
            }
        })
        .collect::<Vec<Vec<Vec<u8>>>>();
    println!(
        "data_matrix 3: {:?} len: {:?}",
        data_matrix,
        data_matrix.len()
    );
    let mut data_matrix = data_matrix
        .iter_mut()
        .enumerate()
        .map(|(i, row)| {
            if i >= sq_root_rounded {
                let r = ReedSolomon::new(sq_root_rounded, sq_root_rounded).unwrap(); // assuming n:k is 1:1
                                                                                     // let mut row = row.clone(); // m.erasure_matrix.push(r.clone());

                let mut x: Vec<&mut [u8]> = row.iter_mut().map(|f| f.as_mut_slice()).collect();

                r.encode(&mut x).unwrap();

                return x;
            } else {
                return row
                    .iter_mut()
                    .map(|f| f.as_mut_slice())
                    .collect::<Vec<&mut [u8]>>();
            }
        })
        .collect::<Vec<Vec<&mut [u8]>>>();
    println!(
        "data_matrix 4: {:?} len: {:?}",
        data_matrix,
        data_matrix.len()
    );

    // let data_matrix = data_matrix[sq_root_rounded..]
    //     .to_vec()
    //     .into_iter()
    //     .map(|row| {
    //         let x = &row[sq_root_rounded..].into_iter().map(f);
    //         return x;
    //     })
    //     .collect::<Vec<Vec<&mut [u8]>>>();
    // for i in 0..sq_root_rounded {
    //     let mut parity_prefill = vec![vec![0u8; shard_len]; sq_root_rounded * 2];
    //     let mut x = parity_prefill
    //         .clone()
    //         .iter_mut()
    //         .map(|f| f.as_mut_slice())
    //         .collect();
    //
    // }
    // for i in 0..sq_root_rounded {
    //     data_matrix[i].append(&mut vec![
    //         vec![0u8; shard_len].as_mut_slice();
    //         sq_root_rounded
    //     ]);
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
