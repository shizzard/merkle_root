mod source_reader;

use clap::Parser;
use source_reader::SourceReader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let reader = SourceReader::new(args.file).unwrap();
    let len = reader.collect::<Vec<[u8; 32]>>().len();
    dbg!(len);
}
