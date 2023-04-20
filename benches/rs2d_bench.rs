use rand::Rng;

use {glassbench::*, rs2d::*};

fn benchmark(b: &mut Bench) {
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
    b.task(
        "Encoding - block size: 2.56MB, shard size: 1280 bytes",
        |task| {
            task.iter(|| encode(original.clone(), shard_len));
        },
    );
}

glassbench!("Benchmark for erasure coding in 2d Reed Solomon", benchmark,);
