use clap::{Parser, ValueEnum};
use merkle_root::calc::{depth_walk::DepthWalk, hash, width_walk::WidthWalk, MerkleTreeRoot};
use merkle_root::source::SourceReader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file, containing one base16 sha256 hash per line
    #[arg(short, long)]
    file: String,
    /// Calculation mode (default: depth-walk)
    #[arg(short, long, value_enum)]
    mode: Option<Mode>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Depth-walk algorithm: time O(n*log(n)), space O(log(n))
    DepthWalk,
    /// Width-walk algorithm: time *O(n*log(n)), space O(n*log(n))
    WidthWalk,
}

fn main() {
    let args = Args::parse();
    let mut reader = SourceReader::new(args.file).unwrap().peekable();
    let mut hash = match args.mode {
        Some(Mode::DepthWalk) | None => DepthWalk::calculate(&mut reader, &hash),
        Some(Mode::WidthWalk) => WidthWalk::calculate(&mut reader, &hash),
    };
    let mut buf = [0u8; 64];
    let root = base16ct::lower::encode_str(&mut hash, &mut buf).unwrap();
    println!("{root}");
}
