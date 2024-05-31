use clap::Parser;
use merkle_root::{
    calc::{depth_walk::DepthWalk, hash, MerkleTreeRoot},
    source::SourceReader,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut reader = SourceReader::new(args.file).unwrap().peekable();
    let mut hash = DepthWalk::calculate(&mut reader, &hash);
    let mut buf = [0u8; 64];
    let str = base16ct::lower::encode_str(&mut hash, &mut buf).unwrap();
    dbg!(str);
}
