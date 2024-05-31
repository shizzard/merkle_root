use merkle_root::calc::depth_walk::DepthWalk;
use merkle_root::calc::{hash, MerkleTreeRoot};
use merkle_root::source::SourceReader;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn depth_walk(c: &mut Criterion) {
    c.bench_function("depth_walk", |b| {
        b.iter(|| {
            let source_file = String::from("input.txt");
            let mut source = SourceReader::new(source_file)
                .expect("input.txt")
                .peekable();
            DepthWalk::calculate(&mut source, &hash)
        })
    });
}

criterion_group!(benches, depth_walk);
criterion_main!(benches);
