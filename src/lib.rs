use std::vec;

use reed_solomon_erasure::*;
use solana_merkle_tree::MerkleTree;
pub type Shard = Vec<u8>;
pub mod macros;

#[derive(Debug)]
pub struct EncodedData {
    pub row_commitments: Vec<Vec<u8>>,
    pub col_commitments: Vec<Vec<u8>>,
    pub final_commitment: Vec<u8>,
}
pub fn encode(data: Vec<Shard>, shard_len: usize) -> EncodedData {
    let data_array_length = data.len();
    let sq_root = (data_array_length as f64).sqrt();

    let mut sq_root_rounded = sq_root.ceil() as usize; // 45

    if sq_root_rounded % 2 != 0 {
        sq_root_rounded += 1; // doing this so we can split data and coding chunks equally
    }

    let mut data_matrix = vec![vec![vec![0; shard_len]; sq_root_rounded]; sq_root_rounded * 2];
    for i in 0..data_array_length {
        // 2000
        let row = i / sq_root_rounded;
        let col = i % sq_root_rounded;
        data_matrix[row][col] = data[i].clone();
    }

    for i in 0..(sq_root_rounded * 2) {
        let mut parity_prefill = vec![vec![0u8; shard_len]; sq_root_rounded];
        data_matrix[i].append(&mut parity_prefill);
    }

    let mut vertical_erasure_swap: Vec<Vec<Shard>> = vec![];
    let data_matrix = data_matrix
        .iter_mut()
        .enumerate()
        .map(|(i, row)| {
            let r = ReedSolomon::new(sq_root_rounded, sq_root_rounded).unwrap(); // assuming n:k is 1:1
                                                                                 // let mut row = row.clone(); // m.erasure_matrix.push(r.clone());

            let mut x: Vec<&mut [u8]> = row.iter_mut().map(|f| f.as_mut_slice()).collect();

            r.encode(&mut x).unwrap();

            if i < sq_root_rounded {
                let y = x.iter().map(|f| f.to_vec()).collect::<Vec<Shard>>();

                vertical_erasure_swap.push(y[sq_root_rounded..y.len()].to_vec().clone());
            }
            return x;
        })
        .collect::<Vec<Vec<&mut [u8]>>>();

    let data_matrix = owned_vec!(data_matrix);
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
        .collect::<Vec<Vec<Shard>>>();

    let data_matrix = data_matrix
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

    let row_commitments = data_matrix
        .iter()
        .map(|row| {
            MerkleTree::new(&row)
                .get_root()
                .unwrap()
                .as_ref()
                .to_owned()
        })
        .collect::<Vec<Vec<u8>>>();

    let data_matrix = owned_vec!(data_matrix);
    let mut col_leaves = vec![];

    for col_index in 0..sq_root_rounded * 2 {
        for row_index in 0..sq_root_rounded * 2 {
            col_leaves.push(data_matrix[row_index][col_index].clone());
        }
    }
    let col_commitments = data_matrix
        .iter()
        .map(|col| {
            MerkleTree::new(&col)
                .get_root()
                .unwrap()
                .as_ref()
                .to_owned()
        })
        .collect::<Vec<Vec<u8>>>();

    let final_commitment =
        MerkleTree::new(&[row_commitments.clone(), col_commitments.clone()].concat())
            .get_root()
            .unwrap()
            .as_ref()
            .to_vec();
    EncodedData {
        row_commitments,
        col_commitments,
        final_commitment,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_encode() {
        // make a 2d vec of 23 numbers from 1 to 23
        let mut original = vec![];
        let data_len = 1560;
        let shard_len = 1280;
        for _ in 0..data_len {
            let mut elem: Shard = Vec::with_capacity(shard_len as usize);
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
