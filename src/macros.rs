#[macro_export]
macro_rules! owned_vec {
    ($data_matrix:expr) => {{
        $data_matrix
            .into_iter()
            .map(|f| f.iter().map(|f| f.to_vec()).collect::<Vec<Shard>>())
            .collect::<Vec<Vec<Shard>>>()
    }};
}
